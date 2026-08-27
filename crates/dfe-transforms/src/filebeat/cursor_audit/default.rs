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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            let _cond = {
                event.get("json").is_some_and(|v| v.is_object()) && event.has_value("json.metadata")
            };
            if _cond {
                // Begin nested pipeline: "s3"
                let _cond = { event.get("json").is_some_and(|v| v.is_object()) };
                if _cond {
                    // Painless script
                    // Source: def doc = ctx.json;\nSet reserved = new HashSet(['metadata', 'team_id', 'ip_address', 'user_email']);\nString eventKey = null;\nfor (def k : doc.keySet()) {\n  if (!reserved.contains(k)) {\n    eventKey = k;\n    break;\n  }\n}\nif (eventKey == null) {\n  return;\n}\ndef payload = doc.remove(eventKey);\ndoc.event_type = eventKey;\nif (payload instanceof Map) {\n  doc.event_data = payload;\n} else if (payload != null) {\n  def wrap = new HashMap();\n  wrap.put('value', payload);\n  doc.event_data = wrap;\n} else {\n  doc.event_data = new HashMap();\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def doc = ctx.json;\nSet reserved = new HashSet(['metadata', 'team_id', 'ip_address', 'user_email']);\nString eventKey = null;\nfor (def k : doc.keySet()) {\n  if (!reserved.contains(k)) {\n    eventKey = k;\n    break;\n  }\n}\nif (eventKey == null) {\n  return;\n}\ndef payload = doc.remove(eventKey);\ndoc.event_type = eventKey;\nif (payload instanceof Map) {\n  doc.event_data = payload;\n} else if (payload != null) {\n  def wrap = new HashMap();\n  wrap.put('value', payload);\n  doc.event_data = wrap;\n} else {\n  doc.event_data = new HashMap();\n}"#
                        ),
                    )?;
                }
                if event.has_value("json.metadata.id") {
                    event.rename("json.metadata.id", "json.event_id")?;
                }
                if event.has_value("json.metadata.timestamp") {
                    event.rename("json.metadata.timestamp", "json.timestamp")?;
                }
                let _cond = {
                    event
                        .get("json.metadata.context")
                        .is_some_and(|v| v.is_object())
                };
                if _cond {
                    // Painless script
                    // Source: def ctxMap = ctx.json.metadata.context;\nfor (def entry : ctxMap.entrySet()) {\n  if (!ctx.json.containsKey(entry.getKey())) {\n    ctx.json[entry.getKey()] = entry.getValue();\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def ctxMap = ctx.json.metadata.context;\nfor (def entry : ctxMap.entrySet()) {\n  if (!ctx.json.containsKey(entry.getKey())) {\n    ctx.json[entry.getKey()] = entry.getValue();\n  }\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("json.privacy_mode")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(event, "json.privacy_mode", "_temp.privacy_json")?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "parse_privacy_mode_json_string",
                        )?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("_temp.privacy_json.privacyMode") };
                if _cond {
                    if let Some(v) = event.get("_temp.privacy_json.privacyMode").cloned() {
                        event.set("json.privacy_mode", v)?;
                    }
                }
                event.remove("json.metadata");
                // End nested pipeline: "s3"
            }

            let _cond = { event.has_value("json.event_id") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.event_id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = { event.has_value("json.event_type") };
            if _cond {
                // Painless script
                // Source: def action = ctx.json.event_type;\nctx.event = ctx.event ?: [:];\nctx.event.action = action;\nctx.event.kind = 'event';\ndef mapping = params.mappings.get(action);\nif (mapping == null && action.startsWith('bugbot_')) {\n  mapping = params.bugbot;\n}\nmapping = mapping ?: params.defaults;\ndef hm = new HashMap(mapping);\nhm.forEach((k, v) -> ctx.event[k] = v);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.json.event_type;\nctx.event = ctx.event ?: [:];\nctx.event.action = action;\nctx.event.kind = 'event';\ndef mapping = params.mappings.get(action);\nif (mapping == null && action.startsWith('bugbot_')) {\n  mapping = params.bugbot;\n}\nmapping = mapping ?: params.defaults;\ndef hm = new HashMap(mapping);\nhm.forEach((k, v) -> ctx.event[k] = v);"#
                    ),
                    cached_params!(
                        "{\"mappings\":{\"login\":{\"category\":[\"authentication\"],\"type\":[\"start\"]},\"logout\":{\"category\":[\"authentication\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"add_user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},\"remove_user\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"],\"outcome\":\"success\"},\"update_user_role\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"team_settings\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"team_api_key\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"user_api_key\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"organization_api_key\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"api_key\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"service_account\":{\"category\":[\"iam\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"invite_link\":{\"category\":[\"iam\"],\"type\":[\"user\",\"admin\"],\"outcome\":\"success\"},\"invite_email_sent\":{\"category\":[\"iam\"],\"type\":[\"user\",\"admin\"],\"outcome\":\"success\"},\"cloud_agent_secret\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\"],\"outcome\":\"success\"},\"cloud_agent_user_settings\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"protected_git_scope\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"protected_git_scope_access_check\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"privacy_mode\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"mcp_server_config\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"user_spend_limit\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"team_rule\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"team_repo\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"team_hook\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"team_command\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"create_directory_group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"],\"outcome\":\"success\"},\"delete_directory_group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"],\"outcome\":\"success\"},\"update_directory_group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"],\"outcome\":\"success\"},\"update_directory_group_permissions\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"],\"outcome\":\"success\"},\"add_user_to_directory_group\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"remove_user_from_directory_group\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"}},\"bugbot\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"defaults\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"unknown\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("json.timestamp") };
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
                    event.set("_ingest.on_failure_processor_tag", "date_parse_timestamp")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.event_id") {
                event.rename("json.event_id", "event.id")?;
            }

            event.remove("json.event_type");

            let _cond = {
                event.has_value("json.ip_address")
                    && event.get_str("json.ip_address") != Some("unknown")
            };
            if _cond {
                if event.has_value("json.ip_address") {
                    event.rename("json.ip_address", "source.ip")?;
                }
            }

            event.remove("json.ip_address");

            let _cond = {
                event.has_value("json.user_email")
                    && event.get_str("json.user_email") != Some("unknown")
            };
            if _cond {
                if event.has_value("json.user_email") {
                    event.rename("json.user_email", "user.email")?;
                }
            }

            event.remove("json.user_email");

            if event.has_value("json.team_id") {
                event.rename("json.team_id", "cursor.audit.team_id")?;
            }

            let _cond =
                { event.has_value("json.auth_id") && event.get_str("json.auth_id") != Some("") };
            if _cond {
                if event.has_value("json.auth_id") {
                    event.rename("json.auth_id", "cursor.audit.auth_id")?;
                }
            }

            event.remove("json.auth_id");

            if event.has_value("json.ghost_mode") {
                event.rename("json.ghost_mode", "cursor.audit.ghost_mode")?;
            }

            if event.has_value("json.request_id") {
                event.rename("json.request_id", "cursor.audit.request_id")?;
            }

            if event.has_value("json.privacy_mode") {
                event.rename("json.privacy_mode", "cursor.audit.privacy_mode")?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("add_user")
                        || event.get_str("event.action") == Some("remove_user")
                        || event.get_str("event.action") == Some("update_user_role")
                        || event.get_str("event.action") == Some("add_user_to_directory_group")
                        || event.get_str("event.action")
                            == Some("remove_user_from_directory_group"))
                    && event.has_value("json.event_data.user_email")
                    && event.get_str("json.event_data.user_email") != Some("unknown")
            };
            if _cond {
                if event.has_value("json.event_data.user_email") {
                    event.rename("json.event_data.user_email", "user.target.email")?;
                }
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("add_user")
                        || event.get_str("event.action") == Some("remove_user")
                        || event.get_str("event.action") == Some("update_user_role")
                        || event.get_str("event.action") == Some("add_user_to_directory_group")
                        || event.get_str("event.action")
                            == Some("remove_user_from_directory_group"))
                    && !event.has_value("user.target.email")
                    && event.has_value("json.event_data.email")
            };
            if _cond {
                if event.has_value("json.event_data.email") {
                    event.rename("json.event_data.email", "user.target.email")?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("user_spend_limit")
                    && event.has_value("json.event_data.target_user_email")
                    && event.get_str("json.event_data.target_user_email") != Some("unknown")
            };
            if _cond {
                if event.has_value("json.event_data.target_user_email") {
                    event.rename("json.event_data.target_user_email", "user.target.email")?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("user_spend_limit")
                    && !event.has_value("user.target.email")
                    && event.has_value("json.event_data.userEmail")
            };
            if _cond {
                if event.has_value("json.event_data.userEmail") {
                    event.rename("json.event_data.userEmail", "user.target.email")?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("invite_email_sent")
                    && event.has_value("json.event_data.recipient_email")
                    && event.get_str("json.event_data.recipient_email") != Some("unknown")
            };
            if _cond {
                if event.has_value("json.event_data.recipient_email") {
                    event.rename("json.event_data.recipient_email", "user.target.email")?;
                }
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("add_user_to_directory_group")
                        || event.get_str("event.action")
                            == Some("remove_user_from_directory_group"))
            };
            if _cond {
                if event.has_value("json.event_data.directory_group_id") {
                    event.rename("json.event_data.directory_group_id", "user.target.group.id")?;
                }
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("add_user_to_directory_group")
                        || event.get_str("event.action")
                            == Some("remove_user_from_directory_group"))
                    && !event.has_value("user.target.group.id")
            };
            if _cond {
                if event.has_value("json.event_data.group_id") {
                    event.rename("json.event_data.group_id", "user.target.group.id")?;
                }
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("add_user_to_directory_group")
                        || event.get_str("event.action")
                            == Some("remove_user_from_directory_group"))
            };
            if _cond {
                if event.has_value("json.event_data.user_id") {
                    event.rename("json.event_data.user_id", "user.target.id")?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("login")
                    && event.get_bool("json.event_data.success") == Some(true)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("login")
                    && event.get_bool("json.event_data.success") == Some(false)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("login")
                    && !event.has_value("json.event_data.success")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("protected_git_scope_access_check")
                    && event.get_str("json.event_data.result") == Some("allowed")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("protected_git_scope_access_check")
                    && event.get_str("json.event_data.result") == Some("blocked")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("team_hook")
                    && event
                        .get("json.event_data.url")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("json.event_data.url") {
                    event.rename("json.event_data.url", "url.original")?;
                }
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                if event.has_value("url.original") {
                    uri_parts(event, "url.original", "url", true, false)?;
                }
            }

            if event.has_value("json.event_data.action") {
                event.rename("json.event_data.action", "cursor.audit.action")?;
            }

            if event.has_value("json.event_data.method") {
                event.rename("json.event_data.method", "cursor.audit.source")?;
            }

            let _cond = { !event.has_value("cursor.audit.source") };
            if _cond {
                if event.has_value("json.event_data.source") {
                    event.rename("json.event_data.source", "cursor.audit.source")?;
                }
            }

            if event.has_value("json.event_data.setting_name") {
                event.rename("json.event_data.setting_name", "cursor.audit.setting_name")?;
            }

            let _cond = { !event.has_value("cursor.audit.setting_name") };
            if _cond {
                if event.has_value("json.event_data.setting") {
                    event.rename("json.event_data.setting", "cursor.audit.setting_name")?;
                }
            }

            if event.has_value("json.event_data.old_value") {
                event.rename("json.event_data.old_value", "cursor.audit.old_value")?;
            }

            if event.has_value("json.event_data.new_value") {
                event.rename("json.event_data.new_value", "cursor.audit.new_value")?;
            }

            let _cond = { !event.has_value("cursor.audit.old_value") };
            if _cond {
                if event.has_value("json.event_data.previous") {
                    event.rename("json.event_data.previous", "cursor.audit.old_value")?;
                }
            }

            let _cond = { !event.has_value("cursor.audit.new_value") };
            if _cond {
                if event.has_value("json.event_data.new") {
                    event.rename("json.event_data.new", "cursor.audit.new_value")?;
                }
            }

            let _cond = { !event.has_value("cursor.audit.new_value") };
            if _cond {
                if event.has_value("json.event_data.value") {
                    event.rename("json.event_data.value", "cursor.audit.new_value")?;
                }
            }

            if event.has_value("json.event_data.old_role") {
                event.rename("json.event_data.old_role", "cursor.audit.old_role")?;
            }

            if event.has_value("json.event_data.new_role") {
                event.rename("json.event_data.new_role", "cursor.audit.new_role")?;
            }

            let _cond = { !event.has_value("cursor.audit.old_role") };
            if _cond {
                if event.has_value("json.event_data.previous_role") {
                    event.rename("json.event_data.previous_role", "cursor.audit.old_role")?;
                }
            }

            if event.has_value("json.event_data.role") {
                event.rename("json.event_data.role", "cursor.audit.role")?;
            }

            let _cond = {
                event.get_str("event.action") == Some("add_user")
                    && event.has_value("cursor.audit.role")
            };
            if _cond {
                if event.has_value("cursor.audit.role") {
                    map_strings(
                        event,
                        "cursor.audit.role",
                        "cursor.audit.role",
                        str::to_lowercase,
                    )?;
                }
            }

            if event.has_value("json.event_data.rule_id") {
                event.rename("json.event_data.rule_id", "cursor.audit.rule_id")?;
            }

            if event.has_value("json.event_data.rule_name") {
                event.rename("json.event_data.rule_name", "cursor.audit.rule_name")?;
            }

            if event.has_value("json.event_data.command_id") {
                event.rename("json.event_data.command_id", "cursor.audit.command_id")?;
            }

            if event.has_value("json.event_data.command_name") {
                event.rename("json.event_data.command_name", "cursor.audit.command_name")?;
            }

            let _cond = {
                (event.get_str("event.action") == Some("team_rule")
                    || event.get_str("event.action") == Some("bugbot_team_rule"))
                    && !event.has_value("cursor.audit.rule_name")
            };
            if _cond {
                if event.has_value("json.event_data.name") {
                    event.rename("json.event_data.name", "cursor.audit.rule_name")?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("team_command")
                    && !event.has_value("cursor.audit.command_name")
            };
            if _cond {
                if event.has_value("json.event_data.name") {
                    event.rename("json.event_data.name", "cursor.audit.command_name")?;
                }
            }

            let _cond = {
                (event.get_str("event.action") == Some("create_directory_group")
                    || event.get_str("event.action") == Some("update_directory_group"))
                    && !event.has_value("cursor.audit.directory_group_name")
            };
            if _cond {
                if event.has_value("json.event_data.name") {
                    event.rename("json.event_data.name", "cursor.audit.directory_group_name")?;
                }
            }

            if event.has_value("json.event_data.hook_id") {
                event.rename("json.event_data.hook_id", "cursor.audit.hook_id")?;
            }

            if event.has_value("json.event_data.hook_step") {
                event.rename("json.event_data.hook_step", "cursor.audit.hook_step")?;
            }

            if event.has_value("json.event_data.hook_type") {
                event.rename("json.event_data.hook_type", "cursor.audit.hook_type")?;
            }

            if event.has_value("json.event_data.script_name") {
                event.rename("json.event_data.script_name", "cursor.audit.script_name")?;
            }

            if event.has_value("json.event_data.script_content") {
                event.rename(
                    "json.event_data.script_content",
                    "cursor.audit.script_content",
                )?;
            }

            if event.has_value("json.event_data.operating_systems") {
                event.rename(
                    "json.event_data.operating_systems",
                    "cursor.audit.operating_systems",
                )?;
            }

            if event.has_value("json.event_data.is_active") {
                event.rename("json.event_data.is_active", "cursor.audit.is_active")?;
            }

            if event.has_value("json.event_data.is_required") {
                event.rename("json.event_data.is_required", "cursor.audit.is_required")?;
            }

            if event.has_value("json.event_data.prompt_content") {
                event.rename(
                    "json.event_data.prompt_content",
                    "cursor.audit.prompt_content",
                )?;
            }

            if event.has_value("json.event_data.prompt_model") {
                event.rename("json.event_data.prompt_model", "cursor.audit.prompt_model")?;
            }

            if event.has_value("json.event_data.installation_id") {
                event.rename(
                    "json.event_data.installation_id",
                    "cursor.audit.installation_id",
                )?;
            }

            if event.has_value("json.event_data.github_app_type") {
                event.rename(
                    "json.event_data.github_app_type",
                    "cursor.audit.github_app_type",
                )?;
            }

            if event.has_value("json.event_data.github_hostname") {
                event.rename(
                    "json.event_data.github_hostname",
                    "cursor.audit.github_hostname",
                )?;
            }

            if event.has_value("json.event_data.repo_url") {
                event.rename("json.event_data.repo_url", "cursor.audit.repo_url")?;
            }

            if event.has_value("json.event_data.repo_node_id") {
                event.rename("json.event_data.repo_node_id", "cursor.audit.repo_node_id")?;
            }

            if event.has_value("json.event_data.repo_name") {
                event.rename("json.event_data.repo_name", "cursor.audit.repo_name")?;
            }

            if event.has_value("json.event_data.repos_updated") {
                event.rename(
                    "json.event_data.repos_updated",
                    "cursor.audit.repos_updated",
                )?;
            }

            let _cond = { !event.has_value("cursor.audit.repos_updated") };
            if _cond {
                if event.has_value("json.event_data.repo_count") {
                    event.rename("json.event_data.repo_count", "cursor.audit.repos_updated")?;
                }
            }

            if event.has_value("json.event_data.bugbot_enabled") {
                event.rename(
                    "json.event_data.bugbot_enabled",
                    "cursor.audit.bugbot_enabled",
                )?;
            }

            if event.has_value("json.event_data.operation") {
                event.rename("json.event_data.operation", "cursor.audit.operation")?;
            }

            if event.has_value("json.event_data.provider") {
                event.rename("json.event_data.provider", "cursor.audit.provider")?;
            }

            if event.has_value("json.event_data.enabled") {
                event.rename("json.event_data.enabled", "cursor.audit.enabled")?;
            }

            if event.has_value("json.event_data.secret_name") {
                event.rename("json.event_data.secret_name", "cursor.audit.secret_name")?;
            }

            if event.has_value("json.event_data.scope") {
                event.rename("json.event_data.scope", "cursor.audit.scope")?;
            }

            if event.has_value("json.event_data.server_name") {
                event.rename("json.event_data.server_name", "cursor.audit.server_name")?;
            }

            if event.has_value("json.event_data.server_type") {
                event.rename("json.event_data.server_type", "cursor.audit.server_type")?;
            }

            if event.has_value("json.event_data.scoped_to_repos") {
                event.rename(
                    "json.event_data.scoped_to_repos",
                    "cursor.audit.scoped_to_repos",
                )?;
            }

            if event.has_value("json.event_data.service_account_id") {
                event.rename(
                    "json.event_data.service_account_id",
                    "cursor.audit.service_account_id",
                )?;
            }

            if event.has_value("json.event_data.service_account_name") {
                event.rename(
                    "json.event_data.service_account_name",
                    "cursor.audit.service_account_name",
                )?;
            }

            if event.has_value("json.event_data.api_key_id") {
                event.rename("json.event_data.api_key_id", "cursor.audit.api_key_id")?;
            }

            if event.has_value("json.event_data.api_key_name") {
                event.rename("json.event_data.api_key_name", "cursor.audit.api_key_name")?;
            }

            let _cond = { !event.has_value("cursor.audit.api_key_id") };
            if _cond {
                if event.has_value("json.event_data.key_id") {
                    event.rename("json.event_data.key_id", "cursor.audit.api_key_id")?;
                }
            }

            if event.has_value("json.event_data.repo_scope") {
                event.rename("json.event_data.repo_scope", "cursor.audit.repo_scope")?;
            }

            if event.has_value("json.event_data.repo_scope_enabled") {
                event.rename(
                    "json.event_data.repo_scope_enabled",
                    "cursor.audit.repo_scope_enabled",
                )?;
            }

            if event.has_value("json.event_data.organization_id") {
                event.rename(
                    "json.event_data.organization_id",
                    "cursor.audit.organization_id",
                )?;
            }

            if event.has_value("json.event_data.scope_id") {
                event.rename("json.event_data.scope_id", "cursor.audit.scope_id")?;
            }

            if event.has_value("json.event_data.git_org_owner") {
                event.rename(
                    "json.event_data.git_org_owner",
                    "cursor.audit.git_org_owner",
                )?;
            }

            if event.has_value("json.event_data.git_provider") {
                event.rename("json.event_data.git_provider", "cursor.audit.git_provider")?;
            }

            let _cond = {
                event.has_value("json.event_data.git_enterprise_uuid")
                    && event.get_str("json.event_data.git_enterprise_uuid") != Some("")
            };
            if _cond {
                if event.has_value("json.event_data.git_enterprise_uuid") {
                    event.rename(
                        "json.event_data.git_enterprise_uuid",
                        "cursor.audit.git_enterprise_uuid",
                    )?;
                }
            }

            event.remove("json.event_data.git_enterprise_uuid");

            if event.has_value("json.event_data.user_id") {
                event.rename("json.event_data.user_id", "cursor.audit.user_id")?;
            }

            if event.has_value("json.event_data.reason") {
                event.rename("json.event_data.reason", "cursor.audit.reason")?;
            }

            if event.has_value("json.event_data.result") {
                event.rename("json.event_data.result", "cursor.audit.result")?;
            }

            if event.has_value("json.event_data.success") {
                event.rename("json.event_data.success", "cursor.audit.success")?;
            }

            if event.has_value("json.event_data.login_type") {
                event.rename("json.event_data.login_type", "cursor.audit.login_type")?;
            }

            let _cond = {
                event.has_value("json.event_data.invite_id")
                    && event.get_str("json.event_data.invite_id") != Some("")
            };
            if _cond {
                if event.has_value("json.event_data.invite_id") {
                    event.rename("json.event_data.invite_id", "cursor.audit.invite_id")?;
                }
            }

            event.remove("json.event_data.invite_id");

            let _cond = {
                event.has_value("json.event_data.invited_by_email")
                    && event.get_str("json.event_data.invited_by_email") != Some("")
            };
            if _cond {
                if event.has_value("json.event_data.invited_by_email") {
                    event.rename(
                        "json.event_data.invited_by_email",
                        "cursor.audit.invited_by_email",
                    )?;
                }
            }

            event.remove("json.event_data.invited_by_email");

            let _cond = {
                event.has_value("json.event_data.invited_by_user_id")
                    && event.get_str("json.event_data.invited_by_user_id") != Some("0")
            };
            if _cond {
                if event.has_value("json.event_data.invited_by_user_id") {
                    event.rename(
                        "json.event_data.invited_by_user_id",
                        "cursor.audit.invited_by_user_id",
                    )?;
                }
            }

            event.remove("json.event_data.invited_by_user_id");

            if event.has_value("json.event_data.creator_email") {
                event.rename(
                    "json.event_data.creator_email",
                    "cursor.audit.creator_email",
                )?;
            }

            if event.has_value("json.event_data.creator_user_id") {
                event.rename(
                    "json.event_data.creator_user_id",
                    "cursor.audit.creator_user_id",
                )?;
            }

            if event.has_value("json.event_data.expires_in_seconds") {
                event.rename(
                    "json.event_data.expires_in_seconds",
                    "cursor.audit.expires_in_seconds",
                )?;
            }

            if event.has_value("json.event_data.sender_email") {
                event.rename("json.event_data.sender_email", "cursor.audit.sender_email")?;
            }

            if event.has_value("json.event_data.sender_user_id") {
                event.rename(
                    "json.event_data.sender_user_id",
                    "cursor.audit.sender_user_id",
                )?;
            }

            if event.has_value("json.event_data.personal_message") {
                event.rename(
                    "json.event_data.personal_message",
                    "cursor.audit.personal_message",
                )?;
            }

            if event.has_value("json.event_data.old_limit_cents") {
                event.rename(
                    "json.event_data.old_limit_cents",
                    "cursor.audit.old_limit_cents",
                )?;
            }

            if event.has_value("json.event_data.new_limit_cents") {
                event.rename(
                    "json.event_data.new_limit_cents",
                    "cursor.audit.new_limit_cents",
                )?;
            }

            if event.has_value("json.event_data.spendLimitDollars") {
                event.rename(
                    "json.event_data.spendLimitDollars",
                    "cursor.audit.spend_limit_dollars",
                )?;
            }

            if event.has_value("json.event_data.old_privacy_mode") {
                event.rename(
                    "json.event_data.old_privacy_mode",
                    "cursor.audit.old_privacy_mode",
                )?;
            }

            if event.has_value("json.event_data.new_privacy_mode") {
                event.rename(
                    "json.event_data.new_privacy_mode",
                    "cursor.audit.new_privacy_mode",
                )?;
            }

            if event.has_value("json.event_data.directory_group_name") {
                event.rename(
                    "json.event_data.directory_group_name",
                    "cursor.audit.directory_group_name",
                )?;
            }

            if event.has_value("json.event_data.permissions") {
                event.rename("json.event_data.permissions", "cursor.audit.permissions")?;
            }

            if event.has_value("json.event_data.metadata") {
                event.rename("json.event_data.metadata", "cursor.audit.metadata")?;
            }

            event.remove("json.event_data.team_id");

            event.remove("json.event_data.ip_address");

            event.remove("json.event_data.user_agent");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cursor.audit.expires_in_seconds") {
                    if let Some(val) = event.get("cursor.audit.expires_in_seconds") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cursor.audit.expires_in_seconds".into(),
                                message,
                            }
                        })?;
                        event.set("cursor.audit.expires_in_seconds", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cursor_audit_expires_in_seconds_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cursor.audit.old_limit_cents") {
                    if let Some(val) = event.get("cursor.audit.old_limit_cents") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cursor.audit.old_limit_cents".into(),
                                message,
                            }
                        })?;
                        event.set("cursor.audit.old_limit_cents", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cursor_audit_old_limit_cents_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cursor.audit.new_limit_cents") {
                    if let Some(val) = event.get("cursor.audit.new_limit_cents") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cursor.audit.new_limit_cents".into(),
                                message,
                            }
                        })?;
                        event.set("cursor.audit.new_limit_cents", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cursor_audit_new_limit_cents_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cursor.audit.spend_limit_dollars") {
                    if let Some(val) = event.get("cursor.audit.spend_limit_dollars") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cursor.audit.spend_limit_dollars".into(),
                                message,
                            }
                        })?;
                        event.set("cursor.audit.spend_limit_dollars", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cursor_audit_spend_limit_dollars_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cursor.audit.repos_updated") {
                    if let Some(val) = event.get("cursor.audit.repos_updated") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cursor.audit.repos_updated".into(),
                                message,
                            }
                        })?;
                        event.set("cursor.audit.repos_updated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cursor_audit_repos_updated_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get("json.event_data").is_some_and(|v| v.is_object())
                    && event.get("json.event_data").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("json.event_data") {
                    event.rename("json.event_data", "cursor.audit.event_data")?;
                }
            }

            event.remove("json.event_data");

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
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

            let _cond =
                { event.has_value("user.email") && event.get_str("user.email") != Some("unknown") };
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

            let _cond = {
                event.has_value("user.target.email")
                    && event.get_str("user.target.email") != Some("unknown")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cursor.audit.creator_email")
                    && event.get_str("cursor.audit.creator_email") != Some("unknown")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cursor.audit.creator_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cursor.audit.sender_email")
                    && event.get_str("cursor.audit.sender_email") != Some("unknown")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cursor.audit.sender_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cursor.audit.sender_user_id")
                    && event.get_str("cursor.audit.sender_user_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cursor.audit.sender_user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set("event.provider", json!("cursor"))?;

            event.remove("json");

            event.remove("_temp");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
