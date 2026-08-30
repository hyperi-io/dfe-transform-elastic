// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_directory` pipeline.
pub struct EcsFromDirectory;

impl Transform for EcsFromDirectory {
    fn name(&self) -> &str {
        "ecs_from_directory"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let v = json!(event.get("_ingest._value.atime").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.accessed", v)?;
        }

        let v = json!(event.get("_ingest._value.ctime").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.created", v)?;
        }

        let v = json!(event.get("_ingest._value.mtime").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.mtime", v)?;
        }

            event.append_unique("_tmp_file.path", json!(event.get("_ingest._value.path").map_or_else(String::new, template_to_string)))?;

        let v = json!("dir");
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.type", v)?;
        }

            // Painless script
            // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.file = ctx.threat.indicator.file ?: [];\nctx.threat.indicator.file.add(ctx._tmp_file);\n"#))?;

            if event.remove("_tmp_file").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_file".into() });
            }

        Ok(TransformResult::Continue)
    }
}
