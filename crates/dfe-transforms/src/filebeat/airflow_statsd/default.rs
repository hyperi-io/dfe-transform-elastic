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

            if event.has_value("statsd") {
                event.rename("statsd", "airflow")?;
            }

            // Painless script
            // Source: for (entry in ctx.airflow.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (!(v instanceof String)) {\n        v.keySet().stream().filter(s -> s.startsWith('1m_rate') || s.startsWith('5m_rate') ||s.startsWith('15m_rate') ||\n        s.startsWith('p99_9') || s.startsWith('p75') || s.startsWith('p99') || s.startsWith('p95'))\n        .collect(Collectors.toList()).forEach(m -> v.remove(m))\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"for (entry in ctx.airflow.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (!(v instanceof String)) {\n        v.keySet().stream().filter(s -> s.startsWith('1m_rate') || s.startsWith('5m_rate') ||s.startsWith('15m_rate') ||\n        s.startsWith('p99_9') || s.startsWith('p75') || s.startsWith('p99') || s.startsWith('p95'))\n        .collect(Collectors.toList()).forEach(m -> v.remove(m))\n    }\n}\n"#
                ),
            )?;

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
