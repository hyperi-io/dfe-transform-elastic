// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dns` pipeline.
pub struct Dns;

impl Transform for Dns {
    fn name(&self) -> &str {
        "dns"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("query["))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("[") else {
                        break 'dissect false;
                    };
                    captured.push(("dns.op_code", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("[") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find("] ") else {
                        break 'dissect false;
                    };
                    captured.push(("dns.question.type", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("] ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" from ") else {
                        break 'dissect false;
                    };
                    captured.push(("dns.question.name", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" from ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("cached"))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(rest) = remaining.strip_prefix("cached ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" is ") else {
                        break 'dissect false;
                    };
                    captured.push(("dns.question.name", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" is ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("_tmp.dns.resolved_ip", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("forwarded"))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(rest) = remaining.strip_prefix("forwarded ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" to ") else {
                        break 'dissect false;
                    };
                    captured.push(("dns.question.name", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" to ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("_tmp.dns.resolved_ip", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = { event.has_value("dns.question.name") };
        if _cond {
            if let Some(domain_str) = event.get_string("dns.question.name") {
                let domain = domain_str.to_string();
                event.set("dns.question.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("dns.question.registered_domain", json!(registered))?;
                    }
                    event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("dns.question.subdomain", json!(sub))?;
                    }
                }
            }
        }

        let _cond = { event.has_value("dns.op_code") };
        if _cond {
            map_strings(event, "dns.op_code", "dns.op_code", str::to_uppercase)?;
        }

        let _cond = { event.has_value("_tmp.dns.resolved_ip") };
        if _cond {
            event.append(
                "dns.resolved_ip",
                json!(
                    event
                        .get("_tmp.dns.resolved_ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("dns.resolved_ip") };
        if _cond {
            event.set(
                "dns.answers.data",
                json!(
                    event
                        .get("dns.resolved_ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("dns.question.name") && event.has_value("dns.resolved_ip") };
        if _cond {
            event.set(
                "dns.answers.name",
                json!(
                    event
                        .get("dns.question.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        event.append("event.category", json!("network"))?;

        event.append("event.type", json!("protocol"))?;

        event.append("event.type", json!("connection"))?;

        let _cond = { event.has_value("dns.question.domain") };
        if _cond {
            if event.remove("dns.question.domain").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "dns.question.domain".into(),
                });
            }
        }

        Ok(TransformResult::Continue)
    }
}
