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

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("parse_message"))
                        }
                        serde_json::Value::String(s) => s.contains("parse_message"),
                        _ => false,
                    })
            };
            if _cond {
                // Begin nested pipeline: "parsed-message"
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
                parse_json_field(event, "event.original", "azure.eventhub")?;
                let _cond = {
                    event
                        .get("azure.eventhub.properties")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if event.has_value("azure.eventhub.properties") {
                        event
                            .rename("azure.eventhub.properties", "azure.eventhub.properties.raw")?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("azure.eventhub.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "M/d/yyyy h:mm:ss a XXX",
                                "M/d/yyyy h:mm:ss a",
                                "M/d/yyyy H:mm:ss",
                                "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "azure.eventhub.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                event.remove("azure.eventhub.time");
                if event.has_value("azure.eventhub.resourceId") {
                    event.rename("azure.eventhub.resourceId", "azure.resource_id")?;
                }
                if event.has_value("azure.eventhub.durationMs") {
                    event.rename("azure.eventhub.durationMs", "event.duration")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}"#
                        ),
                        cached_params!("{\"param_nano\":1000000}"),
                    )?;
                    Ok(())
                })();
                // Painless script
                // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).size() == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).size() == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                    ),
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Begin nested pipeline: "azure-shared-pipeline"
                    event.set("cloud.provider", json!("azure"))?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("azure.resource_id") {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            (
                                                "azure_resource_namespace",
                                                "azure.resource.namespace"
                                            ),
                                            (
                                                "azure_resource_authorization_rule",
                                                "azure.resource.authorization_rule"
                                            )
                                        ]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            (
                                                "azure_resource_namespace",
                                                "azure.resource.namespace"
                                            ),
                                            (
                                                "azure_resource_authorization_rule",
                                                "azure.resource.authorization_rule"
                                            )
                                        ]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { !event.has_value("azure.subscription_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                ),
                                                ("azure_resource_name", "azure.resource.name")
                                            ]
                                        ),
                                        cached_grok_mapped!(
                                            "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                ),
                                                ("azure_resource_name", "azure.resource.name")
                                            ]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                ),
                                                ("azure_resource_name", "azure.resource.name")
                                            ]
                                        ),
                                        cached_grok_mapped!(
                                            "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                ),
                                                ("azure_resource_name", "azure.resource.name")
                                            ]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /providers/(?P<azure_resource_provider>(?:.+))
                                // Grok pattern: /PROVIDERS/(?P<azure_resource_provider>(?:.+))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/providers/(?P<azure_resource_provider>(?:.+))",
                                            [(
                                                "azure_resource_provider",
                                                "azure.resource.provider"
                                            )]
                                        ),
                                        cached_grok_mapped!(
                                            "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                            [(
                                                "azure_resource_provider",
                                                "azure.resource.provider"
                                            )]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                )
                                            ]
                                        ),
                                        cached_grok_mapped!(
                                            "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                (
                                                    "azure_resource_provider",
                                                    "azure.resource.provider"
                                                )
                                            ]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))
                                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group")
                                            ]
                                        ),
                                        cached_grok_mapped!(
                                            "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))",
                                            [
                                                ("azure_subscription_id", "azure.subscription_id"),
                                                ("azure_resource_group", "azure.resource.group")
                                            ]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                            if let Some(input) = event.get_string("azure.resource_id") {
                                // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                                if !extract_first_match(
                                    &[
                                        cached_grok_mapped!(
                                            "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                            [("azure_subscription_id", "azure.subscription_id")]
                                        ),
                                        cached_grok_mapped!(
                                            "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                            [("azure_subscription_id", "azure.subscription_id")]
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
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
                    // End nested pipeline: "azure-shared-pipeline"
                    Ok(())
                })();
                // End nested pipeline: "parsed-message"
            }

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
