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

            if event.has_value("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.message = ctx.message.replace(params.empty_field_name, '')
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.message = ctx.message.replace(params.empty_field_name, '')"#
                    ),
                    cached_params!("{\"empty_field_name\":\"\\\"\\\":\\\"\\\",\"}"),
                )?;
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "azure.app_service")?;
                Ok(())
            })();

            event.set(
                "azure.app_service.properties_raw",
                json!(
                    event
                        .get("azure.app_service.properties")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event
                    .get("azure.app_service.properties")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("azure.app_service.properties")
                        .is_some_and(|s| s.starts_with("{"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "azure.app_service.properties",
                        "azure.app_service.properties",
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.app_service.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.app_service.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.app_service.time");

            let _cond =
                { event.get_str("azure.app_service.Category") == Some("AppServiceAuditLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-auditlogs-inner-pipeline"
                if event.has_value("azure.app_service.ResourceId") {
                    event.rename("azure.app_service.ResourceId", "azure.resource.id")?;
                }
                event.rename("azure.app_service.Category", "azure.app_service.category")?;
                event.rename(
                    "azure.app_service.OperationName",
                    "azure.app_service.operation_name",
                )?;
                if event.has_value("azure.app_service.Properties") {
                    event.rename(
                        "azure.app_service.Properties",
                        "azure.app_service.properties",
                    )?;
                }
                event.rename(
                    "azure.app_service.properties.Protocol",
                    "azure.app_service.properties.protocol",
                )?;
                event.rename(
                    "azure.app_service.properties.User",
                    "azure.app_service.properties.user",
                )?;
                event.rename(
                    "azure.app_service.properties.UserAddress",
                    "azure.app_service.properties.client_ip",
                )?;
                event.rename(
                    "azure.app_service.properties.UserDisplayName",
                    "azure.app_service.properties.user_display_name",
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.app_service.properties.client_ip")
                    {
                        // Grok pattern: %{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}
                        if !cached_grok!("%{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
                if event.has_value("azure.app_service.properties.client_ip") {
                    if let Some(val) = event.get("azure.app_service.properties.client_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.app_service.properties.client_ip".into(),
                                message,
                            }
                        })?;
                        event.set("azure.app_service.properties.client_ip", converted)?;
                    }
                }
                // End nested pipeline: "appservice-auditlogs-inner-pipeline"
            }

            let _cond =
                { event.get_str("azure.app_service.category") == Some("AppServiceHTTPLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-httplogs-inner-pipeline"
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }
                event.rename(
                    "azure.app_service.properties.CIp",
                    "azure.app_service.properties.client_ip",
                )?;
                if event.has_value("azure.app_service.properties.Protocol") {
                    event.rename(
                        "azure.app_service.properties.Protocol",
                        "azure.app_service.properties.protocol",
                    )?;
                }
                event.rename(
                    "azure.app_service.properties.ComputerName",
                    "azure.app_service.properties.computer_name",
                )?;
                event.rename(
                    "azure.app_service.properties.Cookie",
                    "azure.app_service.properties.cookie",
                )?;
                event.rename(
                    "azure.app_service.properties.CsBytes",
                    "azure.app_service.properties.cs_bytes",
                )?;
                if event.has_value("azure.app_service.properties.cs_bytes") {
                    if let Some(val) = event.get("azure.app_service.properties.cs_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.app_service.properties.cs_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("azure.app_service.properties.cs_bytes", converted)?;
                    }
                }
                event.rename(
                    "azure.app_service.properties.CsHost",
                    "azure.app_service.properties.cs_host",
                )?;
                event.rename(
                    "azure.app_service.properties.CsMethod",
                    "azure.app_service.properties.cs_method",
                )?;
                event.rename(
                    "azure.app_service.properties.CsUriQuery",
                    "azure.app_service.properties.cs_uri_query",
                )?;
                event.rename(
                    "azure.app_service.properties.CsUriStem",
                    "azure.app_service.properties.cs_uri_stem",
                )?;
                event.rename(
                    "azure.app_service.properties.CsUsername",
                    "azure.app_service.properties.cs_username",
                )?;
                event.rename(
                    "azure.app_service.properties.Referer",
                    "azure.app_service.properties.referer",
                )?;
                event.rename(
                    "azure.app_service.properties.Result",
                    "azure.app_service.properties.result",
                )?;
                event.rename(
                    "azure.app_service.properties.SPort",
                    "azure.app_service.properties.s_port",
                )?;
                event.rename(
                    "azure.app_service.properties.ScBytes",
                    "azure.app_service.properties.sc_bytes",
                )?;
                if event.has_value("azure.app_service.properties.sc_bytes") {
                    if let Some(val) = event.get("azure.app_service.properties.sc_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.app_service.properties.sc_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("azure.app_service.properties.sc_bytes", converted)?;
                    }
                }
                event.rename(
                    "azure.app_service.properties.ScStatus",
                    "azure.app_service.properties.sc_status",
                )?;
                if event.has_value("azure.app_service.properties.sc_status") {
                    if let Some(val) = event.get("azure.app_service.properties.sc_status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.app_service.properties.sc_status".into(),
                                message,
                            }
                        })?;
                        event.set("azure.app_service.properties.sc_status", converted)?;
                    }
                }
                if event.has_value("azure.app_service.properties.ScSubStatus") {
                    event.rename(
                        "azure.app_service.properties.ScSubStatus",
                        "azure.app_service.properties.sc_substatus",
                    )?;
                }
                if event.has_value("azure.app_service.properties.ScWin32Status") {
                    event.rename(
                        "azure.app_service.properties.ScWin32Status",
                        "azure.app_service.properties.sc_win32status",
                    )?;
                }
                event.rename(
                    "azure.app_service.properties.TimeTaken",
                    "azure.app_service.properties.time_taken",
                )?;
                event.rename(
                    "azure.app_service.properties.UserAgent",
                    "azure.app_service.properties.user_agent",
                )?;
                if event.has_value("azure.app_service.EventIpAddress") {
                    event.rename(
                        "azure.app_service.EventIpAddress",
                        "azure.app_service.event_ip_address",
                    )?;
                }
                if event.has_value("azure.app_service.EventPrimaryStampName") {
                    event.rename(
                        "azure.app_service.EventPrimaryStampName",
                        "azure.app_service.event_primary_stamp_name",
                    )?;
                }
                if event.has_value("azure.app_service.EventStampName") {
                    event.rename(
                        "azure.app_service.EventStampName",
                        "azure.app_service.event_stamp_name",
                    )?;
                }
                if event.has_value("azure.app_service.EventStampType") {
                    event.rename(
                        "azure.app_service.EventStampType",
                        "azure.app_service.event_stamp_type",
                    )?;
                }
                if event.has_value("azure.app_service.Host") {
                    event.rename("azure.app_service.Host", "azure.app_service.host")?;
                }
                event.remove("azure.app_service.EventTime");
                // End nested pipeline: "appservice-httplogs-inner-pipeline"
            }

            let _cond =
                { event.get_str("azure.app_service.Category") == Some("AppServiceIPSecAuditLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-ipsecauditlogs-inner-pipeline"
                if event.has_value("azure.app_service.ResourceId") {
                    event.rename("azure.app_service.ResourceId", "azure.resource.id")?;
                }
                event.rename("azure.app_service.Category", "azure.app_service.category")?;
                event.rename(
                    "azure.app_service.OperationName",
                    "azure.app_service.operation_name",
                )?;
                if event.has_value("azure.app_service.Properties") {
                    event.rename(
                        "azure.app_service.Properties",
                        "azure.app_service.properties",
                    )?;
                }
                event.rename(
                    "azure.app_service.properties.CIp",
                    "azure.app_service.properties.client_ip",
                )?;
                event.rename(
                    "azure.app_service.properties.CsHost",
                    "azure.app_service.properties.cs_host",
                )?;
                event.rename(
                    "azure.app_service.properties.Result",
                    "azure.app_service.properties.result",
                )?;
                event.rename(
                    "azure.app_service.properties.Details",
                    "azure.app_service.properties.details",
                )?;
                event.rename(
                    "azure.app_service.properties.ServiceEndpoint",
                    "azure.app_service.properties.service_endpoint",
                )?;
                event.rename(
                    "azure.app_service.properties.XForwardedFor",
                    "azure.app_service.properties.xforwarded_for",
                )?;
                event.rename(
                    "azure.app_service.properties.XForwardedHost",
                    "azure.app_service.properties.xforwarded_host",
                )?;
                event.rename(
                    "azure.app_service.properties.XAzureFDID",
                    "azure.app_service.properties.xazurefdid",
                )?;
                event.rename(
                    "azure.app_service.properties.XFDHealthProbe",
                    "azure.app_service.properties.xfdhealth_probe",
                )?;
                if event.has_value("azure.app_service.properties.Type") {
                    event.rename(
                        "azure.app_service.properties.Type",
                        "azure.app_service.properties.type",
                    )?;
                }
                if event.has_value("azure.app_service.properties.TimeGenerated") {
                    event.rename(
                        "azure.app_service.properties.TimeGenerated",
                        "azure.app_service.properties.time_generated",
                    )?;
                }
                if event.has_value("azure.app_service.properties.SourceSystem") {
                    event.rename(
                        "azure.app_service.properties.SourceSystem",
                        "azure.app_service.properties.source_system",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.app_service.properties.client_ip")
                    {
                        // Grok pattern: %{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}
                        if !cached_grok!("%{IPORHOST:azure.app_service.properties.client_ip}:%{POSINT:azure.app_service.properties.client_port:long}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
                if event.has_value("azure.app_service.properties.client_ip") {
                    if let Some(val) = event.get("azure.app_service.properties.client_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.app_service.properties.client_ip".into(),
                                message,
                            }
                        })?;
                        event.set("azure.app_service.properties.client_ip", converted)?;
                    }
                }
                // End nested pipeline: "appservice-ipsecauditlogs-inner-pipeline"
            }

            let _cond =
                { event.get_str("azure.app_service.category") == Some("AppServiceConsoleLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-consolelogs-inner-pipeline"
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }
                event.rename(
                    "azure.app_service.containerId",
                    "azure.app_service.container_id",
                )?;
                event.rename(
                    "azure.app_service.operationName",
                    "azure.app_service.operation_name",
                )?;
                event.rename(
                    "azure.app_service.resultDescription",
                    "azure.app_service.result_description",
                )?;
                event.rename(
                    "azure.app_service.EventStampType",
                    "azure.app_service.event_stamp_type",
                )?;
                event.rename(
                    "azure.app_service.EventPrimaryStampName",
                    "azure.app_service.event_primary_stamp_name",
                )?;
                event.rename(
                    "azure.app_service.EventStampName",
                    "azure.app_service.event_stamp_name",
                )?;
                event.rename("azure.app_service.Host", "azure.app_service.host")?;
                event.rename(
                    "azure.app_service.EventIpAddress",
                    "azure.app_service.event_ip_address",
                )?;
                // End nested pipeline: "appservice-consolelogs-inner-pipeline"
            }

            let _cond =
                { event.get_str("azure.app_service.category") == Some("AppServicePlatformLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-platformlogs-inner-pipeline"
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }
                event.rename(
                    "azure.app_service.operationName",
                    "azure.app_service.operation_name",
                )?;
                event.rename(
                    "azure.app_service.EventStampType",
                    "azure.app_service.event_stamp_type",
                )?;
                event.rename(
                    "azure.app_service.EventPrimaryStampName",
                    "azure.app_service.event_primary_stamp_name",
                )?;
                event.rename(
                    "azure.app_service.EventStampName",
                    "azure.app_service.event_stamp_name",
                )?;
                event.rename("azure.app_service.Host", "azure.app_service.host")?;
                event.rename(
                    "azure.app_service.EventIpAddress",
                    "azure.app_service.event_ip_address",
                )?;
                event.rename("azure.app_service.properties_raw", "azure.app_service.log")?;
                event.remove("azure.app_service.properties");
                // End nested pipeline: "appservice-platformlogs-inner-pipeline"
            }

            let _cond =
                { event.get_str("azure.app_service.category") == Some("AppServiceAppLogs") };
            if _cond {
                // Begin nested pipeline: "appservice-applogs-inner-pipeline"
                if event.has_value("azure.app_service.resourceId") {
                    event.rename("azure.app_service.resourceId", "azure.resource.id")?;
                }
                event.rename(
                    "azure.app_service.resultDescription",
                    "azure.app_service.result_description",
                )?;
                event.rename(
                    "azure.app_service.EventStampType",
                    "azure.app_service.event_stamp_type",
                )?;
                event.rename(
                    "azure.app_service.EventPrimaryStampName",
                    "azure.app_service.event_primary_stamp_name",
                )?;
                event.rename(
                    "azure.app_service.EventStampName",
                    "azure.app_service.event_stamp_name",
                )?;
                event.rename("azure.app_service.Host", "azure.app_service.host")?;
                event.rename(
                    "azure.app_service.EventIpAddress",
                    "azure.app_service.event_ip_address",
                )?;
                // End nested pipeline: "appservice-applogs-inner-pipeline"
            }

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource.id") {
                    // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                    if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_namespace", "azure.resource.namespace"), ("azure_resource_authorization_rule", "azure.resource.authorization_rule")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                Ok(())
            })();
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/providers/(?P<azure_resource_provider>(?:.+))
                        if !cached_grok_mapped!(
                            "(?i)/providers/(?P<azure_resource_provider>(?:.+))",
                            [("azure_resource_provider", "azure.resource.provider")]
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_provider", "azure.resource.provider")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource.id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))", [("azure_subscription_id", "azure.subscription_id")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            if event.has_value("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }
            if let Some(v) = event
                .get("azure.subscription_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }
            // End nested pipeline: "azure-shared-pipeline"

            event.remove("azure.app_service.properties_raw");

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
                        "{} {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("json");
                event.remove("_conf");
                event.remove("message");
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
