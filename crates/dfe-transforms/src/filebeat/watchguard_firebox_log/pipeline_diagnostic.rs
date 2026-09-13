// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_diagnostic` pipeline.
pub struct PipelineDiagnostic;

impl Transform for PipelineDiagnostic {
    fn name(&self) -> &str {
        "pipeline_diagnostic"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0029", "3000-002A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IP address ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-003C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Route look up on HTTP redirect host ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for policy \"") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for policy \"") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("\" failed, local redirect may not work") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.policy_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\" failed, local redirect may not work") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0040"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Idle timeout has occurred for blocked site ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-0065"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" used ") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" used ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.quota_info", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-012D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Verify ARP entry for host at ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3000-012E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^Cannot relearn system MAC address, possible loop or MAC spoofing, ip=%{IP:watchguard_firebox.log.ip_address}, mac=%{MAC:watchguard_firebox.log.mac}, interface=%{NUMBER:watchguard_firebox.log.interface_id}$
                    if !cached_grok!("^Cannot relearn system MAC address, possible loop or MAC spoofing, ip=%{IP:watchguard_firebox.log.ip_address}, mac=%{MAC:watchguard_firebox.log.mac}, interface=%{NUMBER:watchguard_firebox.log.interface_id}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Initiating GARP for ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.dev_name", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-000F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Adding bridge ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.dev_name", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0030"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] Sending interface status event, logical=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.dev_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] Sending interface status event, logical=") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" link=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.logical", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" link=") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ip=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.link", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ip=") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" mask=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" mask=") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.mask", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0031"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] Sending interface status event for link ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.dev_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] Sending interface status event for link ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.link", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0034"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
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
                        let Some(pos) = remaining.find(")] External Interface ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")] External Interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" IP address") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.operation", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" IP address") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0035"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
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
                        let Some(pos) = remaining.find(")] Ignoring unknown address operation ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")] Ignoring unknown address operation ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.operation", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0038"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[Cluster] Traffic signal become ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.status", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0036", "3100-0037", "3100-003D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[Cluster] ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" role ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" role ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.cluster_role", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-004F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[ECMP] Fix up ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" multipath gateway successfully") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.num", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" multipath gateway successfully") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-005B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Updating ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" secondary IP (s) setting") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" secondary IP (s) setting") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3100-0070"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[Cluster] Clean up stale IP connections with expired address ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for PPPoE interface ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for PPPoE interface ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.dev_name", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3113-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Capture stopped, ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Response from server: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.response", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.return_code", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Resolved domain ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.dns_ip_address", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0003", "5A00-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" to: ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" / ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.server_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" / ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.server_ip", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Activating DynDNS on interface: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.interface_name", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Could not resolve server: ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.server_name", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Could not connect to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" / ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.server_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" / ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.server_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0009"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" server: ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" server: ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" / ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.server_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" / ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.server_ip", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-000A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" server ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" server ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" / ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.server_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" / ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.server_ip", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-000B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Invalid response from server (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.return_code", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-000C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Next update is on ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.next_update_time", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-000D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Sending update request (") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" bytes): ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.bytes", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" bytes): ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.content", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-001C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^USB auto restore failed %{GREEDYDATA:watchguard_firebox.log.reason}$
                    // Grok pattern: ^USB auto restore failed due to %{GREEDYDATA:watchguard_firebox.log.reason}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^USB auto restore failed %{GREEDYDATA:watchguard_firebox.log.reason}$"),
                            cached_grok!("^USB auto restore failed due to %{GREEDYDATA:watchguard_firebox.log.reason}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5A00-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^Received reply: %{WORD:watchguard_firebox.log.reply_protocol}/%{NUMBER:watchguard_firebox.log.http_version} %{NUMBER:watchguard_firebox.log.http_status:long} %{DATA} Date: %{DATA:watchguard_firebox.log.reply_time} Server:%{DATA} %{IP:watchguard_firebox.log.reply_ip}$
                    if !cached_grok!("^Received reply: %{WORD:watchguard_firebox.log.reply_protocol}/%{NUMBER:watchguard_firebox.log.http_version} %{NUMBER:watchguard_firebox.log.http_status:long} %{DATA} Date: %{DATA:watchguard_firebox.log.reply_time} Server:%{DATA} %{IP:watchguard_firebox.log.reply_ip}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-000C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
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
                        let Some(pos) = remaining.find(" image failed ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.image_source", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" image failed ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-000D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Creation of USB auto restore image failed ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("USB drive format operation was ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.result", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0014"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" system diagnostic file to ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" system diagnostic file to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-0017"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" system diagnostic file to ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" system diagnostic file to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" successfully") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.device", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" successfully") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5501-001B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("System backup to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_device", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3A00-000A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" Virtual Router with cluster ID ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Virtual Router with cluster ID ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" changed state to master due to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.cluster_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" changed state to master due to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" second notification gap from current master with IP ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.notification_gap_duration", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" second notification gap from current master with IP ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^A DHCP server is interfering with static address assignment of cluster IP address %{IP:watchguard_firebox.log.ip_address} on eth%{NUMBER:watchguard_firebox.log.port}. Disable DHCP server access to eth%{NUMBER:watchguard_firebox.log.port}.
                    if !cached_grok!("^A DHCP server is interfering with static address assignment of cluster IP address %{IP:watchguard_firebox.log.ip_address} on eth%{NUMBER:watchguard_firebox.log.port}. Disable DHCP server access to eth%{NUMBER:watchguard_firebox.log.port}.").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-025C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" received updated configuration; version ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" received updated configuration; version ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.version", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3B00-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster channel from member ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to master is ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.member_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to master is ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.state", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3A00-0004", "3A00-0005", "3A00-0006", "3A00-0007", "3A00-0008", "3A00-000B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["3800-0004", "3B00-0002", "3800-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Cluster interface ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.interface_name", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0002", "0203-0003", "0203-0004", "0203-0005", "0203-0006", "0203-0007", "0203-0008", "0203-0009", "0203-000A", "0203-000B", "0203-000C", "0203-000D", "0203-000E", "0203-000F", "0203-0010", "0203-0011", "0203-0012", "0203-0015", "0203-0017", "0203-0018", "0203-0019", "0203-0020", "0203-0026", "0203-0027", "0203-0028", "0203-0029", "0203-002A", "0203-002B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKE phase-1 negotiation from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0002", "0205-0003", "0205-0004", "0205-0005", "0205-0006", "0205-0007", "0205-0008", "0205-000A", "0205-000B", "0205-000C", "0205-000D", "0205-000E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKE phase-2 negotiation from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0021", "0203-0022", "0203-0023", "0203-0024"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Remote gateway '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' with IP ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' with IP ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0001", "021A-0002", "021A-0003", "021A-0004", "021A-0005", "021A-001C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dropped IKEv2 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(". ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(". ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0006", "021A-0007", "021A-0008", "021A-0009", "021A-0019", "021A-001A", "021A-000C", "021A-000D", "021A-0016", "021A-000A", "021A-000B", "021A-000F", "021A-0010", "021A-0015", "021A-0012", "021A-0013", "021A-0014", "021A-0018", "021A-0011", "021A-001E", "021A-001F", "021A-0020"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" exchange from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" exchange from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0200-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Could not read ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" certificate with [") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.certificate_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" certificate with [") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("] ID") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.certificate_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] ID") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0202-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Virtual IP address from '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' address pool is not available for Mobile VPN with IPSec user '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.pool_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' address pool is not available for Mobile VPN with IPSec user '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0002", "0203-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' exchange type. ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' exchange type. ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received DH group ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received DH group ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_value", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected_value", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0208-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" phase-1 ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" phase-1 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" completed successfully as ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.negotiation_mode", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" completed successfully as ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.negotiation_role", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' gateway endpoint. localgw:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' gateway endpoint. localgw:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" remotegw:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" remotegw:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" SA ID:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" SA ID:") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.p1_sa_id", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received hash ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received hash ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-000A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Received message on wrong interface '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Received message on wrong interface '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' (index:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' (index:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("). Expecting it to be received on '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_interface_index", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("). Expecting it to be received on '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.expected_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0001", "021A-0006", "0203-000F", "0203-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Reason=") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0002", "0205-000B", "0205-000C", "0205-000D", "0205-000E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received encryption ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received encryption ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received authentication method ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received authentication method ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received AES key length ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received AES key length ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_value", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected_value", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0027"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Failed to get ID information from certificate ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Failed to get ID information from certificate ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.certificate_id", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0028"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Reason=Received IKE message on wrong interface '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'(index:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'(index:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("). Expecting it to be received on '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_interface_index", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("). Expecting it to be received on '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.expected_interface", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-002B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=Received message with wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$
                    if !cached_grok!("^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=Received message with wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0013"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKE phase-1 negotiation from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.source_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. - ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.destination_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. - ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001B"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 exchange from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. GatewayEndpoint='") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=No response for ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=No response for ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message. Check the connection between the local and remote gateway endpoints.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.msg_info", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message. Check the connection between the local and remote gateway endpoints.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0014"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Received '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' message from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.info_msg", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' message from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' gateway endpoint. Check VPN IKE diagnostic log messages on the remote gateway endpoint for more information.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' gateway endpoint. Check VPN IKE diagnostic log messages on the remote gateway endpoint for more information.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received protocol '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received protocol '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Expecting '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_proto", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Expecting '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' in phase-2 proposal.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.expected_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' in phase-2 proposal.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received AH authentication ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received AH authentication ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received ESP encryption ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received ESP encryption ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received PFS DH group ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received PFS DH group ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_value", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected_value", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received ESP authentication ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received ESP authentication ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0008"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=Received AES key length ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=Received AES key length ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_value", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.expected_value", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-000A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Gateway='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' Reason=No matching tunnel route for peer proposed local:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' Reason=No matching tunnel route for peer proposed local:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" remote:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tr_local", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" remote:") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.tr_remote", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0021", "0203-0023"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" message. ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message. ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" retries left") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.retry_count", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" retries left") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0022", "0203-0024"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("presumed dead due to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failure. ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failure. ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.action", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0016"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Rejected MUVPN IPSec user from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" because maximum allowed user connections has been reached. Maximum:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" because maximum allowed user connections has been reached. Maximum:") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.max_user_connection", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-000F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Rejected ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" because '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" because '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' is not the preferred IKE gateway endpoint.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' is not the preferred IKE gateway endpoint.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0025"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Received IKE message from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for unknown P1 SA. Sending delete message to remote gateway '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for unknown P1 SA. Sending delete message to remote gateway '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0010"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Received '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' message from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.info_msg", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' message from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' tunnel. Check VPN IKE diagnostic log messages on the remote gateway endpoint for more information.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' tunnel. Check VPN IKE diagnostic log messages on the remote gateway endpoint for more information.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0205-0011"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Dropped a simultaneous phase-2 negotiation from the peer ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.peer_address_port", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Received XAuth failed notification from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(". Group:'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(". Group:'") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0002", "0206-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Rejected phase-1 authentication method ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.authentication_method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("XAuth negotiation from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed due to a mismatched XAuthMode.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed due to a mismatched XAuthMode.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("MUVPN user authentication failed due to unresponsive peer at ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.peer_address_port", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0006"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("MUVPN user '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' is authenticated without group information.") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' is authenticated without group information.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0206-0007"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("MUVPN user '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' is a member of '") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' is a member of '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' group.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.group_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' group.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0004"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=IKE SA is in ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=IKE SA is in ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" state.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.ikev2_ikesa_state", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" state.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001C"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Gateway-Endpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Waiting for the ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Waiting for the ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" user authentication result.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.user_auth_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" user authentication result.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0002"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Reason=IKE SA not found to handle message with message ID ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_message_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0003"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Reason='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' gateway endpoint not found to handle message with message ID ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' gateway endpoint not found to handle message with message ID ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received_message_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Invalid message ID in ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Invalid message ID in ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.req_or_resp", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0010", "021A-000F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=DH-Group %{NUMBER:watchguard_firebox.log.received_dh_group:long} in the KE payload does not match DH-Group %{NUMBER:watchguard_firebox.log.selected_dh_group:long} selected in the %{DATA:watchguard_firebox.log.msg_info} proposal.$
                    // Grok pattern: ^Tunnel='%{DATA:watchguard_firebox.log.tunnel_name}'. Reason=DH-Group %{NUMBER:watchguard_firebox.log.received_dh_group:long} in the KE payload does not match DH-Group %{NUMBER:watchguard_firebox.log.selected_dh_group:long} selected in the %{DATA:watchguard_firebox.log.msg_info} proposal.$
                    if !extract_first_match(
                        &[
                            cached_grok!("^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=DH-Group %{NUMBER:watchguard_firebox.log.received_dh_group:long} in the KE payload does not match DH-Group %{NUMBER:watchguard_firebox.log.selected_dh_group:long} selected in the %{DATA:watchguard_firebox.log.msg_info} proposal.$"),
                            cached_grok!("^Tunnel='%{DATA:watchguard_firebox.log.tunnel_name}'. Reason=DH-Group %{NUMBER:watchguard_firebox.log.received_dh_group:long} in the KE payload does not match DH-Group %{NUMBER:watchguard_firebox.log.selected_dh_group:long} selected in the %{DATA:watchguard_firebox.log.msg_info} proposal.$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0011"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Received unacceptable traffic selector in ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Received unacceptable traffic selector in ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.msg_info", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0012"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Received authentication method ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Received authentication method ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expecting ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.received", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expecting ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.expected", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0016"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Received ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Received ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.notify_msg", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0015"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Received ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Received ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" message.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.notify_msg", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" message.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0019"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Peer proposed invalid SPI in ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Peer proposed invalid SPI in ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.msg_info", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001A"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=Could not find child SA by received SPI ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=Could not find child SA by received SPI ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" in ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.spi", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" in ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.msg_info", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["2500-0000", "2500-0001"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.vpn_user_type} (?:%{USERNAME:watchguard_firebox.log.user_name}|%{EMAILADDRESS:watchguard_firebox.log.user_email}) logged in. Virtual IP address is %{IP:watchguard_firebox.log.virtual_ip_address}. Real IP address is %{IP:watchguard_firebox.log.real_ip_address}.$
                    // Grok pattern: ^%{DATA:watchguard_firebox.log.vpn_user_type} (?:%{USERNAME:watchguard_firebox.log.user_name}|%{EMAILADDRESS:watchguard_firebox.log.user_email}) logged off. Virtual IP address is %{IP:watchguard_firebox.log.virtual_ip_address}.$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:watchguard_firebox.log.vpn_user_type} (?:%{USERNAME:watchguard_firebox.log.user_name}|%{EMAILADDRESS:watchguard_firebox.log.user_email}) logged in. Virtual IP address is %{IP:watchguard_firebox.log.virtual_ip_address}. Real IP address is %{IP:watchguard_firebox.log.real_ip_address}.$"),
                            cached_grok!("^%{DATA:watchguard_firebox.log.vpn_user_type} (?:%{USERNAME:watchguard_firebox.log.user_name}|%{EMAILADDRESS:watchguard_firebox.log.user_email}) logged off. Virtual IP address is %{IP:watchguard_firebox.log.virtual_ip_address}.$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001F"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=Received message with wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$
                    if !cached_grok!("^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=Received message with wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0020"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=Received message with the wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$
                    if !cached_grok!("^Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=Received message with the wrong interface IP address %{IP:watchguard_firebox.log.received_ip}. Expecting peer to use remote gateway endpoint IP address %{IP:watchguard_firebox.log.expected_ip}.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0013"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^(?:GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'.)? Reason=Remote gateway endpoint %{DATA:watchguard_firebox.log.authentication_method} authentication failed.$
                    if !cached_grok!("^(?:GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'.)? Reason=Remote gateway endpoint %{DATA:watchguard_firebox.log.authentication_method} authentication failed.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0017"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 IKE SA established successfully as ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for '") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_role", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("' gateway endpoint. local-gw:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' gateway endpoint. local-gw:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" remote-gw:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" remote-gw:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" SA ID:") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" SA ID:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(".") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.sa_id", &remaining[..pos]));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["0203-0017", "0203-0018", "0203-0019", "0203-0009", "0203-000A", "0203-000C", "0203-000B", "0203-000D", "0203-000E", "0203-0011", "0203-0012", "0203-0015", "0203-0020", "0203-0026", "0203-0029", "0203-002A", "021A-0007", "021A-0008", "021A-0009", "021A-000B", "021A-000C", "021A-000D", "021A-0014"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_tmp_msg") {
                    // Grok pattern: ^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=%{GREEDYDATA:watchguard_firebox.log.reason}
                    // Grok pattern: ^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=%{GREEDYDATA:watchguard_firebox.log.reason}
                    // Grok pattern: ^Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=%{GREEDYDATA:watchguard_firebox.log.reason}
                    if !extract_first_match(
                        &[
                            cached_grok!("^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=%{GREEDYDATA:watchguard_firebox.log.reason}"),
                            cached_grok!("^GatewayEndpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=%{GREEDYDATA:watchguard_firebox.log.reason}"),
                            cached_grok!("^Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}' Reason=%{GREEDYDATA:watchguard_firebox.log.reason}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["5B01-0004", "5B01-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" user '") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" user '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("', virtual IP address '") else { break 'dissect false };
                        captured.push(("_tmp_user", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("', virtual IP address '") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'.") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.virtual_ip_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'.") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" EAP exchange from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" EAP exchange from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. Gateway-Endpoint='") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. Gateway-Endpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=") else { break 'dissect false };
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-000E"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" exchange from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" exchange from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. GatewayEndpoint='") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. GatewayEndpoint='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.gateway_endpoint", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.msg_info", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-0018"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("IKEv2 ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" exchange from ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.exchange_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" exchange from ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" to ") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.local_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" to ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" failed. Tunnel='") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.peer_address_port", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" failed. Tunnel='") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("'. Reason=") else { break 'dissect false };
                        captured.push(("watchguard_firebox.log.tunnel_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("'. Reason=") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("watchguard_firebox.log.msg_info", remaining));
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

            let _cond = { event.has_value("watchguard_firebox.log.msg_id") && ["021A-001D"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^IKEv2 %{DATA:watchguard_firebox.log.exchange_type} exchange from %{IP:watchguard_firebox.log.local_address}:?(%{NUMBER:watchguard_firebox.log.local_address_port:long})? to %{IP:watchguard_firebox.log.peer_address}:?(%{NUMBER:watchguard_firebox.log.peer_address_port:long})? failed. Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=The Mobile VPN with IKEv2 profile is not enabled.$
                    if !cached_grok!("^IKEv2 %{DATA:watchguard_firebox.log.exchange_type} exchange from %{IP:watchguard_firebox.log.local_address}:?(%{NUMBER:watchguard_firebox.log.local_address_port:long})? to %{IP:watchguard_firebox.log.peer_address}:?(%{NUMBER:watchguard_firebox.log.peer_address_port:long})? failed. Gateway-Endpoint='%{DATA:watchguard_firebox.log.gateway_endpoint}'. Reason=The Mobile VPN with IKEv2 profile is not enabled.$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
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
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}@%{HOSTNAME:watchguard_firebox.log.user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.bytes") {
                if let Some(val) = event.get("watchguard_firebox.log.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.bytes".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bytes_to_long")?;
                        if event.remove("watchguard_firebox.log.bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.bytes".into() });
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
            if event.has_value("watchguard_firebox.log.expected_value") {
                if let Some(val) = event.get("watchguard_firebox.log.expected_value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.expected_value".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.expected_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_expected_value_to_long")?;
                        if event.remove("watchguard_firebox.log.expected_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.expected_value".into() });
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
            if event.has_value("watchguard_firebox.log.received_value") {
                if let Some(val) = event.get("watchguard_firebox.log.received_value") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.received_value".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.received_value", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_received_value_to_long")?;
                        if event.remove("watchguard_firebox.log.received_value").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.received_value".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
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

            let _cond = { event.get_str("watchguard_firebox.log.expected_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.expected_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.expected_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.expected_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.expected_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_expected_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.expected_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.expected_ip".into() });
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
            if event.has_value("watchguard_firebox.log.http_status") {
                if let Some(val) = event.get("watchguard_firebox.log.http_status") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.http_status".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.http_status", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_http_status_to_long")?;
                        if event.remove("watchguard_firebox.log.http_status").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.http_status".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_ip_address_to_ip")?;
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

            let _cond = { event.get_str("watchguard_firebox.log.dns_ip_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.dns_ip_address") {
                if let Some(val) = event.get("watchguard_firebox.log.dns_ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.dns_ip_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.dns_ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dns_ip_address_to_ip")?;
                        if event.remove("watchguard_firebox.log.dns_ip_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.dns_ip_address".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.local_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.local_address") {
                if let Some(val) = event.get("watchguard_firebox.log.local_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.local_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.local_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_local_addr_to_ip")?;
                        if event.remove("watchguard_firebox.log.local_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.local_address".into() });
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
            if event.has_value("watchguard_firebox.log.local_address_port") {
                if let Some(val) = event.get("watchguard_firebox.log.local_address_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.local_address_port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.local_address_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_local_addr_port_to_long")?;
                        if event.remove("watchguard_firebox.log.local_address_port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.local_address_port".into() });
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
            if event.has_value("watchguard_firebox.log.max_user_connection") {
                if let Some(val) = event.get("watchguard_firebox.log.max_user_connection") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.max_user_connection".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.max_user_connection", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_user_connection_to_long")?;
                        if event.remove("watchguard_firebox.log.max_user_connection").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.max_user_connection".into() });
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
            if event.has_value("watchguard_firebox.log.notification_gap_duration") {
                if let Some(val) = event.get("watchguard_firebox.log.notification_gap_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.notification_gap_duration".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.notification_gap_duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_notification_gap_duration_to_long")?;
                        if event.remove("watchguard_firebox.log.notification_gap_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.notification_gap_duration".into() });
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
            if event.has_value("watchguard_firebox.log.num") {
                if let Some(val) = event.get("watchguard_firebox.log.num") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.num".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.num", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_num_to_long")?;
                        if event.remove("watchguard_firebox.log.num").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.num".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_peer_address_to_ip")?;
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
            if event.has_value("watchguard_firebox.log.peer_address_port") {
                if let Some(val) = event.get("watchguard_firebox.log.peer_address_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.peer_address_port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.peer_address_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_peer_address_port_to_long")?;
                        if event.remove("watchguard_firebox.log.peer_address_port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.peer_address_port".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.real_ip_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.real_ip_address") {
                if let Some(val) = event.get("watchguard_firebox.log.real_ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.real_ip_address".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.real_ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_real_ip_address_to_ip")?;
                        if event.remove("watchguard_firebox.log.real_ip_address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.real_ip_address".into() });
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
            if event.has_value("watchguard_firebox.log.received_dh_group") {
                if let Some(val) = event.get("watchguard_firebox.log.received_dh_group") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.received_dh_group".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.received_dh_group", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_recvd_dh_group_to_long")?;
                        if event.remove("watchguard_firebox.log.received_dh_group").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.received_dh_group".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.received_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.received_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.received_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.received_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.received_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_received_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.received_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.received_ip".into() });
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
            if event.has_value("watchguard_firebox.log.retry_count") {
                if let Some(val) = event.get("watchguard_firebox.log.retry_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.retry_count".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.retry_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_retry_count_to_long")?;
                        if event.remove("watchguard_firebox.log.retry_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.retry_count".into() });
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
            if event.has_value("watchguard_firebox.log.return_code") {
                if let Some(val) = event.get("watchguard_firebox.log.return_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.return_code".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.return_code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_return_code_to_long")?;
                        if event.remove("watchguard_firebox.log.return_code").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.return_code".into() });
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
            if event.has_value("watchguard_firebox.log.selected_dh_group") {
                if let Some(val) = event.get("watchguard_firebox.log.selected_dh_group") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.selected_dh_group".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.selected_dh_group", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_selected_dh_group_to_long")?;
                        if event.remove("watchguard_firebox.log.selected_dh_group").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.selected_dh_group".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("watchguard_firebox.log.server_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.server_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.server_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.server_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.server_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.server_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.server_ip".into() });
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
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
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
                event.set("_ingest.on_failure_processor_tag", "convert_virtual_ip_address_to_ip")?;
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

            let _cond = { event.get_str("watchguard_firebox.log.reply_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.reply_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.reply_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.reply_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.reply_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_log_reply_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.reply_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.reply_ip".into() });
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

            let _cond = { event.get_str("watchguard_firebox.log.mask") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.mask") {
                if let Some(val) = event.get("watchguard_firebox.log.mask") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.mask".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.mask", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_log_mask_to_ip")?;
                        if event.remove("watchguard_firebox.log.mask").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.mask".into() });
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

            if let Some(v) = event.get("watchguard_firebox.log.reply_protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.http_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.version", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.http_status").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.status_code", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.body").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.bytes", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.dev_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.server_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.name", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.mask") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.mask").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dns_ip_address") };
            if _cond {
                event.append_unique("dns.resolved_ip", json!(event.get("watchguard_firebox.log.dns_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dns_ip_address") };
            if _cond {
            event.set("dns.type", json!("answer"))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.server_ip") };
            if _cond {
                event.append_unique("dns.resolved_ip", json!(event.get("watchguard_firebox.log.server_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dns_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.dns_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.server_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.server_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.reply_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.reply_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.group_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.group.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.interface_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.id", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.interface_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.alias", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.authentication_server").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.user_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.local_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.real_ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.policy_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
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

            if let Some(v) = event.get("watchguard_firebox.log.user_email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            if event.has_value("watchguard_firebox.log.mac") {
                gsub_field(event, "watchguard_firebox.log.mac", "watchguard_firebox.log.mac", cached_regex!(":"), "-")?;
            }

            if event.has_value("watchguard_firebox.log.mac") {
                map_strings(event, "watchguard_firebox.log.mac", "watchguard_firebox.log.mac", str::to_uppercase)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.mac") };
            if _cond {
                event.append_unique("observer.mac", json!(event.get("watchguard_firebox.log.mac").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.destination_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.peer_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.peer_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.virtual_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.virtual_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dns_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.dns_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.server_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.server_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.local_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.local_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.real_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.real_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.expected_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.expected_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.source_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.received_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.received_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.server_name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.server_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_email") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.user_email").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.next_update_time") && event.get_str("watchguard_firebox.log.next_update_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.next_update_time") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.next_update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.next_update_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_next_update_time")?;
                        if event.remove("watchguard_firebox.log.next_update_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.next_update_time".into() });
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

            let _cond = { event.has_value("watchguard_firebox.log.reply_time") && event.get_str("watchguard_firebox.log.reply_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("watchguard_firebox.log.reply_time") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss zzz"], None, None) {
                        Some(parsed) => event.set("watchguard_firebox.log.reply_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "watchguard_firebox.log.reply_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_set_reply_time")?;
                        if event.remove("watchguard_firebox.log.reply_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.reply_time".into() });
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
                event.remove("watchguard_firebox.log.bytes");
                event.remove("watchguard_firebox.log.destination_ip");
                event.remove("watchguard_firebox.log.destination_port");
                event.remove("watchguard_firebox.log.dev_name");
                event.remove("watchguard_firebox.log.domain");
                event.remove("watchguard_firebox.log.http_status");
                event.remove("watchguard_firebox.log.http_version");
                event.remove("watchguard_firebox.log.group_name");
                event.remove("watchguard_firebox.log.interface_id");
                event.remove("watchguard_firebox.log.interface_name");
                event.remove("watchguard_firebox.log.local_address");
                event.remove("watchguard_firebox.log.mac");
                event.remove("watchguard_firebox.log.peer_address");
                event.remove("watchguard_firebox.log.policy_name");
                event.remove("watchguard_firebox.log.real_ip_address");
                event.remove("watchguard_firebox.log.reason");
                event.remove("watchguard_firebox.log.reply_protocol");
                event.remove("watchguard_firebox.log.server_name");
                event.remove("watchguard_firebox.log.source_ip");
                event.remove("watchguard_firebox.log.source_port");
                event.remove("watchguard_firebox.log.user_email");
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
