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

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.append(
                    "error.message",
                    json!("event.original is set before start of ingest pipeline"),
                )?;
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

            parse_json_field(event, "event.original", "json")?;

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

            let _cond = { event.has_value("json.uuid") && event.get_str("json.uuid") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.uuid") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.uuid".into(),
                                message,
                            }
                        })?;
                        event.set("_id", converted)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.published") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.published".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.displayMessage") {
                    event.rename("json.displayMessage", "okta.display_message")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.eventType") {
                    event.rename("json.eventType", "okta.event_type")?;
                }
                Ok(())
            })();

            let _cond = {
                [
                    "group.user_membership.add",
                    "group.user_membership.remove",
                    "user.lifecycle.activate",
                    "user.lifecycle.create",
                    "user.lifecycle.deactivate",
                    "user.lifecycle.suspend",
                    "user.lifecycle.unsuspend",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("iam"))?;
            }

            let _cond = {
                [
                    "policy.lifecycle.activate",
                    "policy.lifecycle.create",
                    "policy.lifecycle.deactivate",
                    "policy.lifecycle.delete",
                    "policy.lifecycle.update",
                    "policy.rule.activate",
                    "policy.rule.add",
                    "policy.rule.deactivate",
                    "policy.rule.delete",
                    "application.lifecycle.create",
                    "application.lifecycle.delete",
                    "policy.rule.update",
                    "application.lifecycle.activate",
                    "application.lifecycle.deactivate",
                    "application.lifecycle.update",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = {
                [
                    "user.session.start",
                    "user.session.end",
                    "user.authentication.sso",
                    "policy.evaluate_sign_on",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = {
                ["user.session.start", "user.session.end"]
                    .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("session"))?;
            }

            let _cond = {
                [
                    "system.org.rate_limit.warning",
                    "system.org.rate_limit.violation",
                    "core.concurrency.org.limit.violation",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                ["security.request.blocked"]
                    .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("network"))?;
            }

            let _cond = {
                [
                    "system.org.rate_limit.warning",
                    "system.org.rate_limit.violation",
                    "core.concurrency.org.limit.violation",
                    "security.request.blocked",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("network"))?;
            }

            let _cond = {
                ["user.session.start"].contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("start"))?;
            }

            let _cond =
                { ["user.session.end"].contains(&event.get_str("okta.event_type").unwrap_or("")) };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            let _cond = {
                ["group.user_membership.add", "group.user_membership.remove"]
                    .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("group"))?;
            }

            let _cond = {
                [
                    "user.lifecycle.activate",
                    "user.lifecycle.create",
                    "user.lifecycle.deactivate",
                    "user.lifecycle.suspend",
                    "user.lifecycle.unsuspend",
                    "user.authentication.sso",
                    "user.session.start",
                    "user.session.end",
                    "application.user_membership.add",
                    "application.user_membership.remove",
                    "application.user_membership.change_username",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                [
                    "user.lifecycle.activate",
                    "user.lifecycle.deactivate",
                    "user.lifecycle.suspend",
                    "user.lifecycle.unsuspend",
                    "group.user_membership.add",
                    "group.user_membership.remove",
                    "policy.lifecycle.activate",
                    "policy.lifecycle.deactivate",
                    "policy.lifecycle.update",
                    "policy.rule.activate",
                    "policy.rule.add",
                    "policy.rule.deactivate",
                    "policy.rule.update",
                    "application.user_membership.add",
                    "application.user_membership.remove",
                    "application.user_membership.change_username",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = {
                [
                    "user.lifecycle.create",
                    "policy.lifecycle.create",
                    "application.lifecycle.create",
                ]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
            }

            let _cond = {
                ["policy.lifecycle.delete", "application.lifecycle.delete"]
                    .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("deletion"))?;
            }

            let _cond = {
                ["policy.evaluate_sign_on"]
                    .contains(&event.get_str("okta.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            // Begin nested pipeline: "ecs_category_type"
            let _cond = { event.has_value("okta.event_type") };
            if _cond {
                // Painless script
                // Source: def addUnique(List dst, List src) {\n  src = src ?: [];\n  if (src.length == 0) {\n    return dst ?: [];\n  }\n  HashSet s = new HashSet(dst ?: []);\n  s.addAll(src);\n  return new ArrayList(s);\n}\ndef p = params[ctx.okta.event_type];\nctx.event.type = addUnique(ctx.event.type, p.type);\nctx.event.category = addUnique(ctx.event.category, p.category);\nctx.tags = addUnique(ctx.tags, p.tags);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def addUnique(List dst, List src) {\n  src = src ?: [];\n  if (src.length == 0) {\n    return dst ?: [];\n  }\n  HashSet s = new HashSet(dst ?: []);\n  s.addAll(src);\n  return new ArrayList(s);\n}\ndef p = params[ctx.okta.event_type];\nctx.event.type = addUnique(ctx.event.type, p.type);\nctx.event.category = addUnique(ctx.event.category, p.category);\nctx.tags = addUnique(ctx.tags, p.tags);"#
                    ),
                    cached_params!(
                        "{\"access.request.cancel\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"deletion\"]},\"access.request.condition.activate\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.condition.create\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"access.request.condition.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.condition.delete\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"deletion\"]},\"access.request.condition.invalidate\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.condition.update\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"access.request.create\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"access.request.expire\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.reject\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.resolve\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.request.sequence.create\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"access.request.sequence.delete\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"deletion\"]},\"access.request.sequence.update\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"access.request.settings.update\":{\"category\":[\"iam\"],\"tags\":[\"access\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"access.review.action\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.review.close\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.review.create\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"access.review.remediate\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.review.start\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"access.review.update\":{\"category\":[\"iam\"],\"tags\":[\"access-review\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"account.org.add\":{\"category\":[\"configuration\"],\"tags\":[\"account-service\"],\"type\":[\"creation\"]},\"account.org.delete.cancel\":{\"category\":[\"configuration\"],\"tags\":[\"account-service\"],\"type\":[\"deletion\"]},\"account.org.delete.request\":{\"category\":[\"configuration\"],\"tags\":[\"account-service\"],\"type\":[\"deletion\"]},\"account.org.product.update\":{\"category\":[\"configuration\"],\"tags\":[\"account-service\"],\"type\":[\"change\"]},\"account.org.status.update\":{\"category\":[\"configuration\"],\"tags\":[\"account-service\"],\"type\":[\"change\",\"info\"]},\"analytics.feedback.provide\":{\"category\":[\"configuration\"],\"tags\":[\"risk\",\"security\"],\"type\":[\"info\"]},\"analytics.reports.export.download\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"analytics.reports.export.generate\":{\"category\":[\"configuration\"],\"type\":[\"creation\",\"info\"]},\"analytics.reports.export.request\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"app.access_request.approver.approve\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"admin\"]},\"app.access_request.approver.deny\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"admin\"]},\"app.access_request.delete\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"admin\",\"deletion\"]},\"app.access_request.deny\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"app.access_request.expire\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"app.access_request.grant\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"app.access_request.request\":{\"category\":[\"iam\"],\"tags\":[\"app-instance-request\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"app.ad.api.user_import.account_locked\":{\"category\":[\"iam\"],\"tags\":[\"ad-app\"],\"type\":[\"user\"]},\"app.ad.api.user_import.warn.skipped_contact.attribute_invalid_value\":{\"category\":[\"iam\"],\"tags\":[\"ad-app\"],\"type\":[\"user\"]},\"app.ad.api.user_import.warn.skipped_user.attribute_invalid_value\":{\"category\":[\"iam\"],\"tags\":[\"ad-app\"],\"type\":[\"user\"]},\"app.ad.api.user_import.warn.skipped_user.missing_required_attribute\":{\"category\":[\"iam\"],\"tags\":[\"ad-app\"],\"type\":[\"user\"]},\"app.ad.password_migration_campaign.cancel.end\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"deletion\"]},\"app.ad.password_migration_campaign.cancel.start\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.create\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"creation\"]},\"app.ad.password_migration_campaign.finish.end\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.finish.start\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.group.add\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.user.capture_password\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.user.migrate.end\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"app.ad.password_migration_campaign.user.migrate.start\":{\"category\":[\"configuration\"],\"tags\":[\"ad-app\"],\"type\":[\"info\"]},\"app.app_instance.csr.generate\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.app_instance.csr.publish\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.app_instance.csr.revoke\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"deletion\"]},\"app.app_instance.provision_sync_job.completed\":{\"category\":[\"iam\"],\"tags\":[\"admin\",\"app\",\"user-provision\"],\"type\":[\"creation\"]},\"app.app_instance.provision_sync_job.failed\":{\"category\":[\"iam\"],\"tags\":[\"admin\",\"app\",\"user-provision\"],\"type\":[\"creation\",\"info\"]},\"app.app_instance.provision_sync_job.started\":{\"category\":[\"iam\"],\"tags\":[\"admin\",\"app\",\"user-provision\"],\"type\":[\"creation\"]},\"app.audit_report.download\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.audit_report.download.local.active\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.audit_report.download.local.deprov\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.audit_report.download.rogue.report\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.generic.unauth_app_access_attempt\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.inbound_del_auth.login_success\":{\"category\":[\"authentication\"],\"tags\":[\"delegated-auth\"],\"type\":[\"info\",\"start\"]},\"app.kerberos_rich_client.account_not_found\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"kerberos-rich-client\"],\"type\":[\"info\"]},\"app.kerberos_rich_client.instance_not_found\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"kerberos-rich-client\"],\"type\":[\"info\"]},\"app.kerberos_rich_client.multiple_accounts_found\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"kerberos-rich-client\"],\"type\":[\"info\"]},\"app.kerberos_rich_client.user_authentication_successful\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"kerberos-rich-client\"],\"type\":[\"info\"]},\"app.keys.clone\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.keys.generate\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.keys.rotate\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"app.ldap.password.change.failed\":{\"category\":[\"authentication\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"app.oauth2.admin.consent.grant\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.admin.consent.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.api_resource.create\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-api-resource\"],\"type\":[\"info\"]},\"app.oauth2.api_resource.delete\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-api-resource\"],\"type\":[\"info\"]},\"app.oauth2.api_resource.update\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-api-resource\"],\"type\":[\"info\"]},\"app.oauth2.as.authorize\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.authorize.code\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.authorize.implicit.access_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.authorize.implicit.id_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.authorize.scope_denied\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.grant\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.implicit.as\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.implicit.client\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.implicit.scope\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.implicit.user\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.user\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.consent.revoke.user.client\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.evaluate.claim\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.interact.interaction_code\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.interact.interaction_handle\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.key.rollover\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.resource_server.credentials.lifecycle.activate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"start\"]},\"app.oauth2.as.resource_server.credentials.lifecycle.create\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.resource_server.credentials.lifecycle.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"end\",\"start\"]},\"app.oauth2.as.resource_server.credentials.lifecycle.delete\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.detect_reuse\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.grant\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.grant.access_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.grant.device_secret\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.grant.id_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.grant.refresh_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.as.token.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.authorize\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.authorize.code\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.authorize.implicit.access_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.authorize.implicit.id_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.client.lifecycle.activate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-lifecycle\"],\"type\":[\"start\"]},\"app.oauth2.client.lifecycle.create\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-lifecycle\"],\"type\":[\"info\"]},\"app.oauth2.client.lifecycle.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-lifecycle\"],\"type\":[\"end\",\"start\"]},\"app.oauth2.client.lifecycle.delete\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-lifecycle\"],\"type\":[\"info\"]},\"app.oauth2.client.lifecycle.update\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-lifecycle\"],\"type\":[\"info\"]},\"app.oauth2.client.privilege.grant\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-client\"],\"type\":[\"info\"]},\"app.oauth2.client.privilege.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-client\"],\"type\":[\"info\"]},\"app.oauth2.client.read_client_secret\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\"],\"type\":[\"info\"]},\"app.oauth2.client_id_rate_limit_warning\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\"],\"type\":[\"info\"]},\"app.oauth2.consent.grant\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.credentials.lifecycle.activate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-credentials-lifecycle\"],\"type\":[\"start\"]},\"app.oauth2.credentials.lifecycle.create\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-credentials-lifecycle\"],\"type\":[\"info\"]},\"app.oauth2.credentials.lifecycle.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-credentials-lifecycle\"],\"type\":[\"end\",\"start\"]},\"app.oauth2.credentials.lifecycle.delete\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\",\"oauth2-client-credentials-lifecycle\"],\"type\":[\"info\"]},\"app.oauth2.interact.interaction_code\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.interact.interaction_handle\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.invalid_client_credentials\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.key.rollover\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.signon\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-client\"],\"type\":[\"info\"]},\"app.oauth2.token.detect_reuse\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.grant\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.grant.access_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.grant.id_jag\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.grant.id_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.grant.refresh_token\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.revoke.implicit.as\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.revoke.implicit.client\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.token.revoke.implicit.user\":{\"category\":[\"authentication\"],\"tags\":[\"oauth2\",\"oauth2-as-runtime\",\"oauth2-org-as\"],\"type\":[\"info\"]},\"app.oauth2.trusted_server.add\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.oauth2.trusted_server.delete\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\",\"oauth2-as-runtime\",\"oauth2-custom-as\"],\"type\":[\"info\"]},\"app.office365.api.change.domain.federation.success\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\"]},\"app.office365.api.error.ad.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.check.user.exists\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.create.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"creation\",\"user\"]},\"app.office365.api.error.deactivate.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.download.custom.objects\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.download.groups\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"group\"]},\"app.office365.api.error.download.users\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.endpoint.unavailable\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.get.company.dirsync.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.get.company.dirsync.status.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.get.company.dirsync.status.pending\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.get.object.ids.by.group.id\":{\"category\":[\"iam\"],\"tags\":[\"office365-app\"],\"type\":[\"group\"]},\"app.office365.api.error.group.create.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"creation\",\"group\"]},\"app.office365.api.error.group.create.failure.name.in.use\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"creation\",\"group\"]},\"app.office365.api.error.group.delete.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\",\"group\"]},\"app.office365.api.error.group.membership.update.assignment.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.group.membership.update.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.group.membership.update.group.not.found.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.group.membership.update.removal.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.group.update.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.group.update.failure.not.found\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"change\",\"group\"]},\"app.office365.api.error.import.profile\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.no.endpoints.found\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.push.password\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.push.profile\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.reactivate.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.remove.domain.federation.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\"]},\"app.office365.api.error.remove.domain.federation.failure.access.denied\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\"]},\"app.office365.api.error.remove.domain.federation.failure.domain.not.found\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\"]},\"app.office365.api.error.revoke.refresh.token\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.company.dirsync.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.company.dirsync.status.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.domain.federation.failure\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.domain.federation.failure.access.denied\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.domain.federation.failure.domain.default\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.set.domain.federation.failure.domain.not.found\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.sync.contact\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.sync.finalize\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.sync.group\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"group\"]},\"app.office365.api.error.sync.not.activated\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.sync.set.attribute\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.sync.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.api.error.unable.to.create.graph.client\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"creation\"]},\"app.office365.api.error.validate.admin.creds\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"admin\"]},\"app.office365.api.error.validate.creds\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.validate.creds.unknown.exception\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.error.x-ms-forwarded-client-ip-header.absent\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.remove.domain.federation.success\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\"]},\"app.office365.api.set.domain.federation.success\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.sync.complete\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.sync.heartbeat.sent\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.sync.job.complete\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.sync.job.complete.contact\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.api.sync.job.complete.group\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"group\"]},\"app.office365.api.sync.job.complete.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.clientplatform.conversion.job.processing.app.instance\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.clientplatform.conversion.job.skipping.migration\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.conflict-object\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.critical-system-object\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.non-security-group-invalid-mail\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.reserved-attribute-value\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.systemmailbox\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.dirsync.skipping.without-name-and-displayname\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.error.importing.user\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.graph.api.error.no.mailbox.found\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.graph.api.error.rate-limit.exceeded\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.graph.api.error.service.principal.creation.failed\":{\"category\":[\"configuration\"],\"tags\":[\"office365-app\"],\"type\":[\"info\"]},\"app.office365.graph.api.error.service.principal.msgraph.authentication.failure\":{\"category\":[\"authentication\"],\"tags\":[\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.complete\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.invalid.credentials\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.processing\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.skipping.missing.creds\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.skipping.no.service.principal\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"info\"]},\"app.office365.service.principal.cleanup.job.unable.to.delete.service.principal\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\"]},\"app.office365.user.delete.success\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\",\"user\"]},\"app.office365.user.lifecycle.action.failed\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"user\"]},\"app.office365.user.remove.licenses.success\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"office365-app\"],\"type\":[\"deletion\",\"user\"]},\"app.policy.sign_on.update\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"change\"]},\"app.radius.agent.listener.failed\":{\"category\":[\"network\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"denied\",\"protocol\"]},\"app.radius.agent.listener.succeeded\":{\"category\":[\"network\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"protocol\"]},\"app.radius.agent.port_inaccessible\":{\"category\":[\"network\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"access\",\"protocol\"]},\"app.radius.agent.port_reaccessible\":{\"category\":[\"network\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"access\",\"protocol\"]},\"app.radius.info_access.no_permission\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"info\"]},\"app.radius.info_access.partial_permission\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"info\"]},\"app.realtimesync.import.details.add_user\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.realtimesync.import.details.delete_user\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"deletion\"]},\"app.realtimesync.import.details.update_user\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\",\"info\"]},\"app.request_new.notify\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"app.rum.config.validation.error\":{\"category\":[\"configuration\"],\"tags\":[\"rum\"],\"type\":[\"info\"]},\"app.rum.is.api.account.error\":{\"category\":[\"configuration\"],\"tags\":[\"rum\"],\"type\":[\"info\"]},\"app.rum.package.thrown.error\":{\"category\":[\"configuration\"],\"tags\":[\"rum\"],\"type\":[\"info\"]},\"app.rum.validation.error\":{\"category\":[\"configuration\"],\"tags\":[\"rum\"],\"type\":[\"info\"]},\"app.saml.sensitive.attribute.update\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"cvd\"],\"type\":[\"change\"]},\"app.user_management\":{\"category\":[\"configuration\"],\"tags\":[\"app-user-management\"],\"type\":[\"info\"]},\"app.user_management.grouppush.mapping.created.from.rule\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.user_management.grouppush.mapping.created.from.rule.error.duplicate\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.user_management.grouppush.mapping.created.from.rule.error.validation\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.user_management.grouppush.mapping.created.from.rule.errors\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"app.user_management.grouppush.mapping.okta.users.ignored\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"info\"]},\"app.user_management.import.csv.line.error\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.user_management.push_new_user_success\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"app.user_management.update_from_master_failed\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"app.user_management.user_group_import.create_failure\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"creation\"]},\"app.user_management.user_group_import.delete_success\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"deletion\"]},\"app.user_management.user_group_import.update_failure\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"change\"]},\"app.user_management.user_group_import.upsert_fail\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"info\"]},\"app.user_management.user_group_import.upsert_success\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"app-user-management\"],\"type\":[\"info\"]},\"application.appuser.mapping.invalid.expression\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"user\"]},\"application.cache.invalidate\":{\"category\":[\"configuration\"],\"tags\":[\"invalidate-app-list-cache\"],\"type\":[\"info\"]},\"application.configuration.detect_error\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.configuration.disable_delauth_outbound\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"end\"]},\"application.configuration.disable_fed_broker_mode\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.configuration.enable_delauth_outbound\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"start\"]},\"application.configuration.enable_fed_broker_mode\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.configuration.import_schema\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.configuration.read_client_secret\":{\"category\":[\"configuration\"],\"tags\":[\"agent\",\"app\"],\"type\":[\"info\"]},\"application.configuration.reset_logo\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"application.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"change\"]},\"application.configuration.update_api_credentials_for_pass_change\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.configuration.update_logo\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"application.configuration.update_rate_limits\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"application.integration.api_query\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.integration.authentication_failure\":{\"category\":[\"authentication\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.integration.general_failure\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.integration.rate_limit_exceeded\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.integration.transfer_files\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"application.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"application.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"application.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"deletion\"]},\"application.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"application.policy.sign_on.deny_access\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"access\",\"info\"]},\"application.policy.sign_on.rule.create\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.policy.sign_on.rule.delete\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"deletion\"]},\"application.policy.sign_on.update\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"changeDetails\"],\"type\":[\"change\"]},\"application.provision.field_mapping_rule.change\":{\"category\":[\"configuration\"],\"tags\":[\"field-mapping-rule-modification\"],\"type\":[\"change\",\"creation\"]},\"application.provision.group.add\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group.import\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group.remove\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"deletion\",\"group\"]},\"application.provision.group.update\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"change\",\"creation\",\"group\"]},\"application.provision.group.verify_exists\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_membership.add\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_membership.import\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_membership.remove\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"deletion\",\"group\"]},\"application.provision.group_membership.update\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"change\",\"creation\",\"group\"]},\"application.provision.group_push.activate_mapping\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.deactivate_mapping\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.delete_appgroup\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"deletion\"]},\"application.provision.group_push.mapping.and.groups.deleted.rule.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"deletion\"]},\"application.provision.group_push.mapping.app.group.renamed\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.mapping.app.group.renamed.failed\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.mapping.created\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.mapping.created.from.rule.warning.duplicate.name\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.mapping.created.from.rule.warning.duplicate.name.tobecreated\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.mapping.created.from.rule.warning.upsertGroup.duplicate.name\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.mapping.deactivated.source.group.renamed\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.mapping.deactivated.source.group.renamed.failed\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.mapping.update.or.delete.failed\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\",\"creation\",\"deletion\"]},\"application.provision.group_push.mapping.update.or.delete.failed.with.error\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"change\",\"creation\",\"deletion\"]},\"application.provision.group_push.push_memberships\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"group\"]},\"application.provision.group_push.pushed\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.provision.group_push.removed\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"deletion\"]},\"application.provision.group_push.updated\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\",\"creation\"]},\"application.provision.integration.call_api\":{\"category\":[\"configuration\"],\"tags\":[\"app-api\"],\"type\":[\"creation\"]},\"application.provision.user.activate\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.deprovision\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"creation\",\"deletion\",\"user\"]},\"application.provision.user.import\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.import_profile\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.password\":{\"category\":[\"authentication\"],\"tags\":[\"app-api\"],\"type\":[\"info\"]},\"application.provision.user.push\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.push_okta_password\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.provision.user.push_password\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.provision.user.push_profile\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.reactivate\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.sync\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"creation\",\"user\"]},\"application.provision.user.verify_exists\":{\"category\":[\"iam\"],\"tags\":[\"app-api\"],\"type\":[\"creation\",\"user\"]},\"application.registration_policy.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"creation\"]},\"application.registration_policy.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"application.user_membership.add\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user-provision\"],\"type\":[\"creation\",\"user\"]},\"application.user_membership.approve\":{\"category\":[\"iam\"],\"tags\":[\"user-provision\"],\"type\":[\"user\"]},\"application.user_membership.change_password\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"application.user_membership.change_username\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"change\",\"user\"]},\"application.user_membership.deprovision\":{\"category\":[\"iam\"],\"tags\":[\"user-provision\"],\"type\":[\"deletion\",\"user\"]},\"application.user_membership.provision\":{\"category\":[\"iam\"],\"tags\":[\"user-provision\"],\"type\":[\"creation\",\"user\"]},\"application.user_membership.remove\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user-provision\"],\"type\":[\"deletion\",\"user\"]},\"application.user_membership.restore\":{\"category\":[\"iam\"],\"tags\":[\"app\"],\"type\":[\"user\"]},\"application.user_membership.restore_password\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.user_membership.revoke\":{\"category\":[\"iam\"],\"tags\":[\"user-provision\"],\"type\":[\"deletion\",\"user\"]},\"application.user_membership.show_password\":{\"category\":[\"authentication\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"application.user_membership.update\":{\"category\":[\"iam\"],\"tags\":[\"app\",\"event-hook-eligible\"],\"type\":[\"change\",\"user\"]},\"certification.campaign.close\":{\"category\":[\"configuration\"],\"tags\":[\"certification\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"certification.campaign.context.update\":{\"category\":[\"configuration\"],\"tags\":[\"certification\",\"event-hook-eligible\"],\"type\":[\"change\"]},\"certification.campaign.create\":{\"category\":[\"configuration\"],\"tags\":[\"certification\"],\"type\":[\"creation\"]},\"certification.campaign.delete\":{\"category\":[\"configuration\"],\"tags\":[\"certification\"],\"type\":[\"deletion\"]},\"certification.campaign.item.decide\":{\"category\":[\"configuration\"],\"tags\":[\"certification\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"certification.campaign.item.remediate\":{\"category\":[\"configuration\"],\"tags\":[\"certification\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"certification.campaign.launch\":{\"category\":[\"configuration\"],\"tags\":[\"certification\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"certification.campaign.update\":{\"category\":[\"configuration\"],\"tags\":[\"certification\"],\"type\":[\"change\"]},\"certification.remediation.open\":{\"category\":[\"configuration\"],\"tags\":[\"certification\"],\"type\":[\"info\"]},\"core.concurrency.org.limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"concurrency-limit\"],\"type\":[\"info\"]},\"core.el.evaluate\":{\"category\":[\"configuration\"],\"tags\":[\"okta-el\"],\"type\":[\"info\"]},\"core.user_auth.idp.x509.crl_download_failure\":{\"category\":[\"authentication\"],\"tags\":[\"x509-idp-auth\"],\"type\":[\"info\"]},\"credential.register\":{\"category\":[\"authentication\"],\"tags\":[\"user-factor\"],\"type\":[\"info\"]},\"credential.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"user-factor\"],\"type\":[\"info\"]},\"device.assurance.policy.add\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"creation\"]},\"device.assurance.policy.delete\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"deletion\"]},\"device.assurance.policy.update\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"device-identity\",\"oie-only\"],\"type\":[\"change\"]},\"device.custom_push.send_notification\":{\"category\":[\"authentication\"],\"tags\":[\"custom-push\"],\"type\":[\"info\"]},\"device.desktop_mfa.configuration.update\":{\"category\":[\"authentication\"],\"tags\":[\"changeDetails\",\"device-mfa\",\"oie-only\"],\"type\":[\"info\"]},\"device.desktop_mfa.device_logout.completed\":{\"category\":[\"authentication\"],\"tags\":[\"device-mfa\",\"oie-only\"],\"type\":[\"end\"]},\"device.desktop_mfa.device_logout.started\":{\"category\":[\"authentication\"],\"tags\":[\"device-mfa\",\"oie-only\"],\"type\":[\"end\",\"start\"]},\"device.desktop_mfa.enrollment.create\":{\"category\":[\"authentication\"],\"tags\":[\"device-mfa\",\"oie-only\"],\"type\":[\"info\"]},\"device.desktop_mfa.recovery_pin.generate\":{\"category\":[\"authentication\"],\"tags\":[\"device-mfa\",\"oie-only\"],\"type\":[\"info\"]},\"device.desktop_mfa.recovery_pin.rotate_secret\":{\"category\":[\"authentication\"],\"tags\":[\"device-mfa\",\"oie-only\"],\"type\":[\"info\"]},\"device.enrollment.create\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.integration.endpoint_security.activate\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.integration.endpoint_security.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.lifecycle.activate\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"start\"]},\"device.lifecycle.deactivate\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"end\",\"start\"]},\"device.lifecycle.delete\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.lifecycle.suspend\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"end\"]},\"device.lifecycle.unsuspend\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"end\"]},\"device.local_account.create\":{\"category\":[\"host\"],\"tags\":[\"device-sso\",\"oie-only\"],\"type\":[\"info\"]},\"device.password_sync.authentication\":{\"category\":[\"authentication\"],\"tags\":[\"device-sso\",\"oie-only\"],\"type\":[\"info\"]},\"device.password_sync.enrollment.create\":{\"category\":[\"authentication\"],\"tags\":[\"device-sso\",\"oie-only\"],\"type\":[\"info\"]},\"device.platform.add\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.platform.delete\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.platform.renew\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"device.platform.secret_key.reset\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"change\"]},\"device.platform.update\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"change\"]},\"device.platform_sso.keys.register\":{\"category\":[\"authentication\"],\"tags\":[\"device-sso\",\"oie-only\"],\"type\":[\"info\"]},\"device.posture.check.add\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"device.posture.check.delete\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"device.posture.check.update\":{\"category\":[\"host\"],\"tags\":[\"changeDetails\",\"device-identity\",\"oie-only\"],\"type\":[\"change\"]},\"device.push.provider.create\":{\"category\":[\"host\"],\"tags\":[\"oie-only\",\"push-provider\"],\"type\":[\"info\"]},\"device.push.provider.delete\":{\"category\":[\"host\"],\"tags\":[\"oie-only\",\"push-provider\"],\"type\":[\"info\"]},\"device.push.provider.update\":{\"category\":[\"host\"],\"tags\":[\"oie-only\",\"push-provider\"],\"type\":[\"change\"]},\"device.signals.status.timeout\":{\"category\":[\"host\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"device.token.enrollment.create\":{\"category\":[\"authentication\"],\"tags\":[\"device\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"device.user.add\":{\"category\":[\"iam\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"creation\",\"user\"]},\"device.user.remove\":{\"category\":[\"iam\"],\"tags\":[\"device-identity\",\"event-hook-eligible\",\"oie-only\",\"user\"],\"type\":[\"deletion\",\"user\"]},\"device.user_os_account.sync\":{\"category\":[\"host\"],\"tags\":[\"device-mfa\",\"device-sso\",\"oie-only\"],\"type\":[\"info\"]},\"directory.app_user_profile.bootstrap\":{\"category\":[\"iam\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"creation\",\"user\"]},\"directory.app_user_profile.update\":{\"category\":[\"iam\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"change\",\"user\"]},\"directory.external.group.membership.add\":{\"category\":[\"iam\"],\"tags\":[\"ad-agent\",\"group\",\"user\"],\"type\":[\"creation\",\"group\"]},\"directory.external.group.membership.remove\":{\"category\":[\"iam\"],\"tags\":[\"ad-agent\",\"group\",\"user\"],\"type\":[\"deletion\",\"group\"]},\"directory.linked_object.create\":{\"category\":[\"configuration\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"creation\"]},\"directory.linked_object.delete\":{\"category\":[\"configuration\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"deletion\"]},\"directory.mapping.update\":{\"category\":[\"configuration\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"change\"]},\"directory.non_default_user_profile.create\":{\"category\":[\"iam\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"creation\",\"user\"]},\"directory.user_profile.bootstrap\":{\"category\":[\"iam\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"creation\",\"user\"]},\"directory.user_profile.update\":{\"category\":[\"iam\"],\"tags\":[\"cvd\",\"directory\"],\"type\":[\"change\",\"info\",\"user\"]},\"event_hook.activated\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"change\"]},\"event_hook.created\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"creation\"]},\"event_hook.deactivated\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"change\"]},\"event_hook.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"deletion\"]},\"event_hook.delivery\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"info\"]},\"event_hook.updated\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"change\"]},\"event_hook.verified\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook\"],\"type\":[\"info\"]},\"group.application_assignment.add\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"creation\",\"group\"]},\"group.application_assignment.remove\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"deletion\",\"group\"]},\"group.application_assignment.skip_assignment_reconcile\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\"]},\"group.application_assignment.update\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"change\",\"group\"]},\"group.lifecycle.create\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"creation\",\"group\"]},\"group.lifecycle.delete\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"deletion\",\"group\"]},\"group.privilege.grant\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"group\",\"info\"]},\"group.privilege.revoke\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"deletion\",\"group\"]},\"group.profile.update\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"admin\",\"change\",\"group\"]},\"group.user_membership.add\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"creation\",\"group\",\"user\"]},\"group.user_membership.remove\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"group\"],\"type\":[\"deletion\",\"group\",\"user\"]},\"group.user_membership.rule.add_exclusion\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"creation\",\"group\",\"user\"]},\"group.user_membership.rule.deactivated\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\",\"user\"]},\"group.user_membership.rule.error\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\",\"user\"]},\"group.user_membership.rule.evaluation\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\",\"user\"]},\"group.user_membership.rule.invalidate\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\",\"user\"]},\"group.user_membership.rule.trigger\":{\"category\":[\"iam\"],\"tags\":[\"group\"],\"type\":[\"group\",\"user\"]},\"iam.policy.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\",\"info\"]},\"iam.resourceset.bindings.add\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\",\"info\"]},\"iam.resourceset.bindings.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\",\"info\"]},\"iam.resourceset.create\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\",\"info\"]},\"iam.resourceset.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\",\"info\"]},\"iam.resourceset.resources.add\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\",\"info\"]},\"iam.resourceset.resources.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\",\"info\"]},\"iam.resourceset.resources.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\",\"info\"]},\"iam.resourceset.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin-role\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\",\"info\"]},\"iam.role.create\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\",\"info\"]},\"iam.role.delete\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\",\"info\"]},\"iam.role.permission.conditions.add\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\"]},\"iam.role.permission.conditions.delete\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\"]},\"iam.role.permissions.add\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"creation\",\"info\"]},\"iam.role.permissions.delete\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"event-hook-eligible\"],\"type\":[\"deletion\",\"info\"]},\"iam.role.update\":{\"category\":[\"iam\"],\"tags\":[\"admin-role\",\"changeDetails\",\"event-hook-eligible\"],\"type\":[\"change\",\"info\"]},\"inline_hook.activated\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"change\"]},\"inline_hook.created\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"creation\"]},\"inline_hook.deactivated\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"change\"]},\"inline_hook.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"deletion\"]},\"inline_hook.executed\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"inline-hook\"],\"type\":[\"info\"]},\"inline_hook.response.processed\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"info\"]},\"inline_hook.updated\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"change\"]},\"inline_hook.verified\":{\"category\":[\"configuration\"],\"tags\":[\"inline-hook\"],\"type\":[\"info\"]},\"integration.api_service.lifecycle.authorize\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"integration\"],\"type\":[\"info\"]},\"integration.api_service.lifecycle.revoke\":{\"category\":[\"configuration\"],\"tags\":[\"app\",\"integration\"],\"type\":[\"deletion\"]},\"master_application.user_membership.add\":{\"category\":[\"configuration\"],\"tags\":[\"uncategorized\"],\"type\":[\"info\"]},\"mim.command.generic.acknowledged\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.generic.cancelled\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"deletion\"]},\"mim.command.generic.delegated\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.generic.error\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.generic.new\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.generic.notnow\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.ios.acknowledged\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.ios.cancelled\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"deletion\"]},\"mim.command.ios.error\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.ios.formaterror\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.command.ios.new\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"mim.createEnrollment.ANDROID\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"creation\"]},\"mim.createEnrollment.IOS\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"creation\"]},\"mim.createEnrollment.OSX\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"creation\"]},\"mim.createEnrollment.UNKNOWN\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"creation\"]},\"mim.createEnrollment.WINDOWS\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"creation\"]},\"mim.streamDevicesCSVDownload\":{\"category\":[\"configuration\"],\"tags\":[\"mim\"],\"type\":[\"info\"]},\"network_zone.rule.disabled\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"change\"]},\"oauth2.as.activated\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-as-lifecycle\"],\"type\":[\"change\"]},\"oauth2.as.created\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-as-lifecycle\"],\"type\":[\"creation\"]},\"oauth2.as.deactivated\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-as-lifecycle\"],\"type\":[\"change\"]},\"oauth2.as.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-as-lifecycle\"],\"type\":[\"deletion\"]},\"oauth2.as.updated\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-as-lifecycle\"],\"type\":[\"change\"]},\"oauth2.claim.created\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-claim\"],\"type\":[\"creation\"]},\"oauth2.claim.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-claim\"],\"type\":[\"deletion\"]},\"oauth2.claim.updated\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-claim\"],\"type\":[\"change\"]},\"oauth2.scope.created\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-scope\"],\"type\":[\"creation\"]},\"oauth2.scope.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-scope\"],\"type\":[\"deletion\"]},\"oauth2.scope.updated\":{\"category\":[\"configuration\"],\"tags\":[\"oauth2\",\"oauth2-scope\"],\"type\":[\"info\"]},\"org.not_configured_origin.redirection.usage\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"org\"],\"type\":[\"info\"]},\"pam.active_directory.account_discovery.complete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.active_directory.account_rule.applied\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.active_directory.account_rule.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.active_directory.connection.update\":{\"category\":[\"network\"],\"tags\":[\"pam\"],\"type\":[\"connection\"]},\"pam.ad_connection.create\":{\"category\":[\"network\"],\"tags\":[\"pam\"],\"type\":[\"connection\"]},\"pam.ad_connection.delete\":{\"category\":[\"network\"],\"tags\":[\"pam\"],\"type\":[\"connection\"]},\"pam.ad_connection.update\":{\"category\":[\"network\"],\"tags\":[\"pam\"],\"type\":[\"connection\"]},\"pam.ad_task_settings.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.ad_task_settings.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.ad_task_settings.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.ad_task_settings.update_schedule\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.ad_user_sync_task_settings.activate\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.ad_user_sync_task_settings.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.ad_user_sync_task_settings.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.ad_user_sync_task_settings.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.ad_user_sync_task_settings.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.ad_user_sync_task_settings.update_schedule\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.apikey.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.apikey.rotate\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.auth_token.issue\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.billing_contact.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.client.assign\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.client.enroll\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.client.remove\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.client.state.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.client_enrollment_policies.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.client_enrollment_policies.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.client_enrollment_policies.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.client_enrollment_policy_token.delete\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.client_enrollment_policy_token.rotate\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.cloud_account.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.cloud_account.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.cloud_account.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.entitlement_sudo.add_to_project\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.entitlement_sudo.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.entitlement_sudo.remove\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.entitlement_sudo.remove_from_project\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.entitlement_sudo.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.gateway.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.gateway.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.gateway.setup_token.create\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.gateway.setup_token.delete\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.gateway.setup_token.update\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.gateway.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.gateway_creds.issue\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.group.bulk_membership_change\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"group\"]},\"pam.group.create\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"creation\",\"group\"]},\"pam.group.delete\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"deletion\",\"group\"]},\"pam.incoming_federation.approve\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.incoming_federation.request\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.member.add\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.member.remove\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.offline_disabled_event\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.offline_enabled_event\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.offline_group.secrets.rotate\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"change\",\"group\"]},\"pam.outgoing_federation.approve\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.password.change\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.password.reset\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.permission.change\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.preauthorization.create\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.preauthorization.update\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.project.add_group\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.project.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.project.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.project.remove_group\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.project.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.project_group_selector.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.resource.checkin.end\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.resource.checkin.start\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.resource.checkout\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.resource_group.create\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"admin\",\"creation\",\"group\"]},\"pam.resource_group.delete\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"admin\",\"deletion\",\"group\"]},\"pam.resource_group.update\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"admin\",\"change\",\"group\"]},\"pam.secret.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.secret.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.secret.reveal\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.secret.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.secret_folder.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.secret_folder.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.secret_folder.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.security_policy.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.security_policy.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.security_policy.evaluate\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.security_policy.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.server.enroll\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.server.reassign\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.server.remove\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.server.ssh_login\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"start\"]},\"pam.server_account.discovered\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.server_account.password_change.initiated\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"start\"]},\"pam.server_account.password_change.out_of_band\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.server_account.password_change.update\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.server_account.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.server_labels.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.service.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.service.remove\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.service_account.assign\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.service_account.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.service_account.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.service_account.password.reveal\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.service_account.password.update\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"pam.service_account.password_rotation.end\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"end\"]},\"pam.service_account.password_rotation.start\":{\"category\":[\"authentication\"],\"tags\":[\"pam\"],\"type\":[\"start\"]},\"pam.service_account.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.sudo_command_bundle.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.sudo_command_bundle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.sudo_command_bundle.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.team.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_group_attribute.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_group_attribute.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.team_group_attribute.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.team_invitation.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_project_group_attribute.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_project_group_attribute.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.team_project_group_attribute.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.team_project_user_attribute.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_project_user_attribute.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.team_project_user_attribute.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.team_settings.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.team_user_attribute.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.team_user_attribute.delete\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"deletion\"]},\"pam.team_user_attribute.update\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"change\"]},\"pam.unbound_client.enroll\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.unmanaged_server.create\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"creation\"]},\"pam.user.create\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"creation\",\"user\"]},\"pam.user.remove\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"deletion\",\"user\"]},\"pam.user.update\":{\"category\":[\"iam\"],\"tags\":[\"pam\"],\"type\":[\"change\",\"user\"]},\"pam.user_creds.issue\":{\"category\":[\"configuration\"],\"tags\":[\"pam\"],\"type\":[\"info\"]},\"personal.admin.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"okta-personal\"],\"type\":[\"change\"]},\"personal.user.app_migration.export\":{\"category\":[\"iam\"],\"tags\":[\"okta-personal\"],\"type\":[\"user\"]},\"pki.ca.add\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"creation\"]},\"pki.ca.delete\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"deletion\"]},\"pki.ca.renew\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"pki.cert.bind\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"pki.cert.crl_download_failure\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\"],\"type\":[\"info\"]},\"pki.cert.issue\":{\"category\":[\"configuration\"],\"tags\":[\"device-trust-cert-distribution-and-binding\"],\"type\":[\"info\"]},\"pki.cert.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"pki.cert.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"deletion\"]},\"pki.cert.lifecycle.hold\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"pki.cert.lifecycle.revoke\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"deletion\"]},\"pki.cert.lifecycle.suspend\":{\"category\":[\"configuration\"],\"tags\":[\"device-identity\",\"oie-only\",\"user\"],\"type\":[\"info\"]},\"pki.cert.renew\":{\"category\":[\"configuration\"],\"tags\":[\"device-trust-cert-distribution-and-binding\"],\"type\":[\"info\"]},\"pki.cert.revoke\":{\"category\":[\"configuration\"],\"tags\":[\"device-trust-cert-distribution-and-binding\"],\"type\":[\"deletion\"]},\"plugin.downloaded\":{\"category\":[\"configuration\"],\"tags\":[\"plugin\"],\"type\":[\"info\"]},\"plugin.script_status\":{\"category\":[\"configuration\"],\"tags\":[\"plugin\"],\"type\":[\"info\"]},\"policy.auth_reevaluate.action\":{\"category\":[\"authentication\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"info\"]},\"policy.auth_reevaluate.enforce\":{\"category\":[\"authentication\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"info\"]},\"policy.auth_reevaluate.fail\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"policy\",\"security\",\"session\"],\"type\":[\"info\"]},\"policy.continuous_access.action\":{\"category\":[\"configuration\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"access\"]},\"policy.continuous_access.evaluate\":{\"category\":[\"configuration\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"access\"]},\"policy.entity_risk.action\":{\"category\":[\"configuration\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"info\"]},\"policy.entity_risk.evaluate\":{\"category\":[\"configuration\"],\"tags\":[\"policy\",\"security\",\"session\"],\"type\":[\"info\"]},\"policy.evaluate_sign_on\":{\"category\":[\"authentication\"],\"tags\":[\"policy\"],\"type\":[\"info\"]},\"policy.execute.user.start\":{\"category\":[\"iam\"],\"tags\":[\"policy\"],\"type\":[\"user\"]},\"policy.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"info\"]},\"policy.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"creation\"]},\"policy.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"info\"]},\"policy.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"deletion\"]},\"policy.lifecycle.overwrite\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"info\"]},\"policy.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"event-hook-eligible\",\"policy\"],\"type\":[\"change\"]},\"policy.mapping.create\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"creation\"]},\"policy.rule.action.execute\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"info\"]},\"policy.rule.activate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"info\"]},\"policy.rule.add\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"creation\"]},\"policy.rule.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"info\"]},\"policy.rule.delete\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"deletion\"]},\"policy.rule.invalidate\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"info\"]},\"policy.rule.update\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"policy\"],\"type\":[\"change\"]},\"policy.scheduled.execute\":{\"category\":[\"configuration\"],\"tags\":[\"policy\"],\"type\":[\"info\"]},\"scheduled_action.user_suspension.canceled\":{\"category\":[\"configuration\"],\"tags\":[\"uncategorized\"],\"type\":[\"change\"]},\"scheduled_action.user_suspension.completed\":{\"category\":[\"configuration\"],\"tags\":[\"uncategorized\"],\"type\":[\"info\"]},\"scheduled_action.user_suspension.scheduled\":{\"category\":[\"configuration\"],\"tags\":[\"uncategorized\"],\"type\":[\"info\"]},\"scheduled_action.user_suspension.updated\":{\"category\":[\"configuration\"],\"tags\":[\"uncategorized\"],\"type\":[\"change\"]},\"security.attack.end\":{\"category\":[\"threat\"],\"tags\":[\"threat-insight\"],\"type\":[\"indicator\"]},\"security.attack.start\":{\"category\":[\"threat\"],\"tags\":[\"threat-insight\"],\"type\":[\"indicator\"]},\"security.attack_protection.settings.update\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"mfa\",\"security\"],\"type\":[\"change\"]},\"security.authenticator.lifecycle.activate\":{\"category\":[\"authentication\"],\"tags\":[\"authenticator\",\"event-hook-eligible\",\"oie-only\"],\"type\":[\"info\",\"start\"]},\"security.authenticator.lifecycle.create\":{\"category\":[\"authentication\"],\"tags\":[\"authenticator\",\"event-hook-eligible\",\"oie-only\"],\"type\":[\"info\"]},\"security.authenticator.lifecycle.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"authenticator\",\"event-hook-eligible\",\"oie-only\"],\"type\":[\"end\",\"info\",\"start\"]},\"security.authenticator.lifecycle.update\":{\"category\":[\"authentication\"],\"tags\":[\"authenticator\",\"event-hook-eligible\",\"oie-only\"],\"type\":[\"info\"]},\"security.behavior.settings.create\":{\"category\":[\"configuration\"],\"tags\":[\"behavior-settings\"],\"type\":[\"creation\",\"info\"]},\"security.behavior.settings.delete\":{\"category\":[\"configuration\"],\"tags\":[\"behavior-settings\"],\"type\":[\"deletion\",\"info\"]},\"security.behavior.settings.update\":{\"category\":[\"configuration\"],\"tags\":[\"behavior-settings\"],\"type\":[\"change\",\"info\"]},\"security.breached_credential.detected\":{\"category\":[\"authentication\"],\"tags\":[\"account\",\"event-hook-eligible\",\"security\",\"user\"],\"type\":[\"info\"]},\"security.device.add_request_blacklist_policy\":{\"category\":[\"threat\"],\"tags\":[\"device\",\"security\"]},\"security.device.remove_request_blacklist_policy\":{\"category\":[\"threat\"],\"tags\":[\"device\",\"security\"]},\"security.device.temporarily_disable_blacklisting\":{\"category\":[\"threat\"],\"tags\":[\"device\",\"security\"]},\"security.events.provider.activate\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.provider.create\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.provider.deactivate\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.provider.delete\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.provider.receive_event\":{\"category\":[\"threat\"]},\"security.events.provider.update\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.transmitter.create\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.transmitter.delete\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.events.transmitter.update\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.request.blocked\":{\"category\":[\"threat\"],\"tags\":[\"security\"]},\"security.session.detect_client_roaming\":{\"category\":[\"session\"],\"tags\":[\"security\",\"session\"],\"type\":[\"info\"]},\"security.threat.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"threat-insight-configuration\"],\"type\":[\"change\",\"info\"]},\"security.threat.detected\":{\"category\":[\"threat\"],\"tags\":[\"security\",\"threat-insight\"],\"type\":[\"indicator\"]},\"security.trusted_origin.activate\":{\"category\":[\"threat\"],\"tags\":[\"changeDetails\",\"event-hook-eligible\",\"trusted-origins\"]},\"security.trusted_origin.create\":{\"category\":[\"threat\"],\"tags\":[\"event-hook-eligible\",\"trusted-origins\"]},\"security.trusted_origin.deactivate\":{\"category\":[\"threat\"],\"tags\":[\"changeDetails\",\"event-hook-eligible\",\"trusted-origins\"]},\"security.trusted_origin.delete\":{\"category\":[\"threat\"],\"tags\":[\"event-hook-eligible\",\"trusted-origins\"]},\"security.trusted_origin.update\":{\"category\":[\"threat\"],\"tags\":[\"event-hook-eligible\",\"trusted-origins\"]},\"security.voice.add_country_blacklist\":{\"category\":[\"threat\"],\"tags\":[\"security\",\"voice\"]},\"security.voice.remove_country_blacklist\":{\"category\":[\"threat\"],\"tags\":[\"security\",\"voice\"]},\"security.zone.make_blacklist\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\",\"security\"],\"type\":[\"info\"]},\"security.zone.remove_blacklist\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\",\"security\"],\"type\":[\"deletion\"]},\"self_service.disabled\":{\"category\":[\"configuration\"],\"tags\":[\"self-service\"],\"type\":[\"change\"]},\"self_service.enabled\":{\"category\":[\"configuration\"],\"tags\":[\"self-service\"],\"type\":[\"change\"]},\"support.org.update\":{\"category\":[\"configuration\"],\"tags\":[\"support-audit\"],\"type\":[\"change\",\"info\"]},\"support.org.view\":{\"category\":[\"configuration\"],\"tags\":[\"support-audit\"],\"type\":[\"info\"]},\"system.agent.ad.config_change_detected\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\",\"changeDetails\"],\"type\":[\"info\"]},\"system.agent.ad.connect\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.create\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"creation\"]},\"system.agent.ad.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.delete\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"deletion\"]},\"system.agent.ad.import_ou\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.import_user\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.invoke_dir\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.reactivate\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.read_config\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.read_dirsync\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.read_ldap\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.read_schema\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.read_topology\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.realtimesync\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.reset_user_password\":{\"category\":[\"authentication\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.start\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.unlock_user_account\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.update\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"change\"]},\"system.agent.ad.update_user\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"change\"]},\"system.agent.ad.upgrade\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.upload_iwa_log\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.upload_log\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.ad.write_ldap\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\"],\"type\":[\"info\"]},\"system.agent.auto_update\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\",\"agent-pool\"],\"type\":[\"info\"]},\"system.agent.connector.connect\":{\"category\":[\"configuration\"],\"tags\":[\"connector-agent\"],\"type\":[\"info\"]},\"system.agent.connector.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"connector-agent\"],\"type\":[\"info\"]},\"system.agent.connector.delete\":{\"category\":[\"configuration\"],\"tags\":[\"connector-agent\"],\"type\":[\"deletion\"]},\"system.agent.connector.reactivate\":{\"category\":[\"configuration\"],\"tags\":[\"connector-agent\"],\"type\":[\"info\"]},\"system.agent.ldap.change_user_password\":{\"category\":[\"authentication\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.config_change_detected\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.create_user_JIT\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"creation\"]},\"system.agent.ldap.disconnect\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.realtimesync\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.reconnect\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.reset_user_password\":{\"category\":[\"authentication\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.unlock_user_account\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.ldap.update_user\":{\"category\":[\"configuration\"],\"tags\":[\"ldap-app\"],\"type\":[\"change\"]},\"system.agent.ldap.update_user_password\":{\"category\":[\"authentication\"],\"tags\":[\"ldap-app\"],\"type\":[\"info\"]},\"system.agent.register\":{\"category\":[\"configuration\"],\"tags\":[\"agent\"],\"type\":[\"creation\",\"info\"]},\"system.agent_pools.auto_update\":{\"category\":[\"configuration\"],\"tags\":[\"ad-agent\",\"agent-pool\"],\"type\":[\"info\"]},\"system.api_token.create\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"token\"],\"type\":[\"info\"]},\"system.api_token.enable\":{\"category\":[\"authentication\"],\"tags\":[\"token\"],\"type\":[\"start\"]},\"system.api_token.request_outside_allowed_range\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"token\"],\"type\":[\"info\"]},\"system.api_token.revoke\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"token\"],\"type\":[\"info\"]},\"system.api_token.update\":{\"category\":[\"authentication\"],\"tags\":[\"token\"],\"type\":[\"info\"]},\"system.beta.feature.enable\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"self-service-feature-management\",\"system\"],\"type\":[\"info\"]},\"system.brand.create\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"creation\",\"info\"]},\"system.brand.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"deletion\",\"info\"]},\"system.brand.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"change\",\"info\"]},\"system.captcha.create\":{\"category\":[\"configuration\"],\"tags\":[\"captcha\",\"system\"],\"type\":[\"creation\"]},\"system.captcha.delete\":{\"category\":[\"configuration\"],\"tags\":[\"captcha\",\"system\"],\"type\":[\"deletion\"]},\"system.captcha.update\":{\"category\":[\"configuration\"],\"tags\":[\"captcha\",\"system\"],\"type\":[\"change\"]},\"system.client.concurrency_rate_limit.notification\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.client.concurrency_rate_limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.client.rate_limit.notification\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.client.rate_limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.csv.import_user\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.custom_email_server.lifecycle.activate\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"system.custom_email_server.lifecycle.create\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"system.custom_email_server.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"system.custom_email_server.lifecycle.delete\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"system.custom_email_server.lifecycle.update\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"system.custom_error.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"deletion\"]},\"system.custom_error.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"change\"]},\"system.custom_signin.delete\":{\"category\":[\"authentication\"],\"tags\":[\"admin\"],\"type\":[\"start\"]},\"system.custom_signin.update\":{\"category\":[\"authentication\"],\"tags\":[\"admin\"],\"type\":[\"start\"]},\"system.custom_signout.update\":{\"category\":[\"authentication\"],\"tags\":[\"admin\"],\"type\":[\"end\"]},\"system.custom_url_domain.cert_renew\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.custom_url_domain.cert_upload\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"system\"],\"type\":[\"info\"]},\"system.custom_url_domain.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"deletion\",\"info\"]},\"system.custom_url_domain.initiate\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"info\"]},\"system.custom_url_domain.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"change\",\"info\"]},\"system.custom_url_domain.verify\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"info\"]},\"system.directory.debugger.extend\":{\"category\":[\"configuration\"],\"tags\":[\"agent\"],\"type\":[\"info\"]},\"system.directory.debugger.grant\":{\"category\":[\"configuration\"],\"tags\":[\"agent\"],\"type\":[\"info\"]},\"system.directory.debugger.query_executed\":{\"category\":[\"configuration\"],\"tags\":[\"agent\"],\"type\":[\"info\"]},\"system.directory.debugger.revoke\":{\"category\":[\"configuration\"],\"tags\":[\"agent\"],\"type\":[\"deletion\",\"info\"]},\"system.dr.failback\":{\"category\":[\"configuration\"],\"tags\":[\"dr\"],\"type\":[\"info\"]},\"system.dr.failover\":{\"category\":[\"configuration\"],\"tags\":[\"dr\"],\"type\":[\"info\"]},\"system.email.account_unlock.sent_message\":{\"category\":[\"configuration\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.bounce.removal\":{\"category\":[\"configuration\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.challenge_factor_redeemed\":{\"category\":[\"authentication\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.delivery\":{\"category\":[\"configuration\"],\"tags\":[\"email\",\"event-hook-eligible\"],\"type\":[\"info\"]},\"system.email.mfa_enroll_notification.sent_message\":{\"category\":[\"authentication\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.mfa_reset_notification.sent_message\":{\"category\":[\"authentication\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.new_device_notification.sent_message\":{\"category\":[\"configuration\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.password_reset.sent_message\":{\"category\":[\"authentication\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.send_factor_verify_message\":{\"category\":[\"authentication\"],\"tags\":[\"email\"],\"type\":[\"info\"]},\"system.email.template.create\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"email\"],\"type\":[\"creation\"]},\"system.email.template.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"email\"],\"type\":[\"deletion\"]},\"system.email.template.settings_changed\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"email\"],\"type\":[\"info\"]},\"system.email.template.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"email\"],\"type\":[\"change\"]},\"system.email_domain.create\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"creation\",\"info\"]},\"system.email_domain.delete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"deletion\",\"info\"]},\"system.email_domain.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"changeDetails\"],\"type\":[\"change\",\"info\"]},\"system.email_domain.verify\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"info\"]},\"system.feature.disable\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"self-service-feature-management\",\"system\"],\"type\":[\"info\"]},\"system.feature.ea_auto_enroll\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"self-service-feature-management\",\"system\"],\"type\":[\"info\"]},\"system.feature.enable\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"self-service-feature-management\",\"system\"],\"type\":[\"info\"]},\"system.hook.key.created\":{\"category\":[\"configuration\"],\"tags\":[\"hook-key\"],\"type\":[\"creation\",\"info\"]},\"system.hook.key.deleted\":{\"category\":[\"configuration\"],\"tags\":[\"hook-key\"],\"type\":[\"deletion\",\"info\"]},\"system.hook.key.updated\":{\"category\":[\"configuration\"],\"tags\":[\"hook-key\"],\"type\":[\"change\",\"info\"]},\"system.identity_sources.bulk_delete\":{\"category\":[\"configuration\"],\"tags\":[\"identity-sources\"],\"type\":[\"info\"]},\"system.identity_sources.bulk_group_delete\":{\"category\":[\"configuration\"],\"tags\":[\"identity-sources\"],\"type\":[\"info\"]},\"system.identity_sources.bulk_group_membership_delete\":{\"category\":[\"iam\"],\"tags\":[\"identity-sources\"],\"type\":[\"group\"]},\"system.identity_sources.bulk_group_membership_upsert\":{\"category\":[\"iam\"],\"tags\":[\"identity-sources\"],\"type\":[\"group\"]},\"system.identity_sources.bulk_group_upsert\":{\"category\":[\"configuration\"],\"tags\":[\"identity-sources\"],\"type\":[\"info\"]},\"system.identity_sources.bulk_upsert\":{\"category\":[\"configuration\"],\"tags\":[\"identity-sources\"],\"type\":[\"info\"]},\"system.idp.key.create\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"creation\"]},\"system.idp.key.delete\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"deletion\"]},\"system.idp.key.update\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"change\"]},\"system.idp.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"info\"]},\"system.idp.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"creation\"]},\"system.idp.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"info\"]},\"system.idp.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"deletion\"]},\"system.idp.lifecycle.read_client_secret\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"idp\"],\"type\":[\"info\"]},\"system.idp.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"event-hook-eligible\",\"idp\"],\"type\":[\"change\"]},\"system.import.clear.unconfirmed.users.summary\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"system.import.complete\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.complete_batch\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.custom_object.complete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.custom_object.create\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"creation\"]},\"system.import.custom_object.delete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"deletion\"]},\"system.import.custom_object.update\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"change\"]},\"system.import.download.complete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.download.start\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.entitlement\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.entitlement.mismatch\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.group.complete\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"group\"]},\"system.import.group.create\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"import\",\"system\"],\"type\":[\"creation\",\"group\"]},\"system.import.group.delete\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"import\",\"system\"],\"type\":[\"deletion\",\"group\"]},\"system.import.group.start\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"group\"]},\"system.import.group.update\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"change\",\"group\"]},\"system.import.group_membership.complete\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"group\"]},\"system.import.implicit_deletion.complete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.implicit_deletion.start\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.import_profile\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.import_provisioning_info\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.membership_processing.complete\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.membership_processing.start\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.object_creation.complete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.object_creation.start\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.roadblock\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.roadblock.reschedule_and_resume\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.roadblock.resume\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.roadblock.updated\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"change\"]},\"system.import.schedule\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"system.import.session.cancelled\":{\"category\":[\"session\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.session.created\":{\"category\":[\"session\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.session.expired\":{\"category\":[\"session\"],\"tags\":[\"import\",\"system\"],\"type\":[\"end\"]},\"system.import.session.triggered\":{\"category\":[\"session\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.start\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.user.complete\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.create\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"creation\",\"user\"]},\"system.import.user.delete\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"deletion\",\"user\"]},\"system.import.user.match\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.start\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.suspend\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.unsuspend\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.unsuspend_after_confirm\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"user\"]},\"system.import.user.update\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"change\",\"user\"]},\"system.import.user.update_user_lifecycle_from_master\":{\"category\":[\"iam\"],\"tags\":[\"import\",\"system\"],\"type\":[\"change\",\"user\"]},\"system.import.user_csv.complete\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"csv-upload\",\"user-import\"],\"type\":[\"info\"]},\"system.import.user_csv.start\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"csv-upload\",\"user-import\"],\"type\":[\"info\"]},\"system.import.user_match.confirm\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"system.import.user_match.unignore\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"info\"]},\"system.import.user_match.update\":{\"category\":[\"configuration\"],\"tags\":[\"app\"],\"type\":[\"change\"]},\"system.import.user_matching.complete\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.import.user_matching.start\":{\"category\":[\"configuration\"],\"tags\":[\"import\",\"system\"],\"type\":[\"info\"]},\"system.iwa.create\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"creation\"]},\"system.iwa.go_offline\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa.go_online\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa.promote_primary\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa.remove\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"deletion\"]},\"system.iwa.update\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"change\"]},\"system.iwa.use_default\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa_agentless.auth\":{\"category\":[\"authentication\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa_agentless.auth_after_redirect\":{\"category\":[\"authentication\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa_agentless.redirect\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\"]},\"system.iwa_agentless.update\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"change\"]},\"system.iwa_agentless.user.not_found\":{\"category\":[\"iam\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"info\",\"user\"]},\"system.iwa_agentless_kerberos.update\":{\"category\":[\"configuration\"],\"tags\":[\"iwa\",\"system\"],\"type\":[\"change\"]},\"system.ldapi.admin_limit_exceeded\":{\"category\":[\"configuration\"],\"tags\":[\"ldapi\"],\"type\":[\"info\"]},\"system.ldapi.bind\":{\"category\":[\"authentication\"],\"tags\":[\"ldapi\"],\"type\":[\"info\"]},\"system.ldapi.search\":{\"category\":[\"configuration\"],\"tags\":[\"ldapi\"],\"type\":[\"info\"]},\"system.ldapi.unbind\":{\"category\":[\"authentication\"],\"tags\":[\"ldapi\"],\"type\":[\"info\"]},\"system.log_stream.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"log-stream\"],\"type\":[\"info\"]},\"system.log_stream.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"log-stream\"],\"type\":[\"creation\",\"info\"]},\"system.log_stream.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"log-stream\"],\"type\":[\"info\"]},\"system.log_stream.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"log-stream\"],\"type\":[\"deletion\",\"info\"]},\"system.log_stream.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"log-stream\"],\"type\":[\"change\",\"info\"]},\"system.mfa.factor.activate\":{\"category\":[\"authentication\"],\"tags\":[\"admin\",\"mfa\"],\"type\":[\"start\"]},\"system.mfa.factor.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"admin\",\"mfa\"],\"type\":[\"end\"]},\"system.oauth2.token.request_outside_allowed_range\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"oauth2\"],\"type\":[\"info\"]},\"system.operation.concurrency_limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.operation.rate_limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.operation.rate_limit.warning\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.org.captcha.activate\":{\"category\":[\"configuration\"],\"tags\":[\"captcha\",\"system\"],\"type\":[\"info\"]},\"system.org.captcha.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"captcha\",\"system\"],\"type\":[\"info\"]},\"system.org.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"creation\"]},\"system.org.rate_limit.burst\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.org.rate_limit.expiration.warning\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"info\"]},\"system.org.rate_limit.violation\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"system\"],\"type\":[\"info\"]},\"system.org.rate_limit.warning\":{\"category\":[\"configuration\"],\"tags\":[\"event-hook-eligible\",\"system\"],\"type\":[\"info\"]},\"system.org.task.remove\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"deletion\"]},\"system.push.send_factor_verify_push\":{\"category\":[\"authentication\"],\"tags\":[\"push\"],\"type\":[\"info\"]},\"system.rate_limit.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"system\"],\"type\":[\"change\"]},\"system.self_service.configuration.update\":{\"category\":[\"configuration\"],\"tags\":[\"changeDetails\",\"self-service\"],\"type\":[\"change\"]},\"system.sms.receive_status\":{\"category\":[\"configuration\"],\"tags\":[\"sms\"],\"type\":[\"info\"]},\"system.sms.send_account_unlock_message\":{\"category\":[\"configuration\"],\"tags\":[\"sms\",\"system\"],\"type\":[\"info\"]},\"system.sms.send_factor_verify_message\":{\"category\":[\"authentication\"],\"tags\":[\"sms\",\"system\"],\"type\":[\"info\"]},\"system.sms.send_okta_push_verify_message\":{\"category\":[\"configuration\"],\"tags\":[\"sms\",\"system\"],\"type\":[\"info\"]},\"system.sms.send_password_reset_message\":{\"category\":[\"authentication\"],\"tags\":[\"sms\",\"system\"],\"type\":[\"info\"]},\"system.sms.send_phone_verification_message\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"sms\",\"system\"],\"type\":[\"info\"]},\"system.theme.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\"],\"type\":[\"change\"]},\"system.voice.receive_status\":{\"category\":[\"configuration\"],\"tags\":[\"voice\"],\"type\":[\"info\"]},\"system.voice.send_account_unlock_call\":{\"category\":[\"configuration\"],\"tags\":[\"voice\"],\"type\":[\"info\"]},\"system.voice.send_call\":{\"category\":[\"configuration\"],\"tags\":[\"voice\"],\"type\":[\"info\"]},\"system.voice.send_mfa_challenge_call\":{\"category\":[\"authentication\"],\"tags\":[\"voice\"],\"type\":[\"info\"]},\"system.voice.send_password_reset_call\":{\"category\":[\"authentication\"],\"tags\":[\"voice\"],\"type\":[\"info\"]},\"system.voice.send_phone_verification_call\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"voice\"],\"type\":[\"info\"]},\"system.well_known_uri.update\":{\"category\":[\"configuration\"],\"tags\":[\"admin\",\"changeDetails\"],\"type\":[\"change\"]},\"task.lifecycle.activate\":{\"category\":[\"configuration\"],\"tags\":[\"task\"],\"type\":[\"info\"]},\"task.lifecycle.create\":{\"category\":[\"configuration\"],\"tags\":[\"task\"],\"type\":[\"creation\"]},\"task.lifecycle.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"task\"],\"type\":[\"info\"]},\"task.lifecycle.delete\":{\"category\":[\"configuration\"],\"tags\":[\"task\"],\"type\":[\"deletion\"]},\"task.lifecycle.update\":{\"category\":[\"configuration\"],\"tags\":[\"task\"],\"type\":[\"change\"]},\"user.account.expire_password\":{\"category\":[\"authentication\"],\"tags\":[\"account\",\"user\"],\"type\":[\"end\",\"info\"]},\"user.account.lock\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.account.lock.limit\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\"],\"type\":[\"user\"]},\"user.account.preference_update\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\"],\"type\":[\"user\"]},\"user.account.privilege.grant\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"info\",\"user\"]},\"user.account.privilege.revoke\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"deletion\",\"user\"]},\"user.account.report_suspicious_activity_by_enduser\":{\"category\":[\"iam\"],\"tags\":[\"event-based-trigger-eligible\",\"event-hook-eligible\",\"user\"],\"type\":[\"info\",\"user\"]},\"user.account.reset_password\":{\"category\":[\"authentication\"],\"tags\":[\"account\",\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.account.unlock\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.account.unlock_by_admin\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.account.unlock_failure\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\"],\"type\":[\"user\"]},\"user.account.unlock_token\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\"],\"type\":[\"user\"]},\"user.account.update_password\":{\"category\":[\"authentication\"],\"tags\":[\"account\",\"end-user-visible\",\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.account.update_primary_email\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"end-user-visible\",\"user\",\"user-config\"],\"type\":[\"change\",\"user\"]},\"user.account.update_profile\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"event-hook-eligible\",\"user\",\"user-config\"],\"type\":[\"change\",\"user\"]},\"user.account.update_secondary_email\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"end-user-visible\",\"user\",\"user-config\"],\"type\":[\"change\",\"user\"]},\"user.account.update_user_type\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\",\"user-config\"],\"type\":[\"change\",\"user\"]},\"user.account.use_token\":{\"category\":[\"iam\"],\"tags\":[\"account\",\"user\"],\"type\":[\"user\"]},\"user.authentication.auth\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_unconfigured_identifier\":{\"category\":[\"authentication\"],\"tags\":[\"directory\",\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_AD_agent\":{\"category\":[\"authentication\"],\"tags\":[\"directory\",\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_IDP\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_LDAP_agent\":{\"category\":[\"authentication\"],\"tags\":[\"directory\",\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_inbound_SAML\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_inbound_delauth\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_iwa\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_mfa\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"mfa\"],\"type\":[\"info\"]},\"user.authentication.auth_via_radius\":{\"category\":[\"authentication\"],\"tags\":[\"app\",\"radius\"],\"type\":[\"info\"]},\"user.authentication.auth_via_richclient\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.auth_via_social\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.authentication.authenticate\":{\"category\":[\"authentication\"],\"tags\":[\"device-trust-authentication\",\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.authentication.dsso_via_non_priority_source\":{\"category\":[\"authentication\"],\"tags\":[\"directory\",\"user\"],\"type\":[\"info\"]},\"user.authentication.slo\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.authentication.sso\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.authentication.universal_logout\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"end\"]},\"user.authentication.universal_logout.scheduled\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"end\"]},\"user.authentication.verify\":{\"category\":[\"authentication\"],\"tags\":[\"end-user-visible\",\"user\"],\"type\":[\"info\"]},\"user.behavior.profile.reset\":{\"category\":[\"iam\"],\"tags\":[\"behavior-profile\",\"event-hook-eligible\"],\"type\":[\"admin\",\"change\",\"user\"]},\"user.credential.enroll\":{\"category\":[\"authentication\"],\"tags\":[\"device-trust-cert-distribution-and-binding\",\"event-hook-eligible\",\"user\"],\"type\":[\"info\"]},\"user.identity_snapshot.attestation.create\":{\"category\":[\"iam\"],\"tags\":[\"attestation\",\"user\"],\"type\":[\"admin\",\"creation\",\"user\"]},\"user.identity_verification\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"policy\",\"session\",\"user\"],\"type\":[\"info\"]},\"user.identity_verification.start\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"policy\",\"session\",\"user\"],\"type\":[\"start\"]},\"user.import.password\":{\"category\":[\"authentication\"],\"tags\":[\"credential\",\"event-hook-eligible\",\"import\",\"user\"],\"type\":[\"info\"]},\"user.lifecycle.activate\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.lifecycle.create\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"creation\",\"user\"]},\"user.lifecycle.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.lifecycle.delete.completed\":{\"category\":[\"iam\"],\"tags\":[\"user\"],\"type\":[\"deletion\",\"user\"]},\"user.lifecycle.delete.initiated\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"deletion\",\"user\"]},\"user.lifecycle.jit.error.read_only\":{\"category\":[\"iam\"],\"tags\":[\"user\"],\"type\":[\"user\"]},\"user.lifecycle.password_mass_expiry\":{\"category\":[\"authentication\"],\"tags\":[\"user\"],\"type\":[\"info\"]},\"user.lifecycle.reactivate\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.lifecycle.suspend\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.lifecycle.unsuspend\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"user\"],\"type\":[\"user\"]},\"user.mfa.attempt_bypass\":{\"category\":[\"authentication\"],\"tags\":[\"mfa\"],\"type\":[\"info\"]},\"user.mfa.factor.activate\":{\"category\":[\"authentication\"],\"tags\":[\"end-user-visible\",\"event-hook-eligible\",\"mfa\"],\"type\":[\"info\",\"start\"]},\"user.mfa.factor.deactivate\":{\"category\":[\"authentication\"],\"tags\":[\"end-user-visible\",\"event-hook-eligible\",\"mfa\"],\"type\":[\"end\",\"info\"]},\"user.mfa.factor.reset_all\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"mfa\"],\"type\":[\"info\"]},\"user.mfa.factor.suspend\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"mfa\",\"oie-only\"],\"type\":[\"info\"]},\"user.mfa.factor.unsuspend\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"mfa\",\"oie-only\"],\"type\":[\"info\"]},\"user.mfa.factor.update\":{\"category\":[\"authentication\"],\"tags\":[\"event-hook-eligible\",\"mfa\"],\"type\":[\"info\"]},\"user.mfa.okta_verify\":{\"category\":[\"authentication\"],\"tags\":[\"mfa\"],\"type\":[\"info\"]},\"user.mfa.okta_verify.deny_push\":{\"category\":[\"authentication\"],\"tags\":[\"mfa\"],\"type\":[\"info\"]},\"user.mfa.okta_verify.deny_push_upgrade_needed\":{\"category\":[\"authentication\"],\"tags\":[\"mfa\"],\"type\":[\"info\"]},\"user.risk.change\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"risk\",\"security\"],\"type\":[\"change\",\"user\"]},\"user.risk.detect\":{\"category\":[\"iam\"],\"tags\":[\"event-hook-eligible\",\"risk\",\"security\"],\"type\":[\"user\"]},\"user.session.access_admin_app\":{\"category\":[\"session\"],\"tags\":[\"admin\",\"app\",\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.clear\":{\"category\":[\"session\"],\"tags\":[\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.context.change\":{\"category\":[\"session\"],\"tags\":[\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.end\":{\"category\":[\"session\"],\"tags\":[\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"end\"]},\"user.session.expire\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"end\"]},\"user.session.impersonation.end\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"end\"]},\"user.session.impersonation.extend\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.impersonation.grant\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.impersonation.initiate\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"start\"]},\"user.session.impersonation.revoke\":{\"category\":[\"session\"],\"tags\":[\"session\",\"user\"],\"type\":[\"info\"]},\"user.session.start\":{\"category\":[\"session\"],\"tags\":[\"end-user-visible\",\"event-hook-eligible\",\"session\",\"user\"],\"type\":[\"start\"]},\"workflows.user.connection.create\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"info\",\"user\"]},\"workflows.user.connection.delete\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"info\",\"user\"]},\"workflows.user.connection.reauthorize\":{\"category\":[\"authentication\"],\"tags\":[\"workflows\"],\"type\":[\"info\"]},\"workflows.user.connection.revoke\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"info\",\"user\"]},\"workflows.user.delegatedflow.run\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.execution_log_stream_connection.activate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.execution_log_stream_connection.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.execution_log_stream_connection.update\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"change\",\"user\"]},\"workflows.user.flow.activate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.create\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"user\"]},\"workflows.user.flow.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.delete\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"user\"]},\"workflows.user.flow.execution.cancel\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"user\"]},\"workflows.user.flow.execution_history.activate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.execution_history.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.execution_history.delete\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"user\"]},\"workflows.user.flow.execution_log_stream.activate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.execution_log_stream.deactivate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.export\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.import\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.flow.move\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.flow.save\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.folder.create\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"info\",\"user\"]},\"workflows.user.folder.delete\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"info\",\"user\"]},\"workflows.user.folder.duplicate\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"user\"]},\"workflows.user.folder.export\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.folder.import\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.folder.move\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.folder.rename\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.role.group.add\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"group\",\"info\",\"user\"]},\"workflows.user.role.group.remove\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"group\",\"info\",\"user\"]},\"workflows.user.role.user.add\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"info\",\"user\"]},\"workflows.user.role.user.remove\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"info\",\"user\"]},\"workflows.user.table.create\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"creation\",\"info\",\"user\"]},\"workflows.user.table.delete\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"deletion\",\"info\",\"user\"]},\"workflows.user.table.export\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.table.import\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.table.move\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.table.schema.export\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.table.schema.import\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"workflows.user.table.update\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"change\",\"info\",\"user\"]},\"workflows.user.table.view\":{\"category\":[\"iam\"],\"tags\":[\"workflows\"],\"type\":[\"info\",\"user\"]},\"zone.activate\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"info\"]},\"zone.create\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"creation\"]},\"zone.deactivate\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"info\"]},\"zone.delete\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"deletion\"]},\"zone.make_blacklist\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"info\"]},\"zone.remove_blacklist\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"deletion\"]},\"zone.update\":{\"category\":[\"configuration\"],\"tags\":[\"network-zone\"],\"type\":[\"change\"]}}"
                    ),
                )?;
            }
            // End nested pipeline: "ecs_category_type"

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.uuid") {
                    event.rename("json.uuid", "okta.uuid")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.actor.alternateId") {
                    event.rename("json.actor.alternateId", "okta.actor.alternate_id")?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("okta.actor.alternate_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event
                    .get("user.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            if let Some(v) = event
                .get("user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("source.user.name").cloned() {
                    event.set("source.user.email", v)?;
                }
            }

            if let Some(v) = event
                .get("user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.user.name", v)?;
            }

            let _cond = {
                event.has_value("client.user.name")
                    && event
                        .get_str("client.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("client.user.name").cloned() {
                    event.set("client.user.email", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.actor.displayName") {
                    event.rename("json.actor.displayName", "okta.actor.display_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.actor.id") {
                    event.rename("json.actor.id", "okta.actor.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.actor.type") {
                    event.rename("json.actor.type", "okta.actor.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.device") {
                    event.rename("json.client.device", "okta.client.device")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.device") {
                    event.rename("json.device", "okta.device")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("okta.device.device_integrator") };
            if _cond {
                parse_json_field(
                    event,
                    "okta.device.device_integrator",
                    "okta.device.device_integrator",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.geographicalContext.geolocation") {
                    event.rename(
                        "json.client.geographicalContext.geolocation",
                        "client.geo.location",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.geographicalContext.city") {
                    event.rename(
                        "json.client.geographicalContext.city",
                        "client.geo.city_name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.geographicalContext.state") {
                    event.rename(
                        "json.client.geographicalContext.state",
                        "client.geo.region_name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.geographicalContext.country") {
                    event.rename(
                        "json.client.geographicalContext.country",
                        "client.geo.country_name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.id") {
                    event.rename("json.client.id", "okta.client.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.ipAddress") {
                    if let Some(val) = event.get("json.client.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.client.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set("okta.client.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.userAgent.browser") {
                    event.rename(
                        "json.client.userAgent.browser",
                        "okta.client.user_agent.browser",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.userAgent.os") {
                    event.rename("json.client.userAgent.os", "okta.client.user_agent.os")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.userAgent.rawUserAgent") {
                    event.rename(
                        "json.client.userAgent.rawUserAgent",
                        "okta.client.user_agent.raw_user_agent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.client.zone") {
                    event.rename("json.client.zone", "okta.client.zone")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.outcome.reason") {
                    event.rename("json.outcome.reason", "okta.outcome.reason")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.outcome.result") {
                    event.rename("json.outcome.result", "okta.outcome.result")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.target") {
                    event.rename("json.target", "okta.target")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.transaction.id") {
                    event.rename("json.transaction.id", "okta.transaction.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.transaction.type") {
                    event.rename("json.transaction.type", "okta.transaction.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.transaction.detail.requestApiTokenId") {
                    event.rename(
                        "json.transaction.detail.requestApiTokenId",
                        "okta.transaction.detail.request_api_token_id",
                    )?;
                }
                Ok(())
            })();

            if event.has_value("json.transaction.detail.rootApiTokenId") {
                event.rename(
                    "json.transaction.detail.rootApiTokenId",
                    "okta.transaction.detail.root_api_token_id",
                )?;
            }

            let _cond = { event.get_bool("_conf.remove_flattened_debug") != Some(true) };
            if _cond {
                // Begin nested pipeline: "use_flattened_debug"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.debugContext.debugData").cloned() {
                        event.set("okta.debug_context.debug_data.flattened", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData",
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.flattened.behaviors") {
                        if let Some(input) =
                            event.get_string("okta.debug_context.debug_data.flattened.behaviors")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("{") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("}") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "okta.debug_context.debug_data.flattened.behaviors",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("}") else {
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
                    }
                    Ok(())
                })();
                let _cond =
                    { event.has_value("okta.debug_context.debug_data.flattened.behaviors") };
                if _cond {
                    if let Some(kv_str) =
                        event.get_string("okta.debug_context.debug_data.flattened.behaviors")
                    {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "okta.debug_context.debug_data.flattened.behaviors"
                                        .into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("_behaviors_object.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("_behaviors_object") };
                if _cond {
                    if event
                        .remove("okta.debug_context.debug_data.flattened.behaviors")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.flattened.behaviors".into(),
                        });
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_behaviors_object") {
                        event.rename(
                            "_behaviors_object",
                            "okta.debug_context.debug_data.flattened.behaviors",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data.flattened.risk") };
                if _cond {
                    if let Some(v) = event
                        .get("okta.debug_context.debug_data.flattened.risk")
                        .cloned()
                    {
                        event.set("okta.debug_context.debug_data.flattened.risk_object", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.flattened.risk") {
                        if let Some(input) =
                            event.get_string("okta.debug_context.debug_data.flattened.risk")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("{") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("}") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "okta.debug_context.debug_data.flattened.risk",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("}") else {
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
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data.flattened.risk") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) =
                            event.get_string("okta.debug_context.debug_data.flattened.risk")
                        {
                            for pair in kv_str.split(", ") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "okta.debug_context.debug_data.flattened.risk".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        kv_put(event, &format!("_risk_object.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set("_ingest.on_failure_processor_tag", "kv_risk")?;
                        if event.remove("_risk_object").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "_risk_object".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("_risk_object") };
                if _cond {
                    if event
                        .remove("okta.debug_context.debug_data.flattened.risk_object")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.flattened.risk_object".into(),
                        });
                    }
                }
                let _cond = {
                    event.has_value("okta.debug_context.debug_data.flattened.risk_object")
                        && event.has_value("okta.debug_context.debug_data.flattened.risk")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("okta.debug_context.debug_data.flattened.risk")
                        {
                            // Grok pattern: level=%{NOTSPACE:_risk_object.level}
                            if !cached_grok!("level=%{NOTSPACE:_risk_object.level}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("okta.debug_context.debug_data.flattened.risk_object")
                        && event.has_value("okta.debug_context.debug_data.flattened.risk")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("okta.debug_context.debug_data.flattened.risk")
                        {
                            // Grok pattern: reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)
                            // Grok pattern: reasons=%{DATA:_risk_object.reasons}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)"
                                    ),
                                    cached_grok!("reasons=%{DATA:_risk_object.reasons}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("_risk_object") };
                if _cond {
                    if event
                        .remove("okta.debug_context.debug_data.flattened.risk")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.flattened.risk".into(),
                        });
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_risk_object") {
                        event.rename(
                            "_risk_object",
                            "okta.debug_context.debug_data.flattened.risk",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.deviceFingerprint") {
                        event.rename(
                            "json.debugContext.debugData.deviceFingerprint",
                            "okta.debug_context.debug_data.device_fingerprint",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.requestId") {
                        event.rename(
                            "json.debugContext.debugData.requestId",
                            "okta.debug_context.debug_data.request_id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.requestUri") {
                        event.rename(
                            "json.debugContext.debugData.requestUri",
                            "okta.debug_context.debug_data.request_uri",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.threatSuspected") {
                        event.rename(
                            "json.debugContext.debugData.threatSuspected",
                            "okta.debug_context.debug_data.threat_suspected",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.url") {
                        event.rename(
                            "json.debugContext.debugData.url",
                            "okta.debug_context.debug_data.url",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.dtHash") {
                        event.rename(
                            "json.debugContext.debugData.dtHash",
                            "okta.debug_context.debug_data.dt_hash",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.clientSecret") {
                        event.rename(
                            "json.debugContext.debugData.clientSecret",
                            "okta.debug_context.debug_data.client_secret",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.requestedScopes") {
                        event.rename(
                            "json.debugContext.debugData.requestedScopes",
                            "okta.debug_context.debug_data.requested_scopes",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.grantedScopes") {
                        event.rename(
                            "json.debugContext.debugData.grantedScopes",
                            "okta.debug_context.debug_data.granted_scopes",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.grantType") {
                        event.rename(
                            "json.debugContext.debugData.grantType",
                            "okta.debug_context.debug_data.grant_type",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level",
                    ) && event.get_str(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level",
                    ) != Some("")
                };
                if _cond {
                    event.set("okta.debug_context.debug_data.risk_level", json!(event.get("okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = {
                    event.has_value(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                    ) && event.get_str(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                    ) != Some("")
                };
                if _cond {
                    if let Some(s) = event.get_string(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                    ) {
                        let mut parts: Vec<Value> = cached_regex!(",\\s*")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "okta.debug_context.debug_data.risk_reasons",
                            Value::Array(parts),
                        )?;
                    }
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.risk_level")
                        && event.has_value("okta.debug_context.debug_data.flattened.risk.level")
                        && event.get_str("okta.debug_context.debug_data.flattened.risk.level")
                            != Some("")
                };
                if _cond {
                    event.set(
                        "okta.debug_context.debug_data.risk_level",
                        json!(
                            event
                                .get("okta.debug_context.debug_data.flattened.risk.level")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.factor")
                        && event.has_value("okta.debug_context.debug_data.flattened.factor")
                        && event.get_str("okta.debug_context.debug_data.flattened.factor")
                            != Some("")
                };
                if _cond {
                    event.set(
                        "okta.debug_context.debug_data.factor",
                        json!(
                            event
                                .get("okta.debug_context.debug_data.flattened.factor")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.risk_reasons")
                        && event.has_value("okta.debug_context.debug_data.flattened.risk.reasons")
                        && event.get_str("okta.debug_context.debug_data.flattened.risk.reasons")
                            != Some("")
                };
                if _cond {
                    if let Some(s) =
                        event.get_string("okta.debug_context.debug_data.flattened.risk.reasons")
                    {
                        let mut parts: Vec<Value> = cached_regex!(",\\s*")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "okta.debug_context.debug_data.risk_reasons",
                            Value::Array(parts),
                        )?;
                    }
                }
                let _cond = { event.has_value("okta.debug_context.debug_data.flattened.tunnels") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "okta.debug_context.debug_data.flattened.tunnels",
                            "okta.debug_context.debug_data.flattened.tunnels",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_okta_debug_context_debug_data_flattened_tunnels",
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
                // Painless script
                // Source: def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n"#
                    ),
                )?;
                // End nested pipeline: "use_flattened_debug"
            }

            let _cond = { event.get_bool("_conf.remove_flattened_debug") == Some(true) };
            if _cond {
                // Begin nested pipeline: "no_use_flattened_debug"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "json.debugContext.debugData",
                        "okta.debug_context.debug_data",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "okta.debug_context.debug_data.logOnlySecurityData",
                        "okta.debug_context.debug_data.logOnlySecurityData",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.behaviors") {
                        if let Some(input) =
                            event.get_string("okta.debug_context.debug_data.behaviors")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("{") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("}") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "okta.debug_context.debug_data.behaviors",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("}") else {
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
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data.behaviors") };
                if _cond {
                    if let Some(kv_str) =
                        event.get_string("okta.debug_context.debug_data.behaviors")
                    {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "okta.debug_context.debug_data.behaviors".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("_behaviors_object.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("_behaviors_object") };
                if _cond {
                    if event
                        .remove("okta.debug_context.debug_data.behaviors")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.behaviors".into(),
                        });
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_behaviors_object") {
                        event.rename(
                            "_behaviors_object",
                            "okta.debug_context.debug_data.behaviors",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data.risk") };
                if _cond {
                    if let Some(v) = event.get("okta.debug_context.debug_data.risk").cloned() {
                        event.set("okta.debug_context.debug_data.risk_object", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.risk") {
                        if let Some(input) = event.get_string("okta.debug_context.debug_data.risk")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("{") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("}") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "okta.debug_context.debug_data.risk",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("}") else {
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
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data.risk") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("okta.debug_context.debug_data.risk")
                        {
                            for pair in kv_str.split(", ") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "okta.debug_context.debug_data.risk".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        kv_put(event, &format!("_risk_object.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set("_ingest.on_failure_processor_tag", "kv_risk")?;
                        if event.remove("_risk_object").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "_risk_object".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("_risk_object") };
                if _cond {
                    if event
                        .remove("okta.debug_context.debug_data.risk_object")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.risk_object".into(),
                        });
                    }
                }
                let _cond = {
                    event.has_value("okta.debug_context.debug_data.risk_object")
                        && event.has_value("okta.debug_context.debug_data.risk")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("okta.debug_context.debug_data.risk")
                        {
                            // Grok pattern: level=%{NOTSPACE:_risk_object.level}
                            if !cached_grok!("level=%{NOTSPACE:_risk_object.level}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("okta.debug_context.debug_data.risk_object")
                        && event.has_value("okta.debug_context.debug_data.risk")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("okta.debug_context.debug_data.risk")
                        {
                            // Grok pattern: reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)
                            // Grok pattern: reasons=%{DATA:_risk_object.reasons}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)"
                                    ),
                                    cached_grok!("reasons=%{DATA:_risk_object.reasons}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("_risk_object") };
                if _cond {
                    if event.remove("okta.debug_context.debug_data.risk").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.debug_context.debug_data.risk".into(),
                        });
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_risk_object") {
                        event.rename("_risk_object", "okta.debug_context.debug_data.risk")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                        && event
                            .get_str("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                            != Some("")
                };
                if _cond {
                    event.set(
                        "okta.debug_context.debug_data.risk_level",
                        json!(
                            event
                                .get("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .has_value("okta.debug_context.debug_data.logOnlySecurityData.risk.reasons")
                        && event.get_str(
                            "okta.debug_context.debug_data.logOnlySecurityData.risk.reasons",
                        ) != Some("")
                };
                if _cond {
                    if let Some(s) = event.get_string(
                        "okta.debug_context.debug_data.logOnlySecurityData.risk.reasons",
                    ) {
                        let mut parts: Vec<Value> = cached_regex!(",\\s*")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "okta.debug_context.debug_data.risk_reasons",
                            Value::Array(parts),
                        )?;
                    }
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.risk_level")
                        && event.has_value("okta.debug_context.debug_data.risk.level")
                        && event.get_str("okta.debug_context.debug_data.risk.level") != Some("")
                };
                if _cond {
                    event.set(
                        "okta.debug_context.debug_data.risk_level",
                        json!(
                            event
                                .get("okta.debug_context.debug_data.risk.level")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.factor")
                        && event.has_value("okta.debug_context.debug_data.factor")
                        && event.get_str("okta.debug_context.debug_data.factor") != Some("")
                };
                if _cond {
                    event.set(
                        "okta.debug_context.debug_data.factor",
                        json!(
                            event
                                .get("okta.debug_context.debug_data.factor")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("okta.debug_context.debug_data.risk_reasons")
                        && event.has_value("okta.debug_context.debug_data.risk.reasons")
                        && event.get_str("okta.debug_context.debug_data.risk.reasons") != Some("")
                };
                if _cond {
                    if let Some(s) = event.get_string("okta.debug_context.debug_data.risk.reasons")
                    {
                        let mut parts: Vec<Value> = cached_regex!(",\\s*")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "okta.debug_context.debug_data.risk_reasons",
                            Value::Array(parts),
                        )?;
                    }
                }
                let _cond = { event.has_value("okta.debug_context.debug_data.tunnels") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "okta.debug_context.debug_data.tunnels",
                            "okta.debug_context.debug_data.tunnels",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_okta_debug_context_debug_data_tunnels",
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
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event
                                .remove("okta.debug_context.debug_data.tunnels")
                                .is_none()
                            {
                                return Err(TransformError::FieldNotFound {
                                    path: "okta.debug_context.debug_data.tunnels".into(),
                                });
                            }
                            Ok(())
                        })();
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // Painless script
                // Source: def src = ctx.okta?.debug_context?.debug_data?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def src = ctx.okta?.debug_context?.debug_data?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n"#
                    ),
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.deviceFingerprint") {
                        event.rename(
                            "okta.debug_context.debug_data.deviceFingerprint",
                            "okta.debug_context.debug_data.device_fingerprint",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.dtHash") {
                        event.rename(
                            "okta.debug_context.debug_data.dtHash",
                            "okta.debug_context.debug_data.dt_hash",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.requestId") {
                        event.rename(
                            "okta.debug_context.debug_data.requestId",
                            "okta.debug_context.debug_data.request_id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.requestUri") {
                        event.rename(
                            "okta.debug_context.debug_data.requestUri",
                            "okta.debug_context.debug_data.request_uri",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("okta.debug_context.debug_data.threatSuspected") {
                        event.rename(
                            "okta.debug_context.debug_data.threatSuspected",
                            "okta.debug_context.debug_data.threat_suspected",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.clientSecret") {
                        event.rename(
                            "json.debugContext.debugData.clientSecret",
                            "okta.debug_context.debug_data.client_secret",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.requestedScopes") {
                        event.rename(
                            "json.debugContext.debugData.requestedScopes",
                            "okta.debug_context.debug_data.requested_scopes",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.grantedScopes") {
                        event.rename(
                            "json.debugContext.debugData.grantedScopes",
                            "okta.debug_context.debug_data.granted_scopes",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.debugContext.debugData.grantType") {
                        event.rename(
                            "json.debugContext.debugData.grantType",
                            "okta.debug_context.debug_data.grant_type",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("okta.debug_context.debug_data") };
                if _cond {
                    // Painless script
                    // Source: String underscore(String s) {\n  return /[ -]/.matcher(s).replaceAll('_');\n}\ndef renameKeys(Map src) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      dst[underscore(key)] = renameKeys(value);\n    } else if (value instanceof List) {\n      for (int i = 0; i < value.length; i++) {\n        if (value[i] instanceof Map) {\n          value[i] = renameKeys(value[i]);\n        }\n      }\n      dst[underscore(key)] = value;\n    } else {\n      dst[underscore(key)] = value;\n    }\n  }\n  return dst;\n}\nctx.okta.debug_context.debug_data = renameKeys(ctx.okta.debug_context.debug_data)\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String underscore(String s) {\n  return /[ -]/.matcher(s).replaceAll('_');\n}\ndef renameKeys(Map src) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      dst[underscore(key)] = renameKeys(value);\n    } else if (value instanceof List) {\n      for (int i = 0; i < value.length; i++) {\n        if (value[i] instanceof Map) {\n          value[i] = renameKeys(value[i]);\n        }\n      }\n      dst[underscore(key)] = value;\n    } else {\n      dst[underscore(key)] = value;\n    }\n  }\n  return dst;\n}\nctx.okta.debug_context.debug_data = renameKeys(ctx.okta.debug_context.debug_data)\n"#
                        ),
                    )?;
                }
                // End nested pipeline: "no_use_flattened_debug"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.authenticationProvider") {
                    event.rename(
                        "json.authenticationContext.authenticationProvider",
                        "okta.authentication_context.authentication_provider",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.authenticationStep") {
                    event.rename(
                        "json.authenticationContext.authenticationStep",
                        "okta.authentication_context.authentication_step",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.credentialProvider") {
                    event.rename(
                        "json.authenticationContext.credentialProvider",
                        "okta.authentication_context.credential_provider",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.credentialType") {
                    event.rename(
                        "json.authenticationContext.credentialType",
                        "okta.authentication_context.credential_type",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.externalSessionId") {
                    event.rename(
                        "json.authenticationContext.externalSessionId",
                        "okta.authentication_context.external_session_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.interface") {
                    event.rename(
                        "json.authenticationContext.interface",
                        "okta.authentication_context.authentication_provider",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.authenticationContext.issuer") {
                    event.rename(
                        "json.authenticationContext.issuer",
                        "okta.authentication_context.issuer",
                    )?;
                }
                Ok(())
            })();

            if event.has_value("json.authenticationContext.rootSessionId") {
                event.rename(
                    "json.authenticationContext.rootSessionId",
                    "okta.authentication_context.root_session_id",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.securityContext.asNumber") {
                    event.rename(
                        "json.securityContext.asNumber",
                        "okta.security_context.as.number",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.securityContext.asOrg") {
                    event.rename(
                        "json.securityContext.asOrg",
                        "okta.security_context.as.organization.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.securityContext.domain") {
                    event.rename(
                        "json.securityContext.domain",
                        "okta.security_context.domain",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.securityContext.isProxy") {
                    event.rename(
                        "json.securityContext.isProxy",
                        "okta.security_context.is_proxy",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.securityContext.isp") {
                    event.rename("json.securityContext.isp", "okta.security_context.isp")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.request.ipChain") {
                    event.rename("json.request.ipChain", "okta.request.ip_chain")?;
                }
                Ok(())
            })();

            if event.has_value("okta.request.ip_chain") {
                foreach_array(event, "okta.request.ip_chain", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_ingest._value.geographicalContext") {
                            event.rename(
                                "_ingest._value.geographicalContext",
                                "_ingest._value.geographical_context",
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            if event.has_value("okta.request.ip_chain") {
                foreach_array(event, "okta.request.ip_chain", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_ingest._value.geographical_context.postalCode") {
                            event.rename(
                                "_ingest._value.geographical_context.postalCode",
                                "_ingest._value.geographical_context.postal_code",
                            )?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.client.user_agent.raw_user_agent") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.client.user_agent.raw_user_agent".into(),
                            message,
                        }
                    })?;
                    event.set("user_agent.original", converted)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("okta.client.ip") };
            if _cond {
                if let Some(v) = event.get("okta.client.ip").cloned() {
                    event.set("client.ip", v)?;
                }
            }

            let _cond = { event.has_value("okta.client.ip") };
            if _cond {
                if let Some(v) = event.get("okta.client.ip").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.event_type") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.event_type".into(),
                            message,
                        }
                    })?;
                    event.set("event.action", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.security_context.as.organization.name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.security_context.as.organization.name".into(),
                            message,
                        }
                    })?;
                    event.set("client.as.organization.name", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.security_context.domain") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.security_context.domain".into(),
                            message,
                        }
                    })?;
                    event.set("client.domain", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.security_context.domain") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.security_context.domain".into(),
                            message,
                        }
                    })?;
                    event.set("source.domain", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("okta.uuid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "okta.uuid".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
                Ok(())
            })();

            if event.has_value("okta.outcome.result") {
                map_strings(
                    event,
                    "okta.outcome.result",
                    "okta.outcome.result_lower",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("okta.outcome.result_lower")
                    && (event.get_str("okta.outcome.result_lower") == Some("success")
                        || event.get_str("okta.outcome.result_lower") == Some("allow"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("okta.outcome.result_lower")
                    && (event.get_str("okta.outcome.result_lower") == Some("failure")
                        || event.get_str("okta.outcome.result_lower") == Some("deny"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            event.remove("okta.outcome.result_lower");

            // Painless script
            // Source: def arr = ctx.okta?.target;\nif (arr != null) {\n  for (def i = 0; i < arr.length; i++) {\n    arr[i][\"alternate_id\"] = arr[i][\"alternateId\"];\n    arr[i].remove(\"alternateId\");\n    arr[i][\"display_name\"] = arr[i][\"displayName\"];\n    arr[i].remove(\"displayName\");\n    def de = arr[i].get(\"detailEntry\");\n    if (de != null) {\n      de.entrySet().removeIf(entry -> \n        entry.getKey() != \"methodTypeUsed\" && \n        entry.getKey() != \"methodUsedVerifiedProperties\");\n      if (de.size() == 0) {\n        arr[i].remove(\"detailEntry\");\n      }\n    }\n\n    // Ensure that all entries in changeDetails.{from,to}.* are strings.\n    def cd = arr[i].get(\"changeDetails\");\n    if (cd != null) {\n      if (cd.from instanceof Map) {\n        for (def f: cd.from.entrySet()) {\n          def v = f.getValue();\n          if (v != null && !(v instanceof String)) {\n            cd.from[f.getKey()] = Json.dump(v);\n          }\n        }\n      }\n      if (cd.to instanceof Map) {\n        for (def t: cd.to.entrySet()) {\n          def v = t.getValue();\n          if (v != null && !(v instanceof String)) {\n            cd.to[t.getKey()] = Json.dump(v);\n          }\n        }\n      }\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"user\") {\n      ctx[\"okta_target_user\"] = arr[i];\n      break;\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"usergroup\") {\n      ctx[\"okta_target_group\"] = arr[i];\n      break;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def arr = ctx.okta?.target;\nif (arr != null) {\n  for (def i = 0; i < arr.length; i++) {\n    arr[i][\"alternate_id\"] = arr[i][\"alternateId\"];\n    arr[i].remove(\"alternateId\");\n    arr[i][\"display_name\"] = arr[i][\"displayName\"];\n    arr[i].remove(\"displayName\");\n    def de = arr[i].get(\"detailEntry\");\n    if (de != null) {\n      de.entrySet().removeIf(entry -> \n        entry.getKey() != \"methodTypeUsed\" && \n        entry.getKey() != \"methodUsedVerifiedProperties\");\n      if (de.size() == 0) {\n        arr[i].remove(\"detailEntry\");\n      }\n    }\n\n    // Ensure that all entries in changeDetails.{from,to}.* are strings.\n    def cd = arr[i].get(\"changeDetails\");\n    if (cd != null) {\n      if (cd.from instanceof Map) {\n        for (def f: cd.from.entrySet()) {\n          def v = f.getValue();\n          if (v != null && !(v instanceof String)) {\n            cd.from[f.getKey()] = Json.dump(v);\n          }\n        }\n      }\n      if (cd.to instanceof Map) {\n        for (def t: cd.to.entrySet()) {\n          def v = t.getValue();\n          if (v != null && !(v instanceof String)) {\n            cd.to[t.getKey()] = Json.dump(v);\n          }\n        }\n      }\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"user\") {\n      ctx[\"okta_target_user\"] = arr[i];\n      break;\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"usergroup\") {\n      ctx[\"okta_target_group\"] = arr[i];\n      break;\n    }\n  }\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("okta_target_user.display_name") };
            if _cond {
                if let Some(v) = event.get("okta_target_user.display_name").cloned() {
                    event.set("user.target.full_name", v)?;
                }
            }

            let _cond = { event.has_value("okta_target_user.id") };
            if _cond {
                if let Some(v) = event.get("okta_target_user.id").cloned() {
                    event.set("user.target.id", v)?;
                }
            }

            let _cond = { event.has_value("okta_target_user.login") };
            if _cond {
                if let Some(v) = event.get("okta_target_user.login").cloned() {
                    event.set("user.target.email", v)?;
                }
            }

            let _cond = { event.has_value("okta_target_group.display_name") };
            if _cond {
                if let Some(v) = event.get("okta_target_group.display_name").cloned() {
                    event.set("user.target.group.name", v)?;
                }
            }

            let _cond = { event.has_value("okta_target_group.id") };
            if _cond {
                if let Some(v) = event.get("okta_target_group.id").cloned() {
                    event.set("user.target.group.id", v)?;
                }
            }

            event.remove("okta_target_user");
            event.remove("okta_target_group");

            let _cond = { event.has_value("okta.actor.id") };
            if _cond {
                let v = json!(
                    event
                        .get("okta.actor.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("client.user.id", v)?;
                }
            }

            let _cond = { event.has_value("okta.actor.id") };
            if _cond {
                let v = json!(
                    event
                        .get("okta.actor.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.user.id", v)?;
                }
            }

            let _cond = { event.has_value("okta.actor.display_name") };
            if _cond {
                let v = json!(
                    event
                        .get("okta.actor.display_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("client.user.full_name", v)?;
                }
            }

            let _cond = { event.has_value("okta.actor.display_name") };
            if _cond {
                let v = json!(
                    event
                        .get("okta.actor.display_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.user.full_name", v)?;
                }
            }

            let _cond = { event.has_value("okta.actor.display_name") };
            if _cond {
                let v = json!(
                    event
                        .get("okta.actor.display_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
                }
            }

            let _cond = { event.has_value("okta.actor.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("okta.actor.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.full_name") };
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

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                Ok(())
            })();

            if event.has_value("okta_url") {
                uri_parts(event, "okta_url", "okta_url", false, false)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("okta_url.domain") {
                    event.rename("okta_url.domain", "host.name")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
                event.set("_ingest.on_failure_processor_tag", "rename_domain_2")?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    event.rename("okta_url.domain", "okta.okta_domain")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "rename")?;
                    event.set("_ingest.on_failure_processor_tag", "rename_domain_3")?;
                    event.rename("okta_url.domain", "okta_domain")?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.remove("okta_url");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
