// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cef` pipeline.
pub struct Cef;

impl Transform for Cef {
    fn name(&self) -> &str {
        "cef"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("citrix.cef_format", json!(true))?;

                if let Some(input) = event.get_string("citrix.detail") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("CEF:") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.cef_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.device_vendor", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.device_product", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.device_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.device_event_class_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("citrix.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else { break 'dissect false };
                        captured.push(("event.severity", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("citrix.extended.message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "citrix.detail".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }

            if event.has_value("citrix.extended.message") {
                if let Some(kv_str) = event.get_string("citrix.extended.message") {
                    for pair in cached_regex!(" (?=[a-zA-Z][a-zA-Z0-9]*=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "citrix.extended.message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("citrix.extended_kv.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("citrix.extended_kv") };
            if _cond {
                if event.remove("citrix.extended").is_none() {
                    return Err(TransformError::FieldNotFound { path: "citrix.extended".into() });
                }
            }

            if event.has_value("citrix.extended_kv.src") {
                if let Some(val) = event.get("citrix.extended_kv.src") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix.extended_kv.src".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

                event.remove("citrix.extended_kv.src");

            if event.has_value("citrix.extended_kv.spt") {
                if let Some(val) = event.get("citrix.extended_kv.spt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix.extended_kv.spt".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }

                event.remove("citrix.extended_kv.spt");

                if event.has_value("citrix.extended_kv.method") {
                    event.rename("citrix.extended_kv.method", "http.request.method")?;
                }

                if event.has_value("citrix.extended_kv.request") {
                    event.rename("citrix.extended_kv.request", "url.original")?;
                }

                if event.has_value("citrix.extended_kv.act") {
                    event.rename("citrix.extended_kv.act", "event.action")?;
                }

                if event.has_value("citrix.extended_kv.msg") {
                    event.rename("citrix.extended_kv.msg", "message")?;
                }

                if event.has_value("citrix.extended_kv.cn1") {
                    event.rename("citrix.extended_kv.cn1", "event.id")?;
                }

                if event.has_value("citrix.extended_kv.cn2") {
                    event.rename("citrix.extended_kv.cn2", "http.request.id")?;
                }

                if event.has_value("citrix.extended_kv.cs1") {
                    event.rename("citrix.extended_kv.cs1", "citrix.profile_name")?;
                }

                if event.has_value("citrix.extended_kv.cs2") {
                    event.rename("citrix.extended_kv.cs2", "citrix.ppe_id")?;
                }

                if event.has_value("citrix.extended_kv.cs3") {
                    event.rename("citrix.extended_kv.cs3", "citrix.session_id")?;
                }

                if event.has_value("citrix.extended_kv.cs4") {
                    event.rename("citrix.extended_kv.cs4", "citrix.severity")?;
                }

                if event.has_value("citrix.extended_kv.cs5") {
                    event.rename("citrix.extended_kv.cs5", "citrix.event_year")?;
                }

                if event.has_value("citrix.extended_kv.cs6") {
                    event.rename("citrix.extended_kv.cs6", "citrix.signature_violation_category")?;
                }

                if event.has_value("citrix.extended_kv") {
                    event.rename("citrix.extended_kv", "citrix.extended")?;
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
