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

            parse_json_field(event, "event.original", "box")?;

            let _cond = {
                event.get("box.entries").is_some_and(|v| v.is_array()) && event.get("box.entries").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("_conf.client_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("box.event_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("box.source") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("box.recorded_at") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "box.recorded_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("box.session_id") {
                event.rename("box.session_id", "box.session.id")?;
            }

            event.remove("box.type");

            let _cond = { event.has_value("box.source") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.kind", json!("event"))?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.additional_details.shield_alert") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.kind", json!("alert"))?;
                    Ok(())
                })();
            }

            event.set("event.category", Value::Array(vec![]))?;

            event.set("event.type", Value::Array(vec![]))?;

            // Painless script
            // Source: def eventType = params.getOrDefault(ctx.box.event_type, null);\nif (eventType != null) {\n  for (category in eventType.map.get('category')) {\n    ctx.event.category.add(category);\n  }\n  for ( type in eventType.map.get('type')) {\n    ctx.event.type.add(type);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def eventType = params.getOrDefault(ctx.box.event_type, null);\nif (eventType != null) {\n  for (category in eventType.map.get('category')) {\n    ctx.event.category.add(category);\n  }\n  for ( type in eventType.map.get('type')) {\n    ctx.event.type.add(type);\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"ACCESS_GRANTED\":{\"map\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"access\",\"creation\"]}},\"ACCESS_REVOKED\":{\"map\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"access\",\"deletion\"]}},\"ADD_DEVICE_ASSOCIATION\":{\"map\":{\"category\":[\"host\"],\"type\":[\"access\"]}},\"ADD_LOGIN_ACTIVITY_DEVICE\":{\"map\":{\"category\":[\"host\",\"session\"],\"type\":[\"access\"]}},\"ADMIN_LOGIN\":{\"map\":{\"category\":[\"session\"],\"type\":[\"start\"]}},\"APPLICATION_CREATED\":{\"map\":{\"category\":[\"process\"],\"type\":[\"start\"]}},\"APPLICATION_PUBLIC_KEY_ADDED\":{\"map\":{\"category\":[\"process\",\"authentication\",\"iam\"],\"type\":[\"creation\"]}},\"APPLICATION_PUBLIC_KEY_DELETED\":{\"map\":{\"category\":[\"process\",\"authentication\",\"iam\"],\"type\":[\"deletion\"]}},\"CHANGE_ADMIN_ROLE\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"admin\"]}},\"CHANGE_FOLDER_PERMISSION\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\",\"change\"]}},\"COLLABORATION_ACCEPT\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"start\"]}},\"COLLABORATION_EXPIRATION\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"change\"]}},\"COLLABORATION_INVITE\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"start\"]}},\"COLLABORATION_REMOVE\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"end\"]}},\"COLLABORATION_ROLE_CHANGE\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"change\"]}},\"COLLAB_ADD_COLLABORATOR\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"start\"]}},\"COLLAB_INVITE_COLLABORATOR\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"start\"]}},\"COLLAB_REMOVE_COLLABORATOR\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"info\",\"change\"]}},\"COLLAB_ROLE_CHANGE\":{\"map\":{\"category\":[\"process\"],\"type\":[\"access\",\"change\"]}},\"COMMENT_CREATE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"info\"]}},\"COMMENT_DELETE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"info\"]}},\"CONTENT_ACCESS\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"CONTENT_WORKFLOW_ABNORMAL_DOWNLOAD_ACTIVITY\":{\"map\":{\"category\":[\"process\",\"threat\"],\"type\":[\"access\",\"indicator\"]}},\"CONTENT_WORKFLOW_AUTOMATION_ADD\":{\"map\":{\"category\":[\"process\"],\"type\":[\"change\"]}},\"CONTENT_WORKFLOW_AUTOMATION_DELETE\":{\"map\":{\"category\":[\"process\"],\"type\":[\"change\"]}},\"CONTENT_WORKFLOW_POLICY_ADD\":{\"map\":{\"category\":[\"process\",\"configuration\"],\"type\":[\"creation\"]}},\"CONTENT_WORKFLOW_SHARING_POLICY_VIOLATION\":{\"map\":{\"category\":[\"process\",\"threat\"],\"type\":[\"access\",\"indicator\"]}},\"CONTENT_WORKFLOW_UPLOAD_POLICY_VIOLATION\":{\"map\":{\"category\":[\"process\",\"threat\"],\"type\":[\"access\",\"indicator\"]}},\"COPY\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"DATA_RETENTION_CREATE_RETENTION\":{\"map\":{\"category\":[\"process\",\"configuration\"],\"type\":[\"creation\"]}},\"DATA_RETENTION_REMOVE_RETENTION\":{\"map\":{\"category\":[\"process\",\"configuration\"],\"type\":[\"deletion\"]}},\"DELETE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"deletion\"]}},\"DELETE_USER\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]}},\"DEVICE_TRUST_CHECK_FAILED\":{\"map\":{\"category\":[\"authentication\",\"threat\"],\"type\":[\"info\",\"indicator\"]}},\"DOWNLOAD\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"EDIT\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\",\"change\"]}},\"EDIT_USER\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"change\"]}},\"EMAIL_ALIAS_CONFIRM\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"user\"]}},\"EMAIL_ALIAS_REMOVE\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"user\"]}},\"ENABLE_TWO_FACTOR_AUTH\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"user\"]}},\"ENTERPRISE_APP_AUTHORIZATION_UPDATE\":{\"map\":{\"category\":[\"iam\",\"process\"],\"type\":[\"change\"]}},\"FAILED_LOGIN\":{\"map\":{\"category\":[\"iam\",\"threat\"],\"type\":[\"info\",\"indicator\"]}},\"FILE_MARKED_MALICIOUS\":{\"map\":{\"category\":[\"file\",\"threat\"],\"type\":[\"indicator\"]}},\"FILE_WATERMARKED_DOWNLOAD\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"GROUP_ADD_ITEM\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]}},\"GROUP_ADD_USER\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"creation\",\"group\"]}},\"GROUP_CREATION\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"creation\",\"group\"]}},\"GROUP_DELETION\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"group\"]}},\"GROUP_EDITED\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]}},\"GROUP_REMOVE_ITEM\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"group\"]}},\"GROUP_REMOVE_USER\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"group\",\"user\"]}},\"ITEM_COPY\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"ITEM_CREATE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"ITEM_DOWNLOAD\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"ITEM_MAKE_CURRENT_VERSION\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_MODIFY\":{\"map\":{\"category\":[\"file\"],\"type\":[\"change\"]}},\"ITEM_MOVE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_OPEN\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"ITEM_PREVIEW\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"ITEM_RENAME\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_SHARED\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_SHARED_CREATE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_SHARED_UNSHARE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_SHARED_UPDATE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"ITEM_SYNC\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"access\"]}},\"ITEM_TRASH\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"deletion\"]}},\"ITEM_UNDELETE_VIA_TRASH\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"creation\"]}},\"ITEM_UNSYNC\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"creation\"]}},\"ITEM_UPLOAD\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"LEGAL_HOLD_ASSIGNMENT_CREATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"LEGAL_HOLD_ASSIGNMENT_DELETE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"LEGAL_HOLD_POLICY_CREATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"LEGAL_HOLD_POLICY_DELETE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"LEGAL_HOLD_POLICY_UPDATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"LOCK\":{\"map\":{\"category\":[\"process\",\"file\"],\"type\":[\"access\"]}},\"LOCK_CREATE\":{\"map\":{\"category\":[\"process\",\"file\"],\"type\":[\"creation\"]}},\"LOCK_DESTROY\":{\"map\":{\"category\":[\"process\",\"file\"],\"type\":[\"deletion\"]}},\"LOGIN\":{\"map\":{\"category\":[\"session\"],\"type\":[\"start\"]}},\"MASTER_INVITE_ACCEPT\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"user\"]}},\"MASTER_INVITE_REJECT\":{\"map\":{\"category\":[\"process\"],\"type\":[\"info\"]}},\"METADATA_INSTANCE_CREATE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"info\"]}},\"METADATA_INSTANCE_DELETE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"change\"]}},\"METADATA_INSTANCE_UPDATE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"change\"]}},\"METADATA_TEMPLATE_CREATE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"info\"]}},\"METADATA_TEMPLATE_DELETE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"change\"]}},\"METADATA_TEMPLATE_UPDATE\":{\"map\":{\"category\":[\"database\"],\"type\":[\"change\"]}},\"MOVE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\",\"deletion\"]}},\"NEW_USER\":{\"map\":{\"category\":[\"iam\"],\"type\":[\"creation\"]}},\"PREVIEW\":{\"map\":{\"category\":[\"file\"],\"type\":[\"access\"]}},\"REMOVE_DEVICE_ASSOCIATION\":{\"map\":{\"category\":[\"host\"],\"type\":[\"end\"]}},\"REMOVE_LOGIN_ACTIVITY_DEVICE\":{\"map\":{\"category\":[\"host\",\"session\"],\"type\":[\"end\"]}},\"RENAME\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"change\"]}},\"RETENTION_POLICY_ASSIGNMENT_ADD\":{\"map\":{\"category\":[\"database\",\"process\"],\"type\":[\"info\"]}},\"SHARE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"creation\"]}},\"SHARE_EXPIRATION\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"SHIELD_ALERT\":{\"map\":{\"category\":[\"threat\"],\"type\":[\"indicator\"]}},\"SHIELD_EXTERNAL_COLLAB_ACCESS_BLOCKED\":{\"map\":{\"category\":[\"threat\",\"process\"],\"type\":[\"access\"]}},\"SHIELD_EXTERNAL_COLLAB_ACCESS_BLOCKED_MISSING_JUSTIFICATION\":{\"map\":{\"category\":[\"threat\",\"process\"],\"type\":[\"access\"]}},\"SHIELD_EXTERNAL_COLLAB_INVITE_BLOCKED\":{\"map\":{\"category\":[\"threat\",\"process\"],\"type\":[\"access\"]}},\"SHIELD_EXTERNAL_COLLAB_INVITE_BLOCKED_MISSING_JUSTIFICATION\":{\"map\":{\"category\":[\"threat\",\"process\"],\"type\":[\"access\"]}},\"SHIELD_JUSTIFICATION_APPROVAL\":{\"map\":{\"category\":[\"threat\",\"process\"],\"type\":[\"access\"]}},\"SIGN_DOCUMENT_ASSIGNED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"start\"]}},\"SIGN_DOCUMENT_CANCELLED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"end\"]}},\"SIGN_DOCUMENT_COMPLETED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"end\"]}},\"SIGN_DOCUMENT_CONVERTED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"change\"]}},\"SIGN_DOCUMENT_CREATED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"creation\"]}},\"SIGN_DOCUMENT_DECLINED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"info\"]}},\"SIGN_DOCUMENT_EXPIRED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"change\"]}},\"SIGN_DOCUMENT_SIGNED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"change\"]}},\"SIGN_DOCUMENT_VIEWED_BY_SIGNED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"access\"]}},\"SIGNER_DOWNLOADED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"access\"]}},\"SIGNER_FORWARDED\":{\"map\":{\"category\":[\"process\",\"iam\"],\"type\":[\"change\"]}},\"STORAGE_EXPIRATION\":{\"map\":{\"category\":[\"process\"],\"type\":[\"change\"]}},\"TAG_ITEM_CREATE\":{\"map\":{\"category\":[\"file\",\"database\"],\"type\":[\"creation\"]}},\"TASK_ASSIGNMENT_CREATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"TASK_ASSIGNMENT_DELETE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"TASK_ASSIGNMENT_UPDATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"TASK_CREATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"TASK_UPDATE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"TERMS_OF_SERVICE_ACCEPT\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"TERMS_OF_SERVICE_REJECT\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"info\"]}},\"UNDELETE\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"UNLOCK\":{\"map\":{\"category\":[\"process\"],\"type\":[\"change\"]}},\"UNSHARE\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"UPDATE_COLLABORATION_EXPIRATION\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"UPDATE_SHARE_EXPIRATION\":{\"map\":{\"category\":[\"process\",\"database\"],\"type\":[\"change\"]}},\"UPLOAD\":{\"map\":{\"category\":[\"file\"],\"type\":[\"creation\"]}},\"USER_AUTHENTICATE_OAUTH2_ACCESS_TOKEN_CREATE\":{\"map\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"user\",\"creation\"]}},\"WATERMARK_LABEL_CREATE\":{\"map\":{\"category\":[\"file\",\"process\"],\"type\":[\"creation\"]}},\"WATERMARK_LABEL_DELETE\":{\"map\":{\"category\":[\"file\",\"process\"],\"type\":[\"deletion\"]}}}"
                ),
            )?;

            if event.has_value("box.event_type") {
                event.rename("box.event_type", "event.action")?;
            }

            if event.has_value("box.event_id") {
                event.rename("box.event_id", "event.id")?;
            }

            let _cond = {
                !event.has_value("user.full_name")
                    && event.get_str("box.created_by.type") == Some("user")
            };
            if _cond {
                if let Some(v) = event
                    .get("box.created_by.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.full_name", v)?;
                }
            }

            let _cond =
                { !event.has_value("user.email") && event.has_value("box.created_by.login") };
            if _cond {
                if let Some(v) = event
                    .get("box.created_by.login")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event
                    .get("box.created_by.login")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_as_string("box.created_by.login")
                        .is_some_and(|s| s.len() > 2)
            };
            if _cond {
                if let Some(s) = event.get_string("box.created_by.login") {
                    let mut parts: Vec<Value> = s.split("@").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_tmp.created_login", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.get("_tmp.created_login").is_some_and(|v| v.is_array()) && event.get("_tmp.created_login").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                if let Some(v) = event.get("_tmp.created_login.0").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.get("_tmp.created_login").is_some_and(|v| v.is_array()) && event.get("_tmp.created_login").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                if let Some(v) = event.get("_tmp.created_login.1").cloned() {
                    event.set("user.domain", v)?;
                }
            }

            let _cond = {
                !event.has_value("user.id") && event.get_str("box.created_by.type") == Some("user")
            };
            if _cond {
                if let Some(v) = event
                    .get("box.created_by.login")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond =
                { event.has_value("user.email") && event.get_str("user.email") != Some("") };
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
                event.has_value("user.full_name") && event.get_str("user.full_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("user.target.full_name")
                    && event.has_value("box.accessible_by.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("box.accessible_by.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.full_name", v)?;
                }
            }

            let _cond = {
                !event.has_value("user.target.email") && event.has_value("box.accessible_by.login")
            };
            if _cond {
                if let Some(v) = event
                    .get("box.accessible_by.login")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.email", v)?;
                }
            }

            let _cond = {
                event
                    .get("box.accessible_by.login")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_as_string("box.accessible_by.login")
                        .is_some_and(|s| s.len() > 2)
            };
            if _cond {
                if let Some(s) = event.get_string("box.accessible_by.login") {
                    let mut parts: Vec<Value> = s.split("@").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_tmp.accessible_login", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.get("_tmp.accessible_login").is_some_and(|v| v.is_array()) && event.get("_tmp.accessible_login").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                if let Some(v) = event.get("_tmp.accessible_login.0").cloned() {
                    event.set("user.target.name", v)?;
                }
            }

            let _cond = {
                event.get("_tmp.accessible_login").is_some_and(|v| v.is_array()) && event.get("_tmp.accessible_login").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                if let Some(v) = event.get("_tmp.accessible_login.1").cloned() {
                    event.set("user.target.domain", v)?;
                }
            }

            let _cond = {
                !event.has_value("user.target.id")
                    && event.get_str("box.accessible_by.type") == Some("user")
            };
            if _cond {
                if event.has_value("box.accessible_by.login") {
                    event.rename("box.accessible_by.login", "user.target.id")?;
                }
            }

            let _cond = {
                event.has_value("user.target.email")
                    && event.get_str("user.target.email") != Some("")
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
                event.has_value("user.target.full_name")
                    && event.get_str("user.target.full_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("box.source.id") };
            if _cond {
                if event.has_value("box.source.folder_id") {
                    event.rename("box.source.folder_id", "box.source.id")?;
                }
            }

            if event.has_value("box.source.sequence_id") {
                if let Some(val) = event.get("box.source.sequence_id") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "box.source.sequence_id".into(),
                            message,
                        }
                    })?;
                    event.set("event.sequence", converted)?;
                }
            }

            if event.has_value("box.source.type") {
                event.rename("box.source.type", "file.type")?;
            }

            let _cond = { event.get_str("file.type") != Some("folder") };
            if _cond {
                if event.has_value("box.source.name") {
                    event.rename("box.source.name", "file.name")?;
                }
            }

            let _cond = { event.get_str("file.type") == Some("folder") };
            if _cond {
                if event.has_value("box.source.name") {
                    event.rename("box.source.name", "file.directory")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("box.source.created_at") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                        Some(parsed) => event.set("file.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "box.source.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("box.source.created_at");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("box.source.content_created_at") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                        Some(parsed) => event.set("file.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "box.source.content_created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("box.source.content_created_at");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("box.source.content_modified_at") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                        Some(parsed) => event.set("file.mtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "box.source.content_modified_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("box.source.content_modified_at");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("box.source.modified_at") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                        Some(parsed) => event.set("file.ctime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "box.source.modified_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("box.source.size") {
                event.rename("box.source.size", "file.size")?;
            }

            if event.has_value("box.source.file_version.sha1") {
                event.rename("box.source.file_version.sha1", "file.hash.sha1")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("box.additional_details.is_performed_by_admin") {
                    if let Some(val) = event.get("box.additional_details.is_performed_by_admin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "box.additional_details.is_performed_by_admin".into(),
                                message,
                            }
                        })?;
                        event.set("box.additional_details.is_performed_by_admin", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_performed_by_admin_bool",
                )?;
                if event
                    .remove("box.additional_details.is_performed_by_admin")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "box.additional_details.is_performed_by_admin".into(),
                    });
                }
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("box.ip_address") {
                    if let Some(val) = event.get("box.ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "box.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })();

            event.remove("box.ip_address");

            if event.has_value("box.additional_details.shield_alert.risk_score") {
                event.rename(
                    "box.additional_details.shield_alert.risk_score",
                    "event.risk_score",
                )?;
            }

            if event.has_value("box.additional_details.shield_alert.alert_summary.historical_period.downloaded_files_count") {
                    event.rename("box.additional_details.shield_alert.alert_summary.historical_period.downloaded_files_count", "threat.indicator.sightings")?;
                }

            if event.has_value("box.additional_details.shield_alert.rule_category") {
                event.rename(
                    "box.additional_details.shield_alert.rule_category",
                    "rule.category",
                )?;
            }

            if event.has_value("box.additional_details.shield_alert.rule_id") {
                if let Some(val) = event.get("box.additional_details.shield_alert.rule_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "box.additional_details.shield_alert.rule_id".into(),
                            message,
                        }
                    })?;
                    event.set("rule.id", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("box.additional_details.shield_alert.rule_id");
                Ok(())
            })();

            if event.has_value("box.additional_details.shield_alert.rule_name") {
                event.rename("box.additional_details.shield_alert.rule_name", "rule.name")?;
            }

            if event.has_value("box.additional_details.shield_alert.user.email") {
                event.rename(
                    "box.additional_details.shield_alert.user.email",
                    "user.effective.email",
                )?;
            }

            if event.has_value("box.additional_details.shield_alert.user.name") {
                event.rename(
                    "box.additional_details.shield_alert.user.name",
                    "user.effective.name",
                )?;
            }

            if event.has_value("box.additional_details.shield_alert.user.id") {
                if let Some(val) = event.get("box.additional_details.shield_alert.user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "box.additional_details.shield_alert.user.id".into(),
                            message,
                        }
                    })?;
                    event.set("user.effective.id", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("box.additional_details.shield_alert.user.id");
                Ok(())
            })();

            // Painless script
            // Source: // event.[category|type] _should_ already have been populated by SHIELD_ALERT mapping\nif (ctx.event?.action.equals(\"SHIELD_ALERT\")) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  if (ctx.related.ip == null) {\n    ctx.related.ip = new ArrayList();\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// event.[category|type] _should_ already have been populated by SHIELD_ALERT mapping\nif (ctx.event?.action.equals(\"SHIELD_ALERT\")) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  if (ctx.related.ip == null) {\n    ctx.related.ip = new ArrayList();\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Suspicious Sessions\")) {\n  return;\n}\nctx.event.category.add(\"network\");\nctx.event.type.add(\"access\");\nctx.event.type.add(\"connection\");\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nfor (session in ctx.box.additional_details?.shield_alert?.alert_summary?.sessions) {\n  if (session.session_type?.equals(\"suspicious\")) {\n    for (activity in session.activities) {\n      if (ctx.shield_date == null) {\n        ctx.shield_date = activity.occurred_at;\n      }\n      Map indicator = new HashMap();\n      indicator.indicator = new HashMap();\n      Map location = new HashMap();\n      location.put(\"lon\",activity.ip_info.longitude);\n      location.put(\"lat\",activity.ip_info.latitude);\n      Map geo = new HashMap();\n      geo.put(\"ip\",activity.ip_info.ip);\n      geo.put(\"location\",location);\n      indicator.indicator.put(\"geo\",geo);\n      indicator.indicator.put(\"description\",\n        \"IP \" + activity.ip_info.ip + \" was observed to \" +\n        activity.event_type + \" \" + activity.item_type + \" \" +\n        activity.item_path + \"/\" + activity.item_name + \" by \" +\n      activity.service_name);\n      indicator.indicator.put(\"provider\",activity.service_name);\n      indicator.indicator.put(\"type\",\"user-account\");\n      ctx.threat.enrichments.add(indicator);\n      ctx.related.ip.add(geo.ip);\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Suspicious Sessions\")) {\n  return;\n}\nctx.event.category.add(\"network\");\nctx.event.type.add(\"access\");\nctx.event.type.add(\"connection\");\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nfor (session in ctx.box.additional_details?.shield_alert?.alert_summary?.sessions) {\n  if (session.session_type?.equals(\"suspicious\")) {\n    for (activity in session.activities) {\n      if (ctx.shield_date == null) {\n        ctx.shield_date = activity.occurred_at;\n      }\n      Map indicator = new HashMap();\n      indicator.indicator = new HashMap();\n      Map location = new HashMap();\n      location.put(\"lon\",activity.ip_info.longitude);\n      location.put(\"lat\",activity.ip_info.latitude);\n      Map geo = new HashMap();\n      geo.put(\"ip\",activity.ip_info.ip);\n      geo.put(\"location\",location);\n      indicator.indicator.put(\"geo\",geo);\n      indicator.indicator.put(\"description\",\n        \"IP \" + activity.ip_info.ip + \" was observed to \" +\n        activity.event_type + \" \" + activity.item_type + \" \" +\n        activity.item_path + \"/\" + activity.item_name + \" by \" +\n      activity.service_name);\n      indicator.indicator.put(\"provider\",activity.service_name);\n      indicator.indicator.put(\"type\",\"user-account\");\n      ctx.threat.enrichments.add(indicator);\n      ctx.related.ip.add(geo.ip);\n    }\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Suspicious Locations\")) {\n  return;\n}\nctx.event.category.add(\"network\");\nctx.event.type.add(\"access\");\nctx.event.type.add(\"connection\");\nif (ctx?.threat?.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nfor (alert_activity in ctx.box.additional_details?.shield_alert?.alert_summary?.alert_activities) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = alert_activity.occurred_at;\n  }\n  Map indicator = new HashMap();\n  indicator.indicator = new HashMap();\n  Map location = new HashMap();\n  location.put(\"lon\",alert_activity.ip_info?.longitude);\n  location.put(\"lat\",alert_activity.ip_info?.latitude);\n  Map geo = new HashMap();\n  geo.put(\"ip\",alert_activity.ip_info.ip);\n  geo.put(\"location\",location);\n  indicator.indicator.put(\"geo\",geo);\n  indicator.indicator.put(\"description\",\n    \"IP \" + alert_activity.ip_info.ip + \" was observed to \" +\n    alert_activity.event_type + \" \" + alert_activity.item_type + \" \" +\n    alert_activity.item_path + \"/\" + alert_activity.item_name + \" by \" +\n    alert_activity.service_name);\n  indicator.indicator.put(\"provider\",alert_activity.service_name);\n  if (alert_activity.ip_info.ip.indexOf(\":\") != -1) {\n    indicator.indicator.put(\"type\",\"ipv6-addr\");\n  } else {\n    indicator.indicator.put(\"type\",\"ipv4-addr\");\n  }\n  ctx.threat.enrichments.add(indicator);\n  ctx.related.ip.add(geo.ip);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Suspicious Locations\")) {\n  return;\n}\nctx.event.category.add(\"network\");\nctx.event.type.add(\"access\");\nctx.event.type.add(\"connection\");\nif (ctx?.threat?.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nfor (alert_activity in ctx.box.additional_details?.shield_alert?.alert_summary?.alert_activities) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = alert_activity.occurred_at;\n  }\n  Map indicator = new HashMap();\n  indicator.indicator = new HashMap();\n  Map location = new HashMap();\n  location.put(\"lon\",alert_activity.ip_info?.longitude);\n  location.put(\"lat\",alert_activity.ip_info?.latitude);\n  Map geo = new HashMap();\n  geo.put(\"ip\",alert_activity.ip_info.ip);\n  geo.put(\"location\",location);\n  indicator.indicator.put(\"geo\",geo);\n  indicator.indicator.put(\"description\",\n    \"IP \" + alert_activity.ip_info.ip + \" was observed to \" +\n    alert_activity.event_type + \" \" + alert_activity.item_type + \" \" +\n    alert_activity.item_path + \"/\" + alert_activity.item_name + \" by \" +\n    alert_activity.service_name);\n  indicator.indicator.put(\"provider\",alert_activity.service_name);\n  if (alert_activity.ip_info.ip.indexOf(\":\") != -1) {\n    indicator.indicator.put(\"type\",\"ipv6-addr\");\n  } else {\n    indicator.indicator.put(\"type\",\"ipv4-addr\");\n  }\n  ctx.threat.enrichments.add(indicator);\n  ctx.related.ip.add(geo.ip);\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Anomalous Download\")) {\n  return;\n}\nctx.event.category.add(\"file\");\nctx.event.type.add(\"access\");\nif (ctx?.threat?.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nList l = new ArrayList();\nif (ctx.box.additional_details?.shield_alert?.alert_summary != null) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range?.start_date;\n  }\n  for (ip in ctx.box.additional_details.shield_alert.alert_summary.download_ips) {\n    l.add(ip.ip);\n    Map indicator = new HashMap();\n    indicator.indicator = new HashMap();\n    indicator.indicator.ip = ip.ip;\n    indicator.indicator.description = ctx.box.additional_details.shield_alert.alert_summary.description;\n    indicator.indicator.first_seen = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range.start_date;\n    indicator.indicator.last_seen = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range.end_date;\n    indicator.indicator.sightings = ctx.box.additional_details.shield_alert.alert_summary.historical_period.downloaded_files_count;\n    indicator.indicator.type = \"file\";\n    ctx.threat.enrichments.add(indicator);\n    ctx.related.ip.add(ip.ip);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Anomalous Download\")) {\n  return;\n}\nctx.event.category.add(\"file\");\nctx.event.type.add(\"access\");\nif (ctx?.threat?.enrichments == null) {\n  ctx.threat.enrichments = new ArrayList();\n}\nList l = new ArrayList();\nif (ctx.box.additional_details?.shield_alert?.alert_summary != null) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range?.start_date;\n  }\n  for (ip in ctx.box.additional_details.shield_alert.alert_summary.download_ips) {\n    l.add(ip.ip);\n    Map indicator = new HashMap();\n    indicator.indicator = new HashMap();\n    indicator.indicator.ip = ip.ip;\n    indicator.indicator.description = ctx.box.additional_details.shield_alert.alert_summary.description;\n    indicator.indicator.first_seen = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range.start_date;\n    indicator.indicator.last_seen = ctx.box.additional_details.shield_alert.alert_summary.anomaly_period.date_range.end_date;\n    indicator.indicator.sightings = ctx.box.additional_details.shield_alert.alert_summary.historical_period.downloaded_files_count;\n    indicator.indicator.type = \"file\";\n    ctx.threat.enrichments.add(indicator);\n    ctx.related.ip.add(ip.ip);\n  }\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Malicious Content\")) {\n  return;\n}\nctx.event.category.add(\"file\");\nctx.event.category.add(\"malware\");\nctx.event.type.add(\"info\");\nif (ctx?.threat?.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n}\nif (ctx.box?.additional_details?.shield_alert?.alert_summary?.upload_activity != null &&\n  ctx.box.additional_details.shield_alert.malware_info != null) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = ctx.box?.additional_details?.shield_alert?.alert_summary?.upload_activity?.occurred_at;\n  }\n  ctx.threat.indicator.description =\n    ctx.box.additional_details.shield_alert.malware_info.malware_name +\n    \", \" + ctx.box.additional_details.shield_alert.malware_info.family +\n    \", \" + ctx.box.additional_details.shield_alert.malware_info.file_name +\n    \" Detected by Box Shield from IP \" + ctx.box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.ip +\n    \". \" + ctx.box.additional_details.shield_alert.malware_info.description +\n    \" see \" + ctx.box.additional_details.shield_alert.malware_info.detail_link;\n  ctx.threat.indicator.ip = ctx.box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.ip;\n  ctx.threat.indicator.provider = ctx.box.additional_details.shield_alert.alert_summary.upload_activity.service_name;\n  ctx.threat.indicator.first_seen = ctx.box.additional_details.shield_alert.malware_info.first_seen;\n  ctx.threat.indicator.last_seen = ctx.box.additional_details.shield_alert.malware_info.last_seen;\n  ctx.threat.indicator.reference = ctx.box.additional_details.shield_alert.malware_info.detail_link;\n  ctx.related.ip.add(ctx.threat.indicator.ip);\n}\nctx.threat.indicator.type = \"software\";\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.rule?.category == null || !ctx.rule.category.equals(\"Malicious Content\")) {\n  return;\n}\nctx.event.category.add(\"file\");\nctx.event.category.add(\"malware\");\nctx.event.type.add(\"info\");\nif (ctx?.threat?.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n}\nif (ctx.box?.additional_details?.shield_alert?.alert_summary?.upload_activity != null &&\n  ctx.box.additional_details.shield_alert.malware_info != null) {\n  if (ctx.shield_date == null) {\n    ctx.shield_date = ctx.box?.additional_details?.shield_alert?.alert_summary?.upload_activity?.occurred_at;\n  }\n  ctx.threat.indicator.description =\n    ctx.box.additional_details.shield_alert.malware_info.malware_name +\n    \", \" + ctx.box.additional_details.shield_alert.malware_info.family +\n    \", \" + ctx.box.additional_details.shield_alert.malware_info.file_name +\n    \" Detected by Box Shield from IP \" + ctx.box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.ip +\n    \". \" + ctx.box.additional_details.shield_alert.malware_info.description +\n    \" see \" + ctx.box.additional_details.shield_alert.malware_info.detail_link;\n  ctx.threat.indicator.ip = ctx.box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.ip;\n  ctx.threat.indicator.provider = ctx.box.additional_details.shield_alert.alert_summary.upload_activity.service_name;\n  ctx.threat.indicator.first_seen = ctx.box.additional_details.shield_alert.malware_info.first_seen;\n  ctx.threat.indicator.last_seen = ctx.box.additional_details.shield_alert.malware_info.last_seen;\n  ctx.threat.indicator.reference = ctx.box.additional_details.shield_alert.malware_info.detail_link;\n  ctx.related.ip.add(ctx.threat.indicator.ip);\n}\nctx.threat.indicator.type = \"software\";\n"#
                ),
            )?;

            let _cond = {
                event.has_value("threat.indicator.first_seen")
                    && event.get_str("threat.indicator.first_seen") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threat.indicator.first_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.first_seen".into(),
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
                        "date_threat_indicator_first_seen",
                    )?;
                    if event.remove("threat.indicator.first_seen").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "threat.indicator.first_seen".into(),
                        });
                    }
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
                event.has_value("threat.indicator.last_seen")
                    && event.get_str("threat.indicator.last_seen") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threat.indicator.last_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.last_seen".into(),
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
                        "date_threat_indicator_last_seen",
                    )?;
                    if event.remove("threat.indicator.last_seen").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "threat.indicator.last_seen".into(),
                        });
                    }
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

            if event.has_value("related.ip") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("related.ip").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "related.ip",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("threat.enrichments") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("threat.enrichments").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.indicator.ip") {
                                    if let Some(val) = event.get("_ingest._value.indicator.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.indicator.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.indicator.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                if event.remove("_ingest._value.indicator.ip").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.indicator.ip".into(),
                                    });
                                }
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "threat.enrichments",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threat.indicator.ip") {
                    if let Some(val) = event.get("threat.indicator.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "threat.indicator.ip".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_indicator_ip")?;
                if event.remove("threat.indicator.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("box.additional_details.shield_alert.alert_summary.download_ips") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("box.additional_details.shield_alert.alert_summary.download_ips")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(val) = event.get("_ingest._value.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.ip", converted)?;
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "box.additional_details.shield_alert.alert_summary.download_ips",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            // Painless script
            // Source: if (ctx.event.category != null) {\n  ctx.event.category = ctx.event.category.stream()\n    .distinct()\n    .collect(Collectors.toList());\n}\nif (ctx.event.type != null) {\n  ctx.event.type = ctx.event.type.stream()\n    .distinct()\n    .collect(Collectors.toList());\n}\nif (ctx.related != null) {\n  if (ctx.related.ip != null) {\n    ctx.related.ip = ctx.related.ip.stream()\n      .filter(Objects::nonNull)\n      .distinct()\n      .collect(Collectors.toList());\n  }\n}\nif (ctx.box?.additional_details?.shield_alert?.alert_summary?.download_ips != null) {\n  ctx.box.additional_details.shield_alert.alert_summary.download_ips = ctx.box.additional_details.shield_alert.alert_summary.download_ips.stream()\n    .filter(Objects::nonNull)\n    .distinct()\n    .collect(Collectors.toList());\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.event.category != null) {\n  ctx.event.category = ctx.event.category.stream()\n    .distinct()\n    .collect(Collectors.toList());\n}\nif (ctx.event.type != null) {\n  ctx.event.type = ctx.event.type.stream()\n    .distinct()\n    .collect(Collectors.toList());\n}\nif (ctx.related != null) {\n  if (ctx.related.ip != null) {\n    ctx.related.ip = ctx.related.ip.stream()\n      .filter(Objects::nonNull)\n      .distinct()\n      .collect(Collectors.toList());\n  }\n}\nif (ctx.box?.additional_details?.shield_alert?.alert_summary?.download_ips != null) {\n  ctx.box.additional_details.shield_alert.alert_summary.download_ips = ctx.box.additional_details.shield_alert.alert_summary.download_ips.stream()\n    .filter(Objects::nonNull)\n    .distinct()\n    .collect(Collectors.toList());\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("shield_date") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("shield_date") {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'hh:mm:ssXXX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "shield_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("threat.indicator.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.created_by.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.created_by.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.created_by.login") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.created_by.login")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.created_by.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.created_by.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.modified_by.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.modified_by.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.modified_by.login") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.modified_by.login")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.modified_by.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.modified_by.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.owned_by.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.owned_by.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.owned_by.login") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.owned_by.login")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("box.source.owned_by.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("box.source.owned_by.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.effective.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.effective.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.effective.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.effective.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.effective.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.effective.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("box.additional_details.shield_alert.alert_summary.alert_activities");
                event.remove("box.additional_details.shield_alert.alert_summary.alert_summary.anomaly_period.date_range.end_date");
                event.remove("box.additional_details.shield_alert.alert_summary.alert_summary.anomaly_period.date_range.start_date");
                event.remove(
                    "box.additional_details.shield_alert.alert_summary.alert_summary.description",
                );
                event.remove("box.additional_details.shield_alert.alert_summary.sessions");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.city_name");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.country_code");
                event.remove(
                    "box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.ip",
                );
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.latitude");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.longitude");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.region_name");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.ip_info.registrant");
                event.remove("box.additional_details.shield_alert.alert_summary.upload_activity.service_name");
                event.remove("box.additional_details.shield_alert.created_at");
                event.remove("box.additional_details.shield_alert.link");
                event.remove("box.additional_details.shield_alert.malware_info.detail_link");
                event.remove("box.additional_details.shield_alert.malware_info.first_seen");
                event.remove("box.additional_details.shield_alert.malware_info.last_seen");
                event.remove("box.additional_details.shield_alert.priority");
                event.remove("shield_date");
                Ok(())
            })();

            // Painless script
            // Source: if (ctx.box?.additional_details?.shield_alert?.priority != null) {\n  ctx.threat.indicator.confidence =\n    ctx.box.additional_details.shield_alert.priority.substring(0, 1).toUpperCase() +\n    ctx.box.additional_details.shield_alert.priority.substring(1);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.box?.additional_details?.shield_alert?.priority != null) {\n  ctx.threat.indicator.confidence =\n    ctx.box.additional_details.shield_alert.priority.substring(0, 1).toUpperCase() +\n    ctx.box.additional_details.shield_alert.priority.substring(1);\n}\n"#
                ),
            )?;

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    if event.has_value("_ingest._value.indicator.ip") {
                        if let Some(ip_str) = event.get_string("_ingest._value.indicator.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set(
                                        "_ingest._value.indicator.geo.country_iso_code",
                                        v.clone(),
                                    )?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set(
                                        "_ingest._value.indicator.geo.country_name",
                                        v.clone(),
                                    )?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set(
                                        "_ingest._value.indicator.geo.continent_name",
                                        v.clone(),
                                    )?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set(
                                        "_ingest._value.indicator.geo.region_iso_code",
                                        v.clone(),
                                    )?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set(
                                        "_ingest._value.indicator.geo.region_name",
                                        v.clone(),
                                    )?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event
                                        .set("_ingest._value.indicator.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event
                                        .set("_ingest._value.indicator.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event
                                        .set("_ingest._value.indicator.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("threat.indicator.ip") {
                if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("threat.indicator.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("threat.indicator.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("threat.indicator.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("threat.indicator.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("threat.indicator.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("threat.indicator.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("threat.indicator.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("threat.indicator.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    if event.has_value("_ingest._value.indicator.ip") {
                        if let Some(ip_str) = event.get_string("_ingest._value.indicator.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("_ingest._value.indicator.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set(
                                        "_ingest._value.indicator.as.organization_name",
                                        v.clone(),
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    if event.has_value("_ingest._value.indicator.as.asn") {
                        event.rename(
                            "_ingest._value.indicator.as.asn",
                            "_ingest._value.indicator.as.number",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    if event.has_value("_ingest._value.indicator.as.organization_name") {
                        event.rename(
                            "_ingest._value.indicator.as.organization_name",
                            "_ingest._value.indicator.as.organization.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("_ingest._value.indicator.geo.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.indicator.geo.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.indicator.ip", converted)?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            if event.has_value("threat.enrichments") {
                foreach_array(event, "threat.enrichments", |event| {
                    event.remove("_ingest._value.indicator.geo.ip");
                    Ok(())
                })?;
            }

            if event.has_value("threat.indicator.ip") {
                if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("threat.indicator.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("threat.indicator.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("threat.indicator.as.asn") {
                event.rename("threat.indicator.as.asn", "threat.indicator.as.number")?;
            }

            if event.has_value("threat.indicator.as.organization_name") {
                event.rename(
                    "threat.indicator.as.organization_name",
                    "threat.indicator.as.organization.name",
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            event.remove("_conf");
            event.remove("_tmp");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
