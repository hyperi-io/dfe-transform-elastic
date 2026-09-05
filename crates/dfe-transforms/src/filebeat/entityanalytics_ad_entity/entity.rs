// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `entity` pipeline.
pub struct Entity;

impl Transform for Entity {
    fn name(&self) -> &str {
        "entity"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("asset"))?;

            event.set("event.category", Value::Array(vec![json!("iam")]))?;

            event.set(
                "event.type",
                Value::Array(vec![json!("user"), json!("info")]),
            )?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("activedirectory_entity"))?;

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
