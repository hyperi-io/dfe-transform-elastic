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
                    parse_json_field(event, "event.original", "aws.bedrock_agentcore.gateway")?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("aws.bedrock_agentcore.gateway.event_timestamp")
                {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "aws.bedrock_agentcore.gateway.event_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("aws.bedrock_agentcore.gateway.account_id") {
                event.rename(
                    "aws.bedrock_agentcore.gateway.account_id",
                    "cloud.account.id",
                )?;
            }

            if event.has_value("aws.bedrock_agentcore.gateway.traceId") {
                event.rename("aws.bedrock_agentcore.gateway.traceId", "trace.id")?;
            }

            let _cond = { !event.has_value("trace.id") };
            if _cond {
                if event.has_value("aws.bedrock_agentcore.gateway.trace_id") {
                    event.rename("aws.bedrock_agentcore.gateway.trace_id", "trace.id")?;
                }
            }

            if event.has_value("aws.bedrock_agentcore.gateway.spanId") {
                event.rename("aws.bedrock_agentcore.gateway.spanId", "span.id")?;
            }

            let _cond = { !event.has_value("span.id") };
            if _cond {
                if event.has_value("aws.bedrock_agentcore.gateway.span_id") {
                    event.rename("aws.bedrock_agentcore.gateway.span_id", "span.id")?;
                }
            }

            if event.has_value("aws.bedrock_agentcore.gateway.severityNumber") {
                event.rename(
                    "aws.bedrock_agentcore.gateway.severityNumber",
                    "aws.bedrock_agentcore.gateway.severity_number",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.bedrock_agentcore.gateway.severityText")
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
                Ok(())
            })();

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("cloud.service.name", json!("bedrock-agentcore"))?;

            event.set("cloud.provider", json!("aws"))?;

            let _cond = { event.has_value("aws.bedrock_agentcore.gateway.resource.attributes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def attrs = ctx.aws.bedrock_agentcore.gateway.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def attrs = ctx.aws.bedrock_agentcore.gateway.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.bedrock_agentcore.gateway.resource_arn") {
                    if let Some(input) =
                        event.get_string("aws.bedrock_agentcore.gateway.resource_arn")
                    {
                        // Grok pattern: arn:aws:bedrock-agentcore:%{DATA}:%{DATA}:gateway/%{DATA:aws.bedrock_agentcore.gateway.gateway_name}
                        if !cached_grok!("arn:aws:bedrock-agentcore:%{DATA}:%{DATA}:gateway/%{DATA:aws.bedrock_agentcore.gateway.gateway_name}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            if event.has_value("aws.bedrock_agentcore.gateway.body") {
                event.rename(
                    "aws.bedrock_agentcore.gateway.body",
                    "aws.bedrock_agentcore.gateway.payload_object",
                )?;
            }

            if let Some(v) = event
                .get("aws.bedrock_agentcore.gateway.payload_object.log")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.bedrock_agentcore.gateway.payload_object.log") {
                    if let Some(input) =
                        event.get_string("aws.bedrock_agentcore.gateway.payload_object.log")
                    {
                        // Grok pattern: tool %{DATA:aws.bedrock_agentcore.gateway.tool.name} from target %{DATA:aws.bedrock_agentcore.gateway.target}$
                        if !cached_grok!("tool %{DATA:aws.bedrock_agentcore.gateway.tool.name} from target %{DATA:aws.bedrock_agentcore.gateway.target}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("aws.bedrock_agentcore.gateway.tool.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            event.set(
                "aws.bedrock_agentcore.gateway.provider_name",
                json!("aws_bedrock_agentcore"),
            )?;

            event.set(
                "aws.bedrock_agentcore.gateway.operation_name",
                json!("invoke_gateway"),
            )?;

            let _cond = {
                event.get_bool("aws.bedrock_agentcore.gateway.payload_object.isError") != Some(true)
                    && !event.has_value("error.message")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_bool("aws.bedrock_agentcore.gateway.payload_object.isError") == Some(true)
                    || event.has_value("error.message")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            event.remove("aws.bedrock_agentcore.gateway.attributes");
            event.remove("aws.bedrock_agentcore.gateway.resource");
            event.remove("aws.bedrock_agentcore.gateway.trace_id");
            event.remove("aws.bedrock_agentcore.gateway.span_id");
            event.remove("aws.bedrock_agentcore.gateway.event_timestamp");
            event.remove("aws.bedrock_agentcore.gateway.timeUnixNano");
            event.remove("aws.bedrock_agentcore.gateway.severityText");

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
