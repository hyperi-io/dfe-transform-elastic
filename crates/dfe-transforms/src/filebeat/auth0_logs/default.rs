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

            if let Some(v) = event.get("json.data").cloned() {
                event.set("auth0.logs.data", v)?;
            }

            if let Some(date_str) = event.get_as_string("auth0.logs.data.date") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "auth0.logs.data.date".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.set("log.level", json!("info"))?;

            let _cond = { event.has_value("auth0.logs.data.details.error") };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = { event.has_value("auth0.logs.data.ip") };
            if _cond {
                if let Some(v) = event.get("auth0.logs.data.ip").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("auth0.logs.data.isMobile") {
                    event.rename("auth0.logs.data.isMobile", "auth0.logs.data.is_mobile")?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("source.geo") && event.has_value("source.ip") };
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

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { !event.has_value("network.type") && event.has_value("source.ip") };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("auth0.logs.data.user_name") {
                    if let Some(input) = event.get_string("auth0.logs.data.user_name") {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("auth0.logs.data.user_name")
                    && event
                        .get_str("auth0.logs.data.user_name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("auth0.logs.data.user_name").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("auth0.logs.data.user_id") };
            if _cond {
                if let Some(v) = event.get("auth0.logs.data.user_id").cloned() {
                    event.set("user.id", v)?;
                }
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

            if event.has_value("auth0.logs.data.user_agent") {
                if let Some(ua_str) = event.get_string("auth0.logs.data.user_agent") {
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

            let _cond = { event.has_value("auth0.logs.data.log_id") };
            if _cond {
                if let Some(v) = event.get("auth0.logs.data.log_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("authentication"))?;

            if let Some(v) = event
                .get("auth0.logs.data.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("auth0.logs.data.type_id", v)?;
            }

            let _cond = { event.has_value("auth0.logs.data.type") };
            if _cond {
                // Painless script
                // Source: def eventType = ctx.auth0.logs.data.type;\ndef actions = params.get('actions');\ndef actionData = actions.get(eventType);\nif (actionData == null) {\n    ctx.event.action = 'unknown-' + eventType;\n    ctx.event.type = ['info'];\n    return;\n}\n// overwrite type abbreviation with actual value\ndef eventTypeVal = actionData.get('value');\nif (eventTypeVal != null) {\n    ctx.auth0.logs.data.type = eventTypeVal;\n}\n// event.type\ndef actionType = actionData.get('type');\nif (actionType != null) {\n  ctx.event.type = new ArrayList(actionType);\n}\n// event.category\ndef actionCategory = actionData.get('category');\nif (actionCategory != null) {\n  for (def c : actionCategory) {\n    ctx.event.category.add(c);\n  }\n}\n// event.action\ndef action = actionData.get('action');\nif (action != null) {\n  ctx.event.action = action;\n}\n// auth0 event category / classification group\ndef classification = actionData.get('classification');\nif (classification != null) {\n  ctx.auth0.logs.data.classification = classification;\n}\n// event.outcome\nif (classification.toLowerCase().contains(\"success\")) {\n  ctx.event.outcome = \"success\";\n} else if (classification.toLowerCase().contains(\"failure\")) {\n  ctx.event.outcome = \"failure\";\n} else {\n  ctx.event.outcome = \"unknown\";\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def eventType = ctx.auth0.logs.data.type;\ndef actions = params.get('actions');\ndef actionData = actions.get(eventType);\nif (actionData == null) {\n    ctx.event.action = 'unknown-' + eventType;\n    ctx.event.type = ['info'];\n    return;\n}\n// overwrite type abbreviation with actual value\ndef eventTypeVal = actionData.get('value');\nif (eventTypeVal != null) {\n    ctx.auth0.logs.data.type = eventTypeVal;\n}\n// event.type\ndef actionType = actionData.get('type');\nif (actionType != null) {\n  ctx.event.type = new ArrayList(actionType);\n}\n// event.category\ndef actionCategory = actionData.get('category');\nif (actionCategory != null) {\n  for (def c : actionCategory) {\n    ctx.event.category.add(c);\n  }\n}\n// event.action\ndef action = actionData.get('action');\nif (action != null) {\n  ctx.event.action = action;\n}\n// auth0 event category / classification group\ndef classification = actionData.get('classification');\nif (classification != null) {\n  ctx.auth0.logs.data.classification = classification;\n}\n// event.outcome\nif (classification.toLowerCase().contains(\"success\")) {\n  ctx.event.outcome = \"success\";\n} else if (classification.toLowerCase().contains(\"failure\")) {\n  ctx.event.outcome = \"failure\";\n} else {\n  ctx.event.outcome = \"unknown\";\n}"#
                    ),
                    cached_params!(
                        "{\"actions\":{\"f\":{\"classification\":\"Login - Failure\",\"value\":\"Failed login\",\"type\":[\"info\"],\"action\":\"failed-login\"},\"fc\":{\"classification\":\"Login - Failure\",\"value\":\"Failed connector login\",\"type\":[\"info\"],\"action\":\"failed-connector-login\"},\"fco\":{\"classification\":\"Login - Failure\",\"value\":\"Origin is not in the application's Allowed Origins list\",\"type\":[\"info\"],\"action\":\"origin-not-allowed\"},\"fcoa\":{\"classification\":\"Login - Failure\",\"value\":\"Failed cross-origin authentication\",\"type\":[\"info\"],\"action\":\"failed-cross-origin-authentication\"},\"fens\":{\"classification\":\"Login - Failure\",\"value\":\"Failed native social login\",\"type\":[\"info\"],\"action\":\"failed-native-social-login\"},\"fp\":{\"classification\":\"Login - Failure\",\"value\":\"Incorrect password\",\"type\":[\"info\"],\"action\":\"incorrect-password\"},\"fu\":{\"classification\":\"Login - Failure\",\"value\":\"Invalid email or username\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"invalid-username-or-email\"},\"w\":{\"classification\":\"Login - Notification\",\"value\":\"Warnings during login\",\"type\":[\"info\"],\"action\":\"warnings-during-login\"},\"s\":{\"classification\":\"Login - Success\",\"value\":\"Successful login\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"successful-login\"},\"scoa\":{\"classification\":\"Login - Success\",\"value\":\"Successful cross-origin authentication\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"successful-cross-origin-authentication\"},\"sens\":{\"classification\":\"Login - Success\",\"value\":\"Successful native social login\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"successful-native-social-login\"},\"flo\":{\"classification\":\"Logout - Failure\",\"value\":\"User logout failed\",\"type\":[\"info\"],\"category\":[\"session\"],\"action\":\"user-logout-failed\"},\"slo\":{\"classification\":\"Logout - Success\",\"value\":\"User successfully logged out\",\"type\":[\"info\",\"end\"],\"category\":[\"session\"],\"action\":\"user-logout-successful\"},\"fs\":{\"classification\":\"Signup - Failure\",\"value\":\"User signup failed\",\"type\":[\"info\",\"creation\",\"user\"],\"category\":[\"iam\"],\"action\":\"user-signup-failed\"},\"fsa\":{\"classification\":\"Silent Authentication - Failure\",\"value\":\"Failed silent authentication\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"failed-silent-authentication\"},\"ssa\":{\"classification\":\"Silent Authentication - Success\",\"value\":\"Successful silent authentication\",\"type\":[\"info\"],\"action\":\"successful-silent-authentication\"},\"feacft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Authorization Code for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-auth-code-for-access-token\"},\"feccft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Access Token for a Client Credentials Grant\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-access-token-for-client-cred-grant\"},\"fede\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Device Code for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-device-code-for-access-token\"},\"feoobft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Password and OOB Challenge for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-password-oob-challenge-for-access-token\"},\"feotpft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Password and OTP Challenge for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-password-otp-challenge-for-access-token\"},\"fepft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Password for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-password-for-access-token\"},\"fepotpft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Passwordless OTP for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-passwordless-otp-for-access-token\"},\"fercft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Password and MFA Recovery code for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-password-mfa-recovery-code-for-access-token\"},\"ferrt\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Rotating Refresh Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-rotating-refresh-token\"},\"fertft\":{\"classification\":\"Token Exchange - Failure\",\"value\":\"Failed exchange of Refresh Token for Access Token\",\"type\":[\"info\",\"protocol\",\"error\"],\"category\":[\"network\",\"web\"],\"action\":\"failed-exchange-refresh-token-for-access-token\"},\"seacft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Authorization Code for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-auth-code-for-access-token\"},\"seccft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Access Token for a Client Credentials Grant\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-access-token-for-client-cred-grant\"},\"sede\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Device Code for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-device-code-for-access-token\"},\"seoobft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Password and OOB Challenge for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-password-oob-challange-for-access-token\"},\"seotpft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Password and OTP Challenge for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-password-otp-challenge-for-access-token\"},\"sepft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Password for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-password-for-access-token\"},\"sercft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Password and MFA Recovery code for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-mfa-recovery-code-for-access-token\"},\"sertft\":{\"classification\":\"Token Exchange - Success\",\"value\":\"Successful exchange of Refresh Token for Access Token\",\"type\":[\"info\",\"protocol\",\"access\"],\"category\":[\"network\",\"web\"],\"action\":\"success-exchange-refresh-token-for-access-token\"},\"fapi\":{\"classification\":\"Management API - Failure\",\"value\":\"Failed Management API operation\",\"type\":[\"info\",\"error\"],\"category\":[\"web\"],\"action\":\"failed-mgmt-api-operation\"},\"sapi\":{\"classification\":\"Management API - Success\",\"value\":\"Successful Management API operation\",\"type\":[\"info\",\"access\",\"change\"],\"category\":[\"web\",\"iam\"],\"action\":\"success-mgmt-api-op\"},\"mgmt_api_read\":{\"classification\":\"Management API - Success\",\"value\":\"API GET operation returning secrets completed successfully\",\"type\":[\"info\",\"access\"],\"category\":[\"web\",\"iam\"],\"action\":\"success-mgmt-api-op-secrets-returned\"},\"admin_update_launch\":{\"classification\":\"System - Notification\",\"value\":\"Auth0 Update Launched\",\"type\":[\"change\"],\"category\":[\"configuration\"],\"action\":\"auth0-update-launched\"},\"api_limit\":{\"classification\":\"System - Notification\",\"value\":\"The maximum number of requests to the Authentication or Management APIs in given time has reached\",\"type\":[\"info\",\"access\"],\"category\":[\"network\"],\"action\":\"max-requests-reached\"},\"coff\":{\"classification\":\"System - Notification\",\"value\":\"AD/LDAP Connector is offline\",\"type\":[\"error\",\"connection\"],\"category\":[\"network\",\"web\"],\"action\":\"ad-ldap-connector-offline\"},\"con\":{\"classification\":\"System - Notification\",\"value\":\"AD/LDAP Connector is online and working\",\"type\":[\"info\",\"connection\"],\"category\":[\"network\"],\"action\":\"ad-ldap-connector-online\"},\"depnote\":{\"classification\":\"System - Notification\",\"value\":\"Deprecation Notice\",\"type\":[\"info\"],\"action\":\"deprecation-notice\"},\"fcpro\":{\"classification\":\"System - Notification\",\"value\":\"Failed to provision a AD/LDAP connector\",\"type\":[\"info\",\"connection\",\"error\"],\"category\":[\"network\"],\"action\":\"failed-ad-ldap-provision\"},\"fui\":{\"classification\":\"System - Notification\",\"value\":\"Failed to import users\",\"type\":[\"info\",\"user\",\"error\"],\"category\":[\"iam\",\"web\"],\"action\":\"failed-to-import-users\"},\"limit_delegation\":{\"classification\":\"System - Notification\",\"value\":\"Rate limit exceeded to /delegation endpoint\",\"type\":[\"info\",\"access\"],\"category\":[\"network\"],\"action\":\"rate-limit-exceeded-to-delegation-endpoint\"},\"limit_mu\":{\"classification\":\"System - Notification\",\"value\":\"An IP address is blocked with 100 failed login attempts using different usernames\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"hundred-failed-logins-ip-address-blocked\"},\"limit_wc\":{\"classification\":\"System - Notification\",\"value\":\"An IP address is blocked with 10 failed login attempts into a single account from the same IP address\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"ten-failed-logins-ip-address-blocked\"},\"sys_os_update_start\":{\"classification\":\"System - Notification\",\"value\":\"Auth0 OS Update Started\",\"type\":[\"change\",\"start\",\"installation\"],\"category\":[\"configuration\",\"package\"],\"action\":\"auth0-os-update-started\"},\"sys_os_update_end\":{\"classification\":\"System - Notification\",\"value\":\"Auth0 OS Update Ended\",\"type\":[\"change\",\"end\",\"installation\"],\"category\":[\"configuration\",\"package\"],\"action\":\"auth0-os-update-ended\"},\"sys_update_start\":{\"classification\":\"System - Notification\",\"value\":\"Auth0 Update Started\",\"type\":[\"change\",\"start\",\"installation\"],\"category\":[\"configuration\",\"package\"],\"action\":\"auth0-update-started\"},\"sys_update_end\":{\"classification\":\"System - Notification\",\"value\":\"Auth0 Update Ended\",\"type\":[\"change\",\"end\",\"installation\"],\"category\":[\"configuration\",\"package\"],\"action\":\"auth0-update-ended\"},\"fce\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to change user email\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-to-change-user-email\"},\"fcp\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to change password\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-to-change-password\"},\"fcpn\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to change phone number\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-to-change-phone-number\"},\"fcpr\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed change password request\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-change-password-request\"},\"fcu\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to change username\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-to-change-username\"},\"fd\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to generate delegation token\",\"type\":[\"info\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-to-generate-delegation-token\"},\"fdeaz\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Device authorization request failed\",\"type\":[\"info\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-device-authorization-request\"},\"fdecc\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"User did not confirm device\",\"type\":[\"info\"],\"action\":\"user-device-not-confirmed\"},\"fdu\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed user deletion\",\"type\":[\"deletion\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-user-deletion\"},\"fn\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to send email notification\",\"type\":[\"info\"],\"action\":\"failed-to-send-email-notification\"},\"fv\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to send verification email\",\"type\":[\"info\"],\"action\":\"failed-to-send-verification-email\"},\"fvr\":{\"classification\":\"User/Behavioral - Failure\",\"value\":\"Failed to process verification email request\",\"type\":[\"info\"],\"action\":\"failed-to-process-verification-email\"},\"cs\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"Passwordless login code has been sent\",\"type\":[\"info\"],\"action\":\"passwordless-login-code-sent\"},\"du\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"User has been deleted\",\"type\":[\"info\",\"user\",\"deletion\"],\"category\":[\"iam\"],\"action\":\"user-deleted\"},\"gd_enrollment_complete\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"A first time MFA user has successfully enrolled using one of the factors\",\"type\":[\"info\",\"change\",\"end\"],\"category\":[\"iam\",\"session\"],\"action\":\"mfa-enrollment-completed\"},\"gd_start_enroll\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"Multi-factor authentication enroll has started\",\"type\":[\"info\",\"change\",\"start\"],\"category\":[\"iam\",\"session\"],\"action\":\"mfa-enrollment-started\"},\"gd_unenroll\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"Device used for second factor authentication has been unenrolled\",\"type\":[\"info\",\"deletion\"],\"category\":[\"iam\"],\"action\":\"mfa-device-unenrolled\"},\"gd_update_device_account\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"Device used for second factor authentication has been updated\",\"type\":[\"info\",\"change\"],\"category\":[\"iam\"],\"action\":\"mfa-device-updated\"},\"ublkdu\":{\"classification\":\"User/Behavioral - Notification\",\"value\":\"User block setup by anomaly detection has been released\",\"type\":[\"info\"],\"action\":\"user-login-block-released\"},\"sce\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully changed user email\",\"type\":[\"info\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"user-email-changed-successfully\"},\"scp\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully changed password\",\"type\":[\"info\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"user-password-changed-successfully\"},\"scpn\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully changed phone number\",\"type\":[\"info\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"user-phone-number-changed-successfully\"},\"scpr\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successful change password request\",\"type\":[\"info\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"user-password-change-request-successful\"},\"scu\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully changed username\",\"type\":[\"info\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"username-changed-successfully\"},\"sdu\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"User successfully deleted\",\"type\":[\"info\",\"deletion\"],\"category\":[\"iam\"],\"action\":\"user-deleted-successfully\"},\"srrt\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully revoked a Refresh Token\",\"type\":[\"info\",\"deletion\"],\"category\":[\"iam\"],\"action\":\"revoked-refresh-token-successfully\"},\"sui\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully imported users\",\"type\":[\"info\",\"user\"],\"category\":[\"iam\"],\"action\":\"imported-users-successfully\"},\"sv\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Sent verification email\",\"type\":[\"info\",\"user\"],\"category\":[\"iam\"],\"action\":\"sent-verification-email\"},\"svr\":{\"classification\":\"User/Behavioral - Success\",\"value\":\"Successfully processed verification email request\",\"type\":[\"info\",\"user\"],\"category\":[\"iam\"],\"action\":\"email-verification-processed-successfully\"},\"fcph\":{\"classification\":\"Other\",\"value\":\"Failed Post Change Password Hook\",\"type\":[\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"failed-post-change-password-hook\"},\"fdeac\":{\"classification\":\"Other\",\"value\":\"Failed to activate device\",\"type\":[\"info\"],\"action\":\"failed-to-activate-device\"},\"fi\":{\"classification\":\"Other\",\"value\":\"Failed to accept a user invitation. This could happen if the user accepts an invitation using a different email address than provided in the invitation, or due to a system failure while provisioning the invitation.\",\"type\":[\"info\"],\"action\":\"failed-to-accept-user-invitation\"},\"gd_auth_failed\":{\"classification\":\"Other\",\"value\":\"Multi-factor authentication failed. This could happen due to a wrong code entered for SMS/Voice/Email/TOTP factors, or a system failure.\",\"type\":[\"info\"],\"action\":\"mfa-authentication-failed-wrong-code\"},\"gd_auth_rejected\":{\"classification\":\"Other\",\"value\":\"A user rejected a Multi-factor authentication request via push-notification.\",\"type\":[\"info\"],\"action\":\"user-rejected-mfa-request\"},\"gd_auth_succeed\":{\"classification\":\"Other\",\"value\":\"Multi-factor authentication success.\",\"type\":[\"info\"],\"action\":\"mfa-authentication-succeeded\"},\"gd_otp_rate_limit_exceed\":{\"classification\":\"Other\",\"value\":\"A user, during enrollment or authentication, enters an incorrect code more than the maximum allowed number of times. Ex: A user enrolling in SMS enters the 6-digit code wrong more than 10 times in a row.\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"user-entered-too-many-incorrect-codes\"},\"gd_recovery_failed\":{\"classification\":\"Other\",\"value\":\"A user enters a wrong recovery code when attempting to authenticate.\",\"type\":[\"info\"],\"action\":\"user-entered-wrong-recovery-code\"},\"gd_recovery_rate_limit_exceed\":{\"classification\":\"Other\",\"value\":\"A user enters a wrong recovery code too many times.\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"user-entered-too-many-wrong-codes\"},\"gd_recovery_succeed\":{\"classification\":\"Other\",\"value\":\"A user successfully authenticates with a recovery code\",\"type\":[\"info\"],\"action\":\"recovery-succeeded\"},\"gd_send_pn\":{\"classification\":\"Other\",\"value\":\"Push notification for MFA sent successfully sent.\",\"type\":[\"info\"],\"action\":\"push-notification-sent\"},\"gd_send_sms\":{\"classification\":\"Other\",\"value\":\"SMS for MFA successfully sent.\",\"type\":[\"info\"],\"action\":\"sms-sent\"},\"gd_send_sms_failure\":{\"classification\":\"Other\",\"value\":\"Attempt to send SMS for MFA failed.\",\"type\":[\"info\"],\"action\":\"failed-to-send-sms\"},\"gd_send_voice\":{\"classification\":\"Other\",\"value\":\"Voice call for MFA successfully made.\",\"type\":[\"info\"],\"action\":\"voice-call-made\"},\"gd_send_voice_failure\":{\"classification\":\"Other\",\"value\":\"Attempt to make Voice call for MFA failed.\",\"type\":[\"info\"],\"action\":\"voice-call-failure\"},\"gd_start_auth\":{\"classification\":\"Other\",\"value\":\"Second factor authentication event started for MFA.\",\"type\":[\"info\"],\"action\":\"2fa-auth-event-started\"},\"gd_tenant_update\":{\"classification\":\"Other\",\"value\":\"Guardian tenant update\",\"type\":[\"info\"],\"action\":\"guardian-tenant-update\"},\"limit_sul\":{\"classification\":\"Other\",\"value\":\"A user is temporarily prevented from logging in because more than 20 logins per minute occurred from the same IP address\",\"type\":[\"info\",\"denied\"],\"category\":[\"intrusion_detection\"],\"action\":\"user-blocked-too-many-failed-logins-from-same-ip\"},\"mfar\":{\"classification\":\"Other\",\"value\":\"A user has been prompted for multi-factor authentication (MFA). When using Adaptive MFA, Auth0 includes details about the risk assessment.\",\"type\":[\"info\"],\"action\":\"user-prompted-for-mfa\"},\"pla\":{\"classification\":\"Other\",\"value\":\"This log is generated before a login and helps in monitoring the behavior of bot detection without having to enable it.\",\"type\":[\"info\"],\"action\":\"pre-login-assessment\"},\"pwd_leak\":{\"classification\":\"Other\",\"value\":\"Someone behind the IP address attempted to login with a leaked password.\",\"type\":[\"info\"],\"category\":[\"intrusion_detection\"],\"action\":\"login-with-breached-password\"},\"scph\":{\"classification\":\"Other\",\"value\":\"Success Post Change Password Hook\",\"type\":[\"info\"],\"action\":\"success-post-change-password-hook\"},\"sd\":{\"classification\":\"Other\",\"value\":\"Success delegation\",\"type\":[\"info\"],\"action\":\"success-delegation\"},\"si\":{\"classification\":\"Other\",\"value\":\"Successfully accepted a user invitation\",\"type\":[\"info\"],\"action\":\"successfully-accepted-user-invitation\"},\"ss\":{\"classification\":\"Other\",\"value\":\"Success Signup\",\"type\":[\"info\"],\"action\":\"success-signup\"}}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("auth0.logs.data.details.initiatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.auth0.logs.data.details.initiatedAt = Math.round(Double.parseDouble(ctx.auth0.logs.data.details.initiatedAt.toString()));\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.auth0.logs.data.details.initiatedAt = Math.round(Double.parseDouble(ctx.auth0.logs.data.details.initiatedAt.toString()));\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_timestamp_initiated_at to string removing any scientific notation if present")?;
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

            let _cond = { event.has_value("auth0.logs.data.details.completedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.auth0.logs.data.details.completedAt = Math.round(Double.parseDouble(ctx.auth0.logs.data.details.completedAt.toString()));\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.auth0.logs.data.details.completedAt = Math.round(Double.parseDouble(ctx.auth0.logs.data.details.completedAt.toString()));\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_timestamp_completed_at to string removing any scientific notation if present")?;
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

            let _cond = { event.has_value("auth0.logs.data.details.initiatedAt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("auth0.logs.data.details.initiatedAt") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("auth0.logs.data.login.initiatedAt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "auth0.logs.data.details.initiatedAt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("auth0.logs.data.details.completedAt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("auth0.logs.data.details.completedAt") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("auth0.logs.data.login.completedAt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "auth0.logs.data.details.completedAt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("auth0.logs.data.details.elapsedTime") };
            if _cond {
                if event.has_value("auth0.logs.data.details.elapsedTime") {
                    if let Some(val) = event.get("auth0.logs.data.details.elapsedTime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "auth0.logs.data.details.elapsedTime".into(),
                                message,
                            }
                        })?;
                        event.set("auth0.logs.data.login.elapsedTime", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("auth0.logs.data.type") == Some("Successful login") };
            if _cond {
                if event.has_value("auth0.logs.data.details.stats.loginsCount") {
                    if let Some(val) = event.get("auth0.logs.data.details.stats.loginsCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "auth0.logs.data.details.stats.loginsCount".into(),
                                message,
                            }
                        })?;
                        event.set("auth0.logs.data.login.stats.loginsCount", converted)?;
                    }
                }
            }

            event.remove("json");
            event.remove("auth0.logs.data.$event_schema.version");
            event.remove("auth0.logs.data._id");
            event.remove("auth0.logs.data.ip");
            event.remove("auth0.logs.data.user_name");
            event.remove("auth0.logs.data.user_id");
            event.remove("auth0.logs.data.user_agent");
            event.remove("auth0.logs.data.log_id");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
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
