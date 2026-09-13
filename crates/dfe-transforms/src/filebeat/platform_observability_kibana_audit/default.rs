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
            let _cond = {
                event.has_value("message")
                    && event.get_str("message").is_some_and(|s| s.starts_with("{"))
                    && event.get_str("message").is_some_and(|s| s.ends_with("}"))
                    && event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("\"@timestamp\""))
                        }
                        serde_json::Value::String(s) => s.contains("\"@timestamp\""),
                        _ => false,
                    })
            };
            if _cond {
                // Begin nested pipeline: "ecs-logs-pipeline"
                if event.has_value("message") {
                    event.rename("message", "_ecs_json_message")?;
                }
                let _cond = { event.has("_ecs_json_message") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field_to_root(event, "_ecs_json_message", true)?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        if event.has_value("_ecs_json_message") {
                            event.rename("_ecs_json_message", "message")?;
                        }
                        if !event.has("error.message") {
                            event.set("error.message", json!("Error while parsing JSON"))?;
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                event.remove("_ecs_json_message");
                dot_expand(event, "", "*")?;
                let _cond = { event.get("error.stack_trace").is_some_and(|v| v.is_array()) };
                if _cond {
                    let joined = event
                        .get("error.stack_trace")
                        .and_then(|v| join_values(v, "\n"));
                    if let Some(joined) = joined {
                        event.set("error.stack_trace", json!(joined))?;
                    }
                }
                // End nested pipeline: "ecs-logs-pipeline"
            }

            event.set("data_stream.type", json!("logs"))?;

            event.set("data_stream.dataset", json!("kibana-audit-log"))?;

            event.set("data_stream.namespace", json!("platform-observability"))?;

            if let Some(v) = event.get("data_stream.dataset").cloned() {
                event.set("event.dataset", v)?;
            }

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("event.kind", json!("event"))?;

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
