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
                    parse_json_field(event, "event.original", "aws.bedrock_agentcore")?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("aws.bedrock_agentcore.event_timestamp")
                {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "aws.bedrock_agentcore.event_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("aws.bedrock_agentcore.account_id") {
                event.rename("aws.bedrock_agentcore.account_id", "cloud.account.id")?;
            }

            if event.has_value("aws.bedrock_agentcore.traceId") {
                event.rename("aws.bedrock_agentcore.traceId", "trace.id")?;
            }

            let _cond = { !event.has_value("trace.id") };
            if _cond {
                if event.has_value("aws.bedrock_agentcore.trace_id") {
                    event.rename("aws.bedrock_agentcore.trace_id", "trace.id")?;
                }
            }

            if event.has_value("aws.bedrock_agentcore.spanId") {
                event.rename("aws.bedrock_agentcore.spanId", "span.id")?;
            }

            let _cond = { !event.has_value("span.id") };
            if _cond {
                if event.has_value("aws.bedrock_agentcore.span_id") {
                    event.rename("aws.bedrock_agentcore.span_id", "span.id")?;
                }
            }

            if event.has_value("aws.bedrock_agentcore.severityNumber") {
                event.rename(
                    "aws.bedrock_agentcore.severityNumber",
                    "aws.bedrock_agentcore.severity_number",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("aws.bedrock_agentcore.severityText").cloned() {
                    event.set("log.level", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.bedrock_agentcore.body.request_payload")
                    .cloned()
                {
                    event.set("aws.bedrock_agentcore.request_payload_object", v)?;
                }
                Ok(())
            })();

            let _cond = {
                !event.has_value("aws.bedrock_agentcore.body.request_payload.prompt")
                    || event
                        .get("aws.bedrock_agentcore.body.request_payload.prompt")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("aws.bedrock_agentcore.body.request_payload") {
                    event.rename(
                        "aws.bedrock_agentcore.body.request_payload",
                        "aws.bedrock_agentcore.request_payload",
                    )?;
                }
            }

            if event.has_value("aws.bedrock_agentcore.body.response_payload") {
                event.rename(
                    "aws.bedrock_agentcore.body.response_payload",
                    "aws.bedrock_agentcore.response_payload_object",
                )?;
            }

            event.remove("aws.bedrock_agentcore.body");

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("cloud.service.name", json!("bedrock-agentcore"))?;

            event.set("cloud.provider", json!("aws"))?;

            let _cond = { event.has_value("aws.bedrock_agentcore.resource.attributes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def attrs = ctx.aws.bedrock_agentcore.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def attrs = ctx.aws.bedrock_agentcore.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.set(
                "aws.bedrock_agentcore.provider_name",
                json!("aws_bedrock_agentcore"),
            )?;

            event.set(
                "aws.bedrock_agentcore.operation_name",
                json!("invoke_agent"),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("aws.bedrock_agentcore.session_id").cloned() {
                    event.set("aws.bedrock_agentcore.conversation_id", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("error.message") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("aws.bedrock_agentcore.request_payload.prompt")
                    && event
                        .get("aws.bedrock_agentcore.request_payload.prompt")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.aws.bedrock_agentcore.request_payload.prompt.length() > 32766) {\n  ctx.aws.bedrock_agentcore.prompt_hash = ctx.aws.bedrock_agentcore.request_payload.prompt.sha1(); \n  ctx.aws.bedrock_agentcore.request_payload.remove(\"prompt\");\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.aws.bedrock_agentcore.request_payload.prompt.length() > 32766) {\n  ctx.aws.bedrock_agentcore.prompt_hash = ctx.aws.bedrock_agentcore.request_payload.prompt.sha1(); \n  ctx.aws.bedrock_agentcore.request_payload.remove(\"prompt\");\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("service.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("service.name") {
                        if let Some(input) = event.get_string("service.name") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(".") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("aws.bedrock_agentcore.agent_name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(".") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("aws.bedrock_agentcore.endpoint_name", remaining));
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
            }

            event.remove("aws.bedrock_agentcore.attributes");
            event.remove("aws.bedrock_agentcore.resource");
            event.remove("aws.bedrock_agentcore.trace_id");
            event.remove("aws.bedrock_agentcore.span_id");
            event.remove("aws.bedrock_agentcore.event_timestamp");
            event.remove("aws.bedrock_agentcore.timeUnixNano");
            event.remove("aws.bedrock_agentcore.severityText");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
