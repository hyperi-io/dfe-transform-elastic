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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.get("json.response").is_some_and(|v| v.is_array()) && event.get("json.response").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.action") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.description") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.object") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.username") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
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

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "admin_2fa_error",
                        "admin_account_switch",
                        "admin_activation_create",
                        "admin_activation_delete",
                        "admin_activate_duo_push",
                        "admin_create",
                        "admin_delete",
                        "admin_factor_restrictions_update",
                        "admin_login",
                        "admin_login_error",
                        "admin_reactivates_duo_push",
                        "admin_reset_password",
                        "admin_self_activate",
                        "admin_send_reset_password_email",
                        "admin_update",
                        "adminapi_request_ip_denied",
                        "bypass_create",
                        "bypass_delete",
                        "bypass_view",
                        "phone_associate",
                        "phone_create",
                        "phone_delete",
                        "phone_disassociate",
                        "phone_update",
                        "group_create",
                        "group_delete",
                        "group_update",
                        "user_bulk_activate",
                        "user_bulk_enroll",
                        "user_create",
                        "user_delete",
                        "user_import",
                        "user_pending_delete",
                        "user_restore",
                        "user_update",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("event.outcome", json!("success"))?;

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "ad_sync_failed",
                        "admin_2fa_error",
                        "admin_login_error",
                        "azure_sync_fail",
                        "openldap_sync_failed",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "activation_create_link",
                        "activation_delete_link",
                        "activation_send_link",
                        "admin_2fa_error",
                        "admin_account_switch",
                        "admin_activation_create",
                        "admin_activation_delete",
                        "admin_activate_duo_push",
                        "admin_create",
                        "admin_delete",
                        "admin_factor_restrictions_update",
                        "admin_login",
                        "admin_login_error",
                        "admin_reactivates_duo_push",
                        "admin_reset_password",
                        "admin_self_activate",
                        "admin_send_reset_password_email",
                        "admin_update",
                        "adminapi_request_ip_denied",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("admin"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "group_create",
                        "group_delete",
                        "group_update",
                        "integration_group_policy_add",
                        "integration_group_policy_remove",
                        "policy_create",
                        "policy_delete",
                        "policy_update",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "ad_sync_by_user_begin",
                        "ad_sync_by_user_finish",
                        "azure_sync_by_user_begin",
                        "azure_sync_by_user_finish",
                        "bypass_create",
                        "bypass_delete",
                        "bypass_view",
                        "openldap_sync_begin",
                        "openldap_sync_by_user_begin",
                        "phone_associate",
                        "phone_create",
                        "phone_delete",
                        "phone_disassociate",
                        "phone_update",
                        "user_bulk_activate",
                        "user_bulk_enroll",
                        "user_create",
                        "user_delete",
                        "user_import",
                        "user_pending_delete",
                        "user_restore",
                        "user_update",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && [
                        "ad_sync_begin",
                        "ad_sync_failed",
                        "ad_sync_finish",
                        "azure_directory_create",
                        "azure_directory_update",
                        "azure_directory_delete",
                        "azure_sync_begin",
                        "azure_sync_finish",
                        "azure_sync_fail",
                        "create_child_customer",
                        "credits_update",
                        "customer_update",
                        "delete_child_customer",
                        "directory_create",
                        "directory_delete",
                        "directory_groups_update",
                        "directory_sync_pause",
                        "directory_sync_resume",
                        "directory_update",
                        "edition_update",
                        "feature_add",
                        "feature_delete",
                        "hardtoken_create",
                        "hardtoken_delete",
                        "hardtoken_resync",
                        "hardtoken_update",
                        "integration_create",
                        "integration_delete",
                        "integration_policy_assign",
                        "integration_policy_unassign",
                        "integration_skey_view",
                        "integration_update",
                        "openldap_sync_by_user_finish",
                        "openldap_sync_config_download",
                        "openldap_sync_failed",
                        "openldap_sync_finish",
                        "regen_mobile",
                        "regen_sms",
                        "resend_enroll_codes",
                        "send_enroll_code",
                    ]
                    .contains(&event.get_str("json.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && event.get("json.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("create"))
                        }
                        serde_json::Value::String(s) => s.contains("create"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && event.get("json.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("update"))
                        }
                        serde_json::Value::String(s) => s.contains("update"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get("json.action").is_some_and(|v| v.is_string())
                    && event.get("json.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete"))
                        }
                        serde_json::Value::String(s) => s.contains("delete"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.has_value("json.description") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.description").cloned() {
                        event.set("message", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.description") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(event, "json.description", "cisco_duo.admin.flattened")?;
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("cisco_duo.admin.flattened.Errors")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cisco_duo.admin.errors", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.admin.flattened.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cisco_duo.admin.status", v)?;
            }

            event.set(
                "event.reason",
                json!(
                    event
                        .get("message")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.has_value("json.action") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.action").cloned() {
                        event.set("event.action", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.username").cloned() {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.action") == Some("admin_self_activate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("cisco_duo.admin.flattened.email").cloned() {
                        event.set("user.email", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.action") == Some("user_update") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("cisco_duo.admin.flattened.realname").cloned() {
                        event.set("user.changes.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.action") == Some("user_update") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("cisco_duo.admin.flattened.email").cloned() {
                        event.set("user.changes.email", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.object") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.object").cloned() {
                        event.set("user.target.name", v)?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.action") {
                event.rename("json.action", "cisco_duo.admin.action")?;
            }

            if event.has_value("json.username") {
                event.rename("json.username", "cisco_duo.admin.user.name")?;
            }

            let _cond = { event.has_value("json.object") };
            if _cond {
                event.rename("json.object", "cisco_duo.admin.action_performed_on")?;
            }

            let _cond = { event.has_value("cisco_duo.admin.flattened") };
            if _cond {
                event.remove("message");
                event.remove("event.reason");
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.remove("json").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json".into(),
                });
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
