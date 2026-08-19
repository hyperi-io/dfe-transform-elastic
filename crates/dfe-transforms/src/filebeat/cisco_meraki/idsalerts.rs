// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `idsalerts` pipeline.
pub struct Idsalerts;

impl Transform for Idsalerts {
    fn name(&self) -> &str {
        "idsalerts"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                if let Some(pos) = remaining.find(" ids-alerts ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ids-alerts ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("sig", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("sig", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("pri", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("pri", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("ts", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("ts", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("dir", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("dir", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("prot", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("prot", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                event.set("src", remaining)?;
            }

            event.set("cisco_meraki.event_subtype", json!("ids_alerted"))?;

            event.rename("priority", "cisco_meraki.security.priority")?;

            event.rename("signature", "cisco_meraki.security.signature")?;

            event.rename("direction", "network.direction")?;

            if let Some(s) = event.get_string("protocol") {
                let lowered = s.to_lowercase();
                event.set("network.protocol", lowered)?;
            }

            let _cond = { event.has_value("src") };
            if _cond {
                if let Some(input) = event.get_string("src") {
                    // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                    if !cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                        if !cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                            if !cached_grok_mapped!("^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$", [("_temp_src_ip", "_temp.src_ip")]).extract_into(&input, event)? {
                                // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                                if !cached_grok!("^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$").extract_into(&input, event)? {
                                }
                            }
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("_temp.src_ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp.src_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.ip", s)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
