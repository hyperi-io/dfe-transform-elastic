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

        event.set("event.kind", json!("metric"))?;

        let _cond = { event.has_value("error.message") };
        if _cond {
            event.set("event.kind", json!("error"))?;
        }

        Ok(TransformResult::Continue)
    }
}
