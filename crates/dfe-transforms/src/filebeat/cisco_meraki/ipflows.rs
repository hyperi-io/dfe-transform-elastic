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

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:ip_flow_start|ip_flow_end) %{GREEDYDATA:message}
                    if !cached_grok!("(?:ip_flow_start|ip_flow_end) %{GREEDYDATA:message}")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("_temp.event_type", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("_temp.event", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "event.original".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }

            let _cond = { event.has_value("_temp.event") };
            if _cond {
                if let Some(kv_str) = event.get_string("_temp.event") {
                    let mut kv_gap = false;
                    for pair in kv_str.split(" ") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "_temp.event".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, key, value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("src") };
            if _cond {
                if let Some(val) = event.get("src") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "src".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            let _cond = { event.has_value("sport") };
            if _cond {
                if let Some(val) = event.get("sport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "sport".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            let _cond = { event.has_value("translated_src_ip") };
            if _cond {
                if let Some(val) = event.get("translated_src_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "translated_src_ip".into(),
                            message,
                        })?;
                    event.set("source.nat.ip", converted)?;
                }
            }

            let _cond = { event.has_value("translated_port") && event.has_value("source.nat.ip") };
            if _cond {
                if let Some(val) = event.get("translated_port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "translated_port".into(),
                            message,
                        }
                    })?;
                    event.set("source.nat.port", converted)?;
                }
            }

            let _cond = { event.has_value("dst") };
            if _cond {
                if let Some(val) = event.get("dst") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "dst".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            let _cond = { event.has_value("dport") };
            if _cond {
                if let Some(val) = event.get("dport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "dport".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            let _cond = { event.has_value("translated_dst_ip") };
            if _cond {
                if let Some(val) = event.get("translated_dst_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "translated_dst_ip".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }

            let _cond =
                { event.has_value("translated_port") && event.has_value("destination.nat.ip") };
            if _cond {
                if let Some(val) = event.get("translated_port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "translated_port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.nat.port", converted)?;
                }
            }

            event.rename("protocol", "network.protocol")?;

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
