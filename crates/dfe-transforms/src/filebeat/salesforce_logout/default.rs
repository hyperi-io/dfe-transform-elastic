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
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "message", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to parse JSON: {}",
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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("event.provider") == Some("Object") };
            if _cond {
                // Begin nested pipeline: "object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EventDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EventDate".into(),
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
                        json!(format!(
                            "Failed to parse EventDate field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.RelatedEventIdentifier") {
                        event.rename(
                            "json.RelatedEventIdentifier",
                            "salesforce.logout.related_event_identifier",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginKey") {
                        event.rename("json.LoginKey", "salesforce.logout.login_key")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.SessionLevel") {
                        event.rename("json.SessionLevel", "salesforce.logout.session.level")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.SessionKey") {
                        event.rename("json.SessionKey", "salesforce.logout.session.key")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.CreatedDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.CreatedDate".into(),
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
                        json!(format!(
                            "Failed to parse CreatedDate field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Username") {
                        event.rename("json.Username", "user.email")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.UserId") {
                        event.rename("json.UserId", "user.id")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SourceIp") {
                        if let Some(val) = event.get("json.SourceIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SourceIp".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to convert SourceIp to IP: {}",
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
                // End nested pipeline: "object"
            }

            let _cond = { event.get_str("event.provider") == Some("EventLogFile") };
            if _cond {
                // Begin nested pipeline: "eventlogfile"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TIMESTAMP_DERIVED") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TIMESTAMP_DERIVED".into(),
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
                        json!(format!(
                            "Failed to parse TIMESTAMP_DERIVED field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.SESSION_TYPE") {
                        event.rename("json.SESSION_TYPE", "salesforce.logout.session.type")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def sessionTypes = [\n  \"A\": \"API\",\n  \"I\": \"APIOnlyUser\",\n  \"N\": \"ChatterNetworks\",\n  \"Z\": \"ChatterNetworksAPIOnly\",\n  \"C\": \"Content\",\n  \"P\": \"OauthApprovalUI\",\n  \"O\": \"Oauth2\",\n  \"T\": \"SiteStudio\",\n  \"R\": \"SitePreview\",\n  \"S\": \"SubstituteUser\",\n  \"B\": \"TempContentExchange\",\n  \"G\": \"TempOauthAccessTokenFrontdoor\",\n  \"Y\": \"TempVisualforceExchange\",\n  \"F\": \"TempUIFrontdoor\",\n  \"U\": \"UI\",\n  \"E\": \"UserSite\",\n  \"V\": \"Visualforce\",\n  \"W\": \"WDC_API\"\n];\ndef type = ctx.salesforce?.logout?.session?.type;\nif (type != null && sessionTypes.containsKey(type)) {\n  ctx.salesforce.logout.session.type = sessionTypes.get(type);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def sessionTypes = [\n  \"A\": \"API\",\n  \"I\": \"APIOnlyUser\",\n  \"N\": \"ChatterNetworks\",\n  \"Z\": \"ChatterNetworksAPIOnly\",\n  \"C\": \"Content\",\n  \"P\": \"OauthApprovalUI\",\n  \"O\": \"Oauth2\",\n  \"T\": \"SiteStudio\",\n  \"R\": \"SitePreview\",\n  \"S\": \"SubstituteUser\",\n  \"B\": \"TempContentExchange\",\n  \"G\": \"TempOauthAccessTokenFrontdoor\",\n  \"Y\": \"TempVisualforceExchange\",\n  \"F\": \"TempUIFrontdoor\",\n  \"U\": \"UI\",\n  \"E\": \"UserSite\",\n  \"V\": \"Visualforce\",\n  \"W\": \"WDC_API\"\n];\ndef type = ctx.salesforce?.logout?.session?.type;\nif (type != null && sessionTypes.containsKey(type)) {\n  ctx.salesforce.logout.session.type = sessionTypes.get(type);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.session.type: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.EVENT_TYPE") {
                        event.rename("json.EVENT_TYPE", "salesforce.logout.event_type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.SESSION_LEVEL") {
                        event.rename("json.SESSION_LEVEL", "salesforce.logout.session.level")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def levels = [\"1\": \"Standard Session\", \"2\": \"High-Assurance Session\"];\ndef level = ctx.salesforce?.logout?.session?.level;\nif (level != null && levels.containsKey(level)) {\n  ctx.salesforce.logout.session.level = levels.get(level);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def levels = [\"1\": \"Standard Session\", \"2\": \"High-Assurance Session\"];\ndef level = ctx.salesforce?.logout?.session?.level;\nif (level != null && levels.containsKey(level)) {\n  ctx.salesforce.logout.session.level = levels.get(level);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.session.level: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.BROWSER_TYPE") {
                        event.rename("json.BROWSER_TYPE", "salesforce.logout.browser_type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.PLATFORM_TYPE") {
                        event.rename("json.PLATFORM_TYPE", "salesforce.logout.platform_type")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def platform = ctx.salesforce?.logout?.platform_type;\nif (platform != null && params.platforms.containsKey(platform)) {\n  ctx.salesforce.logout.platform_type = params.platforms.get(platform);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def platform = ctx.salesforce?.logout?.platform_type;\nif (platform != null && params.platforms.containsKey(platform)) {\n  ctx.salesforce.logout.platform_type = params.platforms.get(platform);\n}\n"#
                        ),
                        cached_params!(
                            "{\"platforms\":{\"1000\":\"Windows\",\"1008\":\"Windows 2003\",\"1013\":\"Windows 8.1\",\"1015\":\"Windows 10\",\"2003\":\"Macintosh/Apple OSX\",\"4000\":\"Linux\",\"5005\":\"Android\",\"5006\":\"iPhone\",\"5007\":\"iPad\",\"5200\":\"Android 10.0\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.platform_type: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.RESOLUTION_TYPE") {
                        event
                            .rename("json.RESOLUTION_TYPE", "salesforce.logout.resolution_type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.APP_TYPE") {
                        event.rename("json.APP_TYPE", "salesforce.logout.app_type")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def type = ctx.salesforce?.logout?.app_type;\nif (type != null && params.appTypes.containsKey(type)) {\n  ctx.salesforce.logout.app_type = params.appTypes.get(type);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def type = ctx.salesforce?.logout?.app_type;\nif (type != null && params.appTypes.containsKey(type)) {\n  ctx.salesforce.logout.app_type = params.appTypes.get(type);\n}\n"#
                        ),
                        cached_params!(
                            "{\"appTypes\":{\"1000\":\"Application\",\"1007\":\"SFDC Application\",\"1014\":\"Chat\",\"2501\":\"CTI\",\"2514\":\"OAuth\",\"3475\":\"SFDC Partner Portal\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.app_type: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.CLIENT_VERSION") {
                        event.rename("json.CLIENT_VERSION", "salesforce.logout.client_version")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.API_TYPE") {
                        event.rename("json.API_TYPE", "salesforce.logout.api.type")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def type = ctx.salesforce?.logout?.api?.type;\nif (type != null && params.apiTypes.containsKey(type)) {\n  ctx.salesforce.logout.api.type = params.apiTypes.get(type);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def type = ctx.salesforce?.logout?.api?.type;\nif (type != null && params.apiTypes.containsKey(type)) {\n  ctx.salesforce.logout.api.type = params.apiTypes.get(type);\n}\n"#
                        ),
                        cached_params!(
                            "{\"apiTypes\":{\"D\":\"Apex Class\",\"E\":\"SOAP Enterprise\",\"I\":\"SOAP Cross Instance\",\"M\":\"SOAP Metadata\",\"O\":\"Old SOAP\",\"P\":\"SOAP Partner\",\"S\":\"SOAP Apex\",\"T\":\"SOAP Tooling\",\"X\":\"XmlRPC\",\"f\":\"Feed\",\"l\":\"Live Agent\",\"p\":\"SOAP ClientSync\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.api.type: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.API_VERSION") {
                        event.rename("json.API_VERSION", "salesforce.logout.api.version")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("salesforce.logout.user_initiated_logout", json!(false))?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("json.USER_INITIATED_LOGOUT")
                        && event.get_str("json.USER_INITIATED_LOGOUT") == Some("1")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("salesforce.logout.user_initiated_logout", json!(true))?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LOGIN_KEY") {
                        event.rename("json.LOGIN_KEY", "salesforce.logout.login_key")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.USER_ID") {
                        event.rename("json.USER_ID", "salesforce.logout.user_id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.ORGANIZATION_ID") {
                        event
                            .rename("json.ORGANIZATION_ID", "salesforce.logout.organization_id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.REQUEST_ID") {
                        event.rename("json.REQUEST_ID", "event.code")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.USER_TYPE") {
                        event.rename("json.USER_TYPE", "salesforce.logout.user.roles")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def role = ctx.salesforce?.logout?.user?.roles;\nif (role != null && params.userRoles.containsKey(role)) {\n  ctx.salesforce.logout.user.roles = [params.userRoles.get(role)];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def role = ctx.salesforce?.logout?.user?.roles;\nif (role != null && params.userRoles.containsKey(role)) {\n  ctx.salesforce.logout.user.roles = [params.userRoles.get(role)];\n}\n"#
                        ),
                        cached_params!(
                            "{\"userRoles\":{\"A\":\"Automated Process\",\"b\":\"High Volume Portal\",\"C\":\"Customer Portal User\",\"D\":\"External Who\",\"F\":\"Self-Service\",\"G\":\"Guest\",\"L\":\"Package License Manager\",\"N\":\"Salesforce to Salesforce\",\"n\":\"CSN Only\",\"O\":\"Power Custom\",\"o\":\"Custom\",\"P\":\"Partner\",\"p\":\"Customer Portal Manager\",\"S\":\"Standard\",\"X\":\"Salesforce Administrator\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.logout.user.roles: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.USER_ID_DERIVED") {
                        event.rename("json.USER_ID_DERIVED", "user.id")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("json.CLIENT_IP")
                        && event.get_str("json.CLIENT_IP") != Some("Salesforce.com IP")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.CLIENT_IP") {
                            event.rename("json.CLIENT_IP", "source.ip")?;
                        }
                        Ok(())
                    })();
                }
                // End nested pipeline: "eventlogfile"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("event");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("logout");
                if !painless_is_empty_value(&v) {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("authentication")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("salesforce.logout");
                if !painless_is_empty_value(&v) {
                    event.set("event.dataset", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("salesforce");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return ((Map) object).isEmpty();\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return ((List) object).isEmpty();\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
                event.remove("message");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set(
                    "error.type",
                    json!(
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
