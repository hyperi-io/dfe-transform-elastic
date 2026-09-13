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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("ecs.version", json!("8.11.0"))?;
                Ok(())
            })();

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics") {
                    event.rename("jolokia.metrics", "oracle_weblogic")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.kind", json!("metric"))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.category", Value::Array(vec![json!("web")]))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.module", json!("oracle_weblogic"))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "event.dataset",
                    json!("oracle_weblogic.deployed_application"),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: Map m = new HashMap();\nm.put(0, \"Shutdown\");\nm.put(1, \"Starting\");\nm.put(2, \"Running\");\nctx.oracle_weblogic.deployed_application.deployment.state.name = m.get(ctx.oracle_weblogic.deployed_application.deployment.state.value);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map m = new HashMap();\nm.put(0, \"Shutdown\");\nm.put(1, \"Starting\");\nm.put(2, \"Running\");\nctx.oracle_weblogic.deployed_application.deployment.state.name = m.get(ctx.oracle_weblogic.deployed_application.deployment.state.value);\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("jolokia");
                event.remove("oracle_weblogic.mbean");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
