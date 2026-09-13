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
                if let Some(input) = event.get_string("_temp.remMessage") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.module.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.severity_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.eventid", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("barracuda.waf.module.event_message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "_temp.remMessage".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }

            if event.has_value("_temp.eventid") {
                if let Some(val) = event.get("_temp.eventid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.eventid".into(),
                            message,
                        })?;
                    event.set("barracuda.waf.module.event_id", converted)?;
                }
            }

            event.set("event.category", Value::Array(vec![json!("configuration")]))?;

            let _cond = { event.has_value("barracuda.waf.severity_level") && ["ALER", "EMER", "CRIT", "ALERT", "CRITICAL", "EMERGENCY"].contains(&event.get_str("barracuda.waf.severity_level").unwrap_or("")) };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
            event.set("event.kind", json!("event"))?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
