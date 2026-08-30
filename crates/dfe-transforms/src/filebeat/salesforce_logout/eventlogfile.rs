// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `eventlogfile` pipeline.
pub struct Eventlogfile;

impl Transform for Eventlogfile {
    fn name(&self) -> &str {
        "eventlogfile"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
                        event.append("error.message", json!(format!("Failed to parse TIMESTAMP_DERIVED field: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                painless_exec_plan(event, cached_painless!(r#"def sessionTypes = [\n  \"A\": \"API\",\n  \"I\": \"APIOnlyUser\",\n  \"N\": \"ChatterNetworks\",\n  \"Z\": \"ChatterNetworksAPIOnly\",\n  \"C\": \"Content\",\n  \"P\": \"OauthApprovalUI\",\n  \"O\": \"Oauth2\",\n  \"T\": \"SiteStudio\",\n  \"R\": \"SitePreview\",\n  \"S\": \"SubstituteUser\",\n  \"B\": \"TempContentExchange\",\n  \"G\": \"TempOauthAccessTokenFrontdoor\",\n  \"Y\": \"TempVisualforceExchange\",\n  \"F\": \"TempUIFrontdoor\",\n  \"U\": \"UI\",\n  \"E\": \"UserSite\",\n  \"V\": \"Visualforce\",\n  \"W\": \"WDC_API\"\n];\ndef type = ctx.salesforce?.logout?.session?.type;\nif (type != null && sessionTypes.containsKey(type)) {\n  ctx.salesforce.logout.session.type = sessionTypes.get(type);\n}\n"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.session.type: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                painless_exec_plan(event, cached_painless!(r#"def levels = [\"1\": \"Standard Session\", \"2\": \"High-Assurance Session\"];\ndef level = ctx.salesforce?.logout?.session?.level;\nif (level != null && levels.containsKey(level)) {\n  ctx.salesforce.logout.session.level = levels.get(level);\n}\n"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.session.level: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                painless_exec_plan_params(event, cached_painless!(r#"def platform = ctx.salesforce?.logout?.platform_type;\nif (platform != null && params.platforms.containsKey(platform)) {\n  ctx.salesforce.logout.platform_type = params.platforms.get(platform);\n}\n"#), cached_params!("{\"platforms\":{\"1000\":\"Windows\",\"1008\":\"Windows 2003\",\"1013\":\"Windows 8.1\",\"1015\":\"Windows 10\",\"2003\":\"Macintosh/Apple OSX\",\"4000\":\"Linux\",\"5005\":\"Android\",\"5006\":\"iPhone\",\"5007\":\"iPad\",\"5200\":\"Android 10.0\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.platform_type: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    event.rename("json.RESOLUTION_TYPE", "salesforce.logout.resolution_type")?;
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
                painless_exec_plan_params(event, cached_painless!(r#"def type = ctx.salesforce?.logout?.app_type;\nif (type != null && params.appTypes.containsKey(type)) {\n  ctx.salesforce.logout.app_type = params.appTypes.get(type);\n}\n"#), cached_params!("{\"appTypes\":{\"1000\":\"Application\",\"1007\":\"SFDC Application\",\"1014\":\"Chat\",\"2501\":\"CTI\",\"2514\":\"OAuth\",\"3475\":\"SFDC Partner Portal\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.app_type: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                painless_exec_plan_params(event, cached_painless!(r#"def type = ctx.salesforce?.logout?.api?.type;\nif (type != null && params.apiTypes.containsKey(type)) {\n  ctx.salesforce.logout.api.type = params.apiTypes.get(type);\n}\n"#), cached_params!("{\"apiTypes\":{\"D\":\"Apex Class\",\"E\":\"SOAP Enterprise\",\"I\":\"SOAP Cross Instance\",\"M\":\"SOAP Metadata\",\"O\":\"Old SOAP\",\"P\":\"SOAP Partner\",\"S\":\"SOAP Apex\",\"T\":\"SOAP Tooling\",\"X\":\"XmlRPC\",\"f\":\"Feed\",\"l\":\"Live Agent\",\"p\":\"SOAP ClientSync\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.api.type: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.has_value("json.USER_INITIATED_LOGOUT") && event.get_str("json.USER_INITIATED_LOGOUT") == Some("1") };
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
                    event.rename("json.ORGANIZATION_ID", "salesforce.logout.organization_id")?;
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
                painless_exec_plan_params(event, cached_painless!(r#"def role = ctx.salesforce?.logout?.user?.roles;\nif (role != null && params.userRoles.containsKey(role)) {\n  ctx.salesforce.logout.user.roles = [params.userRoles.get(role)];\n}\n"#), cached_params!("{\"userRoles\":{\"A\":\"Automated Process\",\"b\":\"High Volume Portal\",\"C\":\"Customer Portal User\",\"D\":\"External Who\",\"F\":\"Self-Service\",\"G\":\"Guest\",\"L\":\"Package License Manager\",\"N\":\"Salesforce to Salesforce\",\"n\":\"CSN Only\",\"O\":\"Power Custom\",\"o\":\"Custom\",\"P\":\"Partner\",\"p\":\"Customer Portal Manager\",\"S\":\"Standard\",\"X\":\"Salesforce Administrator\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.logout.user.roles: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.has_value("json.CLIENT_IP") && event.get_str("json.CLIENT_IP") != Some("Salesforce.com IP") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.CLIENT_IP") {
                    event.rename("json.CLIENT_IP", "source.ip")?;
                }
                Ok(())
            })();
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
