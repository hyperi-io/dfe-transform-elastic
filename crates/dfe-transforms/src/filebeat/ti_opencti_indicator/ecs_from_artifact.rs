// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_artifact` pipeline.
pub struct EcsFromArtifact;

impl Transform for EcsFromArtifact {
    fn name(&self) -> &str {
        "ecs_from_artifact"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("_ingest._value.additional_names") {
            foreach_array(event, "_ingest._value.additional_names", |event| {
                event.append_unique("_tmp_file.name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

            // Painless script
            // Source: if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else if (result.length == 1) {\n    ctx._tmp_file.name = result[0];\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else if (result.length == 1) {\n    ctx._tmp_file.name = result[0];\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n"#))?;

        let v = json!(event.get("_ingest._value.mime_type").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.mime_type", v)?;
        }

        let v = json!("file");
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.type", v)?;
        }

        if let Some(v) = event.get("_ingest._value.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("_tmp_file.hash", v)?;
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
