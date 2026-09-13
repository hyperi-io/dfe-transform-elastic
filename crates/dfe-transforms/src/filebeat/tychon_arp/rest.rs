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
            map_strings(event, "tychon.destination.mac", "tychon.destination.mac", str::to_uppercase)?;

        if event.has_value("tychon.network.type") {
            map_strings(event, "tychon.network.type", "tychon.network.type", str::to_lowercase)?;
        }

        event.set("event.category", Value::Array(vec![json!("network")]))?;

        let _cond = { event.has_value("tychon.destination.hostname") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("tychon.destination.hostname").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.destination.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("tychon.destination.ip").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("tychon.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.ip", v)?;
        }

        if let Some(v) = event.get("tychon.destination.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.mac", v)?;
        }

        if let Some(v) = event.get("tychon.network.direction").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.direction", v)?;
        }

        if let Some(v) = event.get("tychon.network.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.type", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
