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
        if event.has_value("tychon.disk.size") {
            if let Some(val) = event.get("tychon.disk.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.disk.size".into(),
                        message,
                    })?;
                event.set("tychon.disk.size", converted)?;
            }
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
