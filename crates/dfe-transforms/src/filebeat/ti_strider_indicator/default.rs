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

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.rename("md5", "ti_strider.indicator.md5")?;

            event.rename("riskSignal", "ti_strider.indicator.risk_signal")?;

            event.rename("itemType", "ti_strider.indicator.type")?;

            event.rename("itemNamePrimary", "ti_strider.indicator.name_primary")?;

            event.rename(
                "itemNameSecondary",
                "ti_strider.indicator.name_secondary_raw",
            )?;

            event.rename("dates_added", "ti_strider.indicator.dates_added")?;

            event.rename("isNew", "ti_strider.indicator.is_new")?;

            // Painless script
            // Source: if (ctx.ti_strider?.indicator?.is_new != null) {\n  ctx.ti_strider.indicator.is_new = (ctx.ti_strider.indicator.is_new == 1);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.ti_strider?.indicator?.is_new != null) {\n  ctx.ti_strider.indicator.is_new = (ctx.ti_strider.indicator.is_new == 1);\n}\n"#
                ),
            )?;

            event.rename("changes", "ti_strider.indicator.changes")?;

            if event.has_value("archive") {
                event.rename("archive", "ti_strider.indicator.archive")?;
            }

            parse_json_field(
                event,
                "ti_strider.indicator.name_secondary_raw",
                "ti_strider.indicator.name_secondary",
            )?;

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                // Painless script
                // Source: ZonedDateTime now = ZonedDateTime.parse(ctx['@timestamp']);\nif (ctx.ti_strider?.indicator?.archive == 1) {\n  ctx.ti_strider.indicator.expires_at = now;\n} else {\n  ctx.ti_strider.indicator.expires_at = now.plusDays(90);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ZonedDateTime now = ZonedDateTime.parse(ctx['@timestamp']);\nif (ctx.ti_strider?.indicator?.archive == 1) {\n  ctx.ti_strider.indicator.expires_at = now;\n} else {\n  ctx.ti_strider.indicator.expires_at = now.plusDays(90);\n}\n"#
                    ),
                )?;
            }

            if event
                .remove("ti_strider.indicator.name_secondary_raw")
                .is_none()
            {
                return Err(TransformError::FieldNotFound {
                    path: "ti_strider.indicator.name_secondary_raw".into(),
                });
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
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
