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

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "event.action")?;
            }

            let _cond = { event.has_value("json.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("json.created_at");

            if event.has_value("json.organization_id") {
                event.rename("json.organization_id", "organization.id")?;
            }

            if event.has_value("json.request_method") {
                event.rename("json.request_method", "http.request.method")?;
            }

            if event.has_value("json.request_id") {
                event.rename("json.request_id", "http.request.id")?;
            }

            if event.has_value("json.status_code") {
                event.rename("json.status_code", "http.response.status_code")?;
            }

            if event.has_value("json.url") {
                event.rename("json.url", "url.original")?;
            }

            if event.has_value("json.invited_email") {
                event.rename("json.invited_email", "user.target.email")?;
            }

            let _cond = { !event.has_value("user.target.email") };
            if _cond {
                if event.has_value("json.deleted_user_email") {
                    event.rename("json.deleted_user_email", "user.target.email")?;
                }
            }

            let _cond = { !event.has_value("user.target.email") };
            if _cond {
                if event.has_value("json.user_email") {
                    event.rename("json.user_email", "user.target.email")?;
                }
            }

            let _cond = { !event.has_value("user.target.email") };
            if _cond {
                if event.has_value("json.invitee_email") {
                    event.rename("json.invitee_email", "user.target.email")?;
                }
            }

            if event.has_value("json.user_id") {
                event.rename("json.user_id", "user.target.id")?;
            }

            let _cond = { !event.has_value("user.target.id") };
            if _cond {
                if event.has_value("json.deleted_user_id") {
                    event.rename("json.deleted_user_id", "user.target.id")?;
                }
            }

            let _cond = { !event.has_value("user.target.id") };
            if _cond {
                if event.has_value("json.requester_user_id") {
                    event.rename("json.requester_user_id", "user.target.id")?;
                }
            }

            let _cond = {
                !event.has_value("user.target.id")
                    && event.get_str("json.principal_type") == Some("user")
            };
            if _cond {
                if event.has_value("json.principal_id") {
                    event.rename("json.principal_id", "user.target.id")?;
                }
            }

            let _cond = {
                !event.has_value("user.target.id")
                    && event.get_str("json.target_type") == Some("user")
            };
            if _cond {
                if event.has_value("json.target_id") {
                    event.rename("json.target_id", "user.target.id")?;
                }
            }

            if event.has_value("json.actor") {
                event.rename("json.actor", "anthropic.audit.actor")?;
            }

            if event.has_value("anthropic.audit.actor.ip_address") {
                event.rename("anthropic.audit.actor.ip_address", "source.ip")?;
            }

            if event.has_value("anthropic.audit.actor.user_agent") {
                event.rename("anthropic.audit.actor.user_agent", "user_agent.original")?;
            }

            let _cond = {
                event.get_str("anthropic.audit.actor.type") == Some("user_actor")
                    || event.get_str("anthropic.audit.actor.type") == Some("anthropic_actor")
            };
            if _cond {
                if event.has_value("anthropic.audit.actor.email_address") {
                    event.rename("anthropic.audit.actor.email_address", "user.email")?;
                }
            }

            let _cond = { event.get_str("anthropic.audit.actor.type") == Some("user_actor") };
            if _cond {
                if event.has_value("anthropic.audit.actor.user_id") {
                    event.rename("anthropic.audit.actor.user_id", "user.id")?;
                }
            }

            if event.has_value("anthropic.audit.actor.unauthenticated_email_address") {
                event.rename(
                    "anthropic.audit.actor.unauthenticated_email_address",
                    "user.email",
                )?;
            }

            if event.has_value("json.organization_uuid") {
                event.rename(
                    "json.organization_uuid",
                    "anthropic.audit.organization_uuid",
                )?;
            }

            if event.has_value("json.api_key_id") {
                event.rename("json.api_key_id", "anthropic.audit.api_key_id")?;
            }

            if event.has_value("json.admin_api_key_id") {
                event.rename("json.admin_api_key_id", "anthropic.audit.admin_api_key_id")?;
            }

            if event.has_value("json.scopes") {
                event.rename("json.scopes", "anthropic.audit.scopes")?;
            }

            if event.has_value("json.request_body") {
                event.rename("json.request_body", "anthropic.audit.request_body")?;
            }

            if event.has_value("json.principal_id") {
                event.rename("json.principal_id", "anthropic.audit.principal_id")?;
            }

            if event.has_value("json.principal_type") {
                event.rename("json.principal_type", "anthropic.audit.principal_type")?;
            }

            if event.has_value("json.role_id") {
                event.rename("json.role_id", "anthropic.audit.role_id")?;
            }

            if event.has_value("json.claude_chat_id") {
                event.rename("json.claude_chat_id", "anthropic.audit.claude_chat_id")?;
            }

            if event.has_value("json.claude_project_id") {
                event.rename(
                    "json.claude_project_id",
                    "anthropic.audit.claude_project_id",
                )?;
            }

            if event.has_value("json.claude_published_artifact_id") {
                event.rename(
                    "json.claude_published_artifact_id",
                    "anthropic.audit.claude_published_artifact_id",
                )?;
            }

            if event.has_value("json.artifact_type") {
                event.rename("json.artifact_type", "anthropic.audit.artifact_type")?;
            }

            if event.has_value("json.title") {
                event.rename("json.title", "anthropic.audit.title")?;
            }

            if event.has_value("json.invite_id") {
                event.rename("json.invite_id", "anthropic.audit.invite_id")?;
            }

            if event.has_value("json.invited_role") {
                event.rename("json.invited_role", "anthropic.audit.invited_role")?;
            }

            if event.has_value("json.mcp_server_id") {
                event.rename("json.mcp_server_id", "anthropic.audit.mcp_server_id")?;
            }

            if event.has_value("json.mcp_server_name") {
                event.rename("json.mcp_server_name", "anthropic.audit.mcp_server_name")?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "anthropic.audit.domain")?;
            }

            if event.has_value("json.workspace_id") {
                event.rename("json.workspace_id", "anthropic.audit.workspace_id")?;
            }

            if event.has_value("json.webhook_id") {
                event.rename("json.webhook_id", "anthropic.audit.webhook_id")?;
            }

            if event.has_value("json.repo_name") {
                event.rename("json.repo_name", "anthropic.audit.repo_name")?;
            }

            if event.has_value("json.repo_owner") {
                event.rename("json.repo_owner", "anthropic.audit.repo_owner")?;
            }

            if event.has_value("json.scan_project_id") {
                event.rename("json.scan_project_id", "anthropic.audit.scan_project_id")?;
            }

            if event.has_value("json.config_id") {
                event.rename("json.config_id", "anthropic.audit.config_id")?;
            }

            if event.has_value("json.ghe_configuration_id") {
                event.rename(
                    "json.ghe_configuration_id",
                    "anthropic.audit.ghe_configuration_id",
                )?;
            }

            if event.has_value("json.access_level") {
                event.rename("json.access_level", "anthropic.audit.access_level")?;
            }

            if event.has_value("json.account_id") {
                event.rename("json.account_id", "anthropic.audit.account_id")?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "anthropic.audit.action")?;
            }

            if event.has_value("json.added_seats") {
                event.rename("json.added_seats", "anthropic.audit.added_seats")?;
            }

            if event.has_value("json.alert_emails") {
                event.rename("json.alert_emails", "anthropic.audit.alert_emails")?;
            }

            if event.has_value("json.alerted_roles") {
                event.rename("json.alerted_roles", "anthropic.audit.alerted_roles")?;
            }

            if event.has_value("json.algorithm") {
                event.rename("json.algorithm", "anthropic.audit.algorithm")?;
            }

            if event.has_value("json.amount") {
                event.rename("json.amount", "anthropic.audit.amount")?;
            }

            if event.has_value("json.api_key_name") {
                event.rename("json.api_key_name", "anthropic.audit.api_key_name")?;
            }

            if event.has_value("json.approved") {
                event.rename("json.approved", "anthropic.audit.approved")?;
            }

            if event.has_value("json.audience") {
                event.rename("json.audience", "anthropic.audit.audience")?;
            }

            if event.has_value("json.auth_method") {
                event.rename("json.auth_method", "anthropic.audit.auth_method")?;
            }

            if event.has_value("json.baa_content_hash") {
                event.rename("json.baa_content_hash", "anthropic.audit.baa_content_hash")?;
            }

            if event.has_value("json.baa_version_label") {
                event.rename(
                    "json.baa_version_label",
                    "anthropic.audit.baa_version_label",
                )?;
            }

            if event.has_value("json.billing_address_updated") {
                event.rename(
                    "json.billing_address_updated",
                    "anthropic.audit.billing_address_updated",
                )?;
            }

            if event.has_value("json.billing_interval") {
                event.rename("json.billing_interval", "anthropic.audit.billing_interval")?;
            }

            if event.has_value("json.billing_name_updated") {
                event.rename(
                    "json.billing_name_updated",
                    "anthropic.audit.billing_name_updated",
                )?;
            }

            if event.has_value("json.cadence") {
                event.rename("json.cadence", "anthropic.audit.cadence")?;
            }

            if event.has_value("json.cc_email_count") {
                event.rename("json.cc_email_count", "anthropic.audit.cc_email_count")?;
            }

            if event.has_value("json.claude_artifact_id") {
                event.rename(
                    "json.claude_artifact_id",
                    "anthropic.audit.claude_artifact_id",
                )?;
            }

            if event.has_value("json.claude_artifact_version_id") {
                event.rename(
                    "json.claude_artifact_version_id",
                    "anthropic.audit.claude_artifact_version_id",
                )?;
            }

            if event.has_value("json.claude_chat_snapshot_id") {
                event.rename(
                    "json.claude_chat_snapshot_id",
                    "anthropic.audit.claude_chat_snapshot_id",
                )?;
            }

            if event.has_value("json.claude_file_id") {
                event.rename("json.claude_file_id", "anthropic.audit.claude_file_id")?;
            }

            if event.has_value("json.claude_project_document_id") {
                event.rename(
                    "json.claude_project_document_id",
                    "anthropic.audit.claude_project_document_id",
                )?;
            }

            if event.has_value("json.cli_name") {
                event.rename("json.cli_name", "anthropic.audit.cli_name")?;
            }

            if event.has_value("json.command_id") {
                event.rename("json.command_id", "anthropic.audit.command_id")?;
            }

            if event.has_value("json.command_name") {
                event.rename("json.command_name", "anthropic.audit.command_name")?;
            }

            if event.has_value("json.compliance_api_enabled") {
                event.rename(
                    "json.compliance_api_enabled",
                    "anthropic.audit.compliance_api_enabled",
                )?;
            }

            if event.has_value("json.compliance_api_logging_enabled") {
                event.rename(
                    "json.compliance_api_logging_enabled",
                    "anthropic.audit.compliance_api_logging_enabled",
                )?;
            }

            if event.has_value("json.connection_id") {
                event.rename("json.connection_id", "anthropic.audit.connection_id")?;
            }

            if event.has_value("json.connection_type") {
                event.rename("json.connection_type", "anthropic.audit.connection_type")?;
            }

            if event.has_value("json.consent_id") {
                event.rename("json.consent_id", "anthropic.audit.consent_id")?;
            }

            if event.has_value("json.consent_type") {
                event.rename("json.consent_type", "anthropic.audit.consent_type")?;
            }

            if event.has_value("json.current_role") {
                event.rename("json.current_role", "anthropic.audit.current_role")?;
            }

            if event.has_value("json.current_value") {
                event.rename("json.current_value", "anthropic.audit.current_value")?;
            }

            if event.has_value("json.current_version") {
                event.rename("json.current_version", "anthropic.audit.current_version")?;
            }

            if event.has_value("json.decision") {
                event.rename("json.decision", "anthropic.audit.decision")?;
            }

            if event.has_value("json.enabled") {
                event.rename("json.enabled", "anthropic.audit.enabled")?;
            }

            if event.has_value("json.entity_id") {
                event.rename("json.entity_id", "anthropic.audit.entity_id")?;
            }

            if event.has_value("json.entity_type") {
                event.rename("json.entity_type", "anthropic.audit.entity_type")?;
            }

            if event.has_value("json.environment_id") {
                event.rename("json.environment_id", "anthropic.audit.environment_id")?;
            }

            if event.has_value("json.extension_id") {
                event.rename("json.extension_id", "anthropic.audit.extension_id")?;
            }

            if event.has_value("json.federation_issuer_id") {
                event.rename(
                    "json.federation_issuer_id",
                    "anthropic.audit.federation_issuer_id",
                )?;
            }

            if event.has_value("json.federation_rule_id") {
                event.rename(
                    "json.federation_rule_id",
                    "anthropic.audit.federation_rule_id",
                )?;
            }

            if event.has_value("json.file_id") {
                event.rename("json.file_id", "anthropic.audit.file_id")?;
            }

            if event.has_value("json.filename") {
                event.rename("json.filename", "file.name")?;
            }

            if event.has_value("json.folder_id") {
                event.rename("json.folder_id", "anthropic.audit.folder_id")?;
            }

            let _cond = { event.has_value("json.from_date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.from_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("anthropic.audit.from_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.from_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("json.from_date");

            if event.has_value("json.github_webhook_id") {
                if let Some(val) = event.get("json.github_webhook_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.github_webhook_id".into(),
                            message,
                        }
                    })?;
                    event.set("anthropic.audit.github_webhook_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("json.github_webhook_id")
                    && event.has_value("anthropic.audit.github_webhook_id")
            };
            if _cond {
                event.remove("json.github_webhook_id");
            }

            if event.has_value("json.group_id") {
                event.rename("json.group_id", "anthropic.audit.group_id")?;
            }

            if event.has_value("json.group_name") {
                event.rename("json.group_name", "anthropic.audit.group_name")?;
            }

            if event.has_value("json.idp_saml_config_updated") {
                event.rename(
                    "json.idp_saml_config_updated",
                    "anthropic.audit.idp_saml_config_updated",
                )?;
            }

            if event.has_value("json.ignore") {
                event.rename("json.ignore", "anthropic.audit.ignore")?;
            }

            if event.has_value("json.installation_preference") {
                event.rename(
                    "json.installation_preference",
                    "anthropic.audit.installation_preference",
                )?;
            }

            if event.has_value("json.integration_id") {
                event.rename("json.integration_id", "anthropic.audit.integration_id")?;
            }

            if event.has_value("json.integration_type") {
                event.rename("json.integration_type", "anthropic.audit.integration_type")?;
            }

            if event.has_value("json.is_enabled") {
                event.rename("json.is_enabled", "anthropic.audit.is_enabled")?;
            }

            if event.has_value("json.is_service_created") {
                event.rename(
                    "json.is_service_created",
                    "anthropic.audit.is_service_created",
                )?;
            }

            if event.has_value("json.item_allocations") {
                event.rename("json.item_allocations", "anthropic.audit.item_allocations")?;
            }

            if event.has_value("json.key_backing_type") {
                event.rename("json.key_backing_type", "anthropic.audit.key_backing_type")?;
            }

            if event.has_value("json.key_group_identifier") {
                event.rename(
                    "json.key_group_identifier",
                    "anthropic.audit.key_group_identifier",
                )?;
            }

            if event.has_value("json.key_name") {
                event.rename("json.key_name", "anthropic.audit.key_name")?;
            }

            if event.has_value("json.limit_action") {
                event.rename("json.limit_action", "anthropic.audit.limit_action")?;
            }

            if event.has_value("json.limit_type") {
                event.rename("json.limit_type", "anthropic.audit.limit_type")?;
            }

            if event.has_value("json.limit_usd") {
                event.rename("json.limit_usd", "anthropic.audit.limit_usd")?;
            }

            if event.has_value("json.limiter_type") {
                event.rename("json.limiter_type", "anthropic.audit.limiter_type")?;
            }

            if event.has_value("json.lti_platform_id") {
                event.rename("json.lti_platform_id", "anthropic.audit.lti_platform_id")?;
            }

            if event.has_value("json.lti_platform_issuer") {
                event.rename(
                    "json.lti_platform_issuer",
                    "anthropic.audit.lti_platform_issuer",
                )?;
            }

            if event.has_value("json.magic_link_enabled") {
                event.rename(
                    "json.magic_link_enabled",
                    "anthropic.audit.magic_link_enabled",
                )?;
            }

            if event.has_value("json.magic_link_toggled") {
                event.rename(
                    "json.magic_link_toggled",
                    "anthropic.audit.magic_link_toggled",
                )?;
            }

            if event.has_value("json.marketplace_id") {
                event.rename("json.marketplace_id", "anthropic.audit.marketplace_id")?;
            }

            if event.has_value("json.max_permission") {
                event.rename("json.max_permission", "anthropic.audit.max_permission")?;
            }

            if event.has_value("json.member_ids") {
                event.rename("json.member_ids", "anthropic.audit.member_ids")?;
            }

            if event.has_value("json.metadata") {
                event.rename("json.metadata", "anthropic.audit.setting_metadata")?;
            }

            if event.has_value("json.mfa_method") {
                event.rename("json.mfa_method", "anthropic.audit.mfa_method")?;
            }

            if event.has_value("json.model") {
                event.rename("json.model", "anthropic.audit.model")?;
            }

            if event.has_value("json.model_group") {
                event.rename("json.model_group", "anthropic.audit.model_group")?;
            }

            if event.has_value("json.new_collection_method") {
                event.rename(
                    "json.new_collection_method",
                    "anthropic.audit.new_collection_method",
                )?;
            }

            if event.has_value("json.new_limit_usd") {
                event.rename("json.new_limit_usd", "anthropic.audit.new_limit_usd")?;
            }

            if event.has_value("json.new_mode") {
                event.rename("json.new_mode", "anthropic.audit.new_mode")?;
            }

            if event.has_value("json.new_owner_id") {
                event.rename("json.new_owner_id", "anthropic.audit.new_owner_id")?;
            }

            if event.has_value("json.new_plan") {
                event.rename("json.new_plan", "anthropic.audit.new_plan")?;
            }

            if event.has_value("json.new_quantity") {
                event.rename("json.new_quantity", "anthropic.audit.new_quantity")?;
            }

            if event.has_value("json.new_signing_key_id") {
                event.rename(
                    "json.new_signing_key_id",
                    "anthropic.audit.new_signing_key_id",
                )?;
            }

            if event.has_value("json.old_plan") {
                event.rename("json.old_plan", "anthropic.audit.old_plan")?;
            }

            if event.has_value("json.old_signing_key_id") {
                event.rename(
                    "json.old_signing_key_id",
                    "anthropic.audit.old_signing_key_id",
                )?;
            }

            if event.has_value("json.op_name") {
                event.rename("json.op_name", "anthropic.audit.op_name")?;
            }

            if event.has_value("json.org_id") {
                event.rename("json.org_id", "anthropic.audit.org_id")?;
            }

            let _cond = { !event.has_value("organization.name") };
            if _cond {
                if event.has_value("json.org_name") {
                    event.rename("json.org_name", "organization.name")?;
                }
            }

            let _cond = { !event.has_value("organization.name") };
            if _cond {
                if event.has_value("json.organization_name") {
                    event.rename("json.organization_name", "organization.name")?;
                }
            }

            if event.has_value("json.per_review_limit_usd") {
                event.rename(
                    "json.per_review_limit_usd",
                    "anthropic.audit.per_review_limit_usd",
                )?;
            }

            if event.has_value("json.plan_type") {
                event.rename("json.plan_type", "anthropic.audit.plan_type")?;
            }

            if event.has_value("json.plugin_id") {
                event.rename("json.plugin_id", "anthropic.audit.plugin_id")?;
            }

            if event.has_value("json.plugin_name") {
                event.rename("json.plugin_name", "anthropic.audit.plugin_name")?;
            }

            if event.has_value("json.preview_only") {
                event.rename("json.preview_only", "anthropic.audit.preview_only")?;
            }

            if event.has_value("json.previous_mode") {
                event.rename("json.previous_mode", "anthropic.audit.previous_mode")?;
            }

            if event.has_value("json.previous_owner_id") {
                event.rename(
                    "json.previous_owner_id",
                    "anthropic.audit.previous_owner_id",
                )?;
            }

            if event.has_value("json.previous_quantity") {
                event.rename(
                    "json.previous_quantity",
                    "anthropic.audit.previous_quantity",
                )?;
            }

            if event.has_value("json.previous_role") {
                event.rename("json.previous_role", "anthropic.audit.previous_role")?;
            }

            if event.has_value("json.previous_value") {
                event.rename("json.previous_value", "anthropic.audit.previous_value")?;
            }

            if event.has_value("json.previous_version") {
                event.rename("json.previous_version", "anthropic.audit.previous_version")?;
            }

            if event.has_value("json.primary_email_set") {
                event.rename(
                    "json.primary_email_set",
                    "anthropic.audit.primary_email_set",
                )?;
            }

            if event.has_value("json.provider") {
                event.rename("json.provider", "anthropic.audit.provider")?;
            }

            if event.has_value("json.reason") {
                event.rename("json.reason", "event.reason")?;
            }

            if event.has_value("json.repository_name") {
                event.rename("json.repository_name", "anthropic.audit.repository_name")?;
            }

            if event.has_value("json.request_type") {
                event.rename("json.request_type", "anthropic.audit.request_type")?;
            }

            if event.has_value("json.resolved_count") {
                event.rename("json.resolved_count", "anthropic.audit.resolved_count")?;
            }

            if event.has_value("json.resource_id") {
                event.rename("json.resource_id", "anthropic.audit.resource_id")?;
            }

            if event.has_value("json.resource_type") {
                event.rename("json.resource_type", "anthropic.audit.resource_type")?;
            }

            if event.has_value("json.resync_uuid") {
                event.rename("json.resync_uuid", "anthropic.audit.resync_uuid")?;
            }

            let _cond = {
                event.has_value("json.role")
                    && event.has_value("event.action")
                    && (event.get_str("event.action") == Some("role_assignment_granted")
                        || event.get_str("event.action") == Some("role_assignment_revoked"))
            };
            if _cond {
                event.append_unique(
                    "user.target.roles",
                    json!(
                        event
                            .get("json.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get_str("event.action") == Some("role_assignment_granted")
                        || event.get_str("event.action") == Some("role_assignment_revoked"))
            };
            if _cond {
                event.remove("json.role");
            }

            if event.has_value("json.role") {
                event.rename("json.role", "anthropic.audit.role")?;
            }

            if event.has_value("json.role_name") {
                event.rename("json.role_name", "anthropic.audit.role_name")?;
            }

            if event.has_value("json.scans_cancelled") {
                event.rename("json.scans_cancelled", "anthropic.audit.scans_cancelled")?;
            }

            if event.has_value("json.seat_count") {
                event.rename("json.seat_count", "anthropic.audit.seat_count")?;
            }

            if event.has_value("json.service_account_id") {
                event.rename(
                    "json.service_account_id",
                    "anthropic.audit.service_account_id",
                )?;
            }

            if event.has_value("json.service_key_id") {
                event.rename("json.service_key_id", "anthropic.audit.service_key_id")?;
            }

            if event.has_value("json.service_name") {
                event.rename("json.service_name", "anthropic.audit.service_name")?;
            }

            if event.has_value("json.session_id") {
                event.rename("json.session_id", "anthropic.audit.session_id")?;
            }

            if event.has_value("json.settings_uuid") {
                event.rename("json.settings_uuid", "anthropic.audit.settings_uuid")?;
            }

            if event.has_value("json.setup_guide_content_hash") {
                event.rename(
                    "json.setup_guide_content_hash",
                    "anthropic.audit.setup_guide_content_hash",
                )?;
            }

            if event.has_value("json.share_id") {
                event.rename("json.share_id", "anthropic.audit.share_id")?;
            }

            if event.has_value("json.shipping_address_updated") {
                event.rename(
                    "json.shipping_address_updated",
                    "anthropic.audit.shipping_address_updated",
                )?;
            }

            if event.has_value("json.shipping_name_updated") {
                event.rename(
                    "json.shipping_name_updated",
                    "anthropic.audit.shipping_name_updated",
                )?;
            }

            if event.has_value("json.signing_key_id") {
                event.rename("json.signing_key_id", "anthropic.audit.signing_key_id")?;
            }

            if event.has_value("json.skill_id") {
                event.rename("json.skill_id", "anthropic.audit.skill_id")?;
            }

            if event.has_value("json.skill_name") {
                event.rename("json.skill_name", "anthropic.audit.skill_name")?;
            }

            if event.has_value("json.spend_limit_id") {
                event.rename("json.spend_limit_id", "anthropic.audit.spend_limit_id")?;
            }

            if event.has_value("json.spend_limit_increase_request_id") {
                event.rename(
                    "json.spend_limit_increase_request_id",
                    "anthropic.audit.spend_limit_increase_request_id",
                )?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "anthropic.audit.status")?;
            }

            if event.has_value("json.sync_destinations") {
                event.rename(
                    "json.sync_destinations",
                    "anthropic.audit.sync_destinations",
                )?;
            }

            if event.has_value("json.taint") {
                event.rename("json.taint", "anthropic.audit.taint")?;
            }

            if event.has_value("json.target_amount") {
                event.rename("json.target_amount", "anthropic.audit.target_amount")?;
            }

            if event.has_value("json.target_id") {
                event.rename("json.target_id", "anthropic.audit.target_id")?;
            }

            if event.has_value("json.target_type") {
                event.rename("json.target_type", "anthropic.audit.target_type")?;
            }

            if event.has_value("json.threshold_amount") {
                event.rename("json.threshold_amount", "anthropic.audit.threshold_amount")?;
            }

            let _cond = { event.has_value("json.to_date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.to_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("anthropic.audit.to_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.to_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.to_email_count") {
                event.rename("json.to_email_count", "anthropic.audit.to_email_count")?;
            }

            if event.has_value("json.token_id") {
                event.rename("json.token_id", "anthropic.audit.token_id")?;
            }

            if event.has_value("json.token_name") {
                event.rename("json.token_name", "anthropic.audit.token_name")?;
            }

            if event.has_value("json.tool_name") {
                event.rename("json.tool_name", "anthropic.audit.tool_name")?;
            }

            if event.has_value("json.trigger_mode") {
                event.rename("json.trigger_mode", "anthropic.audit.trigger_mode")?;
            }

            if event.has_value("json.updates") {
                event.rename("json.updates", "anthropic.audit.updates")?;
            }

            if event.has_value("json.value") {
                event.rename("json.value", "anthropic.audit.value")?;
            }

            if event.has_value("json.version") {
                event.rename("json.version", "anthropic.audit.version")?;
            }

            let _cond = {
                event.get("json").is_some_and(|v| v.is_object())
                    && event.get("json").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("json") {
                    event.rename("json", "anthropic.audit.metadata")?;
                }
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("url.original") {
                        uri_parts(event, "url.original", "url", true, false)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
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
            }

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

            let _cond = { event.has_value("event.action") };
            if _cond {
                // Begin nested pipeline: "categorize"
                let _cond = { event.has_value("event.action") };
                if _cond {
                    // Painless script
                    // Source: def action = ctx.event.action;\nctx.event.kind = 'event';\n\nif (params.exact.containsKey(action)) {\n  def m = params.exact[action];\n  ctx.event.category = m.category;\n  ctx.event.type = m.type;\n  if (m.containsKey('outcome')) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = [];\n  ctx.event.type = ['info'];\n  ctx.event.outcome = 'unknown';\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def action = ctx.event.action;\nctx.event.kind = 'event';\n\nif (params.exact.containsKey(action)) {\n  def m = params.exact[action];\n  ctx.event.category = m.category;\n  ctx.event.type = m.type;\n  if (m.containsKey('outcome')) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = [];\n  ctx.event.type = ['info'];\n  ctx.event.outcome = 'unknown';\n}"#
                        ),
                        cached_params!(
                            "{\"exact\":{\"account_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"],\"outcome\":\"success\"},\"admin_api_key_created\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"creation\"],\"outcome\":\"success\"},\"admin_api_key_deleted\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"deletion\"],\"outcome\":\"success\"},\"admin_api_key_updated\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"admin_connector_request_resolved\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"admin_request_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"age_verified\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"],\"outcome\":\"success\"},\"anonymous_mobile_login_attempted\":{\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"unknown\"},\"api_key_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"audit_log_export_accessed\":{\"category\":[\"file\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"audit_log_export_started\":{\"category\":[\"file\"],\"type\":[\"creation\",\"info\"],\"outcome\":\"success\"},\"billing_emails_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_artifact_access_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\"],\"outcome\":\"failure\"},\"claude_artifact_created\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_artifact_published\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\",\"info\"],\"outcome\":\"success\"},\"claude_artifact_sharing_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_artifact_viewed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"claude_chat_access_failed\":{\"category\":[\"web\"],\"type\":[\"access\"],\"outcome\":\"failure\"},\"claude_chat_created\":{\"category\":[\"web\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"claude_chat_deleted\":{\"category\":[\"web\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"claude_chat_deletion_failed\":{\"category\":[\"web\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"claude_chat_settings_updated\":{\"category\":[\"web\",\"configuration\"],\"type\":[\"info\",\"change\"],\"outcome\":\"success\"},\"claude_chat_snapshot_created\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\",\"info\"],\"outcome\":\"success\"},\"claude_chat_snapshot_viewed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"claude_chat_updated\":{\"category\":[\"web\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"claude_chat_viewed\":{\"category\":[\"web\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"claude_code_review_config_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_review_repository_added\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_code_review_repository_removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_code_review_repository_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_center_config_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_scan_cancelled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_scan_project_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_scan_schedule_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_code_security_scan_schedule_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_webhook_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_code_security_webhook_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_code_security_webhook_secret_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_security_webhook_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_code_team_memory_acl_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"],\"outcome\":\"success\"},\"claude_command_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_command_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_command_replaced\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_file_access_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\",\"info\"],\"outcome\":\"failure\"},\"claude_file_deleted\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_file_uploaded\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_file_viewed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"claude_gdrive_integration_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_gdrive_integration_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_gdrive_integration_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_github_integration_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_github_integration_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_github_integration_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_organization_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_plugin_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_plugin_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_plugin_replaced\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_plugin_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_project_archived\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_project_created\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_project_deleted\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_project_document_access_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\",\"info\"],\"outcome\":\"failure\"},\"claude_project_document_deleted\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_project_document_deletion_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\",\"info\"],\"outcome\":\"failure\"},\"claude_project_document_uploaded\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_project_document_viewed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"claude_project_file_access_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\",\"info\"],\"outcome\":\"failure\"},\"claude_project_file_deleted\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_project_file_deletion_failed\":{\"category\":[\"file\",\"web\"],\"type\":[\"deletion\",\"info\"],\"outcome\":\"failure\"},\"claude_project_file_uploaded\":{\"category\":[\"file\",\"web\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_project_reported\":{\"category\":[\"file\",\"web\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"claude_project_sharing_updated\":{\"category\":[\"file\",\"web\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_project_viewed\":{\"category\":[\"file\",\"web\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"claude_published_artifact_deleted\":{\"category\":[\"file\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_pubsec_identity_configured\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"claude_skill_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"claude_skill_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"claude_skill_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_skill_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_skill_replaced\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"claude_user_role_updated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"claude_user_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"cli_plugin_exec_policy_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"compliance_api_accessed\":{\"category\":[\"api\"],\"type\":[\"access\"],\"outcome\":\"unknown\"},\"desktop_extension_allowlisted\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"desktop_extension_blocklisted\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"desktop_extension_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"desktop_extension_removed_from_allowlist\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"desktop_extension_unblocked\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"desktop_extension_uploaded\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"desktop_extension_version_uploaded\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"domain_claim_initiated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"unknown\"},\"end_user_invite_requested\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"unknown\"},\"extra_usage_billing_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"extra_usage_credit_granted\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"extra_usage_spend_limit_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"extra_usage_spend_limit_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"extra_usage_spend_limit_increase_request_approved\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"extra_usage_spend_limit_increase_request_denied\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"failure\"},\"extra_usage_spend_limit_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"ghe_configuration_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"ghe_configuration_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"ghe_configuration_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"ghe_user_connected\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"ghe_user_disconnected\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"ghe_webhook_signature_invalid\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"group_created\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"],\"outcome\":\"success\"},\"group_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"group\"],\"outcome\":\"success\"},\"group_list_viewed\":{\"category\":[\"iam\"],\"type\":[\"info\",\"group\"],\"outcome\":\"success\"},\"group_member_added\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"],\"outcome\":\"success\"},\"group_member_list_viewed\":{\"category\":[\"iam\"],\"type\":[\"info\",\"group\"],\"outcome\":\"success\"},\"group_member_removed\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"],\"outcome\":\"success\"},\"group_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"],\"outcome\":\"success\"},\"group_viewed\":{\"category\":[\"iam\"],\"type\":[\"info\",\"group\"],\"outcome\":\"success\"},\"integration_user_connected\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"integration_user_disconnected\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"invoice_collection_method_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"lti_launch_initiated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"unknown\"},\"lti_launch_success\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"lti_platform_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"lti_platform_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"magic_link_login_failed\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"failure\"},\"magic_link_login_initiated\":{\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"unknown\"},\"magic_link_login_succeeded\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\",\"info\"],\"outcome\":\"success\"},\"managed_organization_setup_completed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"marketplace_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"marketplace_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"marketplace_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"marketplace_webhook_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"marketplace_webhook_provisioned\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"mcp_server_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"mcp_server_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"mcp_server_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"mcp_tool_policy_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_analytics_api_capability_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_bulk_delete_initiated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_claude_code_data_sharing_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_claude_code_data_sharing_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_claude_code_desktop_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_claude_code_desktop_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_compliance_api_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_cowork_agent_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_cowork_agent_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_cowork_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_cowork_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_creation_blocked\":{\"category\":[\"configuration\"],\"type\":[\"change\",\"info\"],\"outcome\":\"failure\"},\"org_data_export_accessed\":{\"category\":[\"file\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"org_data_export_completed\":{\"category\":[\"file\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"org_data_export_started\":{\"category\":[\"file\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"org_deleted_via_bulk\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_deletion_requested\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_directory_resync_completed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_directory_resync_failed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"failure\"},\"org_directory_resync_started\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_directory_sync_activated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_directory_sync_add_initiated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_directory_sync_deleted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_discoverability_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_discoverability_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_discoverability_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_domain_add_initiated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_domain_removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_domain_verified\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"org_hipaa_self_serve_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_invite_link_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_invite_link_generated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_invite_link_regenerated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_invite_viewed\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"org_invites_listed\":{\"category\":[\"configuration\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"org_ip_restriction_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"org_ip_restriction_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_ip_restriction_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_join_proposal_decided\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"unknown\"},\"org_join_request_approved\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_join_request_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"org_join_request_dismissed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_join_request_instant_approved\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_join_requests_bulk_dismissed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_magic_link_second_factor_toggled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_member_invites_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_member_invites_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_members_exported\":{\"category\":[\"configuration\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"org_parent_join_proposal_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"org_parent_search_performed\":{\"category\":[\"configuration\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"org_sso_add_initiated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_connection_activated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_connection_deactivated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_connection_deleted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_sso_group_role_mappings_updated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_provisioning_mode_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_seat_tier_assignment_toggled\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_seat_tier_mappings_updated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sso_toggled\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sync_deleting_synchronized_files_started\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_sync_synchronized_files_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_taint_added\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_taint_removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"org_user_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"],\"outcome\":\"success\"},\"org_user_invite_accepted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},\"org_user_invite_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"],\"outcome\":\"success\"},\"org_user_invite_re_sent\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},\"org_user_invite_rejected\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"failure\"},\"org_user_invite_sent\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},\"org_user_left\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"org_user_viewed\":{\"category\":[\"iam\"],\"type\":[\"info\",\"user\"],\"outcome\":\"success\"},\"org_users_listed\":{\"category\":[\"iam\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"org_work_across_apps_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"org_work_across_apps_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"organization_address_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"organization_icon_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"organization_icon_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"owned_projects_access_restored\":{\"category\":[\"iam\",\"file\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"payment_method_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"phone_code_sent\":{\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"success\"},\"phone_code_verified\":{\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"success\"},\"platform_api_key_created\":{\"category\":[\"iam\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_api_key_updated\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_cost_report_viewed\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"platform_federation_issuer_archived\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"],\"outcome\":\"success\"},\"platform_federation_issuer_updated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"platform_federation_rule_archived\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"],\"outcome\":\"success\"},\"platform_federation_rule_updated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"platform_federation_rule_workspace_added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"platform_federation_rule_workspace_removed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"],\"outcome\":\"success\"},\"platform_file_content_downloaded\":{\"category\":[\"file\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"platform_file_deleted\":{\"category\":[\"file\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_file_uploaded\":{\"category\":[\"file\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_service_account_archived\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"],\"outcome\":\"success\"},\"platform_service_account_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"platform_service_account_workspace_member_added\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"platform_service_account_workspace_member_removed\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"platform_service_account_workspace_member_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"platform_signing_key_created\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_signing_key_deleted\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_signing_key_rotated\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_skill_version_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_skill_version_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_spend_limit_alert_emails_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_spend_limit_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_spend_limit_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_spend_limit_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_usage_report_claude_code_viewed\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"platform_usage_report_messages_viewed\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"platform_workspace_archived\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_workspace_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"platform_workspace_member_added\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_workspace_member_removed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_workspace_member_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_workspace_member_viewed\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"platform_workspace_members_listed\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"],\"outcome\":\"success\"},\"platform_workspace_rate_limit_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"platform_workspace_rate_limit_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"platform_workspace_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"plugin_installation_preference_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"prepaid_auto_recharge_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"prepaid_auto_recharge_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"prepaid_extra_usage_auto_reload_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"prepaid_extra_usage_auto_reload_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"prepaid_extra_usage_auto_reload_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"primary_owner_transferred\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"rbac_role_assigned\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"rbac_role_created\":{\"category\":[\"iam\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"rbac_role_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"rbac_role_permission_added\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"rbac_role_permission_removed\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"rbac_role_unassigned\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"rbac_role_updated\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"role_assignment_granted\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"role_assignment_revoked\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"scim_user_created\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},\"scim_user_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"],\"outcome\":\"success\"},\"scim_user_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"],\"outcome\":\"success\"},\"scoped_api_key_deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"scoped_api_key_updated\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"seat_tier_changes_cancelled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"seat_tiers_purchased\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"service_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"service_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"service_key_created\":{\"category\":[\"iam\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"service_key_revoked\":{\"category\":[\"iam\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"session_revoked\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"session_share_accessed\":{\"category\":[\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"session_share_created\":{\"category\":[\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"session_share_revoked\":{\"category\":[\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"social_login_succeeded\":{\"category\":[\"authentication\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"sso_login_failed\":{\"category\":[\"authentication\"],\"type\":[\"end\"],\"outcome\":\"failure\"},\"sso_login_initiated\":{\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"unknown\"},\"sso_login_succeeded\":{\"category\":[\"authentication\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"sso_second_factor_magic_link\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"subscription_cancellation_scheduled\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"subscription_quantity_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"subscription_renewed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"subscription_resumed\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"subscription_started\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"subscription_upgraded\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"tunnel_token_minted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"tunnel_token_revoked\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"user_consent_recorded\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"user_consent_revoked\":{\"category\":[\"iam\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"user_logged_out\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"workspace_member_spend_limit_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"workspace_member_spend_limit_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"workspace_member_spend_limit_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"workspace_spend_limit_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"},\"workspace_spend_limit_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"}}}"
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("compliance_api_accessed")
                        && event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n >= 200)
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n < 300)
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("compliance_api_accessed")
                        && event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n >= 400)
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n < 500)
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("compliance_api_accessed")
                        && event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n >= 500)
                };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("org_join_proposal_decided")
                        && event.get_bool("anthropic.audit.approved") == Some(true)
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("org_join_proposal_decided")
                        && event.get_bool("anthropic.audit.approved") != Some(true)
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // End nested pipeline: "categorize"
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

            let _cond = { event.has_value("user.target.email") };
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

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("anthropic.audit.principal_type") == Some("user")
                    && event.has_value("anthropic.audit.principal_id")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("anthropic.audit.principal_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("anthropic.audit.member_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("anthropic.audit.member_ids") {
                    foreach_array(event, "anthropic.audit.member_ids", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("anthropic.audit.alert_emails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("anthropic.audit.alert_emails") {
                    foreach_array(event, "anthropic.audit.alert_emails", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
