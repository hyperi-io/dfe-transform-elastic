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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.get_str("event.action") != Some("started")
                    && event.get_str("event.action") != Some("completed")
            };
            if _cond {
                event.remove("event.action");
            }

            let _cond = { event.has_value("activedirectory") && event.has_value("user.id") };
            if _cond {
                // Begin nested pipeline: "user"
                event.set("event.kind", json!("asset"))?;
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
                event.set(
                    "event.type",
                    Value::Array(vec![json!("user"), json!("info")]),
                )?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("activedirectory_user"))?;
                // Begin nested pipeline: "common"
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "1fc6614a979b8062cde1cda5e56bfa3cb14dbee54d9fe48dd301265e6054455f",
                        r#"{"params":{"RenameKeys":{"names":"Params","coerce":null,"rename_key":null,"write":{"InPlace":"activedirectory"}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"accountExpires\":\"account_expires\",\"adminCount\":\"admin_count\",\"badPasswordTime\":\"bad_password_time\",\"badPwdCount\":\"bad_pwd_count\",\"cn\":\"cn\",\"codePage\":\"code_page\",\"countryCode\":\"country_code\",\"description\":\"description\",\"distinguishedName\":\"distinguished_name\",\"dNSHostName\":\"dns_host_name\",\"dSCorePropagationData\":\"ds_core_propagation_data\",\"groups\":\"groups\",\"groupType\":\"group_type\",\"instanceType\":\"instance_type\",\"isCriticalSystemObject\":\"is_critical_system_object\",\"lastLogoff\":\"last_logoff\",\"lastLogon\":\"last_logon\",\"lastLogonTimestamp\":\"last_logon_timestamp\",\"logonCount\":\"logon_count\",\"mail\":\"mail\",\"member\":\"member\",\"memberOf\":\"member_of\",\"name\":\"name\",\"object_category\":\"object_category\",\"objectCategory\":\"object_category\",\"objectClass\":\"object_class\",\"objectGUID\":\"object_guid\",\"objectSid\":\"object_sid\",\"operatingSystem\":\"operating_system\",\"operatingSystemVersion\":\"operating_system_version\",\"primaryGroupID\":\"primary_group_id\",\"pwdLastSet\":\"pwd_last_set\",\"sAMAccountName\":\"sam_account_name\",\"sAMAccountType\":\"sam_account_type\",\"servicePrincipalName\":\"service_principal_name\",\"showInAdvancedViewOnly\":\"show_in_advanced_view_only\",\"userAccountControl\":\"user_account_control\",\"userPrincipalName\":\"user_principal_name\",\"uSNChanged\":\"usn_changed\",\"uSNCreated\":\"usn_created\",\"whenChanged\":\"when_changed\",\"whenCreated\":\"when_created\",\"directReports\":\"direct_reports\",\"managedObjects\":\"managed_objects\"}"
                    ),
                )?;
                let _cond = {
                    !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_group_member_list")),
                        serde_json::Value::String(s) => s.contains("preserve_group_member_list"),
                        _ => false,
                    }))
                };
                if _cond {
                    if event.has_value("activedirectory.groups") {
                        foreach_array(event, "activedirectory.groups", |event| {
                            event.remove("_ingest._value.member");
                            Ok(())
                        })?;
                    }
                }
                // End nested pipeline: "common"
                let _cond = {
                    event.has_value("activedirectory.user.user_account_control")
                        && event.get_i64("activedirectory.user.user_account_control") != Some(0)
                };
                if _cond {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "554eb2db69e5149aeab09babf2d27299a2255e2c1125da8914026f7db71ca733",
                            r#"{"params":"BitFlags","known":[]}"#
                        ),
                        cached_params!(
                            "{\"0x00000001\":\"SCRIPT\",\"0x00000002\":\"ACCOUNTDISABLE\",\"0x00000008\":\"HOMEDIR_REQUIRED\",\"0x00000010\":\"LOCKOUT\",\"0x00000020\":\"PASSWD_NOTREQD\",\"0x00000040\":\"PASSWD_CANT_CHANGE\",\"0x00000080\":\"ENCRYPTED_TEXT_PWD_ALLOWED\",\"0x00000100\":\"TEMP_DUPLICATE_ACCOUNT\",\"0x00000200\":\"NORMAL_ACCOUNT\",\"0x00000800\":\"INTERDOMAIN_TRUST_ACCOUNT\",\"0x00001000\":\"WORKSTATION_TRUST_ACCOUNT\",\"0x00002000\":\"SERVER_TRUST_ACCOUNT\",\"0x00010000\":\"DONT_EXPIRE_PASSWORD\",\"0x00020000\":\"MNS_LOGON_ACCOUNT\",\"0x00040000\":\"SMARTCARD_REQUIRED\",\"0x00080000\":\"TRUSTED_FOR_DELEGATION\",\"0x00100000\":\"NOT_DELEGATED\",\"0x00200000\":\"USE_DES_KEY_ONLY\",\"0x00400000\":\"DONT_REQUIRE_PREAUTH\",\"0x00800000\":\"PASSWORD_EXPIRED\",\"0x01000000\":\"TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION\",\"0x04000000\":\"PARTIAL_SECRETS_ACCOUNT\"}"
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("activedirectory.user.when_created")
                        && event.get_str("activedirectory.user.when_created") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.user.when_created")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.create_date", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.user.when_created".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_user_created")?;
                        if event.remove("activedirectory.user.when_created").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.user.when_created".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.user.when_changed")
                        && event.get_str("activedirectory.user.when_changed") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.user.when_changed")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.last_updated", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.user.when_changed".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_user_changed")?;
                        if event.remove("activedirectory.user.when_changed").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.user.when_changed".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.user.pwd_last_set")
                        && event.get_str("activedirectory.user.pwd_last_set") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.user.pwd_last_set")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("user.account.password_change_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.user.pwd_last_set".into(),
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
                            "date_user_password_changed",
                        )?;
                        if event.remove("activedirectory.user.pwd_last_set").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.user.pwd_last_set".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.user.last_logon_timestamp")
                        && event.get_str("activedirectory.user.last_logon_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.user.last_logon_timestamp")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("user.entity.lifecycle.last_activity", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.user.last_logon_timestamp".into(),
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
                            "date_user_last_logon_timestamp",
                        )?;
                        if event
                            .remove("activedirectory.user.last_logon_timestamp")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.user.last_logon_timestamp".into(),
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
                                    .get("_ingest.pipeline")
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
                    .get("activedirectory.user.sam_account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("asset.name", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.user.sam_account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.user.mail")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond = { event.has_value("activedirectory.user.distinguished_name") };
                if _cond {
                    gsub_field(
                        event,
                        "activedirectory.user.distinguished_name",
                        "user.domain",
                        cached_regex!("^.*?DC="),
                        "",
                    )?;
                }
                let _cond = { event.has_value("user.domain") };
                if _cond {
                    gsub_field(
                        event,
                        "user.domain",
                        "user.domain",
                        cached_regex!(",DC="),
                        ".",
                    )?;
                }
                if let Some(v) = event.get("activedirectory.user.object_sid").cloned() {
                    event.set("asset.id", v)?;
                }
                if let Some(v) = event.get("activedirectory.user.object_sid").cloned() {
                    event.set("user.id", v)?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SCRIPT"))
                            }
                            serde_json::Value::String(s) => s.contains("SCRIPT"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.logon_script_enabled", json!(true))?;
                }
                let _cond = {
                    !(event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("ACCOUNTDISABLE"))
                            }
                            serde_json::Value::String(s) => s.contains("ACCOUNTDISABLE"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("activedirectory.user.enabled", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("ACCOUNTDISABLE"))
                            }
                            serde_json::Value::String(s) => s.contains("ACCOUNTDISABLE"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.enabled", json!(false))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("LOCKOUT"))
                            }
                            serde_json::Value::String(s) => s.contains("LOCKOUT"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.locked", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("PASSWD_NOTREQD"))
                            }
                            serde_json::Value::String(s) => s.contains("PASSWD_NOTREQD"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.password_not_required", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("ENCRYPTED_TEXT_PWD_ALLOWED")),
                            serde_json::Value::String(s) => {
                                s.contains("ENCRYPTED_TEXT_PWD_ALLOWED")
                            }
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "activedirectory.user.reversible_encryption_password",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("TRUSTED_FOR_DELEGATION")),
                            serde_json::Value::String(s) => s.contains("TRUSTED_FOR_DELEGATION"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.unconstrained_delegation", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("NOT_DELEGATED"))
                            }
                            serde_json::Value::String(s) => s.contains("NOT_DELEGATED"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.sensitive_object", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("USE_DES_KEY_ONLY"))
                            }
                            serde_json::Value::String(s) => s.contains("USE_DES_KEY_ONLY"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.use_des_key_only", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("DONT_REQUIRE_PREAUTH"))
                            }
                            serde_json::Value::String(s) => s.contains("DONT_REQUIRE_PREAUTH"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.dont_require_preauth", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a.iter().any(|x| {
                                x.as_str() == Some("TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION")
                            }),
                            serde_json::Value::String(s) => {
                                s.contains("TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION")
                            }
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.user.constrained_delegation", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.user.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SMARTCARD_REQUIRED"))
                            }
                            serde_json::Value::String(s) => s.contains("SMARTCARD_REQUIRED"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("user.entity.attributes.mfa_enabled", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.groups")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // The pipeline's script, run by the runtime's equivalent
                        painless_exec_plan_params(
                            event,
                            compiled_painless!(
                                "0601159c806d560f62cbd5e3024e28e8ba8328ee0a156b1049c7e415f380df80",
                                r#"{"params":null,"known":[{"CollectFromList":{"source":"activedirectory.groups","columns":[{"member":"name","target":"user.group.name","replace":null,"upper":false}],"firsts":[]}}]}"#
                            ),
                            cached_params!(
                                "{\"512\":true,\"516\":true,\"518\":true,\"519\":true,\"520\":true,\"525\":true,\"526\":true,\"527\":true,\"544\":true,\"548\":true,\"549\":true,\"551\":true}"
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("activedirectory.user.managed_objects")
                        || event.has_value("activedirectory.user.direct_reports")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // The pipeline's script, run by the runtime's equivalent
                        painless_exec_plan(
                            event,
                            compiled_painless!(
                                "c54b89496f23087ae27acb2f5693a5cb509db4c6cf9cbf9aa7890bc32519ad2c",
                                r#"{"params":null,"known":["CollectingLadder","CaseInsensitiveLadder"]}"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("activedirectory.user.account_expires") == Some("0")
                        || event.get_str("activedirectory.user.account_expires")
                            == Some("9223372036854775807")
                };
                if _cond {
                    event.set("activedirectory.user.account_never_expires", json!(true))?;
                }
                let _cond = { event.has_value("activedirectory.user.sam_account_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.sam_account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.user.object_guid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.object_guid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.user.mail") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.mail")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.user.user_principal_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.user_principal_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.rename("activedirectory.id", "activedirectory.user.object_dn")?;
                event.rename("activedirectory", "entityanalytics_ad")?;
                // End nested pipeline: "user"
            }

            let _cond = { event.has_value("activedirectory") && event.has_value("device.id") };
            if _cond {
                // Begin nested pipeline: "device"
                event.set("event.kind", json!("asset"))?;
                event.set("event.category", Value::Array(vec![json!("host")]))?;
                event.set("event.type", Value::Array(vec![json!("info")]))?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("activedirectory_device"))?;
                // Begin nested pipeline: "common"
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "1fc6614a979b8062cde1cda5e56bfa3cb14dbee54d9fe48dd301265e6054455f",
                        r#"{"params":{"RenameKeys":{"names":"Params","coerce":null,"rename_key":null,"write":{"InPlace":"activedirectory"}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"accountExpires\":\"account_expires\",\"adminCount\":\"admin_count\",\"badPasswordTime\":\"bad_password_time\",\"badPwdCount\":\"bad_pwd_count\",\"cn\":\"cn\",\"codePage\":\"code_page\",\"countryCode\":\"country_code\",\"description\":\"description\",\"distinguishedName\":\"distinguished_name\",\"dNSHostName\":\"dns_host_name\",\"dSCorePropagationData\":\"ds_core_propagation_data\",\"groups\":\"groups\",\"groupType\":\"group_type\",\"instanceType\":\"instance_type\",\"isCriticalSystemObject\":\"is_critical_system_object\",\"lastLogoff\":\"last_logoff\",\"lastLogon\":\"last_logon\",\"lastLogonTimestamp\":\"last_logon_timestamp\",\"logonCount\":\"logon_count\",\"mail\":\"mail\",\"member\":\"member\",\"memberOf\":\"member_of\",\"name\":\"name\",\"object_category\":\"object_category\",\"objectCategory\":\"object_category\",\"objectClass\":\"object_class\",\"objectGUID\":\"object_guid\",\"objectSid\":\"object_sid\",\"operatingSystem\":\"operating_system\",\"operatingSystemVersion\":\"operating_system_version\",\"primaryGroupID\":\"primary_group_id\",\"pwdLastSet\":\"pwd_last_set\",\"sAMAccountName\":\"sam_account_name\",\"sAMAccountType\":\"sam_account_type\",\"servicePrincipalName\":\"service_principal_name\",\"showInAdvancedViewOnly\":\"show_in_advanced_view_only\",\"userAccountControl\":\"user_account_control\",\"userPrincipalName\":\"user_principal_name\",\"uSNChanged\":\"usn_changed\",\"uSNCreated\":\"usn_created\",\"whenChanged\":\"when_changed\",\"whenCreated\":\"when_created\",\"directReports\":\"direct_reports\",\"managedObjects\":\"managed_objects\"}"
                    ),
                )?;
                let _cond = {
                    !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_group_member_list")),
                        serde_json::Value::String(s) => s.contains("preserve_group_member_list"),
                        _ => false,
                    }))
                };
                if _cond {
                    if event.has_value("activedirectory.groups") {
                        foreach_array(event, "activedirectory.groups", |event| {
                            event.remove("_ingest._value.member");
                            Ok(())
                        })?;
                    }
                }
                // End nested pipeline: "common"
                let _cond = {
                    event.has_value("activedirectory.device.user_account_control")
                        && event.get_i64("activedirectory.device.user_account_control") != Some(0)
                };
                if _cond {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "68485626173ada45c0d167f6e2564bd412da486197370cdcd7722e08defe7089",
                            r#"{"params":"BitFlags","known":[]}"#
                        ),
                        cached_params!(
                            "{\"0x00000001\":\"SCRIPT\",\"0x00000002\":\"ACCOUNTDISABLE\",\"0x00000008\":\"HOMEDIR_REQUIRED\",\"0x00000010\":\"LOCKOUT\",\"0x00000020\":\"PASSWD_NOTREQD\",\"0x00000040\":\"PASSWD_CANT_CHANGE\",\"0x00000080\":\"ENCRYPTED_TEXT_PWD_ALLOWED\",\"0x00000100\":\"TEMP_DUPLICATE_ACCOUNT\",\"0x00000200\":\"NORMAL_ACCOUNT\",\"0x00000800\":\"INTERDOMAIN_TRUST_ACCOUNT\",\"0x00001000\":\"WORKSTATION_TRUST_ACCOUNT\",\"0x00002000\":\"SERVER_TRUST_ACCOUNT\",\"0x00010000\":\"DONT_EXPIRE_PASSWORD\",\"0x00020000\":\"MNS_LOGON_ACCOUNT\",\"0x00040000\":\"SMARTCARD_REQUIRED\",\"0x00080000\":\"TRUSTED_FOR_DELEGATION\",\"0x00100000\":\"NOT_DELEGATED\",\"0x00200000\":\"USE_DES_KEY_ONLY\",\"0x00400000\":\"DONT_REQUIRE_PREAUTH\",\"0x00800000\":\"PASSWORD_EXPIRED\",\"0x01000000\":\"TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION\",\"0x04000000\":\"PARTIAL_SECRETS_ACCOUNT\"}"
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("activedirectory.device.when_created")
                        && event.get_str("activedirectory.device.when_created") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.device.when_created")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.create_date", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.device.when_created".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_device_created")?;
                        if event
                            .remove("activedirectory.device.when_created")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.device.when_created".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.device.when_changed")
                        && event.get_str("activedirectory.device.when_changed") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.device.when_changed")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.last_updated", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.device.when_changed".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_device_changed")?;
                        if event
                            .remove("activedirectory.device.when_changed")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.device.when_changed".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.device.pwd_last_set")
                        && event.get_str("activedirectory.device.pwd_last_set") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.device.pwd_last_set")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("user.account.password_change_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.device.pwd_last_set".into(),
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
                            "date_user_password_changed",
                        )?;
                        if event
                            .remove("activedirectory.device.pwd_last_set")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.device.pwd_last_set".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.device.last_logon_timestamp")
                        && event.get_str("activedirectory.device.last_logon_timestamp") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.device.last_logon_timestamp")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("host.entity.lifecycle.last_activity", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.device.last_logon_timestamp".into(),
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
                            "date_device_last_logon_timestamp",
                        )?;
                        if event
                            .remove("activedirectory.device.last_logon_timestamp")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.device.last_logon_timestamp".into(),
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
                                    .get("_ingest.pipeline")
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
                if event.has_value("activedirectory.device.dns_host_name") {
                    map_strings(
                        event,
                        "activedirectory.device.dns_host_name",
                        "asset.name",
                        str::to_lowercase,
                    )?;
                }
                if let Some(v) = event
                    .get("activedirectory.device.sam_account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event.get("activedirectory.device.object_sid").cloned() {
                    event.set("asset.id", v)?;
                }
                if let Some(v) = event.get("activedirectory.device.object_sid").cloned() {
                    event.set("device.id", v)?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SCRIPT"))
                            }
                            serde_json::Value::String(s) => s.contains("SCRIPT"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.logon_script_enabled", json!(true))?;
                }
                let _cond = {
                    !(event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("ACCOUNTDISABLE"))
                            }
                            serde_json::Value::String(s) => s.contains("ACCOUNTDISABLE"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("activedirectory.device.enabled", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("ACCOUNTDISABLE"))
                            }
                            serde_json::Value::String(s) => s.contains("ACCOUNTDISABLE"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.enabled", json!(false))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("LOCKOUT"))
                            }
                            serde_json::Value::String(s) => s.contains("LOCKOUT"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.locked", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("PASSWD_NOTREQD"))
                            }
                            serde_json::Value::String(s) => s.contains("PASSWD_NOTREQD"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.password_not_required", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("ENCRYPTED_TEXT_PWD_ALLOWED")),
                            serde_json::Value::String(s) => {
                                s.contains("ENCRYPTED_TEXT_PWD_ALLOWED")
                            }
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "activedirectory.device.reversible_encryption_password",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("TRUSTED_FOR_DELEGATION")),
                            serde_json::Value::String(s) => s.contains("TRUSTED_FOR_DELEGATION"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "activedirectory.device.unconstrained_delegation",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("NOT_DELEGATED"))
                            }
                            serde_json::Value::String(s) => s.contains("NOT_DELEGATED"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.sensitive_object", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("USE_DES_KEY_ONLY"))
                            }
                            serde_json::Value::String(s) => s.contains("USE_DES_KEY_ONLY"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.use_des_key_only", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("DONT_REQUIRE_PREAUTH"))
                            }
                            serde_json::Value::String(s) => s.contains("DONT_REQUIRE_PREAUTH"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.dont_require_preauth", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a.iter().any(|x| {
                                x.as_str() == Some("TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION")
                            }),
                            serde_json::Value::String(s) => {
                                s.contains("TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION")
                            }
                            _ => false,
                        })
                };
                if _cond {
                    event.set("activedirectory.device.constrained_delegation", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.device.uac_list")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("WORKSTATION_TRUST_ACCOUNT")),
                            serde_json::Value::String(s) => s.contains("WORKSTATION_TRUST_ACCOUNT"),
                            _ => false,
                        })
                        || event
                            .get("activedirectory.device.uac_list")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("SERVER_TRUST_ACCOUNT"))
                                }
                                serde_json::Value::String(s) => s.contains("SERVER_TRUST_ACCOUNT"),
                                _ => false,
                            })
                };
                if _cond {
                    event.set("host.entity.attributes.managed", json!(true))?;
                }
                let _cond = {
                    event
                        .get("activedirectory.groups")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // The pipeline's script, run by the runtime's equivalent
                        painless_exec_plan_params(
                            event,
                            compiled_painless!(
                                "48d486ab13787b38a61fadbd89731f5e4fe7010eb5440db5bdcdae0507a8fbd5",
                                r#"{"params":null,"known":[{"CollectFromList":{"source":"activedirectory.groups","columns":[{"member":"name","target":"user.group.name","replace":null,"upper":false}],"firsts":[]}}]}"#
                            ),
                            cached_params!(
                                "{\"512\":true,\"516\":true,\"518\":true,\"519\":true,\"520\":true,\"525\":true,\"526\":true,\"527\":true,\"544\":true,\"548\":true,\"549\":true,\"551\":true}"
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("activedirectory.device.managed_objects")
                        || event.has_value("activedirectory.device.direct_reports")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // The pipeline's script, run by the runtime's equivalent
                        painless_exec_plan(
                            event,
                            compiled_painless!(
                                "0b58725d27662d90f95ee8f9d2d426ac93fa0ac7406f537c911581eb7be1ead6",
                                r#"{"params":null,"known":["CollectingLadder","CaseInsensitiveLadder"]}"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("activedirectory.device.account_expires") == Some("0")
                        || event.get_str("activedirectory.device.account_expires")
                            == Some("9223372036854775807")
                };
                if _cond {
                    event.set("activedirectory.device.account_never_expires", json!(true))?;
                }
                if let Some(v) = event
                    .get("activedirectory.device.cn")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                if event.has_value("activedirectory.device.dns_host_name") {
                    map_strings(
                        event,
                        "activedirectory.device.dns_host_name",
                        "host.name",
                        str::to_lowercase,
                    )?;
                }
                let _cond = {
                    event.get("host.name").is_some_and(|v| v.is_string())
                        && event.get("host.hostname").is_some_and(|v| v.is_string())
                        && event.get("host.name").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("."))
                            }
                            serde_json::Value::String(s) => s.contains("."),
                            _ => false,
                        })
                        && event.get_str("host.hostname").is_some_and(|p| {
                            event
                                .get_str("host.name")
                                .is_some_and(|s| s.starts_with((p.to_lowercase() + ".").as_str()))
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("host.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(".") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(".") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("host.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "host.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                if let Some(v) = event
                    .get("activedirectory.device.operating_system")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.full", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.device.operating_system_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
                let _cond = { event.has_value("activedirectory.device.sam_account_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.device.sam_account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                let _cond = { event.has_value("activedirectory.id") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("activedirectory.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.device.object_guid") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("activedirectory.device.object_guid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.rename("activedirectory.id", "activedirectory.device.object_dn")?;
                event.rename("activedirectory", "entityanalytics_ad")?;
                // End nested pipeline: "device"
            }

            let _cond = { event.has_value("activedirectory") && event.has_value("group.id") };
            if _cond {
                // Begin nested pipeline: "group"
                event.set("event.kind", json!("asset"))?;
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
                event.set(
                    "event.type",
                    Value::Array(vec![json!("group"), json!("info")]),
                )?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("activedirectory_group"))?;
                // Begin nested pipeline: "common"
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "1fc6614a979b8062cde1cda5e56bfa3cb14dbee54d9fe48dd301265e6054455f",
                        r#"{"params":{"RenameKeys":{"names":"Params","coerce":null,"rename_key":null,"write":{"InPlace":"activedirectory"}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"accountExpires\":\"account_expires\",\"adminCount\":\"admin_count\",\"badPasswordTime\":\"bad_password_time\",\"badPwdCount\":\"bad_pwd_count\",\"cn\":\"cn\",\"codePage\":\"code_page\",\"countryCode\":\"country_code\",\"description\":\"description\",\"distinguishedName\":\"distinguished_name\",\"dNSHostName\":\"dns_host_name\",\"dSCorePropagationData\":\"ds_core_propagation_data\",\"groups\":\"groups\",\"groupType\":\"group_type\",\"instanceType\":\"instance_type\",\"isCriticalSystemObject\":\"is_critical_system_object\",\"lastLogoff\":\"last_logoff\",\"lastLogon\":\"last_logon\",\"lastLogonTimestamp\":\"last_logon_timestamp\",\"logonCount\":\"logon_count\",\"mail\":\"mail\",\"member\":\"member\",\"memberOf\":\"member_of\",\"name\":\"name\",\"object_category\":\"object_category\",\"objectCategory\":\"object_category\",\"objectClass\":\"object_class\",\"objectGUID\":\"object_guid\",\"objectSid\":\"object_sid\",\"operatingSystem\":\"operating_system\",\"operatingSystemVersion\":\"operating_system_version\",\"primaryGroupID\":\"primary_group_id\",\"pwdLastSet\":\"pwd_last_set\",\"sAMAccountName\":\"sam_account_name\",\"sAMAccountType\":\"sam_account_type\",\"servicePrincipalName\":\"service_principal_name\",\"showInAdvancedViewOnly\":\"show_in_advanced_view_only\",\"userAccountControl\":\"user_account_control\",\"userPrincipalName\":\"user_principal_name\",\"uSNChanged\":\"usn_changed\",\"uSNCreated\":\"usn_created\",\"whenChanged\":\"when_changed\",\"whenCreated\":\"when_created\",\"directReports\":\"direct_reports\",\"managedObjects\":\"managed_objects\"}"
                    ),
                )?;
                let _cond = {
                    !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_group_member_list")),
                        serde_json::Value::String(s) => s.contains("preserve_group_member_list"),
                        _ => false,
                    }))
                };
                if _cond {
                    if event.has_value("activedirectory.groups") {
                        foreach_array(event, "activedirectory.groups", |event| {
                            event.remove("_ingest._value.member");
                            Ok(())
                        })?;
                    }
                }
                // End nested pipeline: "common"
                let _cond = {
                    event.has_value("activedirectory.group.when_created")
                        && event.get_str("activedirectory.group.when_created") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.group.when_created")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.create_date", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.group.when_created".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_group_created")?;
                        if event.remove("activedirectory.group.when_created").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.group.when_created".into(),
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
                                    .get("_ingest.pipeline")
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
                    event.has_value("activedirectory.group.when_changed")
                        && event.get_str("activedirectory.group.when_changed") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("activedirectory.group.when_changed")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("asset.last_updated", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "activedirectory.group.when_changed".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_group_changed")?;
                        if event.remove("activedirectory.group.when_changed").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "activedirectory.group.when_changed".into(),
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
                                    .get("_ingest.pipeline")
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
                    .get("activedirectory.group.sam_account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("asset.name", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.group.object_sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("asset.id", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.group.object_sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
                if let Some(v) = event
                    .get("activedirectory.group.sam_account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
                let _cond = { event.has_value("activedirectory.group.distinguished_name") };
                if _cond {
                    gsub_field(
                        event,
                        "activedirectory.group.distinguished_name",
                        "group.domain",
                        cached_regex!("^.*?DC="),
                        "",
                    )?;
                }
                let _cond = { event.has_value("group.domain") };
                if _cond {
                    gsub_field(
                        event,
                        "group.domain",
                        "group.domain",
                        cached_regex!(",DC="),
                        ".",
                    )?;
                }
                event.rename("activedirectory.id", "activedirectory.group.object_dn")?;
                event.rename("activedirectory", "entityanalytics_ad")?;
                // End nested pipeline: "group"
            }

            let _cond = {
                event.has_value("activedirectory")
                    && !event.has_value("user.id")
                    && !event.has_value("device.id")
                    && !event.has_value("group.id")
            };
            if _cond {
                // Begin nested pipeline: "entity"
                event.set("event.kind", json!("asset"))?;
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
                event.set(
                    "event.type",
                    Value::Array(vec![json!("user"), json!("info")]),
                )?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("activedirectory_entity"))?;
                // Begin nested pipeline: "common"
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "1fc6614a979b8062cde1cda5e56bfa3cb14dbee54d9fe48dd301265e6054455f",
                        r#"{"params":{"RenameKeys":{"names":"Params","coerce":null,"rename_key":null,"write":{"InPlace":"activedirectory"}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"accountExpires\":\"account_expires\",\"adminCount\":\"admin_count\",\"badPasswordTime\":\"bad_password_time\",\"badPwdCount\":\"bad_pwd_count\",\"cn\":\"cn\",\"codePage\":\"code_page\",\"countryCode\":\"country_code\",\"description\":\"description\",\"distinguishedName\":\"distinguished_name\",\"dNSHostName\":\"dns_host_name\",\"dSCorePropagationData\":\"ds_core_propagation_data\",\"groups\":\"groups\",\"groupType\":\"group_type\",\"instanceType\":\"instance_type\",\"isCriticalSystemObject\":\"is_critical_system_object\",\"lastLogoff\":\"last_logoff\",\"lastLogon\":\"last_logon\",\"lastLogonTimestamp\":\"last_logon_timestamp\",\"logonCount\":\"logon_count\",\"mail\":\"mail\",\"member\":\"member\",\"memberOf\":\"member_of\",\"name\":\"name\",\"object_category\":\"object_category\",\"objectCategory\":\"object_category\",\"objectClass\":\"object_class\",\"objectGUID\":\"object_guid\",\"objectSid\":\"object_sid\",\"operatingSystem\":\"operating_system\",\"operatingSystemVersion\":\"operating_system_version\",\"primaryGroupID\":\"primary_group_id\",\"pwdLastSet\":\"pwd_last_set\",\"sAMAccountName\":\"sam_account_name\",\"sAMAccountType\":\"sam_account_type\",\"servicePrincipalName\":\"service_principal_name\",\"showInAdvancedViewOnly\":\"show_in_advanced_view_only\",\"userAccountControl\":\"user_account_control\",\"userPrincipalName\":\"user_principal_name\",\"uSNChanged\":\"usn_changed\",\"uSNCreated\":\"usn_created\",\"whenChanged\":\"when_changed\",\"whenCreated\":\"when_created\",\"directReports\":\"direct_reports\",\"managedObjects\":\"managed_objects\"}"
                    ),
                )?;
                let _cond = {
                    !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_group_member_list")),
                        serde_json::Value::String(s) => s.contains("preserve_group_member_list"),
                        _ => false,
                    }))
                };
                if _cond {
                    if event.has_value("activedirectory.groups") {
                        foreach_array(event, "activedirectory.groups", |event| {
                            event.remove("_ingest._value.member");
                            Ok(())
                        })?;
                    }
                }
                // End nested pipeline: "common"
                let _cond = { event.has_value("activedirectory.user.sam_account_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.sam_account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("activedirectory.user.object_guid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("activedirectory.user.object_guid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.rename("activedirectory.id", "activedirectory.user.object_dn")?;
                event.rename("activedirectory.user", "activedirectory.entity")?;
                event.rename("activedirectory", "entityanalytics_ad")?;
                // End nested pipeline: "entity"
            }

            let _cond =
                { !event.has_value("activedirectory") && !event.has_value("entityanalytics_ad") };
            if _cond {
                // Begin nested pipeline: "marker"
                event.set("event.kind", json!("asset"))?;
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
                event.set("event.type", Value::Array(vec![json!("info")]))?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("activedirectory_user"))?;
                // End nested pipeline: "marker"
            }

            // The pipeline's script, emitted as the runtime's equivalent
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
