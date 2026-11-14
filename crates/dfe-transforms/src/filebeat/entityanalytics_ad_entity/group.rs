// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `group` pipeline.
pub struct Group;

impl Transform for Group {
    fn name(&self) -> &str {
        "group"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
