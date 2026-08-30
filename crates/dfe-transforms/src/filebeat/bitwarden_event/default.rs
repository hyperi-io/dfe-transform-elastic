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

            event.set("event.kind", json!("event"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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

            let _cond = {
                event.has_value("json.data")
                    && event.get("json.data").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.actingUserId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.collectionId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.date") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.groupId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.installationId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.itemId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.memberId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.policyId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.object") {
                event.rename("json.object", "bitwarden.object")?;
            }

            let _cond = { event.get_str("json.type") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.type") {
                        if let Some(val) = event.get("json.type") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.type".into(),
                                    message,
                                }
                            })?;
                            event.set("bitwarden.event.type.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_type_to_string")?;
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

            // Painless script
            // Source: if (ctx.bitwarden?.event?.type?.value == null || params.get(ctx.bitwarden.event.type.value) == null) {\n  return;\n}\ndef hm = new HashMap(params.get(ctx.bitwarden.event.type.value));\nctx.bitwarden.event.type.name = hm.name;\nctx.event.category = hm.category;\nctx.event.type = hm.type;\nctx.event.outcome = hm.outcome;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.bitwarden?.event?.type?.value == null || params.get(ctx.bitwarden.event.type.value) == null) {\n  return;\n}\ndef hm = new HashMap(params.get(ctx.bitwarden.event.type.value));\nctx.bitwarden.event.type.name = hm.name;\nctx.event.category = hm.category;\nctx.event.type = hm.type;\nctx.event.outcome = hm.outcome;"#
                ),
                cached_params!(
                    "{\"1000\":{\"category\":[\"iam\",\"authentication\"],\"type\":[\"user\",\"start\"],\"outcome\":\"success\",\"name\":\"User_LoggedIn\"},\"1001\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\",\"name\":\"User_ChangedPassword\"},\"1002\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\",\"name\":\"User_Updated2fa\"},\"1003\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\",\"name\":\"User_Disabled2fa\"},\"1004\":{\"category\":[\"iam\"],\"type\":[\"user\"],\"outcome\":\"success\",\"name\":\"User_Recovered2fa\"},\"1005\":{\"category\":[\"iam\",\"authentication\"],\"type\":[\"user\",\"info\"],\"outcome\":\"failure\",\"name\":\"User_FailedLogIn\"},\"1006\":{\"category\":[\"iam\",\"authentication\"],\"type\":[\"user\",\"info\"],\"outcome\":\"failure\",\"name\":\"User_FailedLogIn2fa\"},\"1007\":{\"category\":[\"iam\",\"database\"],\"type\":[\"user\",\"info\"],\"outcome\":\"success\",\"name\":\"User_ClientExportedVault\"},\"1008\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\",\"name\":\"User_UpdatedTempPassword\"},\"1009\":{\"category\":[\"iam\"],\"type\":[\"user\"],\"outcome\":\"success\",\"name\":\"User_MigratedKeyToKeyConnector\"},\"1100\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_Created\"},\"1101\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_Updated\"},\"1102\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_Deleted\"},\"1103\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_AttachmentCreated\"},\"1104\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_AttachmentDeleted\"},\"1105\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_Shared\"},\"1106\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_UpdatedCollections\"},\"1107\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientViewed\"},\"1108\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientToggledPasswordVisible\"},\"1109\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientToggledHiddenFieldVisible\"},\"1110\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientToggledCardCodeVisible\"},\"1111\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientCopiedPassword\"},\"1112\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientCopiedHiddenField\"},\"1113\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientCopiedCardCode\"},\"1114\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientAutofilled\"},\"1115\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_SoftDeleted\"},\"1116\":{\"category\":[\"database\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Cipher_Restored\"},\"1117\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Cipher_ClientToggledCardNumberVisible\"},\"1300\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\",\"name\":\"Collection_Created\"},\"1301\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Collection_Updated\"},\"1302\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\",\"name\":\"Collection_Deleted\"},\"1400\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"group\",\"creation\"],\"outcome\":\"success\",\"name\":\"Group_Created\"},\"1401\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"group\",\"change\"],\"outcome\":\"success\",\"name\":\"Group_Updated\"},\"1402\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"group\",\"deletion\"],\"outcome\":\"success\",\"name\":\"Group_Deleted\"},\"1500\":{\"outcome\":\"success\",\"name\":\"OrganizationUser_Invited\"},\"1501\":{\"outcome\":\"success\",\"name\":\"OrganizationUser_Confirmed\"},\"1502\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_Updated\"},\"1503\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_Removed\"},\"1504\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_UpdatedGroups\"},\"1505\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_UnlinkedSso\"},\"1506\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_ResetPassword_Enroll\"},\"1507\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_ResetPassword_Withdraw\"},\"1508\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_AdminResetPassword\"},\"1509\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_ResetSsoLink\"},\"1510\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_FirstSsoLogin\"},\"1511\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_Revoked\"},\"1512\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationUser_Restored\"},\"1600\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Organization_Updated\"},\"1601\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Organization_PurgedVault\"},\"1602\":{\"category\":[\"database\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"Organization_ClientExportedVault\"},\"1603\":{\"category\":[\"database\"],\"type\":[\"access\"],\"outcome\":\"success\",\"name\":\"Organization_VaultAccessed\"},\"1604\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Organization_EnabledSso\"},\"1605\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Organization_DisabledSso\"},\"1606\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Organization_EnabledKeyConnector\"},\"1607\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Organization_DisabledKeyConnector\"},\"1608\":{\"outcome\":\"success\",\"name\":\"Organization_SponsorshipsSynced\"},\"1700\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"Policy_Updated\"},\"1800\":{\"outcome\":\"success\",\"name\":\"ProviderUser_Invited\"},\"1801\":{\"outcome\":\"success\",\"name\":\"ProviderUser_Confirmed\"},\"1802\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"ProviderUser_Updated\"},\"1803\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"ProviderUser_Removed\"},\"1900\":{\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\",\"name\":\"ProviderOrganization_Created\"},\"1901\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"ProviderOrganization_Added\"},\"1902\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"ProviderOrganization_Removed\"},\"1903\":{\"category\":[\"database\"],\"type\":[\"access\"],\"outcome\":\"success\",\"name\":\"ProviderOrganization_VaultAccessed\"},\"2000\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationDomain_Added\"},\"2001\":{\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\",\"name\":\"OrganizationDomain_Removed\"},\"2002\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"OrganizationDomain_Verified\"},\"2003\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\",\"name\":\"OrganizationDomain_NotVerified\"},\"2100\":{\"outcome\":\"success\",\"name\":\"Secret_Retrieved\"}}"
                ),
            )?;

            if event.has_value("json.itemId") {
                event.rename("json.itemId", "bitwarden.event.item.id")?;
            }

            if event.has_value("json.collectionId") {
                event.rename("json.collectionId", "bitwarden.event.collection.id")?;
            }

            if event.has_value("json.groupId") {
                event.rename("json.groupId", "bitwarden.event.group.id")?;
            }

            if let Some(v) = event
                .get("bitwarden.event.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if event.has_value("json.policyId") {
                event.rename("json.policyId", "bitwarden.event.policy.id")?;
            }

            if event.has_value("json.memberId") {
                event.rename("json.memberId", "bitwarden.event.member.id")?;
            }

            if let Some(v) = event
                .get("bitwarden.event.member.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.actingUserId") {
                event.rename("json.actingUserId", "bitwarden.event.acting_user.id")?;
            }

            if let Some(v) = event
                .get("bitwarden.event.acting_user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            if event.has_value("json.installationId") {
                event.rename("json.installationId", "bitwarden.event.installation.id")?;
            }

            let _cond = { event.has_value("json.date") && event.get_str("json.date") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
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

            let _cond = { event.has_value("json.date") && event.get_str("json.date") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("bitwarden.event.date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_event_date")?;
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

            let _cond = { event.get_str("json.device") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.device") {
                        if let Some(val) = event.get("json.device") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.device".into(),
                                    message,
                                }
                            })?;
                            event.set("bitwarden.event.device.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_to_string",
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

            let _cond = { event.has_value("bitwarden.event.device.value") };
            if _cond {
                // Painless script
                // Source: def deviceTypeValue = Integer.parseInt(ctx.bitwarden.event.device.value);\nif (deviceTypeValue >= 0 && deviceTypeValue < params.DeviceType.length) {\n  ctx.bitwarden.event.device.put('name', params['DeviceType'][deviceTypeValue]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def deviceTypeValue = Integer.parseInt(ctx.bitwarden.event.device.value);\nif (deviceTypeValue >= 0 && deviceTypeValue < params.DeviceType.length) {\n  ctx.bitwarden.event.device.put('name', params['DeviceType'][deviceTypeValue]);\n}"#
                    ),
                    cached_params!(
                        "{\"DeviceType\":[\"Android\",\"iOS\",\"ChromeExtension\",\"FirefoxExtension\",\"OperaExtension\",\"EdgeExtension\",\"WindowsDesktop\",\"MacOsDesktop\",\"LinuxDesktop\",\"ChromeBrowser\",\"FirefoxBrowser\",\"OperaBrowser\",\"EdgeBrowser\",\"IEBrowser\",\"UnknownBrowser\",\"AndroidAmazon\",\"UWP\",\"SafariBrowser\",\"VivaldiBrowser\",\"VivaldiExtension\",\"SafariExtension\",\"SDK\",\"Server\"]}"
                    ),
                )?;
            }

            let _cond = { event.get_str("json.ipAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ipAddress") {
                        if let Some(val) = event.get("json.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("bitwarden.event.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ipAddress_to_string",
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

            let _cond = { event.has_value("bitwarden.event.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("bitwarden.event.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("bitwarden.event.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("bitwarden.event.group.id");
                event.remove("bitwarden.event.member.id");
                event.remove("bitwarden.event.acting_user.id");
                event.remove("bitwarden.event.date");
                event.remove("bitwarden.event.ip_address");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
