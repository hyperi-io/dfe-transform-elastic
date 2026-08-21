// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `firewall_match` pipeline.
pub struct FirewallMatch;

impl Transform for FirewallMatch {
    fn name(&self) -> &str {
        "firewall_match"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.action", json!("firewall_match_event"))?;

            event.append("event.type", json!("start"))?;
            event.append("event.type", json!("connection"))?;

            let _cond = {
                event.has_value("crowdstrike.event.RuleAction")
                    && event.get_str("crowdstrike.event.RuleAction") == Some("1")
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.RuleAction")
                    && event.get_str("crowdstrike.event.RuleAction") == Some("1")
            };
            if _cond {
                event.set("_tmp_.action", json!("Allowed"))?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.RuleAction")
                    && event.get_str("crowdstrike.event.RuleAction") == Some("2")
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.RuleAction")
                    && event.get_str("crowdstrike.event.RuleAction") == Some("2")
            };
            if _cond {
                event.set("_tmp_.action", json!("Blocked"))?;
            }

            let _cond = { !event.has_value("_tmp_.action") };
            if _cond {
                event.set("_tmp_.action", json!("Unknown"))?;
            }

            let _cond = { event.has_value("crowdstrike.event.RuleName") };
            if _cond {
                event.set(
                    "message",
                    json!(format!(
                        "Firewall Rule: '{}' triggered - Action: '{}'",
                        event
                            .get("crowdstrike.event.RuleName")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_tmp_.action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            if event.has("crowdstrike.event.Ipv") {
                event.rename("crowdstrike.event.Ipv", "network.type")?;
            }

            if event.has_value("crowdstrike.event.PID") {
                if let Some(val) = event.get("crowdstrike.event.PID") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "crowdstrike.event.PID".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "crowdstrike.event.PID".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.PID".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("process.pid", converted)?;
                }
            }

            let v = json!(
                event
                    .get("crowdstrike.event.ImageFileName")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.executable", v)?;
            }

            event.remove("crowdstrike.event.ImageFileName");

            if event.has("crowdstrike.event.RuleId") {
                event.rename("crowdstrike.event.RuleId", "rule.id")?;
            }

            if event.has("crowdstrike.event.RuleName") {
                event.rename("crowdstrike.event.RuleName", "rule.name")?;
            }

            if event.has("crowdstrike.event.RuleGroupName") {
                event.rename("crowdstrike.event.RuleGroupName", "rule.ruleset")?;
            }

            if event.has("crowdstrike.event.RuleDescription") {
                event.rename("crowdstrike.event.RuleDescription", "rule.description")?;
            }

            if event.has("crowdstrike.event.RuleFamilyID") {
                event.rename("crowdstrike.event.RuleFamilyID", "rule.category")?;
            }

            if event.has("crowdstrike.event.HostName") {
                event.rename("crowdstrike.event.HostName", "host.name")?;
            }

            if event.has("crowdstrike.event.EventType") {
                event.rename("crowdstrike.event.EventType", "event.code")?;
            }

            let _cond = { event.has_value("crowdstrike.event.ConnectionDirection") };
            if _cond {
                // Painless script
                // Source: def result = [];\nif (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  result.add('egress');\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"4\") {\n  result.add('unknown');\n}\nif (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\nif (result.size() == 1) {\n  ctx.network.direction = result[0];\n} else if (result.size() > 1) {\n  ctx.network.direction = result;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def result = [];\nif (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  result.add('egress');\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"4\") {\n  result.add('unknown');\n}\nif (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\nif (result.size() == 1) {\n  ctx.network.direction = result[0];\n} else if (result.size() > 1) {\n  ctx.network.direction = result;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.RemoteAddress")
                    && event.get_str("network.direction") == Some("ingress")
            };
            if _cond {
                if event.has("crowdstrike.event.RemoteAddress") {
                    event.rename("crowdstrike.event.RemoteAddress", "source.ip")?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.LocalAddress")
                    && event.get_str("network.direction") == Some("ingress")
            };
            if _cond {
                if event.has("crowdstrike.event.LocalAddress") {
                    event.rename("crowdstrike.event.LocalAddress", "destination.ip")?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.LocalPort")
                    && event.get_str("network.direction") == Some("ingress")
            };
            if _cond {
                if event.has_value("crowdstrike.event.LocalPort") {
                    if let Some(val) = event.get("crowdstrike.event.LocalPort") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.LocalPort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.LocalPort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.event.LocalPort".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.port", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.RemotePort")
                    && event.get_str("network.direction") == Some("ingress")
            };
            if _cond {
                if event.has_value("crowdstrike.event.RemotePort") {
                    if let Some(val) = event.get("crowdstrike.event.RemotePort") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.RemotePort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.RemotePort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.event.RemotePort".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.port", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.RemoteAddress")
                    && event.get_str("network.direction") == Some("egress")
            };
            if _cond {
                if event.has("crowdstrike.event.RemoteAddress") {
                    event.rename("crowdstrike.event.RemoteAddress", "destination.ip")?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.LocalAddress")
                    && event.get_str("network.direction") == Some("egress")
            };
            if _cond {
                if event.has("crowdstrike.event.LocalAddress") {
                    event.rename("crowdstrike.event.LocalAddress", "source.ip")?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.LocalPort")
                    && event.get_str("network.direction") == Some("egress")
            };
            if _cond {
                if event.has_value("crowdstrike.event.LocalPort") {
                    if let Some(val) = event.get("crowdstrike.event.LocalPort") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.LocalPort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.LocalPort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.event.LocalPort".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.port", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.RemotePort")
                    && event.get_str("network.direction") == Some("egress")
            };
            if _cond {
                if event.has_value("crowdstrike.event.RemotePort") {
                    if let Some(val) = event.get("crowdstrike.event.RemotePort") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.RemotePort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "crowdstrike.event.RemotePort".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.event.RemotePort".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.port", converted)?;
                    }
                }
            }

            if event.has("crowdstrike.event.Platform") {
                event.rename("crowdstrike.event.Platform", "host.os.platform")?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
