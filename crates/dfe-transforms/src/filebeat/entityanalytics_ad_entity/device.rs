// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `device` pipeline.
pub struct Device;

impl Transform for Device {
    fn name(&self) -> &str {
        "device"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("asset"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("activedirectory_device"))?;

            // Begin nested pipeline: "common"
            // Painless script
            // Source: String hexByte(Byte b) {\n    String x = Integer.toHexString(Byte.toUnsignedInt(b));\n    if (x.length() < 2) {\n        x = \"0\" + x;\n    }\n    return x;\n}\nString guid(String text) {\n    def bytes = Base64.getDecoder().decode(text);\n    def uid = \"\";\n    for (int i = 3; i >= 0; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 5; i > 3; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 7; i > 5; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 8; i < bytes.length; i++) {\n        if (i == 10) {\n            uid += \"-\";\n        }\n        uid += hexByte(bytes[i]);\n    }\n    return uid;\n}\nString sid(String text) {\n    def bytes = Base64.getDecoder().decode(text);\n    def uid = \"S-\"+Byte.toString(bytes[0])+\"-\";\n    int auth = 0;\n    for (int i = 2; i < 8; i++) {\n        auth |= Byte.toUnsignedInt(bytes[i])<<(8*(5-(i-2)));\n    }\n    uid += Integer.toString(auth);\n    int subauths = Byte.toUnsignedInt(bytes[1]);\n    int off = 8;\n    for (int i = 0; i < subauths; i++) {\n        int subauth = 0;\n        for (int k = 0; k < 4; k++) {\n            subauth |= (Byte.toUnsignedInt(bytes[off+k])&0xff)<<(8*k);\n        }\n        uid += \"-\"+Integer.toUnsignedString(subauth);\n        off += 4;\n    }\n    return uid;\n}\ndef renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (value instanceof String) {\n        if (key == \"objectGUID\") {\n          value = guid(value);\n        } else if (key == \"objectSid\") {\n          value = sid(value);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\nctx.activedirectory = renameKeys(ctx.activedirectory, params)\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"String hexByte(Byte b) {\n    String x = Integer.toHexString(Byte.toUnsignedInt(b));\n    if (x.length() < 2) {\n        x = \"0\" + x;\n    }\n    return x;\n}\nString guid(String text) {\n    def bytes = Base64.getDecoder().decode(text);\n    def uid = \"\";\n    for (int i = 3; i >= 0; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 5; i > 3; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 7; i > 5; i--) {\n        uid += hexByte(bytes[i]);\n    }\n    uid += \"-\";\n    for (int i = 8; i < bytes.length; i++) {\n        if (i == 10) {\n            uid += \"-\";\n        }\n        uid += hexByte(bytes[i]);\n    }\n    return uid;\n}\nString sid(String text) {\n    def bytes = Base64.getDecoder().decode(text);\n    def uid = \"S-\"+Byte.toString(bytes[0])+\"-\";\n    int auth = 0;\n    for (int i = 2; i < 8; i++) {\n        auth |= Byte.toUnsignedInt(bytes[i])<<(8*(5-(i-2)));\n    }\n    uid += Integer.toString(auth);\n    int subauths = Byte.toUnsignedInt(bytes[1]);\n    int off = 8;\n    for (int i = 0; i < subauths; i++) {\n        int subauth = 0;\n        for (int k = 0; k < 4; k++) {\n            subauth |= (Byte.toUnsignedInt(bytes[off+k])&0xff)<<(8*k);\n        }\n        uid += \"-\"+Integer.toUnsignedString(subauth);\n        off += 4;\n    }\n    return uid;\n}\ndef renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (value instanceof String) {\n        if (key == \"objectGUID\") {\n          value = guid(value);\n        } else if (key == \"objectSid\") {\n          value = sid(value);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\nctx.activedirectory = renameKeys(ctx.activedirectory, params)\n"#
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
                // Painless script
                // Source: Long newUacValue = Long.decode(ctx.activedirectory.device.user_account_control);\nArrayList uacResult = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((newUacValue.longValue() & flag.longValue()) != 0) {\n    uacResult.add(entry.getValue());\n  }\n}\nif (uacResult.length != 0) {\n  ctx.activedirectory.device.uac_list = uacResult;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"Long newUacValue = Long.decode(ctx.activedirectory.device.user_account_control);\nArrayList uacResult = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((newUacValue.longValue() & flag.longValue()) != 0) {\n    uacResult.add(entry.getValue());\n  }\n}\nif (uacResult.length != 0) {\n  ctx.activedirectory.device.uac_list = uacResult;\n}"#
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
                        serde_json::Value::String(s) => s.contains("ENCRYPTED_TEXT_PWD_ALLOWED"),
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
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION")),
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
                    // Painless script
                    // Source: ctx.user = ctx.user ?: [:];\nctx.user.group = ctx.user.group ?: [:];\n\nfor (def group : ctx.activedirectory.groups) {\n  if (group.name != null) {\n    ctx.user.group.name = ctx.user.group.name ?: new HashSet();\n    ctx.user.group.name.add(group.name);\n  }\n\n  if (group?.object_sid == null) {\n    continue;\n  }\n\n  group.id = group.object_sid;\n  ctx.user.group.id = ctx.user.group.id ?: new HashSet();\n  ctx.user.group.id.add(group.id);\n\n  int idx = group.object_sid.lastIndexOf('-');\n  if (idx < 0) {\n    continue;\n  }\n  def priv = params.get(group.object_sid.substring(idx+1));\n  if (priv != null && ctx.activedirectory.device.privileged_group_member == null) {\n    ctx.activedirectory.device.privileged_group_member = priv;\n  }\n}\nif (ctx.activedirectory.device.privileged_group_member == null) {\n  ctx.activedirectory.device.privileged_group_member = false;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.user = ctx.user ?: [:];\nctx.user.group = ctx.user.group ?: [:];\n\nfor (def group : ctx.activedirectory.groups) {\n  if (group.name != null) {\n    ctx.user.group.name = ctx.user.group.name ?: new HashSet();\n    ctx.user.group.name.add(group.name);\n  }\n\n  if (group?.object_sid == null) {\n    continue;\n  }\n\n  group.id = group.object_sid;\n  ctx.user.group.id = ctx.user.group.id ?: new HashSet();\n  ctx.user.group.id.add(group.id);\n\n  int idx = group.object_sid.lastIndexOf('-');\n  if (idx < 0) {\n    continue;\n  }\n  def priv = params.get(group.object_sid.substring(idx+1));\n  if (priv != null && ctx.activedirectory.device.privileged_group_member == null) {\n    ctx.activedirectory.device.privileged_group_member = priv;\n  }\n}\nif (ctx.activedirectory.device.privileged_group_member == null) {\n  ctx.activedirectory.device.privileged_group_member = false;\n}"#
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
                    // Painless script
                    // Source: def parseDn(def dn) {\n  def result = ['id': dn];\n  int start = 0;\n  def dcParts = new ArrayList();\n  boolean cnFound = false;\n  while (start < dn.length()) {\n    int end = start;\n    while (true) {\n      end = dn.indexOf(',', end);\n      if (end == -1) { end = dn.length(); break; }\n      if (end > 0 && dn.charAt(end - 1) == (char)'\\\\') { end++; continue; }\n      break;\n    }\n    def part = dn.substring(start, end).replace('\\\\,', ',').trim();\n    if (!cnFound && part.length() > 3 && part.substring(0, 3).equalsIgnoreCase('CN=')) {\n      result['name'] = part.substring(3);\n      cnFound = true;\n    } else if (part.length() > 3 && part.substring(0, 3).equalsIgnoreCase('DC=')) {\n      dcParts.add(part.substring(3));\n    }\n    start = end + 1;\n  }\n  if (!dcParts.isEmpty()) {\n    def domain = dcParts[0];\n    for (int i = 1; i < dcParts.size(); i++) {\n      domain += '.' + dcParts[i];\n    }\n    result['domain'] = domain;\n  }\n  return result;\n}\ndef buildHostRel(def dns) {\n  def rel = ['host': [:]];\n  def dnList = (dns instanceof List) ? dns : [dns];\n  for (def dn : dnList) {\n    if (!(dn instanceof String) || dn.isEmpty()) continue;\n    def p = parseDn(dn);\n    if (!rel['host'].containsKey('id')) rel['host']['id'] = new ArrayList();\n    rel['host']['id'].add(p['id']);\n    if (p.containsKey('name')) {\n      if (!rel['host'].containsKey('name')) rel['host']['name'] = new ArrayList();\n      def fqdn = p.containsKey('domain') ? p['name'].toLowerCase() + '.' + p['domain'] : p['name'];\n      rel['host']['name'].add(fqdn);\n    }\n    if (p.containsKey('domain')) {\n      if (!rel.containsKey('user')) rel['user'] = [:];\n      if (!rel['user'].containsKey('domain')) rel['user']['domain'] = new ArrayList();\n      rel['user']['domain'].add(p['domain']);\n    }\n  }\n  return rel;\n}\ndef buildUserRel(def dns) {\n  def rel = ['user': [:]];\n  def dnList = (dns instanceof List) ? dns : [dns];\n  for (def dn : dnList) {\n    if (!(dn instanceof String) || dn.isEmpty()) continue;\n    def p = parseDn(dn);\n    if (!rel['user'].containsKey('id')) rel['user']['id'] = new ArrayList();\n    rel['user']['id'].add(p['id']);\n    if (p.containsKey('name')) {\n      if (!rel['user'].containsKey('name')) rel['user']['name'] = new ArrayList();\n      rel['user']['name'].add(p['name']);\n    }\n    if (p.containsKey('domain')) {\n      if (!rel['user'].containsKey('domain')) rel['user']['domain'] = new ArrayList();\n      rel['user']['domain'].add(p['domain']);\n    }\n  }\n  return rel;\n}\nctx.host = ctx.host ?: [:];\nctx.host.entity = ctx.host.entity ?: [:];\nctx.host.entity.relationships = ctx.host.entity.relationships ?: [:];\nif (ctx.activedirectory?.device?.managed_objects != null) {\n  ctx.host.entity.relationships.administers = buildHostRel(ctx.activedirectory.device.managed_objects);\n}\nif (ctx.activedirectory?.device?.direct_reports != null) {\n  ctx.host.entity.relationships.supervises = buildUserRel(ctx.activedirectory.device.direct_reports);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def parseDn(def dn) {\n  def result = ['id': dn];\n  int start = 0;\n  def dcParts = new ArrayList();\n  boolean cnFound = false;\n  while (start < dn.length()) {\n    int end = start;\n    while (true) {\n      end = dn.indexOf(',', end);\n      if (end == -1) { end = dn.length(); break; }\n      if (end > 0 && dn.charAt(end - 1) == (char)'\\\\') { end++; continue; }\n      break;\n    }\n    def part = dn.substring(start, end).replace('\\\\,', ',').trim();\n    if (!cnFound && part.length() > 3 && part.substring(0, 3).equalsIgnoreCase('CN=')) {\n      result['name'] = part.substring(3);\n      cnFound = true;\n    } else if (part.length() > 3 && part.substring(0, 3).equalsIgnoreCase('DC=')) {\n      dcParts.add(part.substring(3));\n    }\n    start = end + 1;\n  }\n  if (!dcParts.isEmpty()) {\n    def domain = dcParts[0];\n    for (int i = 1; i < dcParts.size(); i++) {\n      domain += '.' + dcParts[i];\n    }\n    result['domain'] = domain;\n  }\n  return result;\n}\ndef buildHostRel(def dns) {\n  def rel = ['host': [:]];\n  def dnList = (dns instanceof List) ? dns : [dns];\n  for (def dn : dnList) {\n    if (!(dn instanceof String) || dn.isEmpty()) continue;\n    def p = parseDn(dn);\n    if (!rel['host'].containsKey('id')) rel['host']['id'] = new ArrayList();\n    rel['host']['id'].add(p['id']);\n    if (p.containsKey('name')) {\n      if (!rel['host'].containsKey('name')) rel['host']['name'] = new ArrayList();\n      def fqdn = p.containsKey('domain') ? p['name'].toLowerCase() + '.' + p['domain'] : p['name'];\n      rel['host']['name'].add(fqdn);\n    }\n    if (p.containsKey('domain')) {\n      if (!rel.containsKey('user')) rel['user'] = [:];\n      if (!rel['user'].containsKey('domain')) rel['user']['domain'] = new ArrayList();\n      rel['user']['domain'].add(p['domain']);\n    }\n  }\n  return rel;\n}\ndef buildUserRel(def dns) {\n  def rel = ['user': [:]];\n  def dnList = (dns instanceof List) ? dns : [dns];\n  for (def dn : dnList) {\n    if (!(dn instanceof String) || dn.isEmpty()) continue;\n    def p = parseDn(dn);\n    if (!rel['user'].containsKey('id')) rel['user']['id'] = new ArrayList();\n    rel['user']['id'].add(p['id']);\n    if (p.containsKey('name')) {\n      if (!rel['user'].containsKey('name')) rel['user']['name'] = new ArrayList();\n      rel['user']['name'].add(p['name']);\n    }\n    if (p.containsKey('domain')) {\n      if (!rel['user'].containsKey('domain')) rel['user']['domain'] = new ArrayList();\n      rel['user']['domain'].add(p['domain']);\n    }\n  }\n  return rel;\n}\nctx.host = ctx.host ?: [:];\nctx.host.entity = ctx.host.entity ?: [:];\nctx.host.entity.relationships = ctx.host.entity.relationships ?: [:];\nif (ctx.activedirectory?.device?.managed_objects != null) {\n  ctx.host.entity.relationships.administers = buildHostRel(ctx.activedirectory.device.managed_objects);\n}\nif (ctx.activedirectory?.device?.direct_reports != null) {\n  ctx.host.entity.relationships.supervises = buildUserRel(ctx.activedirectory.device.direct_reports);\n}"#
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
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
