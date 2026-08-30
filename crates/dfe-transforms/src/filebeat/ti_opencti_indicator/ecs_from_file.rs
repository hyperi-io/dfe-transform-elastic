// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs_from_file` pipeline.
pub struct EcsFromFile;

impl Transform for EcsFromFile {
    fn name(&self) -> &str {
        "ecs_from_file"
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

            event.append_unique("_tmp_file.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;

        if event.has_value("_ingest._value.additional_names") {
            foreach_array(event, "_ingest._value.additional_names", |event| {
                event.append_unique("_tmp_file.name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

            // Painless script
            // Source: if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx._tmp_file?.name != null) {\n  def result = [];\n  for (name in ctx._tmp_file.name) {\n      if (name != null && name != '') {\n          result.add(name);\n      }\n  }\n  if (result.length == 0) {\n    ctx._tmp_file.remove(\"name\");\n  } else {\n    ctx._tmp_file.name = result;\n  }\n}\n"#))?;

            // Painless script
            // Source: if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx._tmp_file.name != null) {\n  def result = [];\n  for (fullname in ctx._tmp_file.name) {\n    // name shouldn't have a directory, but do strip it if it's there\n    def parts = /[\\/\\\\]/.split(fullname);\n    def name = parts[parts.length - 1];\n    if (name.contains(\".\")) {\n      def nameParts = /\\./.split(name);\n      def extension = nameParts[nameParts.length - 1];\n      if (extension.length() > 0) {\n        result.add(extension);\n      }\n    }\n  }\n  if (result.length > 0) {\n    ctx._tmp_file.extension = result;\n  }\n}\n"#))?;

        let v = json!(event.get("_ingest._value.mime_type").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.mime_type", v)?;
        }

        let v = json!(event.get("_ingest._value.size").map_or_else(String::new, template_to_string));
        if !painless_is_empty_value(&v) {
                event.set("_tmp_file.size", v)?;
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
