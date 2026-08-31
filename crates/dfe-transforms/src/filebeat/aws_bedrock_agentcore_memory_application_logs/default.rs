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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "aws.bedrock_agentcore.memory")?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("aws.bedrock_agentcore.memory.event_timestamp")
                {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "aws.bedrock_agentcore.memory.event_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("aws.bedrock_agentcore.memory.severityNumber") {
                event.rename(
                    "aws.bedrock_agentcore.memory.severityNumber",
                    "aws.bedrock_agentcore.memory.severity_number",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.bedrock_agentcore.memory.severityText")
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.bedrock_agentcore.memory.body.requestId")
                    .cloned()
                {
                    event.set("aws.bedrock_agentcore.memory.request_id", v)?;
                }
                Ok(())
            })();

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("cloud.service.name", json!("bedrock-agentcore"))?;

            event.set("cloud.provider", json!("aws"))?;

            let _cond = { event.has_value("aws.bedrock_agentcore.memory.resource.attributes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def attrs = ctx.aws.bedrock_agentcore.memory.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def attrs = ctx.aws.bedrock_agentcore.memory.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.bedrock_agentcore.memory.resource_arn") {
                    if let Some(input) =
                        event.get_string("aws.bedrock_agentcore.memory.resource_arn")
                    {
                        // Grok pattern: arn:aws:bedrock-agentcore:%{DATA}:%{DATA}:memory/%{DATA:aws.bedrock_agentcore.memory.memory_name}
                        if !cached_grok!("arn:aws:bedrock-agentcore:%{DATA}:%{DATA}:memory/%{DATA:aws.bedrock_agentcore.memory.memory_name}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.bedrock_agentcore.memory.memory_strategy_id") {
                    if let Some(input) =
                        event.get_string("aws.bedrock_agentcore.memory.memory_strategy_id")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("-") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "aws.bedrock_agentcore.memory.memory_strategy",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("-") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp.strategy_suffix", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.remove("_temp.strategy_suffix");

            event.set(
                "aws.bedrock_agentcore.memory.provider_name",
                json!("aws_bedrock_agentcore"),
            )?;

            event.set(
                "aws.bedrock_agentcore.memory.operation_name",
                json!("invoke_memory"),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.bedrock_agentcore.memory.session_id")
                    .cloned()
                {
                    event.set("aws.bedrock_agentcore.memory.conversation_id", v)?;
                }
                Ok(())
            })();

            // Painless script
            // Source: if (ctx.aws?.bedrock_agentcore?.memory?.body?.isError == true || ctx.error?.message != null) {\n  ctx.event = ctx.event != null ? ctx.event : new HashMap();\n  ctx.event.outcome = 'failure';\n} else {\n  ctx.event = ctx.event != null ? ctx.event : new HashMap();\n  ctx.event.outcome = 'success';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.aws?.bedrock_agentcore?.memory?.body?.isError == true || ctx.error?.message != null) {\n  ctx.event = ctx.event != null ? ctx.event : new HashMap();\n  ctx.event.outcome = 'failure';\n} else {\n  ctx.event = ctx.event != null ? ctx.event : new HashMap();\n  ctx.event.outcome = 'success';\n}\n"#
                ),
            )?;

            if event.has_value("aws.bedrock_agentcore.memory.body") {
                event.rename(
                    "aws.bedrock_agentcore.memory.body",
                    "aws.bedrock_agentcore.memory.payload_object",
                )?;
            }

            if let Some(v) = event
                .get("aws.bedrock_agentcore.memory.payload_object.log")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.remove("aws.bedrock_agentcore.memory.attributes");
            event.remove("aws.bedrock_agentcore.memory.resource");
            event.remove("aws.bedrock_agentcore.memory.event_timestamp");
            event.remove("aws.bedrock_agentcore.memory.timeUnixNano");
            event.remove("aws.bedrock_agentcore.memory.severityText");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
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
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.outcome", json!("failure"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
