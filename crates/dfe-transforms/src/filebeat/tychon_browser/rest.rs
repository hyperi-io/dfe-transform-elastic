// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = { event.has_value("tychon.package.installed") };
        if _cond {
            // Painless script
            // Source: if (['installed', 'true'].contains(ctx.tychon.package.installed)) {\n  ctx.tychon.package.installed = '1970-01-01T00:00:01Z';\n} else {\n  ctx.tychon.package.remove('installed');\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (['installed', 'true'].contains(ctx.tychon.package.installed)) {\n  ctx.tychon.package.installed = '1970-01-01T00:00:01Z';\n} else {\n  ctx.tychon.package.remove('installed');\n}\n"#))?;
        }

        if event.has_value("tychon.package.size") {
            if let Some(val) = event.get("tychon.package.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.package.size".into(),
                        message,
                    })?;
                event.set("tychon.package.size", converted)?;
            }
        }

        event.set("event.category", Value::Array(vec![json!("network")]))?;

        if let Some(v) = event.get("tychon.event.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.reason", v)?;
        }

        if let Some(v) = event.get("tychon.package.architecture").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.architecture", v)?;
        }

        if let Some(v) = event.get("tychon.package.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.name", v)?;
        }

        if let Some(v) = event.get("tychon.package.path").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.path", v)?;
        }

        if let Some(v) = event.get("tychon.package.size").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.size", v)?;
        }

        if let Some(v) = event.get("tychon.package.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.type", v)?;
        }

        if let Some(v) = event.get("tychon.package.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.version", v)?;
        }

        if event.has_value("tychon.tls.version_protocol") {
            if let Some(input) = event.get_string("tychon.tls.version_protocol") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("v") else { break 'dissect false };
                    captured.push(("tls.version_protocol", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("v") else { break 'dissect false };
                    remaining = rest;
                    captured.push(("tls.version", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                }
                else {
                    return Err(TransformError::ParseError {
                        path: "tychon.tls.version_protocol".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        if event.has_value("tls.version_protocol") {
            map_strings(event, "tls.version_protocol", "tls.version_protocol", str::to_lowercase)?;
        }

        Ok(TransformResult::Continue)
    }
}
