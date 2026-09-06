// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `file` pipeline.
pub struct File;

impl Transform for File {
    fn name(&self) -> &str {
        "file"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Upload")), serde_json::Value::String(s) => s.contains("Upload"), _ => false }) };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" 'Upload' for path '") else { break 'dissect false };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" 'Upload' for path '") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("' ") else { break 'dissect false };
                    captured.push(("vsphere.log.file.path", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("' ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find(" '") else { break 'dissect false };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" '") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("' ") else { break 'dissect false };
                    captured.push(("client.ip", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("' ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find(" '") else { break 'dissect false };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" '") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("'") else { break 'dissect false };
                    captured.push(("event.outcome", &remaining[..pos]));
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
                else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = { event.has_value("event.outcome") };
        if _cond {
            map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
        }

            event.append("event.category", json!("file"))?;

            event.append("event.type", json!("creation"))?;

        let _cond = { event.has_value("client.ip") };
        if _cond {
        if let Some(v) = event.get("client.ip").cloned() {
            event.set("source.ip", v)?;
        }
        }

        let _cond = { event.has_value("client.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
        }

        Ok(TransformResult::Continue)
    }
}
