// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_windows_registry_key` pipeline.
pub struct EcsFromWindowsRegistryKey;

impl Transform for EcsFromWindowsRegistryKey {
    fn name(&self) -> &str {
        "ecs_from_windows_registry_key"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            if let Some(input) = event.get_string("_ingest._value.attribute_key") {
                // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:_tmp_registry.key}$
                if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:_tmp_registry.key}$", [("_tmp_registry_hive", "_tmp_registry.hive")]).extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

        let _cond = { event.has_value("_tmp_registry.hive") };
        if _cond {
            // Painless script
            // Source: def name = ctx._tmp_registry.hive.toUpperCase();\nctx._tmp_registry.hive = params.getOrDefault(name, name);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"def name = ctx._tmp_registry.hive.toUpperCase();\nctx._tmp_registry.hive = params.getOrDefault(name, name);\n"#), cached_params!("{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\",\"HKEY_CURRENT_CONFIG\":\"HKCC\"}"))?;
        }

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
