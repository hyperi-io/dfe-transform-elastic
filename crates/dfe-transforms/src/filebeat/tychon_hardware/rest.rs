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
        if event.has_value("tychon.device.present") {
            map_strings(event, "tychon.device.present", "tychon.device.present", str::to_lowercase)?;
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        if let Some(v) = event.get("tychon.device.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("device.id", v)?;
        }

        if let Some(v) = event.get("tychon.device.manufacturer").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("device.manufacturer", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
