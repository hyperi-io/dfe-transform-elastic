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
            event.set("ecs.version", json!("8.11.0"))?;

            if event.has_value("jolokia.metrics") {
                event.rename("jolokia.metrics", "apache_spark")?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("apache_spark"))?;

            let _cond = {
                event.get("apache_spark.mbean").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("name=application"))
                    }
                    serde_json::Value::String(s) => s.contains("name=application"),
                    _ => false,
                })
            };
            if _cond {
                // Painless script
                // Source: def bean_name = ctx.apache_spark.mbean.toString().splitOnToken(\".\"); def app_name = \"\"; if (bean_name[0].contains(\"name=application\") == true) {\n    app_name = bean_name[1] + \".\" + bean_name[2];\n} ctx.apache_spark.application.name = app_name;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def bean_name = ctx.apache_spark.mbean.toString().splitOnToken(\".\"); def app_name = \"\"; if (bean_name[0].contains(\"name=application\") == true) {\n    app_name = bean_name[1] + \".\" + bean_name[2];\n} ctx.apache_spark.application.name = app_name;"#
                    ),
                )?;
            }

            if event.has_value("apache_spark.mbean") {
                event.rename("apache_spark.mbean", "apache_spark.application.mbean")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("jolokia").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "jolokia".into(),
                    });
                }
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
