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
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.original")
                    && event
                        .get_str("event.original")
                        .is_some_and(|s| s.starts_with("{"))
            };
            if _cond {
                // Begin nested pipeline: "json"
                parse_json_field(event, "event.original", "hashicorp_vault.log")?;
                if let Some(date_str) = event.get_as_string("hashicorp_vault.log.@timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "hashicorp_vault.log.@timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                event.remove("hashicorp_vault.log.@timestamp");
                if event.has_value("hashicorp_vault.log.@level") {
                    event.rename("hashicorp_vault.log.@level", "log.level")?;
                }
                if event.has_value("hashicorp_vault.log.@message") {
                    event.rename("hashicorp_vault.log.@message", "message")?;
                }
                if event.has_value("hashicorp_vault.log.@module") {
                    event.rename("hashicorp_vault.log.@module", "log.logger")?;
                }
                let _cond = {
                    event.has_value("hashicorp_vault.log")
                        && event.get("hashicorp_vault.log").is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
                };
                if _cond {
                    // Painless script
                    // Source: ctx.hashicorp_vault.remove(\"log\");\nif (ctx.hashicorp_vault.isEmpty()) {\n  ctx.remove(\"hashicorp_vault\");\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.hashicorp_vault.remove(\"log\");\nif (ctx.hashicorp_vault.isEmpty()) {\n  ctx.remove(\"hashicorp_vault\");\n}\n"#
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("hashicorp_vault.log.file_path").cloned() {
                        event.set("file.path", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "json"
            }

            let _cond = {
                event.has_value("event.original")
                    && !(event
                        .get_str("event.original")
                        .is_some_and(|s| s.starts_with("{")))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("event.original").cloned() {
                        event.set("message", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
