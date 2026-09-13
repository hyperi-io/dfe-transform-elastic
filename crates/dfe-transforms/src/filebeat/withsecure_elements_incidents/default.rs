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
            let _cond = { !event.has_value("event.original") && event.has_value("message") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("message").cloned() {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.original")
                    && event.has_value("message")
                    && condition_eq(event.get("message"), event.get("event.original"))
            };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event.has_value("event.original")
                    && event.get("event.original").is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "withsecure_elements.incidents")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_decode_b5ae2280")?;
                    event.set(
                        "error.message",
                        json!("Failed to parse JSON from event.original"),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.dataset", json!("withsecure_elements.incidents"))?;

            event.set("event.module", json!("withsecure_elements"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("incident")]))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.provider", json!("withsecure_elements"))?;

            event.set("event.action", json!("detected"))?;

            event.set("event.outcome", json!("success"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("withsecure_elements.incidents.incidentId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("withsecure_elements.incidents.updatedTimestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("withsecure_elements.incidents.incidentId") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("withsecure_elements.incidents.incidentId")
                        .cloned()
                    {
                        event.set("event.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("withsecure_elements.incidents.riskScore") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("withsecure_elements.incidents.riskScore")
                        .cloned()
                    {
                        event.set("event.risk_score", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("withsecure_elements.incidents.riskLevel") };
            if _cond {
                // Painless script
                // Source: def riskLevelValue = ctx.withsecure_elements.incidents.riskLevel; if (riskLevelValue == null) return; if (riskLevelValue instanceof Number) {\n  ctx.event.severity = ((Number) riskLevelValue).intValue();\n  return;\n} def normalized = riskLevelValue.toString().toLowerCase(); if (normalized == 'info' || normalized == 'low') {\n  ctx.event.severity = 21;\n} else if (normalized == 'medium') {\n  ctx.event.severity = 47;\n} else if (normalized == 'high') {\n  ctx.event.severity = 73;\n} else if (normalized == 'severe') {\n  ctx.event.severity = 99;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def riskLevelValue = ctx.withsecure_elements.incidents.riskLevel; if (riskLevelValue == null) return; if (riskLevelValue instanceof Number) {\n  ctx.event.severity = ((Number) riskLevelValue).intValue();\n  return;\n} def normalized = riskLevelValue.toString().toLowerCase(); if (normalized == 'info' || normalized == 'low') {\n  ctx.event.severity = 21;\n} else if (normalized == 'medium') {\n  ctx.event.severity = 47;\n} else if (normalized == 'high') {\n  ctx.event.severity = 73;\n} else if (normalized == 'severe') {\n  ctx.event.severity = 99;\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("withsecure_elements.incidents.createdTimestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("withsecure_elements.incidents.createdTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "withsecure_elements.incidents.createdTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "set_timestamp_0480b0bb")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("@timestamp").cloned() {
                        event.set("event.created", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("@timestamp").cloned() {
                        event.set("event.start", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("withsecure_elements.incidents.updatedTimestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("withsecure_elements.incidents.updatedTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set("event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "withsecure_elements.incidents.updatedTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_end_f6368345")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
