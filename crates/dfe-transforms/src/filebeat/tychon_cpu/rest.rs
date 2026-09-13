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
            gsub_field(event, "tychon.host.cpu.speed", "tychon.host.cpu.speed", cached_regex!("[^0-9]"), "")?;

        if event.has_value("tychon.host.cpu.speed") {
            if let Some(val) = event.get("tychon.host.cpu.speed") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.host.cpu.speed".into(),
                        message,
                    })?;
                event.set("tychon.host.cpu.speed", converted)?;
            }
        }

            gsub_field(event, "tychon.host.cpu.clockspeed", "tychon.host.cpu.clockspeed", cached_regex!("[^0-9]"), "")?;

        if event.has_value("tychon.host.cpu.clockspeed") {
            if let Some(val) = event.get("tychon.host.cpu.clockspeed") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.host.cpu.clockspeed".into(),
                        message,
                    })?;
                event.set("tychon.host.cpu.clockspeed", converted)?;
            }
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
