// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.17.0"))?;

        if let Some(v) = event
            .get("o365.metrics.entra.agent.os_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.name", v)?;
        }

        if let Some(v) = event
            .get("o365.metrics.entra.agent.os_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.version", v)?;
        }

        event.remove("o365.metrics.entra.agent.os_name");
        event.remove("o365.metrics.entra.agent.os_version");

        let _cond = { event.has_value("error.message") };
        if _cond {
            event.set("event.kind", json!("error"))?;
        }

        Ok(TransformResult::Continue)
    }
}
