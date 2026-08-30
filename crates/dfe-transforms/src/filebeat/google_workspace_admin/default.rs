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

            event.set("ecs.version", json!("8.16.0"))?;

            event.append("event.category", json!("iam"))?;

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

            let _cond = { !event.has_value("json.events") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("event.kind", json!("event"))?;

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                                "yyyy/MM/dd HH:mm z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.events") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "event.action")?;
            }

            if event.has_value("json.id.applicationName") {
                event.rename("json.id.applicationName", "event.provider")?;
            }

            if event.has_value("json.id.uniqueQualifier") {
                event.rename("json.id.uniqueQualifier", "event.id")?;
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "source.user.email")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.user.email").cloned() {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actor.profileId") {
                event.rename("json.actor.profileId", "source.user.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.ipAddress") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.ipAddress".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            if event.has_value("json.id.customerId") {
                event.rename("json.id.customerId", "organization.id")?;
            }

            if event.has_value("json.actor.callerType") {
                event.rename("json.actor.callerType", "google_workspace.actor.type")?;
            }

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.event.type")?;
            }

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                if let Some(v) = event.get("source.user.id").cloned() {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                [
                    "ADMIN_EVENTS_TOGGLE_NEW_APP_FEATURES_PREFERENCE",
                    "ALLOW_SERVICE_FOR_OAUTH2_ACCESS",
                    "ALLOW_STRONG_AUTHENTICATION",
                    "CHANGE_ALLOWED_TWO_STEP_VERIFICATION_METHODS",
                    "CHANGE_API_ACCESS",
                    "CHANGE_APPLICATION_SETTING",
                    "CHANGE_APP_ACCESS_SETTINGS_COLLECTION_ID",
                    "CHANGE_CALENDAR_SETTING",
                    "CHANGE_CHAT_SETTING",
                    "CHANGE_CHROME_OS_ANDROID_APPLICATION_SETTING",
                    "CHANGE_CHROME_OS_APPLICATION_SETTING",
                    "CHANGE_CHROME_OS_DEVICE_SETTING",
                    "CHANGE_CHROME_OS_PUBLIC_SESSION_SETTING",
                    "CHANGE_CHROME_OS_SETTING",
                    "CHANGE_CHROME_OS_USER_SETTING",
                    "CHANGE_CONTACTS_SETTING",
                    "CHANGE_DATA_LOCALIZATION_SETTING",
                    "CHANGE_DOCS_SETTING",
                    "CHANGE_EMAIL_SETTING",
                    "CHANGE_GMAIL_SETTING",
                    "CHANGE_MOBILE_APPLICATION_SETTINGS",
                    "CHANGE_MOBILE_SETTING",
                    "CHANGE_SESSION_LENGTH",
                    "CHANGE_SITES_SETTING",
                    "CHANGE_SITES_WEB_ADDRESS_MAPPING_UPDATES",
                    "CHANGE_SSO_SETTINGS",
                    "CHANGE_TWO_STEP_VERIFICATION_ENROLLMENT_PERIOD_DURATION",
                    "CHANGE_TWO_STEP_VERIFICATION_FREQUENCY",
                    "CHANGE_TWO_STEP_VERIFICATION_GRACE_PERIOD_DURATION",
                    "CHANGE_TWO_STEP_VERIFICATION_START_DATE",
                    "CHANGE_WHITELIST_SETTING",
                    "COMMUNICATION_PREFERENCES_SETTING_CHANGE",
                    "DELETE_APPLICATION_SETTING",
                    "DELETE_GMAIL_SETTING",
                    "DISALLOW_SERVICE_FOR_OAUTH2_ACCESS",
                    "ENABLE_API_ACCESS",
                    "ENABLE_FEEDBACK_SOLICITATION",
                    "ENABLE_NON_ADMIN_USER_PASSWORD_RECOVERY",
                    "ENABLE_SERVICE_OR_FEATURE_NOTIFICATIONS",
                    "ENFORCE_STRONG_AUTHENTICATION",
                    "FLASHLIGHT_EDU_NON_FEATURED_SERVICES_SELECTED",
                    "GPLUS_PREMIUM_FEATURES",
                    "MEET_INTEROP_MODIFY_GATEWAY",
                    "SESSION_CONTROL_SETTINGS_CHANGE",
                    "TOGGLE_ALLOW_ADMIN_PASSWORD_RESET",
                    "TOGGLE_CONTACT_SHARING",
                    "TOGGLE_ENABLE_OAUTH_CONSUMER_KEY",
                    "TOGGLE_NEW_APP_FEATURES",
                    "TOGGLE_OAUTH_ACCESS_TO_ALL_APIS",
                    "TOGGLE_OPEN_ID_ENABLED",
                    "TOGGLE_OUTBOUND_RELAY",
                    "TOGGLE_SSL",
                    "TOGGLE_SSO_ENABLED",
                    "TOGGLE_USE_CUSTOM_LOGO",
                    "TOGGLE_USE_NEXT_GEN_CONTROL_PANEL",
                    "UPDATE_CALENDAR_RESOURCE_FEATURE",
                    "UPDATE_ERROR_MSG_FOR_RESTRICTED_OAUTH2_APPS",
                    "UPDATE_MANAGED_CONFIGURATION",
                    "WEAK_PROGRAMMATIC_LOGIN_SETTINGS_CHANGED",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                [
                    "ADD_APPLICATION",
                    "ADD_APPLICATION_TO_WHITELIST",
                    "ADD_DOMAIN_ALIAS",
                    "ADD_GROUP_MEMBER",
                    "ADD_MOBILE_APPLICATION_TO_WHITELIST",
                    "ADD_MOBILE_CERTIFICATE",
                    "ADD_MOBILE_WIRELESS_NETWORK",
                    "ADD_NICKNAME",
                    "ADD_PRIVILEGE",
                    "ADD_RECOVERY_EMAIL",
                    "ADD_RECOVERY_PHONE",
                    "ADD_SECONDARY_DOMAIN",
                    "ADD_TO_TRUSTED_OAUTH2_APPS",
                    "ADD_TRUSTED_DOMAINS",
                    "ADMIN_EVENTS_TOGGLE_NEW_APP_FEATURES_PREFERENCE",
                    "ALERT_RECEIVERS_CHANGED",
                    "ALERT_STATUS_CHANGED",
                    "ALLOW_SERVICE_FOR_OAUTH2_ACCESS",
                    "ALLOW_STRONG_AUTHENTICATION",
                    "ARCHIVE_USER",
                    "ASSIGN_CUSTOM_LOGO",
                    "ASSIGN_ROLE",
                    "AUTHORIZE_API_CLIENT_ACCESS",
                    "BLOCK_ON_DEVICE_ACCESS",
                    "CANCEL_CALENDAR_EVENTS",
                    "CANCEL_USER_INVITE",
                    "CHANGE_ACCOUNT_AUTO_RENEWAL",
                    "CHANGE_ADMIN_RESTRICTIONS_PIN",
                    "CHANGE_ADVERTISEMENT_OPTION",
                    "CHANGE_ALERT_CRITERIA",
                    "CHANGE_ALLOWED_TWO_STEP_VERIFICATION_METHODS",
                    "CHANGE_API_ACCESS",
                    "CHANGE_APPLICATION_SETTING",
                    "CHANGE_APP_ACCESS",
                    "CHANGE_APP_ACCESS_SETTINGS_COLLECTION_ID",
                    "CHANGE_CAA_APP_ASSIGNMENTS",
                    "CHANGE_CAA_ERROR_MESSAGE",
                    "CHANGE_CALENDAR_SETTING",
                    "CHANGE_CHAT_SETTING",
                    "CHANGE_CHROME_OS_ANDROID_APPLICATION_SETTING",
                    "CHANGE_CHROME_OS_APPLICATION_SETTING",
                    "CHANGE_CHROME_OS_DEVICE_ANNOTATION",
                    "CHANGE_CHROME_OS_DEVICE_SETTING",
                    "CHANGE_CHROME_OS_DEVICE_STATE",
                    "CHANGE_CHROME_OS_PUBLIC_SESSION_SETTING",
                    "CHANGE_CHROME_OS_SETTING",
                    "CHANGE_CHROME_OS_USER_SETTING",
                    "CHANGE_CONFLICT_ACCOUNT_ACTION",
                    "CHANGE_CONTACTS_SETTING",
                    "CHANGE_CUSTOM_LOGO",
                    "CHANGE_DATA_LOCALIZATION_FOR_RUSSIA",
                    "CHANGE_DATA_LOCALIZATION_SETTING",
                    "CHANGE_DATA_PROTECTION_OFFICER_CONTACT_INFO",
                    "CHANGE_DEVICE_STATE",
                    "CHANGE_DOCS_SETTING",
                    "CHANGE_DOMAIN_DEFAULT_LOCALE",
                    "CHANGE_DOMAIN_DEFAULT_TIMEZONE",
                    "CHANGE_DOMAIN_NAME",
                    "CHANGE_DOMAIN_SUPPORT_MESSAGE",
                    "CHANGE_EDU_TYPE",
                    "CHANGE_EMAIL_SETTING",
                    "CHANGE_EU_REPRESENTATIVE_CONTACT_INFO",
                    "CHANGE_FIRST_NAME",
                    "CHANGE_GMAIL_SETTING",
                    "CHANGE_GROUP_DESCRIPTION",
                    "CHANGE_GROUP_NAME",
                    "CHANGE_GROUP_SETTING",
                    "CHANGE_LAST_NAME",
                    "CHANGE_LICENSE_AUTO_ASSIGN",
                    "CHANGE_LOGIN_ACTIVITY_TRACE",
                    "CHANGE_LOGIN_BACKGROUND_COLOR",
                    "CHANGE_LOGIN_BORDER_COLOR",
                    "CHANGE_MOBILE_APPLICATION_PERMISSION_GRANT",
                    "CHANGE_MOBILE_APPLICATION_PRIORITY_ORDER",
                    "CHANGE_MOBILE_APPLICATION_SETTINGS",
                    "CHANGE_MOBILE_SETTING",
                    "CHANGE_MOBILE_WIRELESS_NETWORK",
                    "CHANGE_MOBILE_WIRELESS_NETWORK_PASSWORD",
                    "CHANGE_ORGANIZATION_NAME",
                    "CHANGE_PASSWORD",
                    "CHANGE_PASSWORD_MAX_LENGTH",
                    "CHANGE_PASSWORD_MIN_LENGTH",
                    "CHANGE_PASSWORD_ON_NEXT_LOGIN",
                    "CHANGE_PRIMARY_DOMAIN",
                    "CHANGE_RECOVERY_EMAIL",
                    "CHANGE_RECOVERY_PHONE",
                    "CHANGE_RENEW_DOMAIN_REGISTRATION",
                    "CHANGE_RESELLER_ACCESS",
                    "CHANGE_RULE_CRITERIA",
                    "CHANGE_SESSION_LENGTH",
                    "CHANGE_SITES_SETTING",
                    "CHANGE_SITES_WEB_ADDRESS_MAPPING_UPDATES",
                    "CHANGE_SSO_SETTINGS",
                    "CHANGE_TWO_STEP_VERIFICATION_ENROLLMENT_PERIOD_DURATION",
                    "CHANGE_TWO_STEP_VERIFICATION_FREQUENCY",
                    "CHANGE_TWO_STEP_VERIFICATION_GRACE_PERIOD_DURATION",
                    "CHANGE_TWO_STEP_VERIFICATION_START_DATE",
                    "CHANGE_UNCONFIGURED_APPS_ACCESS",
                    "CHANGE_UNDERAGE_UNCONFIGURED_APPS_ACCESS",
                    "CHANGE_USER_ADDRESS",
                    "CHANGE_USER_CUSTOM_FIELD",
                    "CHANGE_USER_EXTERNAL_ID",
                    "CHANGE_USER_GENDER",
                    "CHANGE_USER_IM",
                    "CHANGE_USER_KEYWORD",
                    "CHANGE_USER_LANGUAGE",
                    "CHANGE_USER_LOCATION",
                    "CHANGE_USER_ORGANIZATION",
                    "CHANGE_USER_PHONE_NUMBER",
                    "CHANGE_USER_RELATION",
                    "CHANGE_WHITELIST_SETTING",
                    "CHROME_APPLICATION_LICENSE_RESERVATION_UPDATED",
                    "CHROME_LICENSES_ALLOWED",
                    "CHROME_LICENSES_ENABLED",
                    "CHROME_LICENSES_REDEEMED",
                    "COMMUNICATION_PREFERENCES_SETTING_CHANGE",
                    "COMPANY_OWNED_DEVICE_BLOCKED",
                    "COMPANY_OWNED_DEVICE_UNBLOCKED",
                    "COMPANY_OWNED_DEVICE_WIPED",
                    "CREATE_APPLICATION_SETTING",
                    "CREATE_GMAIL_SETTING",
                    "DISALLOW_SERVICE_FOR_OAUTH2_ACCESS",
                    "DOWNGRADE_USER_FROM_GPLUS",
                    "DROP_FROM_QUARANTINE",
                    "EDIT_ORG_UNIT_DESCRIPTION",
                    "EDIT_ORG_UNIT_NAME",
                    "ENABLE_API_ACCESS",
                    "ENABLE_FEEDBACK_SOLICITATION",
                    "ENABLE_NON_ADMIN_USER_PASSWORD_RECOVERY",
                    "ENABLE_SERVICE_OR_FEATURE_NOTIFICATIONS",
                    "ENABLE_USER_IP_WHITELIST",
                    "ENFORCE_STRONG_AUTHENTICATION",
                    "FLASHLIGHT_EDU_NON_FEATURED_SERVICES_SELECTED",
                    "GMAIL_RESET_USER",
                    "GPLUS_PREMIUM_FEATURES",
                    "GRANT_ADMIN_PRIVILEGE",
                    "GRANT_DELEGATED_ADMIN_PRIVILEGES",
                    "GROUP_MEMBER_BULK_UPLOAD",
                    "MAIL_ROUTING_DESTINATION_ADDED",
                    "MAIL_ROUTING_DESTINATION_REMOVED",
                    "MEET_INTEROP_MODIFY_GATEWAY",
                    "MOBILE_ACCOUNT_WIPE",
                    "MOBILE_DEVICE_APPROVE",
                    "MOBILE_DEVICE_BLOCK",
                    "MOBILE_DEVICE_CANCEL_WIPE_THEN_APPROVE",
                    "MOBILE_DEVICE_CANCEL_WIPE_THEN_BLOCK",
                    "MOBILE_DEVICE_WIPE",
                    "MOVE_DEVICE_TO_ORG_UNIT_DETAILED",
                    "MOVE_ORG_UNIT",
                    "MOVE_USER_TO_ORG_UNIT",
                    "ORG_ALL_USERS_LICENSE_ASSIGNMENT",
                    "ORG_LICENSE_REVOKE",
                    "ORG_USERS_LICENSE_ASSIGNMENT",
                    "PLAY_FOR_WORK_ENROLL",
                    "PLAY_FOR_WORK_UNENROLL",
                    "REJECT_FROM_QUARANTINE",
                    "RELEASE_CALENDAR_RESOURCES",
                    "RELEASE_FROM_QUARANTINE",
                    "REMOVE_API_CLIENT_ACCESS",
                    "REMOVE_APPLICATION",
                    "REMOVE_APPLICATION_FROM_WHITELIST",
                    "REMOVE_DOMAIN_ALIAS",
                    "REMOVE_FROM_TRUSTED_OAUTH2_APPS",
                    "REMOVE_GROUP_MEMBER",
                    "REMOVE_MOBILE_APPLICATION_FROM_WHITELIST",
                    "REMOVE_MOBILE_CERTIFICATE",
                    "REMOVE_MOBILE_WIRELESS_NETWORK",
                    "REMOVE_NICKNAME",
                    "REMOVE_PRIVILEGE",
                    "REMOVE_RECOVERY_EMAIL",
                    "REMOVE_RECOVERY_PHONE",
                    "REMOVE_SECONDARY_DOMAIN",
                    "REMOVE_TRUSTED_DOMAINS",
                    "RENAME_ALERT",
                    "RENAME_CALENDAR_RESOURCE",
                    "RENAME_ROLE",
                    "RENAME_RULE",
                    "RENAME_USER",
                    "REORDER_GROUP_BASED_POLICIES_EVENT",
                    "RESET_SIGNIN_COOKIES",
                    "REVOKE_3LO_DEVICE_TOKENS",
                    "REVOKE_3LO_TOKEN",
                    "REVOKE_ADMIN_PRIVILEGE",
                    "REVOKE_ASP",
                    "REVOKE_DEVICE_ENROLLMENT_TOKEN",
                    "REVOKE_ENROLLMENT_TOKEN",
                    "REVOKE_SECURITY_KEY",
                    "RULE_ACTIONS_CHANGED",
                    "RULE_STATUS_CHANGED",
                    "SECURITY_KEY_REGISTERED_FOR_USER",
                    "SEND_CHROME_OS_DEVICE_COMMAND",
                    "SESSION_CONTROL_SETTINGS_CHANGE",
                    "SUSPEND_USER",
                    "TOGGLE_ALLOW_ADMIN_PASSWORD_RESET",
                    "TOGGLE_AUTOMATIC_CONTACT_SHARING",
                    "TOGGLE_AUTO_ADD_NEW_SERVICE",
                    "TOGGLE_CAA_ENABLEMENT",
                    "TOGGLE_CONTACT_SHARING",
                    "TOGGLE_ENABLE_OAUTH_CONSUMER_KEY",
                    "TOGGLE_ENABLE_PRE_RELEASE_FEATURES",
                    "TOGGLE_NEW_APP_FEATURES",
                    "TOGGLE_OAUTH_ACCESS_TO_ALL_APIS",
                    "TOGGLE_OPEN_ID_ENABLED",
                    "TOGGLE_OUTBOUND_RELAY",
                    "TOGGLE_SERVICE_ENABLED",
                    "TOGGLE_SSL",
                    "TOGGLE_SSO_ENABLED",
                    "TOGGLE_USE_CUSTOM_LOGO",
                    "TOGGLE_USE_NEXT_GEN_CONTROL_PANEL",
                    "TRANSFER_DOCUMENT_OWNERSHIP",
                    "TRUST_DOMAIN_OWNED_OAUTH2_APPS",
                    "TURN_OFF_2_STEP_VERIFICATION",
                    "UNARCHIVE_USER",
                    "UNASSIGN_CUSTOM_LOGO",
                    "UNASSIGN_ROLE",
                    "UNBLOCK_ON_DEVICE_ACCESS",
                    "UNBLOCK_USER_SESSION",
                    "UNENROLL_USER_FROM_STRONG_AUTH",
                    "UNENROLL_USER_FROM_TITANIUM",
                    "UNSUSPEND_USER",
                    "UNTRUST_DOMAIN_OWNED_OAUTH2_APPS",
                    "UPDATE_BIRTHDATE",
                    "UPDATE_BUILDING",
                    "UPDATE_CALENDAR_RESOURCE",
                    "UPDATE_CALENDAR_RESOURCE_FEATURE",
                    "UPDATE_CHROME_OS_PRINTER",
                    "UPDATE_CHROME_OS_PRINT_SERVER",
                    "UPDATE_DEVICE",
                    "UPDATE_DOMAIN_PRIMARY_ADMIN_EMAIL",
                    "UPDATE_DOMAIN_SECONDARY_EMAIL",
                    "UPDATE_DYNAMIC_LICENSE",
                    "UPDATE_ERROR_MSG_FOR_RESTRICTED_OAUTH2_APPS",
                    "UPDATE_GROUP_MEMBER",
                    "UPDATE_GROUP_MEMBER_DELIVERY_SETTINGS",
                    "UPDATE_GROUP_MEMBER_DELIVERY_SETTINGS_CAN_EMAIL_OVERRIDE",
                    "UPDATE_MANAGED_CONFIGURATION",
                    "UPDATE_ROLE",
                    "UPDATE_RULE",
                    "UPGRADE_USER_TO_GPLUS",
                    "USER_ENROLLED_IN_TWO_STEP_VERIFICATION",
                    "USER_LICENSE_ASSIGNMENT",
                    "USER_LICENSE_REASSIGNMENT",
                    "USER_LICENSE_REVOKE",
                    "USER_PUT_IN_TWO_STEP_VERIFICATION_GRACE_PERIOD",
                    "WEAK_PROGRAMMATIC_LOGIN_SETTINGS_CHANGED",
                    "WHITELISTED_GROUPS_UPDATED",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                [
                    "ACTION_CANCELLED",
                    "ACTION_REQUESTED",
                    "ADD_NICKNAME",
                    "ADD_RECOVERY_EMAIL",
                    "ADD_RECOVERY_PHONE",
                    "ARCHIVE_USER",
                    "BULK_UPLOAD_NOTIFICATION_SENT",
                    "CANCEL_USER_INVITE",
                    "CHANGE_FIRST_NAME",
                    "CHANGE_LAST_NAME",
                    "CHANGE_PASSWORD",
                    "CHANGE_PASSWORD_ON_NEXT_LOGIN",
                    "CHANGE_RECOVERY_EMAIL",
                    "CHANGE_RECOVERY_PHONE",
                    "CHANGE_USER_ADDRESS",
                    "CHANGE_USER_CUSTOM_FIELD",
                    "CHANGE_USER_EXTERNAL_ID",
                    "CHANGE_USER_GENDER",
                    "CHANGE_USER_IM",
                    "CHANGE_USER_KEYWORD",
                    "CHANGE_USER_LANGUAGE",
                    "CHANGE_USER_LOCATION",
                    "CHANGE_USER_ORGANIZATION",
                    "CHANGE_USER_PHONE_NUMBER",
                    "CHANGE_USER_RELATION",
                    "CREATE_DATA_TRANSFER_REQUEST",
                    "CREATE_EMAIL_MONITOR",
                    "CREATE_USER",
                    "DELETE_2SV_SCRATCH_CODES",
                    "DELETE_ACCOUNT_INFO_DUMP",
                    "DELETE_EMAIL_MONITOR",
                    "DELETE_MAILBOX_DUMP",
                    "DELETE_USER",
                    "DOWNGRADE_USER_FROM_GPLUS",
                    "ENABLE_USER_IP_WHITELIST",
                    "GENERATE_2SV_SCRATCH_CODES",
                    "GMAIL_RESET_USER",
                    "GRANT_ADMIN_PRIVILEGE",
                    "GRANT_DELEGATED_ADMIN_PRIVILEGES",
                    "MAIL_ROUTING_DESTINATION_ADDED",
                    "MAIL_ROUTING_DESTINATION_REMOVED",
                    "MOBILE_ACCOUNT_WIPE",
                    "MOBILE_DEVICE_APPROVE",
                    "MOBILE_DEVICE_BLOCK",
                    "MOBILE_DEVICE_CANCEL_WIPE_THEN_APPROVE",
                    "MOBILE_DEVICE_CANCEL_WIPE_THEN_BLOCK",
                    "MOBILE_DEVICE_DELETE",
                    "MOBILE_DEVICE_WIPE",
                    "MOVE_USER_TO_ORG_UNIT",
                    "REMOVE_NICKNAME",
                    "REMOVE_RECOVERY_EMAIL",
                    "REMOVE_RECOVERY_PHONE",
                    "RENAME_USER",
                    "REQUEST_ACCOUNT_INFO",
                    "REQUEST_MAILBOX_DUMP",
                    "RESEND_USER_INVITE",
                    "RESET_SIGNIN_COOKIES",
                    "REVOKE_3LO_DEVICE_TOKENS",
                    "REVOKE_3LO_TOKEN",
                    "REVOKE_ADMIN_PRIVILEGE",
                    "REVOKE_ASP",
                    "REVOKE_SECURITY_KEY",
                    "SECURITY_KEY_REGISTERED_FOR_USER",
                    "SUSPEND_USER",
                    "TOGGLE_AUTOMATIC_CONTACT_SHARING",
                    "TURN_OFF_2_STEP_VERIFICATION",
                    "UNARCHIVE_USER",
                    "UNBLOCK_USER_SESSION",
                    "UNDELETE_USER",
                    "UNENROLL_USER_FROM_STRONG_AUTH",
                    "UNENROLL_USER_FROM_TITANIUM",
                    "UNSUSPEND_USER",
                    "UPDATE_BIRTHDATE",
                    "UPGRADE_USER_TO_GPLUS",
                    "USERS_BULK_UPLOAD_NOTIFICATION_SENT",
                    "USER_ENROLLED_IN_TWO_STEP_VERIFICATION",
                    "USER_INVITE",
                    "USER_PUT_IN_TWO_STEP_VERIFICATION_GRACE_PERIOD",
                    "VIEW_TEMP_PASSWORD",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                [
                    "ADD_WEB_ADDRESS",
                    "CHROME_APPLICATION_LICENSE_RESERVATION_CREATED",
                    "COMPANY_DEVICES_BULK_CREATION",
                    "CREATE_ALERT",
                    "CREATE_APPLICATION_SETTING",
                    "CREATE_BUILDING",
                    "CREATE_CALENDAR_RESOURCE",
                    "CREATE_CALENDAR_RESOURCE_FEATURE",
                    "CREATE_DATA_TRANSFER_REQUEST",
                    "CREATE_DEVICE_ENROLLMENT_TOKEN",
                    "CREATE_EMAIL_MONITOR",
                    "CREATE_ENROLLMENT_TOKEN",
                    "CREATE_GMAIL_SETTING",
                    "CREATE_GROUP",
                    "CREATE_MANAGED_CONFIGURATION",
                    "CREATE_ORG_UNIT",
                    "CREATE_PLAY_FOR_WORK_TOKEN",
                    "CREATE_ROLE",
                    "CREATE_RULE",
                    "CREATE_USER",
                    "EMAIL_UNDELETE",
                    "GENERATE_2SV_SCRATCH_CODES",
                    "GENERATE_PIN",
                    "GENERATE_TRANSFER_TOKEN",
                    "INSERT_CHROME_OS_PRINTER",
                    "INSERT_CHROME_OS_PRINT_SERVER",
                    "MEET_INTEROP_CREATE_GATEWAY",
                    "REGENERATE_OAUTH_CONSUMER_SECRET",
                    "UNDELETE_USER",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                [
                    "CHROME_APPLICATION_LICENSE_RESERVATION_DELETED",
                    "COMPANY_DEVICE_DELETION",
                    "DELETE_2SV_SCRATCH_CODES",
                    "DELETE_ACCOUNT_INFO_DUMP",
                    "DELETE_ALERT",
                    "DELETE_APPLICATION_SETTING",
                    "DELETE_BUILDING",
                    "DELETE_CALENDAR_RESOURCE",
                    "DELETE_CALENDAR_RESOURCE_FEATURE",
                    "DELETE_CHROME_OS_PRINTER",
                    "DELETE_CHROME_OS_PRINT_SERVER",
                    "DELETE_EMAIL_MONITOR",
                    "DELETE_GMAIL_SETTING",
                    "DELETE_GROUP",
                    "DELETE_MAILBOX_DUMP",
                    "DELETE_MANAGED_CONFIGURATION",
                    "DELETE_PLAY_FOR_WORK_TOKEN",
                    "DELETE_ROLE",
                    "DELETE_RULE",
                    "DELETE_USER",
                    "DELETE_WEB_ADDRESS",
                    "MEET_INTEROP_DELETE_GATEWAY",
                    "MOBILE_DEVICE_DELETE",
                    "REMOVE_CHROME_OS_APPLICATION_SETTINGS",
                    "REMOVE_ORG_UNIT",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                [
                    "ADD_GROUP_MEMBER",
                    "CHANGE_GROUP_DESCRIPTION",
                    "CHANGE_GROUP_NAME",
                    "CHANGE_GROUP_SETTING",
                    "GROUP_LIST_DOWNLOAD",
                    "GROUP_MEMBERS_DOWNLOAD",
                    "GROUP_MEMBER_BULK_UPLOAD",
                    "REMOVE_GROUP_MEMBER",
                    "REORDER_GROUP_BASED_POLICIES_EVENT",
                    "UPDATE_GROUP_MEMBER",
                    "UPDATE_GROUP_MEMBER_DELIVERY_SETTINGS",
                    "UPDATE_GROUP_MEMBER_DELIVERY_SETTINGS_CAN_EMAIL_OVERRIDE",
                    "WHITELISTED_GROUPS_UPDATED",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                [
                    "ACTION_CANCELLED",
                    "ACTION_REQUESTED",
                    "BULK_UPLOAD",
                    "BULK_UPLOAD_NOTIFICATION_SENT",
                    "DOWNLOAD_PENDING_INVITES_LIST",
                    "DOWNLOAD_USERLIST_CSV",
                    "DRIVE_DATA_RESTORE",
                    "EMAIL_LOG_SEARCH",
                    "ENROLL_FOR_GOOGLE_DEVICE_MANAGEMENT",
                    "GROUP_LIST_DOWNLOAD",
                    "GROUP_MEMBERS_DOWNLOAD",
                    "ISSUE_DEVICE_COMMAND",
                    "MX_RECORD_VERIFICATION_CLAIM",
                    "REQUEST_ACCOUNT_INFO",
                    "REQUEST_MAILBOX_DUMP",
                    "RESEND_USER_INVITE",
                    "SKIP_DOMAIN_ALIAS_MX",
                    "SKIP_SECONDARY_DOMAIN_MX",
                    "UPLOAD_OAUTH_CERTIFICATE",
                    "USERS_BULK_UPLOAD",
                    "USERS_BULK_UPLOAD_NOTIFICATION_SENT",
                    "USER_INVITE",
                    "USE_GOOGLE_MOBILE_MANAGEMENT",
                    "USE_GOOGLE_MOBILE_MANAGEMENT_FOR_IOS",
                    "USE_GOOGLE_MOBILE_MANAGEMENT_FOR_NON_IOS",
                    "VERIFY_DOMAIN_ALIAS",
                    "VERIFY_DOMAIN_ALIAS_MX",
                    "VERIFY_SECONDARY_DOMAIN",
                    "VERIFY_SECONDARY_DOMAIN_MX",
                    "VIEW_DNS_LOGIN_DETAILS",
                    "VIEW_SITE_DETAILS",
                    "VIEW_TEMP_PASSWORD",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("json.events.parameters")
                    && event
                        .get("json.events.parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.google_workspace.admin == null) {\n  ctx.google_workspace.admin = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx.json.events.parameters[i].value != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].value;\n  }\n  if (ctx.json.events.parameters[i].intValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].intValue;\n  }\n  if (ctx.json.events.parameters[i].multiValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].multiValue;\n  }\n  if (ctx.json.events.parameters[i].messageValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].messageValue;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.google_workspace.admin == null) {\n  ctx.google_workspace.admin = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx.json.events.parameters[i].value != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].value;\n  }\n  if (ctx.json.events.parameters[i].intValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].intValue;\n  }\n  if (ctx.json.events.parameters[i].multiValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].multiValue;\n  }\n  if (ctx.json.events.parameters[i].messageValue != null) {\n    ctx.google_workspace.admin[ctx.json.events.parameters[i].name] = ctx.json.events.parameters[i].messageValue;\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_workspace.admin.SETTING_METADATA.parameter")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: for (int i = 0; i < ctx.google_workspace.admin.SETTING_METADATA.parameter.length; ++i) {\n  def value = ctx.google_workspace.admin.SETTING_METADATA.parameter[i].value;\n  if (value != null) {\n    ctx.google_workspace.admin.SETTING_METADATA[ctx.google_workspace.admin.SETTING_METADATA.parameter[i].name] = value;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (int i = 0; i < ctx.google_workspace.admin.SETTING_METADATA.parameter.length; ++i) {\n  def value = ctx.google_workspace.admin.SETTING_METADATA.parameter[i].value;\n  if (value != null) {\n    ctx.google_workspace.admin.SETTING_METADATA[ctx.google_workspace.admin.SETTING_METADATA.parameter[i].name] = value;\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_setting_metadata",
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

            if event.has_value("google_workspace.admin.SETTING_METADATA.DESCRIPTION") {
                event.rename(
                    "google_workspace.admin.SETTING_METADATA.DESCRIPTION",
                    "google_workspace.admin.setting.metadata.description",
                )?;
            }

            if event.has_value("google_workspace.admin.SETTING_METADATA.rule_key") {
                event.rename(
                    "google_workspace.admin.SETTING_METADATA.rule_key",
                    "google_workspace.admin.setting.metadata.rule.key",
                )?;
            }

            if event.has_value("google_workspace.admin.SETTING_METADATA.rule_type") {
                event.rename(
                    "google_workspace.admin.SETTING_METADATA.rule_type",
                    "google_workspace.admin.setting.metadata.rule.type",
                )?;
            }

            if event.has_value("google_workspace.admin.SETTING_METADATA.USER_DEFINED_NAME") {
                event.rename(
                    "google_workspace.admin.SETTING_METADATA.USER_DEFINED_NAME",
                    "google_workspace.admin.setting.metadata.user_defined.name",
                )?;
            }

            event.remove("json.events.parameters");

            if event.has_value("google_workspace.admin.APPLICATION_EDITION") {
                event.rename(
                    "google_workspace.admin.APPLICATION_EDITION",
                    "google_workspace.admin.application.edition",
                )?;
            }

            if event.has_value("google_workspace.admin.APPLICATION_NAME") {
                event.rename(
                    "google_workspace.admin.APPLICATION_NAME",
                    "google_workspace.admin.application.name",
                )?;
            }

            if event.has_value("google_workspace.admin.APPLICATION_ENABLED") {
                event.rename(
                    "google_workspace.admin.APPLICATION_ENABLED",
                    "google_workspace.admin.application.enabled",
                )?;
            }

            if event.has_value("google_workspace.admin.APP_LICENSES_ORDER_NUMBER") {
                event.rename(
                    "google_workspace.admin.APP_LICENSES_ORDER_NUMBER",
                    "google_workspace.admin.application.licences_order_number",
                )?;
            }

            if event.has_value("google_workspace.admin.CHROME_NUM_LICENSES_PURCHASED") {
                event.rename(
                    "google_workspace.admin.CHROME_NUM_LICENSES_PURCHASED",
                    "google_workspace.admin.application.licences_purchased",
                )?;
            }

            if event.has_value("google_workspace.admin.REAUTH_APPLICATION") {
                event.rename(
                    "google_workspace.admin.REAUTH_APPLICATION",
                    "google_workspace.admin.application.name",
                )?;
            }

            if event.has_value("google_workspace.admin.GROUP_EMAIL") {
                event.rename(
                    "google_workspace.admin.GROUP_EMAIL",
                    "google_workspace.admin.group.email",
                )?;
            }

            if event.has_value("google_workspace.admin.GROUP_NAME") {
                event.rename("google_workspace.admin.GROUP_NAME", "group.name")?;
            }

            if event.has_value("google_workspace.admin.NEW_VALUE") {
                event.rename(
                    "google_workspace.admin.NEW_VALUE",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.OLD_VALUE") {
                event.rename(
                    "google_workspace.admin.OLD_VALUE",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.ORG_UNIT_NAME") {
                event.rename(
                    "google_workspace.admin.ORG_UNIT_NAME",
                    "google_workspace.admin.org_unit.name",
                )?;
            }

            if event.has_value("google_workspace.admin.SETTING_NAME") {
                event.rename(
                    "google_workspace.admin.SETTING_NAME",
                    "google_workspace.admin.setting.name",
                )?;
            }

            if event.has_value("google_workspace.admin.SETTING_DESCRIPTION") {
                event.rename(
                    "google_workspace.admin.SETTING_DESCRIPTION",
                    "google_workspace.admin.setting.description",
                )?;
            }

            if event.has_value("google_workspace.admin.GROUP_PRIORITIES") {
                event.rename(
                    "google_workspace.admin.GROUP_PRIORITIES",
                    "google_workspace.admin.group.priorities",
                )?;
            }

            if event.has_value("google_workspace.admin.DOMAIN_NAME") {
                event.rename(
                    "google_workspace.admin.DOMAIN_NAME",
                    "google_workspace.admin.domain.name",
                )?;
            }

            if event.has_value("google_workspace.admin.DOMAIN_ALIAS") {
                event.rename(
                    "google_workspace.admin.DOMAIN_ALIAS",
                    "google_workspace.admin.domain.alias",
                )?;
            }

            if event.has_value("google_workspace.admin.SECONDARY_DOMAIN_NAME") {
                event.rename(
                    "google_workspace.admin.SECONDARY_DOMAIN_NAME",
                    "google_workspace.admin.domain.secondary_name",
                )?;
            }

            if event.has_value("google_workspace.admin.MANAGED_CONFIGURATION_NAME") {
                event.rename(
                    "google_workspace.admin.MANAGED_CONFIGURATION_NAME",
                    "google_workspace.admin.managed_configuration",
                )?;
            }

            if event.has_value("google_workspace.admin.MOBILE_APP_PACKAGE_ID") {
                event.rename(
                    "google_workspace.admin.MOBILE_APP_PACKAGE_ID",
                    "google_workspace.admin.application.package_id",
                )?;
            }

            if event
                .has_value("google_workspace.admin.FLASHLIGHT_EDU_NON_FEATURED_SERVICES_SELECTION")
            {
                event.rename(
                    "google_workspace.admin.FLASHLIGHT_EDU_NON_FEATURED_SERVICES_SELECTION",
                    "google_workspace.admin.non_featured_services_selection",
                )?;
            }

            if event.has_value("google_workspace.admin.FIELD_NAME") {
                event.rename(
                    "google_workspace.admin.FIELD_NAME",
                    "google_workspace.admin.field",
                )?;
            }

            if event.has_value("google_workspace.admin.RESOURCE_IDENTIFIER") {
                event.rename(
                    "google_workspace.admin.RESOURCE_IDENTIFIER",
                    "google_workspace.admin.resource.id",
                )?;
            }

            if event.has_value("google_workspace.admin.USER_EMAIL") {
                event.rename(
                    "google_workspace.admin.USER_EMAIL",
                    "google_workspace.admin.user.email",
                )?;
            }

            if event.has_value("google_workspace.admin.GATEWAY_NAME") {
                event.rename(
                    "google_workspace.admin.GATEWAY_NAME",
                    "google_workspace.admin.gateway.name",
                )?;
            }

            if event.has_value("google_workspace.admin.APP_ID") {
                event.rename(
                    "google_workspace.admin.APP_ID",
                    "google_workspace.admin.application.id",
                )?;
            }

            if event.has_value("google_workspace.admin.ASP_ID") {
                event.rename(
                    "google_workspace.admin.ASP_ID",
                    "google_workspace.admin.application.asp_id",
                )?;
            }

            if event.has_value("google_workspace.admin.CHROME_OS_SESSION_TYPE") {
                event.rename(
                    "google_workspace.admin.CHROME_OS_SESSION_TYPE",
                    "google_workspace.admin.chrome_os.session_type",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_NEW_STATE") {
                event.rename(
                    "google_workspace.admin.DEVICE_NEW_STATE",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_PREVIOUS_STATE") {
                event.rename(
                    "google_workspace.admin.DEVICE_PREVIOUS_STATE",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_SERIAL_NUMBER") {
                event.rename(
                    "google_workspace.admin.DEVICE_SERIAL_NUMBER",
                    "google_workspace.admin.device.serial_number",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_ID") {
                event.rename(
                    "google_workspace.admin.DEVICE_ID",
                    "google_workspace.admin.device.id",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_TYPE") {
                event.rename(
                    "google_workspace.admin.DEVICE_TYPE",
                    "google_workspace.admin.device.type",
                )?;
            }

            if event.has_value("google_workspace.admin.PRINT_SERVER_NAME") {
                event.rename(
                    "google_workspace.admin.PRINT_SERVER_NAME",
                    "google_workspace.admin.print_server.name",
                )?;
            }

            if event.has_value("google_workspace.admin.PRINTER_NAME") {
                event.rename(
                    "google_workspace.admin.PRINTER_NAME",
                    "google_workspace.admin.printer.name",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_COMMAND_DETAILS") {
                event.rename(
                    "google_workspace.admin.DEVICE_COMMAND_DETAILS",
                    "google_workspace.admin.device.command_details",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_NEW_ORG_UNIT") {
                event.rename(
                    "google_workspace.admin.DEVICE_NEW_ORG_UNIT",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.DEVICE_PREVIOUS_ORG_UNIT") {
                event.rename(
                    "google_workspace.admin.DEVICE_PREVIOUS_ORG_UNIT",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.ROLE_NAME") {
                event.rename(
                    "google_workspace.admin.ROLE_NAME",
                    "google_workspace.admin.role.name",
                )?;
            }

            if event.has_value("google_workspace.admin.ROLE_ID") {
                event.rename(
                    "google_workspace.admin.ROLE_ID",
                    "google_workspace.admin.role.id",
                )?;
            }

            if event.has_value("google_workspace.admin.PRIVILEGE_NAME") {
                event.rename(
                    "google_workspace.admin.PRIVILEGE_NAME",
                    "google_workspace.admin.privilege.name",
                )?;
            }

            if event.has_value("google_workspace.admin.SITE_LOCATION") {
                event.rename("google_workspace.admin.SITE_LOCATION", "url.path")?;
            }

            if event.has_value("google_workspace.admin.WEB_ADDRESS") {
                event.rename("google_workspace.admin.WEB_ADDRESS", "url.full")?;
            }

            let _cond = { event.has_value("url.full") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.full", "url", true, false)?;
                    Ok(())
                })();
            }

            if event.has_value("google_workspace.admin.SITE_NAME") {
                event.rename(
                    "google_workspace.admin.SITE_NAME",
                    "google_workspace.admin.url.name",
                )?;
            }

            if event.has_value("google_workspace.admin.SERVICE_NAME") {
                event.rename(
                    "google_workspace.admin.SERVICE_NAME",
                    "google_workspace.admin.service.name",
                )?;
            }

            if event.has_value("google_workspace.admin.PRODUCT_NAME") {
                event.rename(
                    "google_workspace.admin.PRODUCT_NAME",
                    "google_workspace.admin.product.name",
                )?;
            }

            if event.has_value("google_workspace.admin.SKU_NAME") {
                event.rename(
                    "google_workspace.admin.SKU_NAME",
                    "google_workspace.admin.product.sku",
                )?;
            }

            if event.has_value("google_workspace.admin.GROUP_MEMBER_BULK_UPLOAD_FAILED_NUMBER") {
                event.rename(
                    "google_workspace.admin.GROUP_MEMBER_BULK_UPLOAD_FAILED_NUMBER",
                    "google_workspace.admin.bulk_upload.failed",
                )?;
            }

            if event.has_value("google_workspace.admin.GROUP_MEMBER_BULK_UPLOAD_TOTAL_NUMBER") {
                event.rename(
                    "google_workspace.admin.GROUP_MEMBER_BULK_UPLOAD_TOTAL_NUMBER",
                    "google_workspace.admin.bulk_upload.total",
                )?;
            }

            if event.has_value("google_workspace.admin.BULK_UPLOAD_FAIL_USERS_NUMBER") {
                event.rename(
                    "google_workspace.admin.BULK_UPLOAD_FAIL_USERS_NUMBER",
                    "google_workspace.admin.bulk_upload.failed",
                )?;
            }

            if event.has_value("google_workspace.admin.BULK_UPLOAD_TOTAL_USERS_NUMBER") {
                event.rename(
                    "google_workspace.admin.BULK_UPLOAD_TOTAL_USERS_NUMBER",
                    "google_workspace.admin.bulk_upload.total",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_MSG_ID") {
                event.rename(
                    "google_workspace.admin.EMAIL_LOG_SEARCH_MSG_ID",
                    "google_workspace.admin.email.log_search_filter.message_id",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_RECIPIENT") {
                event.rename(
                    "google_workspace.admin.EMAIL_LOG_SEARCH_RECIPIENT",
                    "google_workspace.admin.email.log_search_filter.recipient.value",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_SENDER") {
                event.rename(
                    "google_workspace.admin.EMAIL_LOG_SEARCH_SENDER",
                    "google_workspace.admin.email.log_search_filter.sender.value",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP") {
                    if let Some(val) =
                        event.get("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP") {
                event.rename(
                    "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_RECIPIENT_IP",
                    "google_workspace.admin.email.log_search_filter.recipient.ip",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP") {
                    if let Some(val) =
                        event.get("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP") {
                event.rename(
                    "google_workspace.admin.EMAIL_LOG_SEARCH_SMTP_SENDER_IP",
                    "google_workspace.admin.email.log_search_filter.sender.ip",
                )?;
            }

            if event.has_value("google_workspace.admin.QUARANTINE_NAME") {
                event.rename(
                    "google_workspace.admin.QUARANTINE_NAME",
                    "google_workspace.admin.email.quarantine_name",
                )?;
            }

            if event.has_value("google_workspace.admin.CHROME_LICENSES_ENABLED") {
                event.rename(
                    "google_workspace.admin.CHROME_LICENSES_ENABLED",
                    "google_workspace.admin.chrome_licenses.enabled",
                )?;
            }

            if event.has_value("google_workspace.admin.CHROME_LICENSES_ALLOWED") {
                event.rename(
                    "google_workspace.admin.CHROME_LICENSES_ALLOWED",
                    "google_workspace.admin.chrome_licenses.allowed",
                )?;
            }

            if event.has_value("google_workspace.admin.FULL_ORG_UNIT_PATH") {
                event.rename(
                    "google_workspace.admin.FULL_ORG_UNIT_PATH",
                    "google_workspace.admin.org_unit.full",
                )?;
            }

            if event.has_value("google_workspace.admin.OAUTH2_SERVICE_NAME") {
                event.rename(
                    "google_workspace.admin.OAUTH2_SERVICE_NAME",
                    "google_workspace.admin.oauth2.service.name",
                )?;
            }

            if event.has_value("google_workspace.admin.OAUTH2_APP_ID") {
                event.rename(
                    "google_workspace.admin.OAUTH2_APP_ID",
                    "google_workspace.admin.oauth2.application.id",
                )?;
            }

            if event.has_value("google_workspace.admin.OAUTH2_APP_NAME") {
                event.rename(
                    "google_workspace.admin.OAUTH2_APP_NAME",
                    "google_workspace.admin.oauth2.application.name",
                )?;
            }

            if event.has_value("google_workspace.admin.OAUTH2_APP_TYPE") {
                event.rename(
                    "google_workspace.admin.OAUTH2_APP_TYPE",
                    "google_workspace.admin.oauth2.application.type",
                )?;
            }

            if event.has_value("google_workspace.admin.ALLOWED_TWO_STEP_VERIFICATION_METHOD") {
                event.rename(
                    "google_workspace.admin.ALLOWED_TWO_STEP_VERIFICATION_METHOD",
                    "google_workspace.admin.verification_method",
                )?;
            }

            if event.has_value("google_workspace.admin.DOMAIN_VERIFICATION_METHOD") {
                event.rename(
                    "google_workspace.admin.DOMAIN_VERIFICATION_METHOD",
                    "google_workspace.admin.verification_method",
                )?;
            }

            if event.has_value("google_workspace.admin.CAA_ASSIGNMENTS_NEW") {
                event.rename(
                    "google_workspace.admin.CAA_ASSIGNMENTS_NEW",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.CAA_ASSIGNMENTS_OLD") {
                event.rename(
                    "google_workspace.admin.CAA_ASSIGNMENTS_OLD",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.REAUTH_SETTING_NEW") {
                event.rename(
                    "google_workspace.admin.REAUTH_SETTING_NEW",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.REAUTH_SETTING_OLD") {
                event.rename(
                    "google_workspace.admin.REAUTH_SETTING_OLD",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.ALERT_ID") {
                event.rename(
                    "google_workspace.admin.ALERT_ID",
                    "google_workspace.admin.alert.id",
                )?;
            }

            let _cond = { event.get_str("google_workspace.admin.RELATED_ALERT_ID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_workspace.admin.RELATED_ALERT_ID") {
                        if let Some(s) = event.get_string("google_workspace.admin.RELATED_ALERT_ID")
                        {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set(
                                "google_workspace.admin.alert.related_id",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            if event.has_value("google_workspace.admin.CHART_FILTERS") {
                event.rename(
                    "google_workspace.admin.CHART_FILTERS",
                    "google_workspace.admin.chart.filters",
                )?;
            }

            if event.has_value("google_workspace.admin.CHART_NAME") {
                event.rename(
                    "google_workspace.admin.CHART_NAME",
                    "google_workspace.admin.chart.name",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_ACTION") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_ACTION",
                    "google_workspace.admin.investigation.action",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_DATA_SOURCE") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_DATA_SOURCE",
                    "google_workspace.admin.investigation.data_source",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_ENTITY_IDS") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_ENTITY_IDS",
                    "google_workspace.admin.investigation.entity_ids",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_OBJECT_IDENTIFIER") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_OBJECT_IDENTIFIER",
                    "google_workspace.admin.investigation.object_identifier",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_QUERY") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_QUERY",
                    "google_workspace.admin.investigation.query",
                )?;
            }

            if event.has_value("google_workspace.admin.INVESTIGATION_URL_DISPLAY_TEXT") {
                event.rename(
                    "google_workspace.admin.INVESTIGATION_URL_DISPLAY_TEXT",
                    "google_workspace.admin.investigation.url_display_text",
                )?;
            }

            if event.has_value("google_workspace.admin.ALERT_NAME") {
                event.rename(
                    "google_workspace.admin.ALERT_NAME",
                    "google_workspace.admin.alert.name",
                )?;
            }

            if event.has_value("google_workspace.admin.API_CLIENT_NAME") {
                event.rename(
                    "google_workspace.admin.API_CLIENT_NAME",
                    "google_workspace.admin.api.client.name",
                )?;
            }

            if event.has_value("google_workspace.admin.API_SCOPES") {
                event.rename(
                    "google_workspace.admin.API_SCOPES",
                    "google_workspace.admin.api.scopes",
                )?;
            }

            if event.has_value("google_workspace.admin.PLAY_FOR_WORK_TOKEN_ID") {
                event.rename(
                    "google_workspace.admin.PLAY_FOR_WORK_TOKEN_ID",
                    "google_workspace.admin.mdm.token",
                )?;
            }

            if event.has_value("google_workspace.admin.PLAY_FOR_WORK_MDM_VENDOR_NAME") {
                event.rename(
                    "google_workspace.admin.PLAY_FOR_WORK_MDM_VENDOR_NAME",
                    "google_workspace.admin.mdm.vendor",
                )?;
            }

            if event.has_value("google_workspace.admin.INFO_TYPE") {
                event.rename(
                    "google_workspace.admin.INFO_TYPE",
                    "google_workspace.admin.info_type",
                )?;
            }

            if event.has_value("google_workspace.admin.RULE_NAME") {
                event.rename(
                    "google_workspace.admin.RULE_NAME",
                    "google_workspace.admin.rule.name",
                )?;
            }

            if event.has_value("google_workspace.admin.USER_CUSTOM_FIELD") {
                event.rename(
                    "google_workspace.admin.USER_CUSTOM_FIELD",
                    "google_workspace.admin.setting.name",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_MONITOR_DEST_EMAIL") {
                event.rename(
                    "google_workspace.admin.EMAIL_MONITOR_DEST_EMAIL",
                    "google_workspace.admin.email_monitor.dest_email",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_MONITOR_LEVEL_CHAT") {
                event.rename(
                    "google_workspace.admin.EMAIL_MONITOR_LEVEL_CHAT",
                    "google_workspace.admin.email_monitor.level.chat",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_MONITOR_LEVEL_DRAFT_EMAIL") {
                event.rename(
                    "google_workspace.admin.EMAIL_MONITOR_LEVEL_DRAFT_EMAIL",
                    "google_workspace.admin.email_monitor.level.draft",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_MONITOR_LEVEL_INCOMING_EMAIL") {
                event.rename(
                    "google_workspace.admin.EMAIL_MONITOR_LEVEL_INCOMING_EMAIL",
                    "google_workspace.admin.email_monitor.level.incoming",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_MONITOR_LEVEL_OUTGOING_EMAIL") {
                event.rename(
                    "google_workspace.admin.EMAIL_MONITOR_LEVEL_OUTGOING_EMAIL",
                    "google_workspace.admin.email_monitor.level.outgoing",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_EXPORT_INCLUDE_DELETED") {
                event.rename(
                    "google_workspace.admin.EMAIL_EXPORT_INCLUDE_DELETED",
                    "google_workspace.admin.email_dump.include_deleted",
                )?;
            }

            if event.has_value("google_workspace.admin.EMAIL_EXPORT_PACKAGE_CONTENT") {
                event.rename(
                    "google_workspace.admin.EMAIL_EXPORT_PACKAGE_CONTENT",
                    "google_workspace.admin.email_dump.package_content",
                )?;
            }

            if event.has_value("google_workspace.admin.SEARCH_QUERY_FOR_DUMP") {
                event.rename(
                    "google_workspace.admin.SEARCH_QUERY_FOR_DUMP",
                    "google_workspace.admin.email_dump.query",
                )?;
            }

            if event.has_value("google_workspace.admin.DESTINATION_USER_EMAIL") {
                event.rename(
                    "google_workspace.admin.DESTINATION_USER_EMAIL",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.REQUEST_ID") {
                event.rename(
                    "google_workspace.admin.REQUEST_ID",
                    "google_workspace.admin.request.id",
                )?;
            }

            if event.has_value("google_workspace.admin.GMAIL_RESET_REASON") {
                event.rename("google_workspace.admin.GMAIL_RESET_REASON", "message")?;
            }

            if event.has_value("google_workspace.admin.USER_NICKNAME") {
                event.rename(
                    "google_workspace.admin.USER_NICKNAME",
                    "google_workspace.admin.user.nickname",
                )?;
            }

            if event.has_value("google_workspace.admin.ACTION_ID") {
                event.rename(
                    "google_workspace.admin.ACTION_ID",
                    "google_workspace.admin.mobile.action.id",
                )?;
            }

            if event.has_value("google_workspace.admin.ACTION_TYPE") {
                event.rename(
                    "google_workspace.admin.ACTION_TYPE",
                    "google_workspace.admin.mobile.action.type",
                )?;
            }

            if event.has_value("google_workspace.admin.MOBILE_CERTIFICATE_COMMON_NAME") {
                event.rename(
                    "google_workspace.admin.MOBILE_CERTIFICATE_COMMON_NAME",
                    "google_workspace.admin.mobile.certificate.name",
                )?;
            }

            if event.has_value("google_workspace.admin.NUMBER_OF_COMPANY_OWNED_DEVICES") {
                event.rename(
                    "google_workspace.admin.NUMBER_OF_COMPANY_OWNED_DEVICES",
                    "google_workspace.admin.mobile.company_owned_devices",
                )?;
            }

            if event.has_value("google_workspace.admin.COMPANY_DEVICE_ID") {
                event.rename(
                    "google_workspace.admin.COMPANY_DEVICE_ID",
                    "google_workspace.admin.device.id",
                )?;
            }

            if event.has_value("google_workspace.admin.DISTRIBUTION_ENTITY_NAME") {
                event.rename(
                    "google_workspace.admin.DISTRIBUTION_ENTITY_NAME",
                    "google_workspace.admin.distribution.entity.name",
                )?;
            }

            if event.has_value("google_workspace.admin.DISTRIBUTION_ENTITY_TYPE") {
                event.rename(
                    "google_workspace.admin.DISTRIBUTION_ENTITY_TYPE",
                    "google_workspace.admin.distribution.entity.type",
                )?;
            }

            if event.has_value("google_workspace.admin.MOBILE_APP_PACKAGE_ID") {
                event.rename(
                    "google_workspace.admin.MOBILE_APP_PACKAGE_ID",
                    "google_workspace.admin.application.package_id",
                )?;
            }

            if event.has_value("google_workspace.admin.NEW_PERMISSION_GRANT_STATE") {
                event.rename(
                    "google_workspace.admin.NEW_PERMISSION_GRANT_STATE",
                    "google_workspace.admin.new_value",
                )?;
            }

            if event.has_value("google_workspace.admin.OLD_PERMISSION_GRANT_STATE") {
                event.rename(
                    "google_workspace.admin.OLD_PERMISSION_GRANT_STATE",
                    "google_workspace.admin.old_value",
                )?;
            }

            if event.has_value("google_workspace.admin.PERMISSION_GROUP_NAME") {
                event.rename(
                    "google_workspace.admin.PERMISSION_GROUP_NAME",
                    "google_workspace.admin.setting.name",
                )?;
            }

            if event.has_value("google_workspace.admin.MOBILE_WIRELESS_NETWORK_NAME") {
                event.rename(
                    "google_workspace.admin.MOBILE_WIRELESS_NETWORK_NAME",
                    "network.name",
                )?;
            }

            let _cond = { event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_END_DATE") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("google_workspace.admin.EMAIL_LOG_SEARCH_END_DATE")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "google_workspace.admin.email.log_search_filter.end_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.EMAIL_LOG_SEARCH_END_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.EMAIL_LOG_SEARCH_START_DATE") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("google_workspace.admin.EMAIL_LOG_SEARCH_START_DATE")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "google_workspace.admin.email.log_search_filter.start_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.EMAIL_LOG_SEARCH_START_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.BIRTHDATE") };
            if _cond {
                if let Some(date_str) = event.get_as_string("google_workspace.admin.BIRTHDATE") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("google_workspace.admin.user.birthdate", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.BIRTHDATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.BEGIN_DATE_TIME") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("google_workspace.admin.BEGIN_DATE_TIME")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.BEGIN_DATE_TIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.START_DATE") };
            if _cond {
                if let Some(date_str) = event.get_as_string("google_workspace.admin.START_DATE") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.START_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.END_DATE") };
            if _cond {
                if let Some(date_str) = event.get_as_string("google_workspace.admin.END_DATE") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.END_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("google_workspace.admin.END_DATE_TIME") };
            if _cond {
                if let Some(date_str) = event.get_as_string("google_workspace.admin.END_DATE_TIME")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'HH:mm:ss",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                            "yyyy/MM/dd HH:mm:ss z",
                            "yyyy/MM/dd HH:mm z",
                        ],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_workspace.admin.END_DATE_TIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.admin.group.email")
                    && event
                        .get("google_workspace.admin.group.email")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.google_workspace.admin.group.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.group == null) {\n  ctx.group = new HashMap();\n} ctx.group.name = splitmail[0]; ctx.group.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.google_workspace.admin.group.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.group == null) {\n  ctx.group = new HashMap();\n} ctx.group.name = splitmail[0]; ctx.group.domain = splitmail[1];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.admin.user.email")
                    && event
                        .get("google_workspace.admin.user.email")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.google_workspace.admin.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.user.target.email = ctx.google_workspace.admin.user.email;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.google_workspace.admin.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.user.target.email = ctx.google_workspace.admin.user.email;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("group.name") };
            if _cond {
                if let Some(v) = event.get("group.name").cloned() {
                    event.set("user.target.group.name", v)?;
                }
            }

            let _cond = { event.has_value("group.domain") };
            if _cond {
                if let Some(v) = event.get("group.domain").cloned() {
                    event.set("user.target.group.domain", v)?;
                }
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#
                    ),
                )?;
            }

            if event.has_value("google_workspace.admin.bulk_upload.total") {
                if let Some(val) = event.get("google_workspace.admin.bulk_upload.total") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "google_workspace.admin.bulk_upload.total".into(),
                            message,
                        }
                    })?;
                    event.set("google_workspace.admin.bulk_upload.total", converted)?;
                }
            }

            if event.has_value("google_workspace.admin.bulk_upload.failed") {
                if let Some(val) = event.get("google_workspace.admin.bulk_upload.failed") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "google_workspace.admin.bulk_upload.failed".into(),
                            message,
                        }
                    })?;
                    event.set("google_workspace.admin.bulk_upload.failed", converted)?;
                }
            }

            let _cond = {
                event.has_value("google_workspace.admin.group.bulk_upload.failed")
                    && event.get_i64("google_workspace.admin.group.bulk_upload.failed") == Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("google_workspace.admin.group.bulk_upload.failed")
                    && event.get_i64("google_workspace.admin.group.bulk_upload.failed") != Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("google_workspace.admin.WHITELISTED_GROUPS") {
                if let Some(s) = event.get_string("google_workspace.admin.WHITELISTED_GROUPS") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set(
                        "google_workspace.admin.group.allowed_list",
                        Value::Array(parts),
                    )?;
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

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("event.id") {
                if let Some(val) = event.get("event.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            if event.has_value("source.user.id") {
                if let Some(val) = event.get("source.user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.user.id".into(),
                            message,
                        }
                    })?;
                    event.set("source.user.id", converted)?;
                }
            }

            if event.has_value("user.id") {
                if let Some(val) = event.get("user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "user.id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            event.remove("json");
            event.remove("google_workspace.admin.EMAIL_LOG_SEARCH_END_DATE");
            event.remove("google_workspace.admin.EMAIL_LOG_SEARCH_START_DATE");
            event.remove("google_workspace.admin.BIRTHDATE");
            event.remove("google_workspace.admin.BEGIN_DATE_TIME");
            event.remove("google_workspace.admin.START_DATE");
            event.remove("google_workspace.admin.END_DATE");
            event.remove("google_workspace.admin.END_DATE_TIME");
            event.remove("google_workspace.admin.WHITELISTED_GROUPS");
            event.remove("google_workspace.admin.RELATED_ALERT_ID");
            event.remove("google_workspace.admin.SETTING_METADATA");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
