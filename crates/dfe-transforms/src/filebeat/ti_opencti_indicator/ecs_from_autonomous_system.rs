// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_autonomous_system` pipeline.
pub struct EcsFromAutonomousSystem;

impl Transform for EcsFromAutonomousSystem {
    fn name(&self) -> &str {
        "ecs_from_autonomous_system"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("_tmp_as.number", json!(event.get("_ingest._value.number").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_as.organization.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;

            // Painless script
            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.as = ctx.threat.indicator.as ?: [];\nctx.threat.indicator.as.add(ctx._tmp_as);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.as = ctx.threat.indicator.as ?: [];\nctx.threat.indicator.as.add(ctx._tmp_as);\n"#))?;

            if event.remove("_tmp_as").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_as".into() });
            }

        Ok(TransformResult::Continue)
    }
}
