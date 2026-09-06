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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
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

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("wiz.audit.timestamp", v)?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && event.get_str("json.action") != Some("")
            };
            if _cond {
                if event.has_value("json.action") {
                    map_strings(event, "json.action", "wiz.audit.action", |s| {
                        s.trim().to_string()
                    })?;
                }
            }

            let _cond = {
                event.get("wiz.audit.action").is_some_and(|v| v.is_string())
                    && event.get_str("wiz.audit.action") != Some("")
            };
            if _cond {
                if event.has_value("wiz.audit.action") {
                    map_strings(
                        event,
                        "wiz.audit.action",
                        "wiz.audit.action",
                        str::to_lowercase,
                    )?;
                }
            }

            let _cond = {
                event.get("wiz.audit.action").is_some_and(|v| v.is_string())
                    && event.get_str("wiz.audit.action") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("wiz.audit.action") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("wiz.audit.action", Value::Array(parts))?;
                }
            }

            let _cond = { event.get("wiz.audit.action").is_some_and(|v| v.is_array()) };
            if _cond {
                let joined = event
                    .get("wiz.audit.action")
                    .and_then(|v| join_values(v, "-"));
                if let Some(joined) = joined {
                    event.set("event.action", json!(joined))?;
                }
            }

            event.remove("wiz.audit.action");

            if event.has_value("json.action") {
                event.rename("json.action", "wiz.audit.action")?;
            }

            let _cond = {
                event.get("wiz.audit.action").is_some_and(|v| v.is_string())
                    && event.get_str("wiz.audit.action") != Some("")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.wiz.audit.action.contains('Login')) {\n  ctx.event.category = ['authentication'];\n} else if (ctx.wiz.audit.action.contains('Report')){\n  ctx.event.category = ['file'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.wiz.audit.action.contains('Login')) {\n  ctx.event.category = ['authentication'];\n} else if (ctx.wiz.audit.action.contains('Report')){\n  ctx.event.category = ['file'];\n}"#
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "wiz.audit.id")?;
            }

            if let Some(v) = event
                .get("wiz.audit.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            event.set("event.kind", json!("event"))?;

            if event.has_value("json.status") {
                event.rename("json.status", "wiz.audit.status")?;
            }

            let _cond = { event.get("wiz.audit.status").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.get("wiz.audit.status").is_some_and(|v| v.is_string())
                    && event
                        .get_str("wiz.audit.status")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get("wiz.audit.status").is_some_and(|v| v.is_string())
                    && event
                        .get_str("wiz.audit.status")
                        .is_some_and(|s| s.to_lowercase().contains("fail"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("wiz.audit.action") && event.get_str("wiz.audit.action") != Some("")
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            if event.has_value("json.requestId") {
                event.rename("json.requestId", "wiz.audit.request_id")?;
            }

            if let Some(v) = event
                .get("wiz.audit.request_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.id", v)?;
            }

            let _cond = { event.get_str("json.sourceIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.sourceIP") {
                        if let Some(val) = event.get("json.sourceIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.sourceIP".into(),
                                    message,
                                }
                            })?;
                            event.set("wiz.audit.source_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_sourceIP_to_ip")?;
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

            if let Some(v) = event
                .get("wiz.audit.source_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("wiz.audit.source_ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.userAgent") {
                event.rename("json.userAgent", "wiz.audit.user_agent")?;
            }

            if let Some(ua_str) = event.get_string("wiz.audit.user_agent") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
                    event.set("user_agent.original", json!(ua_str))?;
                    if let Some(name) = ua.name {
                        event.set("user_agent.name", json!(name))?;
                    }
                    if let Some(version) = ua.version {
                        event.set("user_agent.version", json!(version))?;
                    }
                    if let Some(os_name) = ua.os_name {
                        event.set("user_agent.os.name", json!(os_name))?;
                        if let Some(os_version) = ua.os_version {
                            event.set("user_agent.os.version", json!(os_version))?;
                            event.set(
                                "user_agent.os.full",
                                json!(format!("{} {}", os_name, os_version)),
                            )?;
                        }
                    }
                    if let Some(device) = ua.device {
                        event.set("user_agent.device.name", json!(device))?;
                    }
                }
            }

            if event.has_value("json.user.id") {
                event.rename("json.user.id", "wiz.audit.user.id")?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("wiz.audit.user.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("wiz.audit.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.user.name") {
                event.rename("json.user.name", "wiz.audit.user.name")?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("wiz.audit.user.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("wiz.audit.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.actionParameters.clientID") {
                event.rename(
                    "json.actionParameters.clientID",
                    "wiz.audit.action_parameters.client_id",
                )?;
            }

            if event.has_value("json.actionParameters.groups") {
                event.rename(
                    "json.actionParameters.groups",
                    "wiz.audit.action_parameters.groups",
                )?;
            }

            if event.has_value("json.actionParameters.name") {
                event.rename(
                    "json.actionParameters.name",
                    "wiz.audit.action_parameters.name",
                )?;
            }

            if event.has_value("json.actionParameters.products") {
                event.rename(
                    "json.actionParameters.products",
                    "wiz.audit.action_parameters.products",
                )?;
            }

            if event.has_value("json.actionParameters.role") {
                event.rename(
                    "json.actionParameters.role",
                    "wiz.audit.action_parameters.role",
                )?;
            }

            if event.has_value("json.actionParameters.scopes") {
                event.rename(
                    "json.actionParameters.scopes",
                    "wiz.audit.action_parameters.scopes",
                )?;
            }

            if event.has_value("json.actionParameters.userEmail") {
                event.rename(
                    "json.actionParameters.userEmail",
                    "wiz.audit.action_parameters.user.email",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("wiz.audit.action_parameters.user.email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.actionParameters.userID") {
                event.rename(
                    "json.actionParameters.userID",
                    "wiz.audit.action_parameters.user.id",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("wiz.audit.action_parameters.user.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.actionParameters.userpoolID") {
                event.rename(
                    "json.actionParameters.userpoolID",
                    "wiz.audit.action_parameters.userpool_id",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("wiz.audit.action_parameters.userpool_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.serviceAccount.id") {
                event.rename("json.serviceAccount.id", "wiz.audit.service_account.id")?;
            }

            if event.has_value("json.serviceAccount.name") {
                event.rename("json.serviceAccount.name", "wiz.audit.service_account.name")?;
            }

            event.remove("json");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("wiz.audit.timestamp");
                event.remove("wiz.audit.id");
                event.remove("wiz.audit.request_id");
                event.remove("wiz.audit.source_ip");
                event.remove("wiz.audit.user_agent");
                event.remove("wiz.audit.user.id");
                event.remove("wiz.audit.user.name");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
                            .get("_ingest.on_failure_pipeline")
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
