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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = { event.get("event").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("event") {
                    event.rename("event", "json.event")?;
                }
            }

            let _cond = { event.has_value("json.event") && event.has_value("data") };
            if _cond {
                // Painless script
                // Source: if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }"#
                    ),
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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.get_str("json.type") == Some("audit_log") };
            if _cond {
                // Begin nested pipeline: "s3"
                let _cond = { event.get("json.data").is_some_and(|v| v.is_object()) };
                if _cond {
                    // Painless script
                    // Source: Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');"#
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("user.email")
                        && event.has_value("json.actor_email")
                        && event.get_str("json.actor_email") != Some("")
                };
                if _cond {
                    if event.has_value("json.actor_email") {
                        event.rename("json.actor_email", "user.email")?;
                    }
                }
                event.remove("json.actor_email");
                let _cond = { !event.has_value("kolide.audit.actor_type") };
                if _cond {
                    if event.has_value("json.actor_type") {
                        event.rename("json.actor_type", "kolide.audit.actor_type")?;
                    }
                }
                let _cond = {
                    !event.has_value("source.ip")
                        && event.has_value("json.ip_address")
                        && event.get_str("json.ip_address") != Some("")
                };
                if _cond {
                    if event.has_value("json.ip_address") {
                        event.rename("json.ip_address", "source.ip")?;
                    }
                }
                event.remove("json.ip_address");
                // End nested pipeline: "s3"
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
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
            }

            let _cond = { !event.has_value("event.id") };
            if _cond {
                if event.has_value("json.id") {
                    event.rename("json.id", "event.id")?;
                }
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                let _cond = {
                    event.get_str("json.event") != Some("audit_log.recorded")
                        && !event.has_value("event.action")
                };
                if _cond {
                    if event.has_value("json.event") {
                        event.rename("json.event", "event.action")?;
                    }
                }
                let _cond = { !event.has_value("user.name") };
                if _cond {
                    if event.has_value("json.data.actor_name") {
                        event.rename("json.data.actor_name", "user.name")?;
                    }
                }
                let _cond = { !event.has_value("user.email") };
                if _cond {
                    if event.has_value("json.data.actor_email") {
                        event.rename("json.data.actor_email", "user.email")?;
                    }
                }
                let _cond = { !event.has_value("kolide.audit.actor_type") };
                if _cond {
                    if event.has_value("json.data.actor_type") {
                        event.rename("json.data.actor_type", "kolide.audit.actor_type")?;
                    }
                }
                let _cond = { !event.has_value("source.ip") };
                if _cond {
                    if event.has_value("json.data.ip_address") {
                        event.rename("json.data.ip_address", "source.ip")?;
                    }
                }
                let _cond = { !event.has_value("message") };
                if _cond {
                    if event.has_value("json.data.description") {
                        event.rename("json.data.description", "message")?;
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if event.has_value("json.actor_info.actor_name") {
                    event.rename("json.actor_info.actor_name", "user.name")?;
                }
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if event.has_value("json.actor_name") {
                    event.rename("json.actor_name", "user.name")?;
                }
            }

            event.remove("json.actor_name");

            let _cond = { !event.has_value("user.email") };
            if _cond {
                if event.has_value("json.actor_info.actor_email") {
                    event.rename("json.actor_info.actor_email", "user.email")?;
                }
            }

            let _cond = { !event.has_value("kolide.audit.actor_type") };
            if _cond {
                if event.has_value("json.actor_info.actor_type") {
                    event.rename("json.actor_info.actor_type", "kolide.audit.actor_type")?;
                }
            }

            let _cond = { !event.has_value("message") };
            if _cond {
                if event.has_value("json.description") {
                    event.rename("json.description", "message")?;
                }
            }

            let _cond = {
                event.get_str("kolide.audit.actor_type") == Some("ApiKey")
                    || event.get_str("kolide.audit.actor_type") == Some("System")
            };
            if _cond {
                event.set("kolide.audit.actor_automated", json!(true))?;
            }

            let _cond = {
                event.has_value("kolide.audit.actor_type")
                    && event.get_str("kolide.audit.actor_type") != Some("ApiKey")
                    && event.get_str("kolide.audit.actor_type") != Some("System")
            };
            if _cond {
                event.set("kolide.audit.actor_automated", json!(false))?;
            }

            let _cond = { event.has_value("source.ip") && !event.has_value("source.geo") };
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

            let _cond = { event.has_value("source.ip") && !event.has_value("source.as") };
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

            let _cond = { !event.has_value("source.as.number") };
            if _cond {
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
            }

            let _cond = { !event.has_value("source.as.organization.name") };
            if _cond {
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
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

            // Begin nested pipeline: "extended-mappings"
            let _cond = { event.has_value("message") };
            if _cond {
                // Painless script
                // Source: String m = ctx.message;\nString a = null;\nString c = null;\n\n// Exact-match, fixed-description events (no extractable fields; no grok).\nif (m == 'Created an API key') { a = 'api_key_created'; }\nelse if (m == 'Removed an API key') { a = 'api_key_deleted'; }\nelse if (m == 'Rotated API key') { a = 'api_key_secret_rotated'; }\nelse if (m == 'Enabled the logging pipeline') { a = 'logging_pipeline_enabled'; }\nelse if (m == 'Disabled the logging pipeline') { a = 'logging_pipeline_disabled'; }\nelse if (m == 'Updated osquery options') { a = 'updated_osquery_options'; }\nelse if (m == 'Reset all osquery options to their default value') { a = 'reset_osquery_options'; }\nelse if (m == 'Updated SAML configuration') { a = 'updated_saml_configuration'; c = 'sso'; }\nelse if (m == 'Okta event hooks verified') { a = 'saml_webhook_verification'; c = 'okta'; }\nelse if (m == 'Duplicate device found') { a = 'duplicate_device_found'; }\nelse if (m == 'created a Vanta integration') { a = 'vanta_integration_created'; c = 'integrations'; }\nelse if (m == 'removed Vanta integration') { a = 'vanta_integration_deleted'; c = 'integrations'; }\nelse if (m == \"modified the organization's extended device compliance configuration\") { a = 'extended_device_compliance_configuration_changed'; }\n\n// Checks.\nelse if (m.startsWith('Deleted Check ')) { a = 'check_deleted'; c = 'check'; }\nelse if (m.startsWith('Reverted Check ')) { a = 'check_reverted'; c = 'check'; }\nelse if (m.startsWith('Updated existing Check ')) { a = 'check_updated'; c = 'check'; }\nelse if (m.startsWith('Published new Check ')) { a = 'check_published'; c = 'check'; }\nelse if (m.startsWith('Changed Fix Instructions Template ') || m.startsWith('Changed Rationale Template ')) { a = 'check_configuration_changed'; c = 'check'; }\nelse if (m.startsWith('Updated device trust settings for ') || m.startsWith('Updated run targets for ')) { a = 'updated_check_device_trust_settings'; c = 'check'; }\nelse if (m.startsWith('Paused check ')) { a = 'check_paused'; c = 'check'; }\nelse if (m.startsWith('Enabled check ')) { a = 'check_enabled'; c = 'check'; }\n\n// Devices.\nelse if (m.startsWith('Changed device name from ')) { a = 'device_display_name_changed'; c = 'device'; }\nelse if (m.startsWith('Cancelled pending deletion for ')) { a = 'canceled_device_removal'; c = 'device'; }\nelse if (m.startsWith('Requested data export for device ')) { a = 'device_data_export_requested'; c = 'device'; }\n\n// Device registrations.\nelse if (m.startsWith('Approved pending device registration for ')) { a = 'device_registration_approved'; c = 'device_reg'; }\nelse if (m.startsWith('Denied pending device registration for ')) { a = 'device_registration_denied'; c = 'device_reg'; }\nelse if (m.startsWith('Reopened previously ') && m.contains(' device registration for ')) { a = 'device_registration_reopened'; c = 'device_reg'; }\nelse if (m.contains(' registration self-approved by ')) { a = 'device_registration_self_approved'; c = 'device_reg'; }\nelse if (m.startsWith('Removed device registration for ')) { a = 'device_registration_removed'; c = 'device_reg'; }\nelse if (m.startsWith('TOFU device registration re-enabled for ')) { a = 'tofu_device_registration_re_enabled'; c = 'device_reg'; }\nelse if (m.startsWith('Updated device registration for ') && m.contains(' changed auth mode from ')) { a = 'device_registration_auth_mode_changed'; c = 'device_reg'; }\nelse if (m.startsWith('Changed Default Authentication Mode') || m.startsWith(\"Changed 'Allows \") || m.startsWith(\"Set 'Required \") || m.startsWith('Updated device registration configuration: ')) { a = 'device_registration_configuration_changed'; c = 'device_reg'; }\n\n// Exemption requests.\nelse if (m.startsWith('Approved exemption request for ')) { a = 'exemption_request_approved'; c = 'exemption'; }\nelse if (m.startsWith('Denied exemption request for ')) { a = 'exemption_request_denied'; c = 'exemption'; }\nelse if (m.startsWith('Reopened previously ') && m.contains(' exemption for ')) { a = 'exemption_request_reopened'; c = 'exemption'; }\nelse if (m.contains(' withdrew exemption request for check: ')) { a = 'exemption_request_withdrawn'; c = 'exemption'; }\nelse if (m.startsWith('Exempted all future issues for ')) { a = 'check_marked_as_out_of_scope'; c = 'exemption'; }\nelse if (m.startsWith('Exempted issue ')) { a = 'issue_exempted'; c = 'exemption'; }\n\n// People.\nelse if (m.startsWith('Reset factor enrollment for ')) { a = 'factor_enrollment_reset'; c = 'people'; }\nelse if (m.startsWith('Verified factor enrollment for ')) { a = 'factor_enrollment_verified'; c = 'people'; }\nelse if (m.startsWith('Merged the person, ')) { a = 'person_record_merged'; c = 'people'; }\nelse if (m.startsWith('Restored the person, ')) { a = 'person_record_unmerged'; c = 'people'; }\n\n// Managed web app.\nelse if (m.startsWith('Created managed app ')) { a = 'managed_app_created'; c = 'managed_app'; }\nelse if (m.startsWith('Deleted managed app ')) { a = 'managed_app_deleted'; c = 'managed_app'; }\nelse if (m.startsWith('Sign on settings were updated for ')) { a = 'managed_app_sign_on_settings_updated'; c = 'managed_app'; }\nelse if (m.contains(' directly assigned people membership from ')) { a = 'managed_app_direct_assigned_people_membership_changed'; c = 'managed_app'; }\nelse if (m.contains(' directly assigned person groups membership from ')) { a = 'managed_app_person_groups_membership_changed'; c = 'managed_app'; }\nelse if (m.startsWith('Updated managed app ')) { a = 'managed_app_updated'; c = 'managed_app'; }\n\n// Groups.\nelse if (m.startsWith('Mass-Removed members from device group: ')) { a = 'device_group_memberships_removed'; c = 'groups'; }\nelse if (m.startsWith('Created device group ')) { a = 'device_group_created'; c = 'groups'; }\nelse if (m.startsWith('Deleted device group ')) { a = 'device_group_deleted'; c = 'groups'; }\n\n// Log pipeline.\nelse if (m.startsWith('Added device property logger ')) { a = 'device_property_logger_added'; c = 'log_pipeline'; }\nelse if (m.startsWith('Removed device property logger ')) { a = 'device_property_logger_removed'; c = 'log_pipeline'; }\nelse if (m.startsWith('Enabled the log pipeline destination ')) { a = 'enabled_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Disabled the log pipeline destination ')) { a = 'disabled_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Deleted the log pipeline destination ')) { a = 'deleted_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Updated the ') && m.contains(' log destination ')) { a = 'updated_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Created a ') && m.contains(' log pipeline destination named ')) { a = 'created_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.contains('osquery decorator ')) { a = 'osquery_decorator_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery FIM category ')) { a = 'osquery_fim_category_changed'; c = 'log_pipeline'; }\nelse if (m.contains('discovery query ')) { a = 'discovery_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery pack query ')) { a = 'osquery_pack_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery query ')) { a = 'osquery_pack_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery pack ')) { a = 'osquery_pack_changed'; c = 'log_pipeline'; }\n\n// Live queries.\nelse if (m.startsWith('Created and ran Live Query Campaign ')) { a = 'live_query_created_and_run'; c = 'live_query'; }\nelse if (m.startsWith('Updated and ran Live Query Campaign ')) { a = 'live_query_updated_and_run'; c = 'live_query'; }\nelse if (m.startsWith('Deleted Live Query Campaign ')) { a = 'live_query_deleted'; c = 'live_query'; }\nelse if (m.startsWith('CSV Downloaded For Device ')) { a = 'live_query_single_result_csv_exported'; c = 'live_query'; }\nelse if (m.startsWith('Unpublished Live Query Campaign ')) { a = 'live_query_unpublished'; c = 'live_query'; }\nelse if (m.startsWith('CSV Downloaded For Live Query Campaign ')) { a = 'live_query_csv_exported'; c = 'live_query'; }\nelse if (m.startsWith('Published Live Query Campaign ')) { a = 'live_query_published'; c = 'live_query'; }\n\n// Okta webhooks (dynamic event name; action set after grok extracts okta_event).\nelse if (m.startsWith('Okta event hook received for ')) { c = 'okta'; }\n\n// Billing.\nelse if (m.startsWith('Updated billing email to ')) { a = 'billing_email_updated'; c = 'billing'; }\n\n// End user portal (privacy center).\nelse if (m.startsWith(\"Changed 'Privacy Center Access Restriction Settings'\") || m.startsWith('Changed Privacy Center Custom Resource Section visibility ')) { a = 'privacy_center_configuration_changed'; c = 'privacy_center'; }\n\n// Automatic device removal.\nelse if (m.startsWith('Device ') && m.endsWith(' removed')) { a = 'device_removed'; c = 'device_removal'; }\nelse if (m.startsWith('triggered device deletion for ') || m.startsWith('Requested device deletion for ')) { a = 'device_deletion_requested'; c = 'device_removal'; }\n\n// Restrictions.\nelse if (m.startsWith('Osquery Blocklist Updated From: ')) { a = 'osquery_blocklist_updated'; c = 'restrictions'; }\n\n// Admin users.\nelse if (m.startsWith('user ') && m.endsWith(' accepted invitation to Kolide')) { a = 'kolide_team_member_created'; c = 'admin_users'; }\nelse if (m.startsWith('Invited ')) { a = 'kolide_team_member_invited'; c = 'admin_users'; }\nelse if (m.startsWith('Removed access for Kolide user ')) { a = 'kolide_team_member_deleted'; c = 'admin_users'; }\nelse if (m.contains(' restriction on ') && m.startsWith('Updated ')) { a = 'user_feature_restriction_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Updated access for user ')) { a = 'user_access_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Revoked invitations created by ')) { a = 'invitations_revoked_because_inviter_access_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Added EPM user ')) { a = 'user_added'; c = 'admin_users'; }\n\n// SSO settings.\nelse if (m.startsWith('Kolide IdP settings were updated from:')) { a = 'idp_settings_changed'; c = 'sso'; }\nelse if (m.startsWith('IdP Proxy webhook token generated for ')) { a = 'webhook_token_generated'; c = 'sso'; }\nelse if (m.startsWith('Kolide IdP: Additional Factor Sequencing was ')) { a = 'factor_sequencing_changed'; c = 'sso'; }\nelse if (m.startsWith('Identity provider ') && m.endsWith(' was activated')) { a = 'identity_provider_activated'; c = 'sso'; }\nelse if (m.startsWith('Updated SSO configuration for IdP: ')) { a = 'updated_sso_configuration'; c = 'sso'; }\nelse if (m.startsWith('SCIM provider bearer token generated for ')) { a = 'scim_provider_bearer_token_generated'; c = 'sso'; }\n\n// Integrations (Google Workspace OAuth importer).\nelse if (m.startsWith('created a OAuth grant importer for a Google Workspace integration ')) { a = 'oauth_grant_importer_for_google_workspace_integration_created'; c = 'integrations'; }\nelse if (m.startsWith('removed a OAuth grant importer for a Google Workspace integration ')) { a = 'oauth_grant_importer_for_google_workspace_integration_deleted'; c = 'integrations'; }\n\n// Device management providers.\nelse if (m.startsWith('Added a Device Management Provider')) { a = 'device_management_provider_added'; c = 'dmp'; }\nelse if (m.startsWith('Updated Device Management Provider: ')) { a = 'device_management_provider_updated'; c = 'dmp'; }\nelse if (m.startsWith('Removed Device Management Provider: ')) { a = 'device_management_provider_deleted'; c = 'dmp'; }\n\n// Settings: auto-snooze policy.\nelse if (m.startsWith('Set auto-snooze policy for org ')) { a = 'auto_snooze_policy_changed'; c = 'auto_snooze'; }\n\n// End user remediation configuration.\nelse if (m == \"modified the organization's end user remediation configuration\" || m.startsWith('Updated End-User Remediation Configuration')) { a = 'end_user_remediation_configuration_changed'; }\n\n// Developers (API keys & webhooks).\nelse if (m.startsWith('Revealed full API Key token ')) { a = 'api_key_secret_viewed'; c = 'developers'; }\nelse if (m.startsWith('API Key ')) { a = 'updated_api_key'; c = 'developers'; }\nelse if (m.startsWith('Created webhook with url ')) { a = 'webhook_created'; c = 'developers'; }\nelse if (m.startsWith('Deleted webhook with url ')) { a = 'webhook_deleted'; c = 'developers'; }\nelse if (m.startsWith('Enabled webhook with url ')) { a = 'webhook_enabled'; c = 'developers'; }\nelse if (m.startsWith('Disabled webhook with url ')) { a = 'webhook_disabled'; c = 'developers'; }\nelse if (m.startsWith('Revealed full webhook signing secret ')) { a = 'webhook_signing_secret_viewed'; c = 'developers'; }\nelse if (m.startsWith('Updated webhook url from ')) { a = 'webhook_url_changed'; c = 'developers'; }\nelse if (m.startsWith('Updated event subscriptions for webhook ')) { a = 'webhook_event_subscriptions_changed'; c = 'developers'; }\nelse if (m.startsWith('Rolled signing secret for webhook with url ')) { a = 'webhook_signing_secret_rolled'; c = 'developers'; }\n\n// Generic feature restriction change fallback (action set after grok confirms a match).\nelse if (m.startsWith(\"Changed '\")) { c = 'setting_generic'; }\n\nif (a != null) {\n  if (ctx.event == null) { ctx.event = new HashMap(); }\n  ctx.event.action = a;\n}\nif (c != null) {\n  if (ctx._tmp == null) { ctx._tmp = new HashMap(); }\n  ctx._tmp.cat = c;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String m = ctx.message;\nString a = null;\nString c = null;\n\n// Exact-match, fixed-description events (no extractable fields; no grok).\nif (m == 'Created an API key') { a = 'api_key_created'; }\nelse if (m == 'Removed an API key') { a = 'api_key_deleted'; }\nelse if (m == 'Rotated API key') { a = 'api_key_secret_rotated'; }\nelse if (m == 'Enabled the logging pipeline') { a = 'logging_pipeline_enabled'; }\nelse if (m == 'Disabled the logging pipeline') { a = 'logging_pipeline_disabled'; }\nelse if (m == 'Updated osquery options') { a = 'updated_osquery_options'; }\nelse if (m == 'Reset all osquery options to their default value') { a = 'reset_osquery_options'; }\nelse if (m == 'Updated SAML configuration') { a = 'updated_saml_configuration'; c = 'sso'; }\nelse if (m == 'Okta event hooks verified') { a = 'saml_webhook_verification'; c = 'okta'; }\nelse if (m == 'Duplicate device found') { a = 'duplicate_device_found'; }\nelse if (m == 'created a Vanta integration') { a = 'vanta_integration_created'; c = 'integrations'; }\nelse if (m == 'removed Vanta integration') { a = 'vanta_integration_deleted'; c = 'integrations'; }\nelse if (m == \"modified the organization's extended device compliance configuration\") { a = 'extended_device_compliance_configuration_changed'; }\n\n// Checks.\nelse if (m.startsWith('Deleted Check ')) { a = 'check_deleted'; c = 'check'; }\nelse if (m.startsWith('Reverted Check ')) { a = 'check_reverted'; c = 'check'; }\nelse if (m.startsWith('Updated existing Check ')) { a = 'check_updated'; c = 'check'; }\nelse if (m.startsWith('Published new Check ')) { a = 'check_published'; c = 'check'; }\nelse if (m.startsWith('Changed Fix Instructions Template ') || m.startsWith('Changed Rationale Template ')) { a = 'check_configuration_changed'; c = 'check'; }\nelse if (m.startsWith('Updated device trust settings for ') || m.startsWith('Updated run targets for ')) { a = 'updated_check_device_trust_settings'; c = 'check'; }\nelse if (m.startsWith('Paused check ')) { a = 'check_paused'; c = 'check'; }\nelse if (m.startsWith('Enabled check ')) { a = 'check_enabled'; c = 'check'; }\n\n// Devices.\nelse if (m.startsWith('Changed device name from ')) { a = 'device_display_name_changed'; c = 'device'; }\nelse if (m.startsWith('Cancelled pending deletion for ')) { a = 'canceled_device_removal'; c = 'device'; }\nelse if (m.startsWith('Requested data export for device ')) { a = 'device_data_export_requested'; c = 'device'; }\n\n// Device registrations.\nelse if (m.startsWith('Approved pending device registration for ')) { a = 'device_registration_approved'; c = 'device_reg'; }\nelse if (m.startsWith('Denied pending device registration for ')) { a = 'device_registration_denied'; c = 'device_reg'; }\nelse if (m.startsWith('Reopened previously ') && m.contains(' device registration for ')) { a = 'device_registration_reopened'; c = 'device_reg'; }\nelse if (m.contains(' registration self-approved by ')) { a = 'device_registration_self_approved'; c = 'device_reg'; }\nelse if (m.startsWith('Removed device registration for ')) { a = 'device_registration_removed'; c = 'device_reg'; }\nelse if (m.startsWith('TOFU device registration re-enabled for ')) { a = 'tofu_device_registration_re_enabled'; c = 'device_reg'; }\nelse if (m.startsWith('Updated device registration for ') && m.contains(' changed auth mode from ')) { a = 'device_registration_auth_mode_changed'; c = 'device_reg'; }\nelse if (m.startsWith('Changed Default Authentication Mode') || m.startsWith(\"Changed 'Allows \") || m.startsWith(\"Set 'Required \") || m.startsWith('Updated device registration configuration: ')) { a = 'device_registration_configuration_changed'; c = 'device_reg'; }\n\n// Exemption requests.\nelse if (m.startsWith('Approved exemption request for ')) { a = 'exemption_request_approved'; c = 'exemption'; }\nelse if (m.startsWith('Denied exemption request for ')) { a = 'exemption_request_denied'; c = 'exemption'; }\nelse if (m.startsWith('Reopened previously ') && m.contains(' exemption for ')) { a = 'exemption_request_reopened'; c = 'exemption'; }\nelse if (m.contains(' withdrew exemption request for check: ')) { a = 'exemption_request_withdrawn'; c = 'exemption'; }\nelse if (m.startsWith('Exempted all future issues for ')) { a = 'check_marked_as_out_of_scope'; c = 'exemption'; }\nelse if (m.startsWith('Exempted issue ')) { a = 'issue_exempted'; c = 'exemption'; }\n\n// People.\nelse if (m.startsWith('Reset factor enrollment for ')) { a = 'factor_enrollment_reset'; c = 'people'; }\nelse if (m.startsWith('Verified factor enrollment for ')) { a = 'factor_enrollment_verified'; c = 'people'; }\nelse if (m.startsWith('Merged the person, ')) { a = 'person_record_merged'; c = 'people'; }\nelse if (m.startsWith('Restored the person, ')) { a = 'person_record_unmerged'; c = 'people'; }\n\n// Managed web app.\nelse if (m.startsWith('Created managed app ')) { a = 'managed_app_created'; c = 'managed_app'; }\nelse if (m.startsWith('Deleted managed app ')) { a = 'managed_app_deleted'; c = 'managed_app'; }\nelse if (m.startsWith('Sign on settings were updated for ')) { a = 'managed_app_sign_on_settings_updated'; c = 'managed_app'; }\nelse if (m.contains(' directly assigned people membership from ')) { a = 'managed_app_direct_assigned_people_membership_changed'; c = 'managed_app'; }\nelse if (m.contains(' directly assigned person groups membership from ')) { a = 'managed_app_person_groups_membership_changed'; c = 'managed_app'; }\nelse if (m.startsWith('Updated managed app ')) { a = 'managed_app_updated'; c = 'managed_app'; }\n\n// Groups.\nelse if (m.startsWith('Mass-Removed members from device group: ')) { a = 'device_group_memberships_removed'; c = 'groups'; }\nelse if (m.startsWith('Created device group ')) { a = 'device_group_created'; c = 'groups'; }\nelse if (m.startsWith('Deleted device group ')) { a = 'device_group_deleted'; c = 'groups'; }\n\n// Log pipeline.\nelse if (m.startsWith('Added device property logger ')) { a = 'device_property_logger_added'; c = 'log_pipeline'; }\nelse if (m.startsWith('Removed device property logger ')) { a = 'device_property_logger_removed'; c = 'log_pipeline'; }\nelse if (m.startsWith('Enabled the log pipeline destination ')) { a = 'enabled_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Disabled the log pipeline destination ')) { a = 'disabled_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Deleted the log pipeline destination ')) { a = 'deleted_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Updated the ') && m.contains(' log destination ')) { a = 'updated_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.startsWith('Created a ') && m.contains(' log pipeline destination named ')) { a = 'created_log_pipeline_destination'; c = 'log_pipeline'; }\nelse if (m.contains('osquery decorator ')) { a = 'osquery_decorator_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery FIM category ')) { a = 'osquery_fim_category_changed'; c = 'log_pipeline'; }\nelse if (m.contains('discovery query ')) { a = 'discovery_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery pack query ')) { a = 'osquery_pack_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery query ')) { a = 'osquery_pack_query_changed'; c = 'log_pipeline'; }\nelse if (m.contains('osquery pack ')) { a = 'osquery_pack_changed'; c = 'log_pipeline'; }\n\n// Live queries.\nelse if (m.startsWith('Created and ran Live Query Campaign ')) { a = 'live_query_created_and_run'; c = 'live_query'; }\nelse if (m.startsWith('Updated and ran Live Query Campaign ')) { a = 'live_query_updated_and_run'; c = 'live_query'; }\nelse if (m.startsWith('Deleted Live Query Campaign ')) { a = 'live_query_deleted'; c = 'live_query'; }\nelse if (m.startsWith('CSV Downloaded For Device ')) { a = 'live_query_single_result_csv_exported'; c = 'live_query'; }\nelse if (m.startsWith('Unpublished Live Query Campaign ')) { a = 'live_query_unpublished'; c = 'live_query'; }\nelse if (m.startsWith('CSV Downloaded For Live Query Campaign ')) { a = 'live_query_csv_exported'; c = 'live_query'; }\nelse if (m.startsWith('Published Live Query Campaign ')) { a = 'live_query_published'; c = 'live_query'; }\n\n// Okta webhooks (dynamic event name; action set after grok extracts okta_event).\nelse if (m.startsWith('Okta event hook received for ')) { c = 'okta'; }\n\n// Billing.\nelse if (m.startsWith('Updated billing email to ')) { a = 'billing_email_updated'; c = 'billing'; }\n\n// End user portal (privacy center).\nelse if (m.startsWith(\"Changed 'Privacy Center Access Restriction Settings'\") || m.startsWith('Changed Privacy Center Custom Resource Section visibility ')) { a = 'privacy_center_configuration_changed'; c = 'privacy_center'; }\n\n// Automatic device removal.\nelse if (m.startsWith('Device ') && m.endsWith(' removed')) { a = 'device_removed'; c = 'device_removal'; }\nelse if (m.startsWith('triggered device deletion for ') || m.startsWith('Requested device deletion for ')) { a = 'device_deletion_requested'; c = 'device_removal'; }\n\n// Restrictions.\nelse if (m.startsWith('Osquery Blocklist Updated From: ')) { a = 'osquery_blocklist_updated'; c = 'restrictions'; }\n\n// Admin users.\nelse if (m.startsWith('user ') && m.endsWith(' accepted invitation to Kolide')) { a = 'kolide_team_member_created'; c = 'admin_users'; }\nelse if (m.startsWith('Invited ')) { a = 'kolide_team_member_invited'; c = 'admin_users'; }\nelse if (m.startsWith('Removed access for Kolide user ')) { a = 'kolide_team_member_deleted'; c = 'admin_users'; }\nelse if (m.contains(' restriction on ') && m.startsWith('Updated ')) { a = 'user_feature_restriction_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Updated access for user ')) { a = 'user_access_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Revoked invitations created by ')) { a = 'invitations_revoked_because_inviter_access_changed'; c = 'admin_users'; }\nelse if (m.startsWith('Added EPM user ')) { a = 'user_added'; c = 'admin_users'; }\n\n// SSO settings.\nelse if (m.startsWith('Kolide IdP settings were updated from:')) { a = 'idp_settings_changed'; c = 'sso'; }\nelse if (m.startsWith('IdP Proxy webhook token generated for ')) { a = 'webhook_token_generated'; c = 'sso'; }\nelse if (m.startsWith('Kolide IdP: Additional Factor Sequencing was ')) { a = 'factor_sequencing_changed'; c = 'sso'; }\nelse if (m.startsWith('Identity provider ') && m.endsWith(' was activated')) { a = 'identity_provider_activated'; c = 'sso'; }\nelse if (m.startsWith('Updated SSO configuration for IdP: ')) { a = 'updated_sso_configuration'; c = 'sso'; }\nelse if (m.startsWith('SCIM provider bearer token generated for ')) { a = 'scim_provider_bearer_token_generated'; c = 'sso'; }\n\n// Integrations (Google Workspace OAuth importer).\nelse if (m.startsWith('created a OAuth grant importer for a Google Workspace integration ')) { a = 'oauth_grant_importer_for_google_workspace_integration_created'; c = 'integrations'; }\nelse if (m.startsWith('removed a OAuth grant importer for a Google Workspace integration ')) { a = 'oauth_grant_importer_for_google_workspace_integration_deleted'; c = 'integrations'; }\n\n// Device management providers.\nelse if (m.startsWith('Added a Device Management Provider')) { a = 'device_management_provider_added'; c = 'dmp'; }\nelse if (m.startsWith('Updated Device Management Provider: ')) { a = 'device_management_provider_updated'; c = 'dmp'; }\nelse if (m.startsWith('Removed Device Management Provider: ')) { a = 'device_management_provider_deleted'; c = 'dmp'; }\n\n// Settings: auto-snooze policy.\nelse if (m.startsWith('Set auto-snooze policy for org ')) { a = 'auto_snooze_policy_changed'; c = 'auto_snooze'; }\n\n// End user remediation configuration.\nelse if (m == \"modified the organization's end user remediation configuration\" || m.startsWith('Updated End-User Remediation Configuration')) { a = 'end_user_remediation_configuration_changed'; }\n\n// Developers (API keys & webhooks).\nelse if (m.startsWith('Revealed full API Key token ')) { a = 'api_key_secret_viewed'; c = 'developers'; }\nelse if (m.startsWith('API Key ')) { a = 'updated_api_key'; c = 'developers'; }\nelse if (m.startsWith('Created webhook with url ')) { a = 'webhook_created'; c = 'developers'; }\nelse if (m.startsWith('Deleted webhook with url ')) { a = 'webhook_deleted'; c = 'developers'; }\nelse if (m.startsWith('Enabled webhook with url ')) { a = 'webhook_enabled'; c = 'developers'; }\nelse if (m.startsWith('Disabled webhook with url ')) { a = 'webhook_disabled'; c = 'developers'; }\nelse if (m.startsWith('Revealed full webhook signing secret ')) { a = 'webhook_signing_secret_viewed'; c = 'developers'; }\nelse if (m.startsWith('Updated webhook url from ')) { a = 'webhook_url_changed'; c = 'developers'; }\nelse if (m.startsWith('Updated event subscriptions for webhook ')) { a = 'webhook_event_subscriptions_changed'; c = 'developers'; }\nelse if (m.startsWith('Rolled signing secret for webhook with url ')) { a = 'webhook_signing_secret_rolled'; c = 'developers'; }\n\n// Generic feature restriction change fallback (action set after grok confirms a match).\nelse if (m.startsWith(\"Changed '\")) { c = 'setting_generic'; }\n\nif (a != null) {\n  if (ctx.event == null) { ctx.event = new HashMap(); }\n  ctx.event.action = a;\n}\nif (c != null) {\n  if (ctx._tmp == null) { ctx._tmp = new HashMap(); }\n  ctx._tmp.cat = c;\n}"#
                    ),
                )?;
            }
            let _cond = { event.get_str("_tmp.cat") == Some("check") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Deleted Check \"%{DATA:rule.name}\"$
                        // Grok pattern: ^Reverted Check \"%{DATA:rule.name}\" to a prior version$
                        // Grok pattern: ^Reverted Check \"%{DATA:rule.name}\" %{DATA} to a prior version$
                        // Grok pattern: ^Updated existing Check \"%{DATA:rule.name}\"$
                        // Grok pattern: ^Published new Check \"%{DATA:rule.name}\"$
                        // Grok pattern: ^Changed Fix Instructions Template Text for Check '%{DATA:rule.name}'$
                        // Grok pattern: ^Changed Rationale Template Text for Check '%{DATA:rule.name}'$
                        // Grok pattern: ^Changed Fix Instructions Template Strategy for Check '%{DATA:rule.name}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Changed Rationale Template Strategy for Check '%{DATA:rule.name}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!("^Deleted Check \"%{DATA:rule.name}\"$"),
                                cached_grok!(
                                    "^Reverted Check \"%{DATA:rule.name}\" to a prior version$"
                                ),
                                cached_grok!(
                                    "^Reverted Check \"%{DATA:rule.name}\" %{DATA} to a prior version$"
                                ),
                                cached_grok!("^Updated existing Check \"%{DATA:rule.name}\"$"),
                                cached_grok!("^Published new Check \"%{DATA:rule.name}\"$"),
                                cached_grok!(
                                    "^Changed Fix Instructions Template Text for Check '%{DATA:rule.name}'$"
                                ),
                                cached_grok!(
                                    "^Changed Rationale Template Text for Check '%{DATA:rule.name}'$"
                                ),
                                cached_grok!(
                                    "^Changed Fix Instructions Template Strategy for Check '%{DATA:rule.name}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Changed Rationale Template Strategy for Check '%{DATA:rule.name}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("check") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Updated device trust settings for '%{DATA:rule.name}' from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Updated run targets for '%{DATA:rule.name}' from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Updated device trust settings for '%{DATA:rule.name}' from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Updated run targets for '%{DATA:rule.name}' from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("check") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Paused check '%{DATA:rule.name}'$
                        // Grok pattern: ^Enabled check '%{DATA:rule.name}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!("^Paused check '%{DATA:rule.name}'$"),
                                cached_grok!("^Enabled check '%{DATA:rule.name}'$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Changed device name from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        let _ = cached_grok!("^Changed device name from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Cancelled pending deletion for '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\)$
                        // Grok pattern: ^Cancelled pending deletion for '%{DATA:host.name}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Cancelled pending deletion for '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\)$"
                                ),
                                cached_grok!(
                                    "^Cancelled pending deletion for '%{DATA:host.name}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Requested data export for device '%{DATA:host.name}' \\(id %{DATA:kolide.audit.target.device_id}\\)$
                        let _ = cached_grok!("^Requested data export for device '%{DATA:host.name}' \\(id %{DATA:kolide.audit.target.device_id}\\)$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Approved pending device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$
                        // Grok pattern: ^Denied pending device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Approved pending device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$"
                                ),
                                cached_grok!(
                                    "^Denied pending device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Reopened previously %{WORD:kolide.audit.target.prev_status} device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\.$
                        let _ = cached_grok!("^Reopened previously %{WORD:kolide.audit.target.prev_status} device registration for \"%{DATA:user.target.email}\" and device \"%{DATA:host.name}\"\\.$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Device \"%{DATA:host.name}\" \\(%{DATA:kolide.audit.target.device_serial}\\) registration self-approved by %{DATA:user.target.email} from another trusted device$
                        let _ = cached_grok!("^Device \"%{DATA:host.name}\" \\(%{DATA:kolide.audit.target.device_serial}\\) registration self-approved by %{DATA:user.target.email} from another trusted device$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Removed device registration for \"%{DATA:host.name}\" that was registered to \"%{DATA:user.target.email}\"$
                        // Grok pattern: ^Removed device registration for \"%{DATA:host.name}\" that was registered to %{GREEDYDATA:user.target.email}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Removed device registration for \"%{DATA:host.name}\" that was registered to \"%{DATA:user.target.email}\"$"
                                ),
                                cached_grok!(
                                    "^Removed device registration for \"%{DATA:host.name}\" that was registered to %{GREEDYDATA:user.target.email}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^TOFU device registration re-enabled for '%{DATA:host.name}'$
                        let _ = cached_grok!(
                            "^TOFU device registration re-enabled for '%{DATA:host.name}'$"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Updated device registration for \"%{DATA:host.name}\" changed auth mode from \"%{DATA:kolide.audit.change.from}\" to \"%{DATA:kolide.audit.change.to}\" and changed allowed groups from %{DATA:kolide.audit.change.groups_from} to %{GREEDYDATA:kolide.audit.change.groups_to}$
                        // Grok pattern: ^Updated device registration for \"%{DATA:host.name}\" changed auth mode from \"%{DATA:kolide.audit.change.from}\" to \"%{GREEDYDATA:kolide.audit.change.to}\"$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Updated device registration for \"%{DATA:host.name}\" changed auth mode from \"%{DATA:kolide.audit.change.from}\" to \"%{DATA:kolide.audit.change.to}\" and changed allowed groups from %{DATA:kolide.audit.change.groups_from} to %{GREEDYDATA:kolide.audit.change.groups_to}$"
                                ),
                                cached_grok!(
                                    "^Updated device registration for \"%{DATA:host.name}\" changed auth mode from \"%{DATA:kolide.audit.change.from}\" to \"%{GREEDYDATA:kolide.audit.change.to}\"$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Changed Default Authentication Mode changed who can authenticate from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Changed 'Allows %{DATA:kolide.audit.target.platform} device registration' from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Set 'Required %{DATA:kolide.audit.target.platform} checks' to %{GREEDYDATA:kolide.audit.target.check_names}$
                        // Grok pattern: ^Updated device registration configuration: %{GREEDYDATA:kolide.audit.target.config_type}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Changed Default Authentication Mode changed who can authenticate from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Changed 'Allows %{DATA:kolide.audit.target.platform} device registration' from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Set 'Required %{DATA:kolide.audit.target.platform} checks' to %{GREEDYDATA:kolide.audit.target.check_names}$"
                                ),
                                cached_grok!(
                                    "^Updated device registration configuration: %{GREEDYDATA:kolide.audit.target.config_type}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_reg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Changed Default Authentication Mode\" who can authenticate from \"%{DATA:kolide.audit.change.from}\" to \"%{GREEDYDATA:kolide.audit.change.to}\"$
                        let _ = cached_grok!("^Changed Default Authentication Mode\" who can authenticate from \"%{DATA:kolide.audit.change.from}\" to \"%{GREEDYDATA:kolide.audit.change.to}\"$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("exemption") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Approved exemption request for \"%{DATA:user.target.email}\" for check: \"%{DATA:rule.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$
                        // Grok pattern: ^Denied exemption request for \"%{DATA:user.target.email}\" for check: \"%{DATA:rule.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$
                        // Grok pattern: ^Reopened previously %{WORD:kolide.audit.target.prev_status} exemption for \"%{DATA:rule.name}\"\\.$
                        // Grok pattern: ^%{DATA:user.target.email} withdrew exemption request for check: \"%{DATA:rule.name}\"$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Approved exemption request for \"%{DATA:user.target.email}\" for check: \"%{DATA:rule.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$"
                                ),
                                cached_grok!(
                                    "^Denied exemption request for \"%{DATA:user.target.email}\" for check: \"%{DATA:rule.name}\"\\. Reason: \"%{DATA:kolide.audit.reason}\"$"
                                ),
                                cached_grok!(
                                    "^Reopened previously %{WORD:kolide.audit.target.prev_status} exemption for \"%{DATA:rule.name}\"\\.$"
                                ),
                                cached_grok!(
                                    "^%{DATA:user.target.email} withdrew exemption request for check: \"%{DATA:rule.name}\"$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("exemption") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Exempted all future issues for \"%{DATA:rule.name}\" for device: \"%{DATA:host.name}\"\\. Reason: %{DATA:kolide.audit.reason}\\. \\(Expires %{DATA:kolide.audit.expires_at}\\)$
                        // Grok pattern: ^Exempted issue %{DATA:kolide.audit.target.issue_id}: '%{DATA:rule.name}', reason: '%{DATA:kolide.audit.reason}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Exempted all future issues for \"%{DATA:rule.name}\" for device: \"%{DATA:host.name}\"\\. Reason: %{DATA:kolide.audit.reason}\\. \\(Expires %{DATA:kolide.audit.expires_at}\\)$"
                                ),
                                cached_grok!(
                                    "^Exempted issue %{DATA:kolide.audit.target.issue_id}: '%{DATA:rule.name}', reason: '%{DATA:kolide.audit.reason}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("people") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Reset factor enrollment for '%{DATA:user.target.name}'$
                        // Grok pattern: ^Verified factor enrollment for '%{DATA:user.target.name}'$
                        // Grok pattern: ^Merged the person, %{DATA:kolide.audit.change.from}, with %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Restored the person, %{DATA:user.target.name}, to it.s original state$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Reset factor enrollment for '%{DATA:user.target.name}'$"
                                ),
                                cached_grok!(
                                    "^Verified factor enrollment for '%{DATA:user.target.name}'$"
                                ),
                                cached_grok!(
                                    "^Merged the person, %{DATA:kolide.audit.change.from}, with %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Restored the person, %{DATA:user.target.name}, to it.s original state$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("managed_app") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Created managed app \"%{DATA:kolide.audit.target.app_name}\"$
                        // Grok pattern: ^Deleted managed app \"%{DATA:kolide.audit.target.app_name}\" with #?%{NUMBER:kolide.audit.target.count:long} people$
                        // Grok pattern: ^Updated managed app \"%{DATA:kolide.audit.target.app_name}\" from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Sign on settings were updated for \"%{DATA:kolide.audit.target.app_name}\" from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Updated managed app \"#?%{DATA:kolide.audit.target.app_name}\" directly assigned people membership from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Updated managed app \"#?%{DATA:kolide.audit.target.app_name}\" directly assigned person groups membership from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Created managed app \"%{DATA:kolide.audit.target.app_name}\"$"
                                ),
                                cached_grok!(
                                    "^Deleted managed app \"%{DATA:kolide.audit.target.app_name}\" with #?%{NUMBER:kolide.audit.target.count:long} people$"
                                ),
                                cached_grok!(
                                    "^Updated managed app \"%{DATA:kolide.audit.target.app_name}\" from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Sign on settings were updated for \"%{DATA:kolide.audit.target.app_name}\" from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Updated managed app \"#?%{DATA:kolide.audit.target.app_name}\" directly assigned people membership from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Updated managed app \"#?%{DATA:kolide.audit.target.app_name}\" directly assigned person groups membership from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("groups") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Mass-Removed members from device group: \"%{DATA:kolide.audit.target.group_name}\"\\. Device ID\\(s\\) removed: %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Created device group \"%{DATA:kolide.audit.target.group_name}\"$
                        // Grok pattern: ^Deleted device group \"%{DATA:kolide.audit.target.group_name}\" with %{NUMBER:kolide.audit.target.count:long} members$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Mass-Removed members from device group: \"%{DATA:kolide.audit.target.group_name}\"\\. Device ID\\(s\\) removed: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Created device group \"%{DATA:kolide.audit.target.group_name}\"$"
                                ),
                                cached_grok!(
                                    "^Deleted device group \"%{DATA:kolide.audit.target.group_name}\" with %{NUMBER:kolide.audit.target.count:long} members$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("log_pipeline") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Added device property logger '%{DATA:kolide.audit.target.logger_name}'$
                        // Grok pattern: ^Removed device property logger '%{DATA:kolide.audit.target.logger_name}'$
                        // Grok pattern: ^Enabled the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$
                        // Grok pattern: ^Disabled the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$
                        // Grok pattern: ^Deleted the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$
                        // Grok pattern: ^Updated the %{DATA:kolide.audit.target.destination_type} log destination '%{DATA:kolide.audit.target.destination_name}'$
                        // Grok pattern: ^Created a %{DATA:kolide.audit.target.destination_type} log pipeline destination named '%{DATA:kolide.audit.target.destination_name}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Added device property logger '%{DATA:kolide.audit.target.logger_name}'$"
                                ),
                                cached_grok!(
                                    "^Removed device property logger '%{DATA:kolide.audit.target.logger_name}'$"
                                ),
                                cached_grok!(
                                    "^Enabled the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$"
                                ),
                                cached_grok!(
                                    "^Disabled the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$"
                                ),
                                cached_grok!(
                                    "^Deleted the log pipeline destination '%{DATA:kolide.audit.target.destination_name}'$"
                                ),
                                cached_grok!(
                                    "^Updated the %{DATA:kolide.audit.target.destination_type} log destination '%{DATA:kolide.audit.target.destination_name}'$"
                                ),
                                cached_grok!(
                                    "^Created a %{DATA:kolide.audit.target.destination_type} log pipeline destination named '%{DATA:kolide.audit.target.destination_name}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("log_pipeline") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Updated the osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$
                        // Grok pattern: ^Added an osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$
                        // Grok pattern: ^Enabled osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$
                        // Grok pattern: ^Deleted osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$
                        // Grok pattern: ^Updated the osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$
                        // Grok pattern: ^Created osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$
                        // Grok pattern: ^Enabled osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$
                        // Grok pattern: ^Disabled osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$
                        // Grok pattern: ^Deleted the osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Updated the osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$"
                                ),
                                cached_grok!(
                                    "^Added an osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$"
                                ),
                                cached_grok!(
                                    "^Enabled osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$"
                                ),
                                cached_grok!(
                                    "^Deleted osquery decorator '%{DATA:kolide.audit.target.osquery_name}'$"
                                ),
                                cached_grok!(
                                    "^Updated the osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$"
                                ),
                                cached_grok!(
                                    "^Created osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$"
                                ),
                                cached_grok!(
                                    "^Enabled osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$"
                                ),
                                cached_grok!(
                                    "^Disabled osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$"
                                ),
                                cached_grok!(
                                    "^Deleted the osquery FIM category '%{DATA:kolide.audit.target.fim_category}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("log_pipeline") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Created discovery query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Updated the osquery discovery query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Deleted discovery query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Created osquery pack query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Updated the osquery query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Deleted osquery pack query '%{DATA:kolide.audit.target.query_name}'$
                        // Grok pattern: ^Created osquery pack '%{DATA:kolide.audit.target.pack_name}'$
                        // Grok pattern: ^Updated the osquery pack '%{DATA:kolide.audit.target.pack_name}'$
                        // Grok pattern: ^Enabled osquery pack '%{DATA:kolide.audit.target.pack_name}'$
                        // Grok pattern: ^Disabled osquery pack '%{DATA:kolide.audit.target.pack_name}'$
                        // Grok pattern: ^Deleted the osquery pack '%{DATA:kolide.audit.target.pack_name}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Created discovery query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Updated the osquery discovery query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Deleted discovery query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Created osquery pack query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Updated the osquery query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Deleted osquery pack query '%{DATA:kolide.audit.target.query_name}'$"
                                ),
                                cached_grok!(
                                    "^Created osquery pack '%{DATA:kolide.audit.target.pack_name}'$"
                                ),
                                cached_grok!(
                                    "^Updated the osquery pack '%{DATA:kolide.audit.target.pack_name}'$"
                                ),
                                cached_grok!(
                                    "^Enabled osquery pack '%{DATA:kolide.audit.target.pack_name}'$"
                                ),
                                cached_grok!(
                                    "^Disabled osquery pack '%{DATA:kolide.audit.target.pack_name}'$"
                                ),
                                cached_grok!(
                                    "^Deleted the osquery pack '%{DATA:kolide.audit.target.pack_name}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("live_query") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Created and ran Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} that targets %{DATA} that uses table\\(s\\): %{GREEDYDATA:kolide.audit.target.tables}$
                        // Grok pattern: ^Updated and ran Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} that targets %{DATA} that uses table\\(s\\): %{GREEDYDATA:kolide.audit.target.tables}$
                        // Grok pattern: ^Deleted Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} : %{GREEDYDATA:rule.name}$
                        // Grok pattern: ^CSV Downloaded For Device %{DATA:host.name} - Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$
                        // Grok pattern: ^Unpublished Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$
                        // Grok pattern: ^CSV Downloaded For Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$
                        // Grok pattern: ^Published Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Created and ran Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} that targets %{DATA} that uses table\\(s\\): %{GREEDYDATA:kolide.audit.target.tables}$"
                                ),
                                cached_grok!(
                                    "^Updated and ran Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} that targets %{DATA} that uses table\\(s\\): %{GREEDYDATA:kolide.audit.target.tables}$"
                                ),
                                cached_grok!(
                                    "^Deleted Live Query Campaign ID %{DATA:kolide.audit.target.campaign_id} : %{GREEDYDATA:rule.name}$"
                                ),
                                cached_grok!(
                                    "^CSV Downloaded For Device %{DATA:host.name} - Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$"
                                ),
                                cached_grok!(
                                    "^Unpublished Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$"
                                ),
                                cached_grok!(
                                    "^CSV Downloaded For Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$"
                                ),
                                cached_grok!(
                                    "^Published Live Query Campaign ID %{NUMBER:kolide.audit.target.campaign_id}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("okta") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Okta event hook received for '%{DATA:kolide.audit.target.okta_event}'$
                        // Grok pattern: ^Okta event hooks verified$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Okta event hook received for '%{DATA:kolide.audit.target.okta_event}'$"
                                ),
                                cached_grok!("^Okta event hooks verified$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("billing") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Updated billing email to '%{DATA:kolide.audit.change.to}' from '%{DATA:kolide.audit.change.from}'$
                        let _ = cached_grok!("^Updated billing email to '%{DATA:kolide.audit.change.to}' from '%{DATA:kolide.audit.change.from}'$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("privacy_center") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Changed 'Privacy Center Access Restriction Settings' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Changed Privacy Center Custom Resource Section visibility from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Changed 'Privacy Center Access Restriction Settings' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Changed Privacy Center Custom Resource Section visibility from %{DATA:kolide.audit.change.from} to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_removal") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Device '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\) removed$
                        // Grok pattern: ^Device '%{DATA:host.name}' removed$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Device '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\) removed$"
                                ),
                                cached_grok!("^Device '%{DATA:host.name}' removed$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("device_removal") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^triggered device deletion for inactive device named '%{DATA:host.name}'$
                        // Grok pattern: ^Requested device deletion for '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\) \\(\\)$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^triggered device deletion for inactive device named '%{DATA:host.name}'$"
                                ),
                                cached_grok!(
                                    "^Requested device deletion for '%{DATA:host.name}' \\(%{DATA:kolide.audit.target.device_serial}\\) \\(\\)$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("restrictions") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Osquery Blocklist Updated From: \"%{DATA:kolide.audit.change.from}\" To: \"%{GREEDYDATA:kolide.audit.change.to}\"$
                        let _ = cached_grok!("^Osquery Blocklist Updated From: \"%{DATA:kolide.audit.change.from}\" To: \"%{GREEDYDATA:kolide.audit.change.to}\"$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("admin_users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^user '%{DATA:user.target.email}' accepted invitation to Kolide$
                        // Grok pattern: ^Invited '%{DATA:user.target.email}' to access kolide$
                        // Grok pattern: ^Removed access for Kolide user '%{DATA:user.target.name}'$
                        // Grok pattern: ^Updated %{DATA:kolide.audit.target.feature} restriction on %{DATA:user.target.name} from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Updated access for user '%{DATA:user.target.email}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Revoked invitations created by '%{DATA:user.target.email}' because admin access was removed\\. Revoked invitations with email address\\(es\\): %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Added EPM user '#?%{DATA:user.target.name}' to Kolide$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^user '%{DATA:user.target.email}' accepted invitation to Kolide$"
                                ),
                                cached_grok!(
                                    "^Invited '%{DATA:user.target.email}' to access kolide$"
                                ),
                                cached_grok!(
                                    "^Removed access for Kolide user '%{DATA:user.target.name}'$"
                                ),
                                cached_grok!(
                                    "^Updated %{DATA:kolide.audit.target.feature} restriction on %{DATA:user.target.name} from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Updated access for user '%{DATA:user.target.email}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Revoked invitations created by '%{DATA:user.target.email}' because admin access was removed\\. Revoked invitations with email address\\(es\\): %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Added EPM user '#?%{DATA:user.target.name}' to Kolide$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("sso") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Kolide IdP settings were updated from:%{GREEDYDATA}$
                        // Grok pattern: ^IdP Proxy webhook token generated for '%{DATA:kolide.audit.target.org_name}'$
                        // Grok pattern: ^Kolide IdP: Additional Factor Sequencing was '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Updated SAML configuration$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Kolide IdP settings were updated from:%{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^IdP Proxy webhook token generated for '%{DATA:kolide.audit.target.org_name}'$"
                                ),
                                cached_grok!(
                                    "^Kolide IdP: Additional Factor Sequencing was '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!("^Updated SAML configuration$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("sso") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Identity provider '%{DATA:kolide.audit.target.idp_url}' was activated$
                        // Grok pattern: ^Updated SSO configuration for IdP: %{GREEDYDATA:kolide.audit.target.idp_url}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Identity provider '%{DATA:kolide.audit.target.idp_url}' was activated$"
                                ),
                                cached_grok!(
                                    "^Updated SSO configuration for IdP: %{GREEDYDATA:kolide.audit.target.idp_url}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("sso") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^SCIM provider bearer token generated for '%{DATA:kolide.audit.target.org_name}'$
                        let _ = cached_grok!("^SCIM provider bearer token generated for '%{DATA:kolide.audit.target.org_name}'$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("integrations") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^created a Vanta integration$
                        // Grok pattern: ^removed Vanta integration$
                        // Grok pattern: ^created a OAuth grant importer for a Google Workspace integration with email %{GREEDYDATA:user.target.email}$
                        // Grok pattern: ^removed a OAuth grant importer for a Google Workspace integration with email %{GREEDYDATA:user.target.email}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!("^created a Vanta integration$"),
                                cached_grok!("^removed Vanta integration$"),
                                cached_grok!(
                                    "^created a OAuth grant importer for a Google Workspace integration with email %{GREEDYDATA:user.target.email}$"
                                ),
                                cached_grok!(
                                    "^removed a OAuth grant importer for a Google Workspace integration with email %{GREEDYDATA:user.target.email}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("dmp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Added a Device Management Provider:? %{GREEDYDATA:kolide.audit.target.provider_name}$
                        // Grok pattern: ^Updated Device Management Provider: %{DATA:kolide.audit.target.provider_name} from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        // Grok pattern: ^Removed Device Management Provider: %{GREEDYDATA:kolide.audit.target.provider_name}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Added a Device Management Provider:? %{GREEDYDATA:kolide.audit.target.provider_name}$"
                                ),
                                cached_grok!(
                                    "^Updated Device Management Provider: %{DATA:kolide.audit.target.provider_name} from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                                cached_grok!(
                                    "^Removed Device Management Provider: %{GREEDYDATA:kolide.audit.target.provider_name}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("auto_snooze") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Set auto-snooze policy for org %{DATA:kolide.audit.target.org_id} to %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = cached_grok!("^Set auto-snooze policy for org %{DATA:kolide.audit.target.org_id} to %{GREEDYDATA:kolide.audit.change.to}$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("developers") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Revealed full API Key token %{GREEDYDATA:kolide.audit.target.api_key_name}$
                        // Grok pattern: ^API Key %{DATA:kolide.audit.target.api_key_name} from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Revealed full API Key token %{GREEDYDATA:kolide.audit.target.api_key_name}$"
                                ),
                                cached_grok!(
                                    "^API Key %{DATA:kolide.audit.target.api_key_name} from: %{DATA:kolide.audit.change.from} to: %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = { event.get_str("_tmp.cat") == Some("developers") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Created webhook with url '%{DATA:kolide.audit.target.webhook_url}'$
                        // Grok pattern: ^Deleted webhook with url '%{DATA:kolide.audit.target.webhook_url}'$
                        // Grok pattern: ^Enabled webhook with url '%{DATA:kolide.audit.target.webhook_url}'$
                        // Grok pattern: ^Disabled webhook with url '%{DATA:kolide.audit.target.webhook_url}'$
                        // Grok pattern: ^Revealed full webhook signing secret %{GREEDYDATA:kolide.audit.target.webhook_url}$
                        // Grok pattern: ^Updated webhook url from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Updated event subscriptions for webhook '%{DATA:kolide.audit.target.webhook_url}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Rolled signing secret for webhook with url '%{DATA:kolide.audit.target.webhook_url}'$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Created webhook with url '%{DATA:kolide.audit.target.webhook_url}'$"
                                ),
                                cached_grok!(
                                    "^Deleted webhook with url '%{DATA:kolide.audit.target.webhook_url}'$"
                                ),
                                cached_grok!(
                                    "^Enabled webhook with url '%{DATA:kolide.audit.target.webhook_url}'$"
                                ),
                                cached_grok!(
                                    "^Disabled webhook with url '%{DATA:kolide.audit.target.webhook_url}'$"
                                ),
                                cached_grok!(
                                    "^Revealed full webhook signing secret %{GREEDYDATA:kolide.audit.target.webhook_url}$"
                                ),
                                cached_grok!(
                                    "^Updated webhook url from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Updated event subscriptions for webhook '%{DATA:kolide.audit.target.webhook_url}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Rolled signing secret for webhook with url '%{DATA:kolide.audit.target.webhook_url}'$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event.get_str("_tmp.cat") == Some("setting_generic")
                    && !event.has_value("kolide.audit.change.field")
                    && !event.has_value("kolide.audit.change.from")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Changed '%{DATA:kolide.audit.change.field}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$
                        // Grok pattern: ^Changed '%{DATA:kolide.audit.change.field}' from '%{DATA:kolide.audit.change.from}' to %{GREEDYDATA:kolide.audit.change.to}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Changed '%{DATA:kolide.audit.change.field}' from '%{DATA:kolide.audit.change.from}' to '%{DATA:kolide.audit.change.to}'$"
                                ),
                                cached_grok!(
                                    "^Changed '%{DATA:kolide.audit.change.field}' from '%{DATA:kolide.audit.change.from}' to %{GREEDYDATA:kolide.audit.change.to}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event.get_str("_tmp.cat") == Some("setting_generic")
                    && event.has_value("kolide.audit.change.field")
            };
            if _cond {
                event.set("event.action", json!("feature_restriction_changed"))?;
            }
            let _cond = { event.has_value("kolide.audit.target.okta_event") };
            if _cond {
                if let Some(v) = event
                    .get("kolide.audit.target.okta_event")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }
            event.remove("_tmp");
            let _cond = { event.has_value("kolide.audit.target.idp_url") };
            if _cond {
                if let Some(v) = event
                    .get("kolide.audit.target.idp_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.domain", v)?;
                }
            }
            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("host.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
            }
            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
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
            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            // End nested pipeline: "extended-mappings"

            let _cond = { !event.has_value("event.action") };
            if _cond {
                event.set("event.action", json!("audit_log"))?;
            }

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\nctx.event.kind = 'event';\n\nif (params.exact.containsKey(action)) {\n  def m = params.exact[action];\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome')) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  // Verb-derived fallback for actions not in the lookup table.\n  String type = 'change';\n  if (action.endsWith('_created') || action.endsWith('_added') ||\n      action.endsWith('_invited') || action.endsWith('_generated') ||\n      action.endsWith('_created_and_run')) {\n    type = 'creation';\n  } else if (action.endsWith('_deleted') || action.endsWith('_removed')) {\n    type = 'deletion';\n  }\n  ctx.event.category = ['configuration'];\n  ctx.event.type = [type];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\nctx.event.kind = 'event';\n\nif (params.exact.containsKey(action)) {\n  def m = params.exact[action];\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome')) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  // Verb-derived fallback for actions not in the lookup table.\n  String type = 'change';\n  if (action.endsWith('_created') || action.endsWith('_added') ||\n      action.endsWith('_invited') || action.endsWith('_generated') ||\n      action.endsWith('_created_and_run')) {\n    type = 'creation';\n  } else if (action.endsWith('_deleted') || action.endsWith('_removed')) {\n    type = 'deletion';\n  }\n  ctx.event.category = ['configuration'];\n  ctx.event.type = [type];\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"api_key_created\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"api_key_deleted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"]},\"api_key_secret_rotated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"api_key_secret_viewed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"updated_api_key\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"webhook_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"webhook_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"webhook_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"webhook_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"webhook_url_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"webhook_event_subscriptions_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"webhook_signing_secret_viewed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"access\"],\"outcome\":\"success\"},\"webhook_signing_secret_rolled\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"idp_settings_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"identity_provider_activated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"updated_sso_configuration\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"updated_saml_configuration\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"saml_webhook_verification\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"info\"]},\"factor_sequencing_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"webhook_token_generated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"scim_provider_bearer_token_generated\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"kolide_team_member_created\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\",\"user\"]},\"kolide_team_member_invited\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\",\"user\"]},\"kolide_team_member_deleted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\",\"user\"]},\"user_added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\",\"user\"]},\"user_access_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"user_feature_restriction_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"invitations_revoked_because_inviter_access_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"factor_enrollment_reset\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"factor_enrollment_verified\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"device_registration_approved\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"device_registration_self_approved\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"device_registration_denied\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"failure\"},\"device_registration_reopened\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"device_registration_removed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"]},\"device_registration_auth_mode_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"device_registration_configuration_changed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"tofu_device_registration_re_enabled\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"exemption_request_approved\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"exemption_request_denied\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"failure\"},\"exemption_request_reopened\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"exemption_request_withdrawn\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"]},\"issue_exempted\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"check_marked_as_out_of_scope\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"person_record_merged\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"person_record_unmerged\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"user\"]},\"check_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"check_reverted\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"check_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"check_published\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"check_configuration_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"updated_check_device_trust_settings\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"check_paused\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"check_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_display_name_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"canceled_device_removal\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"device_deletion_requested\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_data_export_requested\":{\"category\":[\"configuration\"],\"type\":[\"access\"]},\"duplicate_device_found\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"managed_app_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"managed_app_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"managed_app_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"managed_app_sign_on_settings_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"managed_app_direct_assigned_people_membership_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"managed_app_person_groups_membership_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_group_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"device_group_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"device_group_memberships_removed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_property_logger_added\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"device_property_logger_removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"enabled_log_pipeline_destination\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"disabled_log_pipeline_destination\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"deleted_log_pipeline_destination\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"updated_log_pipeline_destination\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"created_log_pipeline_destination\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"logging_pipeline_enabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"logging_pipeline_disabled\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"osquery_decorator_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"osquery_fim_category_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"discovery_query_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"osquery_pack_query_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"osquery_pack_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"updated_osquery_options\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"reset_osquery_options\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"live_query_created_and_run\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"live_query_updated_and_run\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"live_query_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"live_query_published\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"live_query_unpublished\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"live_query_csv_exported\":{\"category\":[\"configuration\"],\"type\":[\"access\"]},\"live_query_single_result_csv_exported\":{\"category\":[\"configuration\"],\"type\":[\"access\"]},\"vanta_integration_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"vanta_integration_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"oauth_grant_importer_for_google_workspace_integration_created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"oauth_grant_importer_for_google_workspace_integration_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"device_management_provider_added\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"device_management_provider_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"device_management_provider_deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"osquery_blocklist_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"auto_snooze_policy_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"feature_restriction_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"privacy_center_configuration_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"end_user_remediation_configuration_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"extended_device_compliance_configuration_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"billing_email_updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"audit_log\":{\"category\":[\"configuration\"],\"type\":[\"change\"]}}}"
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.id".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            let _cond = { !event.has_value("event.id") && event.has_value("event.original") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.original") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.original".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");
            event.remove("data");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);      "#
                ),
            )?;

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
