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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("error.message", json!("Received invalid JSON from the Azure Function service. Unable to parse the source log message"))?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.time") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.category") {
                event.rename("json.category", "azure.category")?;
            }

            if event.has_value("json.resourceId") {
                event.rename("json.resourceId", "azure.resource_id")?;
            }

            if event.has_value("json.operationName") {
                event.rename("json.operationName", "azure.operation_name")?;
            }

            let _cond = { event.get("json.properties").is_some_and(|v| v.is_string()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "temp_properties",
                        json!(
                            event
                                .get("json.properties")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.properties instanceof String) {\n  ctx.temp_properties = ctx.json.properties.replace(\"'\", \"\\\"\");\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.properties instanceof String) {\n  ctx.temp_properties = ctx.json.properties.replace(\"'\", \"\\\"\");\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("temp_properties") };
            if _cond {
                event.remove("json.properties");
            }

            let _cond = { event.has_value("temp_properties") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(event, "temp_properties", "json.properties")?;
                    Ok(())
                })();
            }

            event.remove("temp_properties");

            if event.has_value("json.properties.level") {
                event.rename("json.properties.level", "azure.function.level")?;
            }

            if event.has_value("json.properties.levelId") {
                event.rename("json.properties.levelId", "azure.function.level_id")?;
            }

            if event.has_value("json.properties.appName") {
                event.rename("json.properties.appName", "azure.function.app_name")?;
            }

            if event.has_value("json.properties.category") {
                event.rename("json.properties.category", "azure.function.category")?;
            }

            if event.has_value("json.properties.roleInstance") {
                event.rename(
                    "json.properties.roleInstance",
                    "azure.function.role_instance",
                )?;
            }

            if event.has_value("json.properties.hostVersion") {
                event.rename("json.properties.hostVersion", "azure.function.host_version")?;
            }

            if event.has_value("json.properties.functionInvocationId") {
                event.rename(
                    "json.properties.functionInvocationId",
                    "azure.function.invocation_id",
                )?;
            }

            if event.has_value("json.properties.functionName") {
                event.rename("json.properties.functionName", "azure.function.name")?;
            }

            if event.has_value("json.properties.hostInstanceId") {
                event.rename(
                    "json.properties.hostInstanceId",
                    "azure.function.host_instance_id",
                )?;
            }

            if event.has_value("json.properties.processId") {
                event.rename("json.properties.processId", "azure.function.process_id")?;
            }

            if event.has_value("json.properties.eventName") {
                event.rename("json.properties.eventName", "azure.function.event_name")?;
            }

            if event.has_value("json.properties.eventId") {
                event.rename("json.properties.eventId", "azure.function.event_id")?;
            }

            if event.has_value("json.properties.message") {
                event.rename("json.properties.message", "azure.function.message")?;
            }

            event.set("observer.type", json!("functions"))?;

            event.set("observer.vendor", json!("Azure"))?;

            event.set("observer.product", json!("Azure Functions"))?;

            if event.has_value("json.properties.exceptionDetails") {
                event.rename(
                    "json.properties.exceptionDetails",
                    "azure.function.exception_details",
                )?;
            }

            if event.has_value("json.properties.exceptionMessage") {
                event.rename(
                    "json.properties.exceptionMessage",
                    "azure.function.exception_message",
                )?;
            }

            if event.has_value("json.EventStampType") {
                event.rename("json.EventStampType", "azure.event_stamp_type")?;
            }

            if event.has_value("json.EventPrimaryStampName") {
                event.rename(
                    "json.EventPrimaryStampName",
                    "azure.event_primary_stamp_name",
                )?;
            }

            if event.has_value("json.EventStampName") {
                event.rename("json.EventStampName", "azure.event_stamp_name")?;
            }

            if event.has_value("json.properties.exceptionType") {
                event.rename(
                    "json.properties.exceptionType",
                    "azure.function.exception_type",
                )?;
            }

            event.remove("json");

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:.+$))
                    let _ = cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:.+$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)?;
                }
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

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
