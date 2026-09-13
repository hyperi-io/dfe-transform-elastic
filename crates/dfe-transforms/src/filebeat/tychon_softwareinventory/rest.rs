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
            if event.has_value("tychon.edition") {
                event.rename("tychon.edition", "tychon.package.edition")?;
            }

            if event.has_value("tychon.comment") {
                event.rename("tychon.comment", "tychon.package.description")?;
            }

        let _cond = { event.has_value("tychon.package.installed") };
        if _cond {
            // Painless script
            // Source: if (['installed', 'true'].contains(ctx.tychon.package.installed)) {\n  ctx.tychon.package.installed = '1970-01-01T00:00:01Z';\n} else {\n  ctx.tychon.package.remove('installed');\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (['installed', 'true'].contains(ctx.tychon.package.installed)) {\n  ctx.tychon.package.installed = '1970-01-01T00:00:01Z';\n} else {\n  ctx.tychon.package.remove('installed');\n}\n"#))?;
        }

            gsub_field(event, "tychon.package.size", "tychon.package.size", cached_regex!("[^0-9]"), "")?;

        let _cond = { !event.has_value("tychon.package.size") };
        if _cond {
        event.set("tychon.package.size", json!(0))?;
        }

        if event.has_value("tychon.package.size") {
            if let Some(val) = event.get("tychon.package.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.package.size".into(),
                        message,
                    })?;
                event.set("tychon.package.size", converted)?;
            }
        }

        if event.has_value("tychon.package.version_build") {
            gsub_field(event, "tychon.package.version_build", "tychon.package.version_build", cached_regex!("[^0-9]"), "")?;
        }

        if event.has_value("tychon.package.version_major") {
            gsub_field(event, "tychon.package.version_major", "tychon.package.version_major", cached_regex!("[^0-9]"), "")?;
        }

        if event.has_value("tychon.package.version_minor") {
            gsub_field(event, "tychon.package.version_minor", "tychon.package.version_minor", cached_regex!("[^0-9]"), "")?;
        }

        if event.has_value("tychon.package.version_release") {
            gsub_field(event, "tychon.package.version_release", "tychon.package.version_release", cached_regex!("[^0-9]"), "")?;
        }

        let _cond = { !event.has_value("tychon.package.type") };
        if _cond {
        event.set("tychon.package.type", json!("rpm"))?;
        }

        let _cond = { event.get_str("tychon.package.cpe") == Some("") && event.has_value("tychon.package.name") && event.has_value("tychon.package.version") };
        if _cond {
            // Painless script
            // Source: ctx.tychon.package.cpe = \"cpe:/a:\" + ctx.tychon.package.name + \":\" + ctx.tychon.package.version
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.tychon.package.cpe = \"cpe:/a:\" + ctx.tychon.package.name + \":\" + ctx.tychon.package.version"#))?;
        }

        event.set("event.category", Value::Array(vec![json!("package")]))?;

        if let Some(v) = event.get("tychon.package.architecture").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.architecture", v)?;
        }

        if let Some(v) = event.get("tychon.package.description").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.description", v)?;
        }

        if let Some(v) = event.get("tychon.package.installed").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.installed", v)?;
        }

        if let Some(v) = event.get("tychon.package.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.name", v)?;
        }

        if let Some(v) = event.get("tychon.package.path").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.path", v)?;
        }

        if let Some(v) = event.get("tychon.package.size").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.size", v)?;
        }

        if let Some(v) = event.get("tychon.package.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.type", v)?;
        }

        if let Some(v) = event.get("tychon.package.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("package.version", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
