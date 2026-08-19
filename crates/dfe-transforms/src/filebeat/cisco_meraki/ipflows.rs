// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ipflows` pipeline.
pub struct Ipflows;

impl Transform for Ipflows {
    fn name(&self) -> &str {
        "ipflows"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        if let Some(input) = event.get_string("event.original") {
            let mut remaining: &str = &input;
            if let Some(pos) = remaining.find(" ") {
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                event.set("_temp.event_type", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            event.set("_temp.event", remaining)?;
        }

        let _cond = { event.has("_temp.event") };
        if _cond {
            if let Some(kv_str) = event.get_string("_temp.event") {
                for pair in kv_str.split(" ") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            event.set(key, value)?;
                        }
                    }
                }
            }
        }

        let _cond = { event.has("translated_src_ip") };
        if _cond {
            if let Some(s) = event.get_string("translated_src_ip") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "translated_src_ip".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("source.ip", s)?;
            }
        }

        let _cond = { !event.has("translated_src_ip") && event.has("src") };
        if _cond {
            if let Some(s) = event.get_string("src") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "src".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("source.ip", s)?;
            }
        }

        let _cond = { event.has("translated_src_ip") && event.has("translated_port") };
        if _cond {
            if let Some(val) = event.get("translated_port") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "translated_port".into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path: "translated_port".into(),
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
                            path: "translated_port".into(),
                            message: "cannot convert to integer".into(),
                        });
                    }
                };
                event.set("source.port", converted)?;
            }
        }

        let _cond = { !event.has("translated_src_ip") && event.has("sport") };
        if _cond {
            if let Some(val) = event.get("sport") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "sport".into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path: "sport".into(),
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
                            path: "sport".into(),
                            message: "cannot convert to integer".into(),
                        });
                    }
                };
                event.set("source.port", converted)?;
            }
        }

        let _cond = { event.has("translated_dst_ip") };
        if _cond {
            if let Some(s) = event.get_string("translated_dst_ip") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "translated_dst_ip".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("destination.ip", s)?;
            }
        }

        let _cond = { !event.has("translated_dst_ip") && event.has("dst") };
        if _cond {
            if let Some(s) = event.get_string("dst") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "dst".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("destination.ip", s)?;
            }
        }

        let _cond = { event.has("translated_dst_ip") && event.has("translated_port") };
        if _cond {
            if let Some(val) = event.get("translated_port") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "translated_port".into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path: "translated_port".into(),
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
                            path: "translated_port".into(),
                            message: "cannot convert to integer".into(),
                        });
                    }
                };
                event.set("destination.port", converted)?;
            }
        }

        let _cond = { !event.has("translated_dst_ip") && event.has("dport") };
        if _cond {
            if let Some(val) = event.get("dport") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "dport".into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path: "dport".into(),
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
                            path: "dport".into(),
                            message: "cannot convert to integer".into(),
                        });
                    }
                };
                event.set("destination.port", converted)?;
            }
        }

        event.rename("protocol", "network.protocol")?;

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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
