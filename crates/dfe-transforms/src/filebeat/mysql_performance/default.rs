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
            if event.has_value("mysql.performance.events_statements.last.seen") {
                gsub_field(
                    event,
                    "mysql.performance.events_statements.last.seen",
                    "mysql.performance.events_statements.last.seen",
                    cached_regex!(" "),
                    "T",
                )?;
            }

            // Painless script
            // Source: if (ctx.mysql?.performance?.events_statements?.digest == null) {\n    return;\n}\ndef digest = ctx.mysql.performance.events_statements.digest;\nctx.mysql.performance.events_statements.query = digest.text;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.mysql?.performance?.events_statements?.digest == null) {\n    return;\n}\ndef digest = ctx.mysql.performance.events_statements.digest;\nctx.mysql.performance.events_statements.query = digest.text;\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("mysql.performance.events_statements.query") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("mysql.performance.events_statements.schemaname") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "mysql.performance.events_statements.query_id",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("mysql.performance.events_statements.query");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
