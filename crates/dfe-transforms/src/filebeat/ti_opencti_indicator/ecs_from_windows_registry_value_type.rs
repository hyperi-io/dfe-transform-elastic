// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_windows_registry_value_type` pipeline.
pub struct EcsFromWindowsRegistryValueType;

impl Transform for EcsFromWindowsRegistryValueType {
    fn name(&self) -> &str {
        "ecs_from_windows_registry_value_type"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("_tmp_registry.value", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;

        event.set("_tmp_registry.data.type", json!(event.get("_ingest._value.data_type").map_or_else(String::new, template_to_string)))?;

            event.append("_tmp_registry.data.strings", json!(event.get("_ingest._value.data").map_or_else(String::new, template_to_string)))?;

            // Painless script
            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.registry = ctx.threat.indicator.registry ?: [];\nctx.threat.indicator.registry.add(ctx._tmp_registry);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.registry = ctx.threat.indicator.registry ?: [];\nctx.threat.indicator.registry.add(ctx._tmp_registry);\n"#))?;

            if event.remove("_tmp_registry").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_registry".into() });
            }

        Ok(TransformResult::Continue)
    }
}
