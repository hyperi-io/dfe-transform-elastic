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
        if event.has_value("tychon.process.pid") {
            if let Some(val) = event.get("tychon.process.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.process.pid".into(),
                        message,
                    })?;
                event.set("tychon.process.pid", converted)?;
            }
        }

        if event.has_value("tychon.source.port") {
            if let Some(val) = event.get("tychon.source.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.source.port".into(),
                        message,
                    })?;
                event.set("tychon.source.port", converted)?;
            }
        }

        let _cond = { !event.has_value("tychon.source.ip") };
        if _cond {
        event.set("tychon.source.ip", json!("0.0.0.0"))?;
        }

        let _cond = { !event.has_value("tychon.service.name") };
        if _cond {
        event.set("tychon.service.name", json!(event.get("tychon.process.name").map_or_else(String::new, template_to_string)))?;
        }

        event.set("event.category", Value::Array(vec![json!("network")]))?;

        let _cond = { event.has_value("tychon.process.hash.sha1") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.process.hash.sha1").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.source.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("tychon.source.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.process.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("tychon.process.user.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("tychon.network.transport").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.transport", v)?;
        }

        if let Some(v) = event.get("tychon.process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.command_line", v)?;
        }

        if let Some(v) = event.get("tychon.process.executable").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.executable", v)?;
        }

        if let Some(v) = event.get("tychon.process.hash.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.hash.sha1", v)?;
        }

        if let Some(v) = event.get("tychon.process.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.name", v)?;
        }

        if let Some(v) = event.get("tychon.process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.pid", v)?;
        }

        if let Some(v) = event.get("tychon.process.start").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.start", v)?;
        }

        if let Some(v) = event.get("tychon.process.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.user.name", v)?;
        }

        if let Some(v) = event.get("tychon.service.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.name", v)?;
        }

        if let Some(v) = event.get("tychon.service.state").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.state", v)?;
        }

        if let Some(v) = event.get("tychon.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
        }

        if let Some(v) = event.get("tychon.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.port", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
