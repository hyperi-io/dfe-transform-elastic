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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: if (ctx.json.containsKey('workspace_id') && ctx.json.workspace_id == null) {\n  ctx.json.workspace_id = 'Default';\n} ctx.json.entrySet().removeIf(e -> e.getValue() == null);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json.containsKey('workspace_id') && ctx.json.workspace_id == null) {\n  ctx.json.workspace_id = 'Default';\n} ctx.json.entrySet().removeIf(e -> e.getValue() == null);"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.bucket_start_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.bucket_start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.bucket_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.bucket_start_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.bucket_start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("anthropic.usage.bucket_start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.bucket_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.bucket_end_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.bucket_end_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("anthropic.usage.bucket_end_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.bucket_end_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.model") {
                event.rename("json.model", "anthropic.usage.model")?;
            }

            if event.has_value("json.workspace_id") {
                event.rename("json.workspace_id", "anthropic.usage.workspace_id")?;
            }

            if event.has_value("json.api_key_id") {
                event.rename("json.api_key_id", "anthropic.usage.api_key_id")?;
            }

            if event.has_value("json.service_tier") {
                event.rename("json.service_tier", "anthropic.usage.service_tier")?;
            }

            if event.has_value("json.context_window") {
                event.rename("json.context_window", "anthropic.usage.context_window")?;
            }

            if event.has_value("json.inference_geo") {
                event.rename("json.inference_geo", "anthropic.usage.inference_geo")?;
            }

            if event.has_value("json.speed") {
                event.rename("json.speed", "anthropic.usage.speed")?;
            }

            if event.has_value("json.uncached_input_tokens") {
                event.rename(
                    "json.uncached_input_tokens",
                    "anthropic.usage.uncached_input_tokens",
                )?;
            }

            if event.has_value("json.cache_read_input_tokens") {
                event.rename(
                    "json.cache_read_input_tokens",
                    "anthropic.usage.cached_input_tokens",
                )?;
            }

            let _cond = { event.has_value("json.cache_creation") };
            if _cond {
                // Painless script
                // Source: long total = 0; if (ctx.json.cache_creation.containsKey('ephemeral_5m_input_tokens')) {\n  total += ctx.json.cache_creation.ephemeral_5m_input_tokens;\n} if (ctx.json.cache_creation.containsKey('ephemeral_1h_input_tokens')) {\n  total += ctx.json.cache_creation.ephemeral_1h_input_tokens;\n} ctx.anthropic = ctx.anthropic != null ? ctx.anthropic : new HashMap(); ctx.anthropic.usage = ctx.anthropic.usage != null ? ctx.anthropic.usage : new HashMap(); ctx.anthropic.usage.cache_creation_input_tokens = total;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"long total = 0; if (ctx.json.cache_creation.containsKey('ephemeral_5m_input_tokens')) {\n  total += ctx.json.cache_creation.ephemeral_5m_input_tokens;\n} if (ctx.json.cache_creation.containsKey('ephemeral_1h_input_tokens')) {\n  total += ctx.json.cache_creation.ephemeral_1h_input_tokens;\n} ctx.anthropic = ctx.anthropic != null ? ctx.anthropic : new HashMap(); ctx.anthropic.usage = ctx.anthropic.usage != null ? ctx.anthropic.usage : new HashMap(); ctx.anthropic.usage.cache_creation_input_tokens = total;"#
                    ),
                )?;
            }

            if event.has_value("json.output_tokens") {
                event.rename("json.output_tokens", "anthropic.usage.output_tokens")?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("api"))?;

            event.append("event.type", json!("info"))?;

            event.remove("json");

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
