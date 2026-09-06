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
                (event.get("tags").is_some_and(|v| v.is_array())
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
                    || (event.get("attributes").is_some_and(|v| v.is_object())
                        && event.has("attributes.elastic.preserve_original_event")
                        && event.get_str("attributes.elastic.preserve_original_event")
                            == Some("true"))
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:]; ctx.event.original = Json.dump(ctx); ctx.tags = ctx.tags ?: []; if (!ctx.tags.contains('preserve_original_event')) {\n  ctx.tags.add('preserve_original_event');\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:]; ctx.event.original = Json.dump(ctx); ctx.tags = ctx.tags ?: []; if (!ctx.tags.contains('preserve_original_event')) {\n  ctx.tags.add('preserve_original_event');\n}\n"#
                    ),
                )?;
            }

            event.remove("attributes.elastic.preserve_original_event");

            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = { event.has_value("event_name") };
            if _cond {
                // Painless script
                // Source: def name = ctx.event_name; ctx.event = ctx.event ?: new HashMap(); ctx.event.kind = 'event'; if (name == 'user_prompt' || name == 'skill_activated') {\n  return;\n} if (name == 'tool_decision') {\n  ctx.event.category = ['iam'];\n  ctx.event.type = ['info'];\n  return;\n} def entry = params.categories.getOrDefault(name, null); if (entry != null) {\n  ctx.event.category = entry.category;\n  ctx.event.type = entry.type;\n} else {\n  ctx.event.category = ['host'];\n  ctx.event.type = ['info'];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def name = ctx.event_name; ctx.event = ctx.event ?: new HashMap(); ctx.event.kind = 'event'; if (name == 'user_prompt' || name == 'skill_activated') {\n  return;\n} if (name == 'tool_decision') {\n  ctx.event.category = ['iam'];\n  ctx.event.type = ['info'];\n  return;\n} def entry = params.categories.getOrDefault(name, null); if (entry != null) {\n  ctx.event.category = entry.category;\n  ctx.event.type = entry.type;\n} else {\n  ctx.event.category = ['host'];\n  ctx.event.type = ['info'];\n}\n"#
                    ),
                    cached_params!(
                        "{\"categories\":{\"tool_result\":{\"category\":[\"process\"],\"type\":[\"info\"]},\"api_request\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"api_error\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"api_retries_exhausted\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"api_refusal\":{\"category\":[\"api\"],\"type\":[\"denied\"]},\"auth\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"permission_mode_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"mcp_server_connection\":{\"category\":[\"network\"],\"type\":[\"connection\"]},\"hook_registered\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"hook_execution_start\":{\"category\":[\"process\"],\"type\":[\"start\"]},\"hook_execution_complete\":{\"category\":[\"process\"],\"type\":[\"end\"]},\"plugin_loaded\":{\"category\":[\"library\"],\"type\":[\"start\"]},\"plugin_installed\":{\"category\":[\"package\"],\"type\":[\"info\"]}}}"
                    ),
                )?;
            }

            if event.has_value("attributes.prompt") {
                event.rename("attributes.prompt", "attributes.prompt_text")?;
            }

            let _cond = { event.has_value("attributes") };
            if _cond {
                dot_expand(event, "attributes", "*")?;
            }

            if event.has_value("attributes") {
                event.rename("attributes", "claude_cowork.events")?;
            }

            event.remove("body");
            event.remove("event_name");

            if event.has_value("claude_cowork.events.duration_ms") {
                if let Some(val) = event.get("claude_cowork.events.duration_ms") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.duration_ms".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.duration_ms", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.event.sequence") {
                if let Some(val) = event.get("claude_cowork.events.event.sequence") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.event.sequence".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.event.sequence", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.tool_input_size_bytes") {
                if let Some(val) = event.get("claude_cowork.events.tool_input_size_bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.tool_input_size_bytes".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.tool_input_size_bytes", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.tool_result_size_bytes") {
                if let Some(val) = event.get("claude_cowork.events.tool_result_size_bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.tool_result_size_bytes".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.tool_result_size_bytes", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.input_tokens") {
                if let Some(val) = event.get("claude_cowork.events.input_tokens") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.input_tokens".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.input_tokens", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.output_tokens") {
                if let Some(val) = event.get("claude_cowork.events.output_tokens") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.output_tokens".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.output_tokens", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.cache_read_tokens") {
                if let Some(val) = event.get("claude_cowork.events.cache_read_tokens") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.cache_read_tokens".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.cache_read_tokens", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.cache_creation_tokens") {
                if let Some(val) = event.get("claude_cowork.events.cache_creation_tokens") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.cache_creation_tokens".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.cache_creation_tokens", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.cost_usd") {
                if let Some(val) = event.get("claude_cowork.events.cost_usd") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.cost_usd".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.cost_usd", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.cost_usd_micros") {
                if let Some(val) = event.get("claude_cowork.events.cost_usd_micros") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.cost_usd_micros".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.cost_usd_micros", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.prompt_length") {
                if let Some(val) = event.get("claude_cowork.events.prompt_length") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.prompt_length".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.prompt_length", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.num_hooks") {
                if let Some(val) = event.get("claude_cowork.events.num_hooks") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.num_hooks".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.num_hooks", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.num_success") {
                if let Some(val) = event.get("claude_cowork.events.num_success") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.num_success".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.num_success", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.num_blocking") {
                if let Some(val) = event.get("claude_cowork.events.num_blocking") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.num_blocking".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.num_blocking", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.num_non_blocking_error") {
                if let Some(val) = event.get("claude_cowork.events.num_non_blocking_error") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.num_non_blocking_error".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.num_non_blocking_error", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.num_cancelled") {
                if let Some(val) = event.get("claude_cowork.events.num_cancelled") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.num_cancelled".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.num_cancelled", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.total_duration_ms") {
                if let Some(val) = event.get("claude_cowork.events.total_duration_ms") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.total_duration_ms".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.total_duration_ms", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.agent_path_count") {
                if let Some(val) = event.get("claude_cowork.events.agent_path_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.agent_path_count".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.agent_path_count", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.command_path_count") {
                if let Some(val) = event.get("claude_cowork.events.command_path_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.command_path_count".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.command_path_count", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.skill_path_count") {
                if let Some(val) = event.get("claude_cowork.events.skill_path_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.skill_path_count".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.skill_path_count", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.is_plugin") {
                if let Some(val) = event.get("claude_cowork.events.is_plugin") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.is_plugin".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.is_plugin", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.has_hooks") {
                if let Some(val) = event.get("claude_cowork.events.has_hooks") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.has_hooks".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.has_hooks", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.has_mcp") {
                if let Some(val) = event.get("claude_cowork.events.has_mcp") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.has_mcp".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.has_mcp", converted)?;
                }
            }

            if event.has_value("claude_cowork.events.host_owned_mcp") {
                if let Some(val) = event.get("claude_cowork.events.host_owned_mcp") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "claude_cowork.events.host_owned_mcp".into(),
                            message,
                        }
                    })?;
                    event.set("claude_cowork.events.host_owned_mcp", converted)?;
                }
            }

            let _cond = { event.has_value("claude_cowork.events.event.name") };
            if _cond {
                event.set("gen_ai.provider.name", json!("anthropic"))?;
            }

            let _cond = { event.has_value("claude_cowork.events.model") };
            if _cond {
                if event.has_value("claude_cowork.events.model") {
                    event.rename("claude_cowork.events.model", "gen_ai.response.model")?;
                }
            }

            let _cond = { event.has_value("gen_ai.response.model") };
            if _cond {
                if let Some(v) = event.get("gen_ai.response.model").cloned() {
                    event.set("gen_ai.request.model", v)?;
                }
            }

            if event.has_value("claude_cowork.events.request_id") {
                event.rename("claude_cowork.events.request_id", "gen_ai.response.id")?;
            }

            if event.has_value("claude_cowork.events.input_tokens") {
                event.rename(
                    "claude_cowork.events.input_tokens",
                    "gen_ai.usage.input_tokens",
                )?;
            }

            if event.has_value("claude_cowork.events.output_tokens") {
                event.rename(
                    "claude_cowork.events.output_tokens",
                    "gen_ai.usage.output_tokens",
                )?;
            }

            if event.has_value("claude_cowork.events.cache_read_tokens") {
                event.rename(
                    "claude_cowork.events.cache_read_tokens",
                    "gen_ai.usage.cache_read.input_tokens",
                )?;
            }

            if event.has_value("claude_cowork.events.cache_creation_tokens") {
                event.rename(
                    "claude_cowork.events.cache_creation_tokens",
                    "gen_ai.usage.cache_creation.input_tokens",
                )?;
            }

            if event.has_value("claude_cowork.events.query_source") {
                event.rename("claude_cowork.events.query_source", "gen_ai.operation.name")?;
            }

            let _cond = { event.has_value("claude_cowork.events.mcp_tool.name") };
            if _cond {
                if event.has_value("claude_cowork.events.mcp_tool.name") {
                    event.rename("claude_cowork.events.mcp_tool.name", "gen_ai.tool.name")?;
                }
            }

            let _cond = {
                !event.has_value("gen_ai.tool.name")
                    && event.has_value("claude_cowork.events.tool_name")
            };
            if _cond {
                if event.has_value("claude_cowork.events.tool_name") {
                    event.rename("claude_cowork.events.tool_name", "gen_ai.tool.name")?;
                }
            }

            let _cond = {
                event.has_value("gen_ai.tool.name")
                    && event.has_value("claude_cowork.events.tool_name")
            };
            if _cond {
                event.remove("claude_cowork.events.tool_name");
            }

            if event.has_value("claude_cowork.events.tool_use_id") {
                event.rename("claude_cowork.events.tool_use_id", "gen_ai.tool.call.id")?;
            }

            if event.has_value("claude_cowork.events.error_code") {
                event.rename("claude_cowork.events.error_code", "error.code")?;
            }

            if event.has_value("claude_cowork.events.error_type") {
                event.rename("claude_cowork.events.error_type", "error.type")?;
            }

            if event.has_value("claude_cowork.events.organization.id") {
                event.rename("claude_cowork.events.organization.id", "organization.id")?;
            }

            if event.has_value("claude_cowork.events.user.email") {
                event.rename("claude_cowork.events.user.email", "user.email")?;
            }

            if event.has_value("claude_cowork.events.user.id") {
                event.rename("claude_cowork.events.user.id", "user.id")?;
            }

            let _cond = { event.has_value("claude_cowork.events.event.name") };
            if _cond {
                if let Some(v) = event.get("claude_cowork.events.event.name").cloned() {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { event.has_value("claude_cowork.events.duration_ms") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = ((long)ctx.claude_cowork.events.duration_ms) * 1000000L;\n
                scale_field(
                    event,
                    &ScaleField::new(
                        "claude_cowork.events.duration_ms",
                        "event.duration",
                        Factor::Long(1000000),
                    ),
                );
            }

            event.remove("claude_cowork.events.duration_ms");

            let _cond = { event.get_str("claude_cowork.events.success") == Some("true") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("claude_cowork.events.success") == Some("false")
                    || event.get_str("claude_cowork.events.event.name") == Some("api_error")
                    || event.get_str("claude_cowork.events.event.name")
                        == Some("api_retries_exhausted")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.get_str("claude_cowork.events.decision") == Some("accept")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.get_str("claude_cowork.events.decision") == Some("reject")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            event.set("event.provider", json!("claude-cowork"))?;

            let _cond = { event.has_value("claude_cowork.events.error") };
            if _cond {
                if let Some(v) = event.get("claude_cowork.events.error").cloned() {
                    event.set("event.reason", v)?;
                }
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n    map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nvoid handleList(List list) {\n    list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nhandleMap(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
