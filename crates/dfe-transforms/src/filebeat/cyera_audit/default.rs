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
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.17.0"))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_a68ecd77",
                )?;
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

            event.set("event.kind", json!("event"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.createdAt") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.createdAt".into(),
                    });
                }
                if let Some(v) = event.get("json.frontegg_id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.frontegg_id".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_createdAt_2367397d",
                    )?;
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

            let _cond = {
                event.has_value("json.updatedAt") && event.get_str("json.updatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cyera.audit.updated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updatedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_updatedAt_f1a8c773",
                    )?;
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

            if event.has_value("json.action") {
                event.rename("json.action", "event.action")?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "message")?;
            }

            if event.has_value("json.email") {
                event.rename("json.email", "user.email")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ip") {
                    if let Some(val) = event.get("json.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ip_to_source_ip_07e29321",
                )?;
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "log.level")?;
            }

            let _cond = { event.has_value("json.userAgent") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.userAgent") {
                        if let Some(ua_str) = event.get_string("json.userAgent") {
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
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.environmentName") {
                event.rename("json.environmentName", "cyera.audit.environment_name")?;
            }

            if event.has_value("json.frontegg_id") {
                event.rename("json.frontegg_id", "event.id")?;
            }

            if event.has_value("json.tenantId") {
                event.rename("json.tenantId", "cyera.audit.tenant_id")?;
            }

            if event.has_value("json.vendorId") {
                event.rename("json.vendorId", "cyera.audit.vendor_id")?;
            }

            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: String action = ctx.event.action;\nMap entry = params.exact.get(action);\nif (entry == null) {\n  String a = action.toLowerCase();\n  if (a.contains('logged in') || a.contains('login') || a.contains('log in')) {\n    entry = ['category': ['authentication', 'session'], 'type': ['start'], 'outcome': a.contains('fail') ? 'failure' : 'success'];\n  } else if (a.contains('logged out') || a.contains('logout') || a.contains('log out')) {\n    entry = ['category': ['authentication', 'session'], 'type': ['end'], 'outcome': 'success'];\n  } else if (a.startsWith('removed user') || a.startsWith('deleted user')) {\n    entry = ['category': ['iam'], 'type': ['user', 'deletion']];\n  } else if (a.startsWith('added user') || a.startsWith('invited user')) {\n    entry = ['category': ['iam'], 'type': ['user', 'creation']];\n  }\n}\nif (entry == null) {\n  return;\n}\nif (entry.category != null) {\n  ctx.event.category = entry.category;\n}\nif (entry.type != null) {\n  ctx.event.type = entry.type;\n}\nif (entry.outcome != null) {\n  ctx.event.outcome = entry.outcome;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String action = ctx.event.action;\nMap entry = params.exact.get(action);\nif (entry == null) {\n  String a = action.toLowerCase();\n  if (a.contains('logged in') || a.contains('login') || a.contains('log in')) {\n    entry = ['category': ['authentication', 'session'], 'type': ['start'], 'outcome': a.contains('fail') ? 'failure' : 'success'];\n  } else if (a.contains('logged out') || a.contains('logout') || a.contains('log out')) {\n    entry = ['category': ['authentication', 'session'], 'type': ['end'], 'outcome': 'success'];\n  } else if (a.startsWith('removed user') || a.startsWith('deleted user')) {\n    entry = ['category': ['iam'], 'type': ['user', 'deletion']];\n  } else if (a.startsWith('added user') || a.startsWith('invited user')) {\n    entry = ['category': ['iam'], 'type': ['user', 'creation']];\n  }\n}\nif (entry == null) {\n  return;\n}\nif (entry.category != null) {\n  ctx.event.category = entry.category;\n}\nif (entry.type != null) {\n  ctx.event.type = entry.type;\n}\nif (entry.outcome != null) {\n  ctx.event.outcome = entry.outcome;\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"User logged in\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"User logged out\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"Created API key\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"Created personal API key\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"Deleted API key\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"Deleted personal API key\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"Added user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"Invited user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"Accepted invitation\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"Assigned roles\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Unassigned roles\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Updated profile\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Activated account\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Enabled MFA\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Disabled MFA\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}}"
                    ),
                )?;
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
