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
            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == \"null\" || o == \"Null\" || o == \"NULL\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["null".into(), "Null".into(), "NULL".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.get("error.message").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ArrayList temp = new ArrayList(ctx.error.message); ArrayList output = new ArrayList(); for(data in temp){\n    if (!(data.contains('error in field ') || data.contains('strconv.Parse'))) {\n        output.add(data);\n    }\nctx.error.put(\"message\", output); }
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ArrayList temp = new ArrayList(ctx.error.message); ArrayList output = new ArrayList(); for(data in temp){\n    if (!(data.contains('error in field ') || data.contains('strconv.Parse'))) {\n        output.add(data);\n    }\nctx.error.put(\"message\", output); }"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_rename_error_message",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("error.message")
                    && event.get("error.message").is_some_and(|v| v.is_string())
                    && (event.get("error.message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("error in field "))
                        }
                        serde_json::Value::String(s) => s.contains("error in field "),
                        _ => false,
                    }) || event.get("error.message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("strconv.Parse"))
                        }
                        serde_json::Value::String(s) => s.contains("strconv.Parse"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("error.message");
            }

            if event.has_value("cef.extensions.PanOSX-Forwarded-ForIP") {
                event.rename(
                    "cef.extensions.PanOSX-Forwarded-ForIP",
                    "cef.extensions.PanOSXForwardedForIP",
                )?;
            }

            if event.has_value("cef.extensions.PanOSX-Forwarded-For") {
                event.rename(
                    "cef.extensions.PanOSX-Forwarded-For",
                    "cef.extensions.PanOSXForwardedFor",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSplit-tunnelconfiguration") {
                event.rename(
                    "cef.extensions.PanOSSplit-tunnelconfiguration",
                    "cef.extensions.PanOSSplitTunnelconfiguration",
                )?;
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("observer.vendor", json!("Palo Alto Networks"))?;

            event.set("observer.product", json!("Prisma Access"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "CONFIG",
                        "SYSTEM",
                        "GLOBALPROTECT APP TROUBLESHOOTING",
                        "AUTH",
                        "DNS SECURITY",
                        "DECRYPTION",
                        "FILE",
                        "GLOBALPROTECT",
                        "HIPMATCH",
                        "IPTAG",
                        "SCTP",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                        "USERID",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions.Name")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && event.has_value("cef.extensions")
                    && !(["url", "file"]
                        .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("CONFIG") };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["GLOBALPROTECT APP TROUBLESHOOTING"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["AUTH", "USERID"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "DNS SECURITY",
                        "DECRYPTION",
                        "GLOBALPROTECT",
                        "IPTAG",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                        "GLOBALPROTECT APP TROUBLESHOOTING",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && event.get_str("cef.extensions.Name") == Some("file")
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["HIPMATCH", "SYSTEM"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && (event.has_value("cef.extensions")
                        && !(["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or(""))))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "CONFIG",
                        "SYSTEM",
                        "GLOBALPROTECT APP TROUBLESHOOTING",
                        "AUTH",
                        "DNS SECURITY",
                        "DECRYPTION",
                        "FILE",
                        "GLOBALPROTECT",
                        "HIPMATCH",
                        "IPTAG",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                        "USERID",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("SCTP") };
            if _cond {
                event.append("event.type", json!("protocol"))?;
            }

            if event.has_value("cef.extensions.PanOSAccessPointName") {
                event.rename(
                    "cef.extensions.PanOSAccessPointName",
                    "prisma_access.event.access_point_name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentContentVersion") {
                event.rename(
                    "cef.extensions.PanOSAgentContentVersion",
                    "prisma_access.event.agent.content_version",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentDataCollectionStatus") {
                event.rename(
                    "cef.extensions.PanOSAgentDataCollectionStatus",
                    "prisma_access.event.agent.data_collection_status",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentID") {
                event.rename(
                    "cef.extensions.PanOSAgentID",
                    "prisma_access.event.agent.id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentIsolationStatus") {
                event.rename(
                    "cef.extensions.PanOSAgentIsolationStatus",
                    "prisma_access.event.agent.isolation_status",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentStatus") {
                event.rename(
                    "cef.extensions.PanOSAgentStatus",
                    "prisma_access.event.agent.status",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentTimeZoneOffset") {
                event.rename(
                    "cef.extensions.PanOSAgentTimeZoneOffset",
                    "prisma_access.event.agent.timezone_offset",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAgentVersion") {
                event.rename(
                    "cef.extensions.PanOSAgentVersion",
                    "prisma_access.event.agent.version",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSAppTampered") {
                    if let Some(val) = event.get("cef.extensions.PanOSAppTampered") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSAppTampered".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.app_tampered", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSAppTampered_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSApplianceOrCloud") {
                event.rename(
                    "cef.extensions.PanOSApplianceOrCloud",
                    "prisma_access.event.appliance_or_cloud",
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationCategory") {
                event.rename(
                    "cef.extensions.PanOSApplicationCategory",
                    "prisma_access.event.application.category",
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationCharacteristics") {
                event.rename(
                    "cef.extensions.PanOSApplicationCharacteristics",
                    "prisma_access.event.application.characteristics",
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationContainer") {
                event.rename(
                    "cef.extensions.PanOSApplicationContainer",
                    "prisma_access.event.application.container",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("SCTP") };
            if _cond {
                if event.has_value("cef.extensions.PanOSApplication") {
                    event.rename(
                        "cef.extensions.PanOSApplication",
                        "prisma_access.event.application.protocol",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["DECRYPTION", "FILE", "THREAT", "TRAFFIC", "TUNNEL", "URL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename(
                        "cef.extensions.applicationProtocol",
                        "prisma_access.event.application.protocol",
                    )?;
                }
            }

            if let Some(v) = event
                .get("prisma_access.event.application.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.application", v)?;
            }

            if event.has_value("network.application") {
                map_strings(
                    event,
                    "network.application",
                    "network.application",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationRisk") {
                event.rename(
                    "cef.extensions.PanOSApplicationRisk",
                    "prisma_access.event.application.risk",
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationSubcategory") {
                event.rename(
                    "cef.extensions.PanOSApplicationSubcategory",
                    "prisma_access.event.application.subcategory",
                )?;
            }

            if event.has_value("cef.extensions.PanOSApplicationTechnology") {
                event.rename(
                    "cef.extensions.PanOSApplicationTechnology",
                    "prisma_access.event.application.technology",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAssocationEndReason") {
                event.rename(
                    "cef.extensions.PanOSAssocationEndReason",
                    "prisma_access.event.assocation_end_reason",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.assocation_end_reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has_value("cef.extensions.PanOSAttemptedGateways") {
                event.rename(
                    "cef.extensions.PanOSAttemptedGateways",
                    "prisma_access.event.attempted_gateways",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAuthCacheServiceRegion") {
                event.rename(
                    "cef.extensions.PanOSAuthCacheServiceRegion",
                    "prisma_access.event.auth.cache_service_region",
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["AUTH", "USERID"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomNumber1") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomNumber1") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomNumber1".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.auth.factor_no", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomNumber1_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSAuthMethod") {
                event.rename(
                    "cef.extensions.PanOSAuthMethod",
                    "prisma_access.event.auth.method",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("AUTH") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString1") {
                    event.rename(
                        "cef.extensions.deviceCustomString1",
                        "prisma_access.event.auth.server_profile",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString1Label") {
                event.rename(
                    "cef.extensions.deviceCustomString1Label",
                    "prisma_access.event.label.cs1",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAuthenticatedUserDomain") {
                event.rename(
                    "cef.extensions.PanOSAuthenticatedUserDomain",
                    "prisma_access.event.authenticated.user.domain",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.authenticated.user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.authenticated.user.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.authenticated.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSAuthenticatedUserName") {
                event.rename(
                    "cef.extensions.PanOSAuthenticatedUserName",
                    "prisma_access.event.authenticated.user.name",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.authenticated.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.authenticated.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.authenticated.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSAuthenticatedUserUUID") {
                event.rename(
                    "cef.extensions.PanOSAuthenticatedUserUUID",
                    "prisma_access.event.authenticated.user.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.authenticated.user.uuid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.authenticated.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.authenticated.user.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("cef.extensions.PanOSAuthenticationDescription") {
                event.rename(
                    "cef.extensions.PanOSAuthenticationDescription",
                    "prisma_access.event.authentication.description",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("AUTH") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString4") {
                    event.rename(
                        "cef.extensions.deviceCustomString4",
                        "prisma_access.event.authentication.policy",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString4Label") {
                event.rename(
                    "cef.extensions.deviceCustomString4Label",
                    "prisma_access.event.label.cs4",
                )?;
            }

            if event.has_value("cef.extensions.PanOSAuthenticationProtocol") {
                event.rename(
                    "cef.extensions.PanOSAuthenticationProtocol",
                    "prisma_access.event.authentication.protocol",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.baseEventCount") {
                    if let Some(val) = event.get("cef.extensions.baseEventCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.baseEventCount".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.base_event_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_baseEventCount_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.device.event_class_id")
                    && ["THREAT", "URL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || event.get_str("cef.extensions.Name") == Some("url")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.base_event_count")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.sightings", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSBytes") {
                    if let Some(val) = event.get("cef.extensions.PanOSBytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSBytes".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.bytes.total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSBytes_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.bytes.total")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.bytes", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.bytesIn") {
                    if let Some(val) = event.get("cef.extensions.bytesIn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.bytesIn".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.bytes.in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_bytesIn_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.bytes.in")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.bytes", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.bytesOut") {
                    if let Some(val) = event.get("cef.extensions.bytesOut") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.bytesOut".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.bytes.out", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_bytesOut_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.bytes.out")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSCachedConfiguration") {
                    if let Some(val) = event.get("cef.extensions.PanOSCachedConfiguration") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSCachedConfiguration".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.cached_configuration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSCachedConfiguration_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSCaptivePortal") {
                    if let Some(val) = event.get("cef.extensions.PanOSCaptivePortal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSCaptivePortal".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.captive_portal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSCaptivePortal_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSCertificateFlags") {
                event.rename(
                    "cef.extensions.PanOSCertificateFlags",
                    "prisma_access.event.certificate.flags",
                )?;
            }

            if event.has_value("cef.extensions.PanOSCertificateSerial") {
                event.rename(
                    "cef.extensions.PanOSCertificateSerial",
                    "prisma_access.event.certificate.serial",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.certificate.serial")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.serial_number", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSCertificateSize") {
                    if let Some(val) = event.get("cef.extensions.PanOSCertificateSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSCertificateSize".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.certificate.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSCertificateSize_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSCertificateVersion") {
                event.rename(
                    "cef.extensions.PanOSCertificateVersion",
                    "prisma_access.event.certificate.version",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.certificate.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.version_number", v)?;
            }

            if event.has_value("cef.extensions.PanOSChainStatus") {
                event.rename(
                    "cef.extensions.PanOSChainStatus",
                    "prisma_access.event.chain_status",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSChunksReceived") {
                    if let Some(val) = event.get("cef.extensions.PanOSChunksReceived") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSChunksReceived".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.chunks.received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSChunksReceived_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSChunksSent") {
                    if let Some(val) = event.get("cef.extensions.PanOSChunksSent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSChunksSent".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.chunks.sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSChunksSent_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSChunksTotal") {
                    if let Some(val) = event.get("cef.extensions.PanOSChunksTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSChunksTotal".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.chunks.total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSChunksTotal_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("cef.device.event_class_id") == Some("AUTH") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString5") {
                    event.rename(
                        "cef.extensions.deviceCustomString5",
                        "prisma_access.event.client.type.value",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString5Label") {
                event.rename(
                    "cef.extensions.deviceCustomString5Label",
                    "prisma_access.event.label.cs5",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSClientToFirewall") {
                    if let Some(val) = event.get("cef.extensions.PanOSClientToFirewall") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSClientToFirewall".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.client.to_firewall", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSClientToFirewall_to_boolean",
                )?;
                if event.has_value("cef.extensions.PanOSClientToFirewall") {
                    event.rename(
                        "cef.extensions.PanOSClientToFirewall",
                        "prisma_access.event.client.to_firewall_str",
                    )?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("cef.extensions.PanOSClientTypeName") {
                event.rename(
                    "cef.extensions.PanOSClientTypeName",
                    "prisma_access.event.client.type.name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSCloudHostname") {
                event.rename(
                    "cef.extensions.PanOSCloudHostname",
                    "prisma_access.event.cloud.hostname",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.cloud.hostname") };
            if _cond {
                event.append_unique(
                    "cloud.instance.name",
                    json!(
                        event
                            .get("prisma_access.event.cloud.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.cloud.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.cloud.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSCloudReportID") {
                event.rename(
                    "cef.extensions.PanOSCloudReportID",
                    "prisma_access.event.cloud.report_id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSCommonName") {
                event.rename(
                    "cef.extensions.PanOSCommonName",
                    "prisma_access.event.common.name.value",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.common.name.value") };
            if _cond {
                event.append_unique(
                    "tls.client.x509.subject.common_name",
                    json!(
                        event
                            .get("prisma_access.event.common.name.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSCommonNameLength") {
                    if let Some(val) = event.get("cef.extensions.PanOSCommonNameLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSCommonNameLength".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.common.name.length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSCommonNameLength_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSConfigVersion") {
                if let Some(val) = event.get("cef.extensions.PanOSConfigVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.PanOSConfigVersion".into(),
                            message,
                        }
                    })?;
                    event.set("prisma_access.event.config_version", converted)?;
                }
            }

            if let Some(v) = event
                .get("prisma_access.event.config_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSConfigurationRefresh") {
                    if let Some(val) = event.get("cef.extensions.PanOSConfigurationRefresh") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSConfigurationRefresh".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.configuration_refresh", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSConfigurationRefresh_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSConnectionError") {
                event.rename(
                    "cef.extensions.PanOSConnectionError",
                    "prisma_access.event.connection.error.value",
                )?;
            }

            if event.has_value("cef.extensions.PanOSConnectionErrorID") {
                if let Some(val) = event.get("cef.extensions.PanOSConnectionErrorID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.PanOSConnectionErrorID".into(),
                            message,
                        }
                    })?;
                    event.set("prisma_access.event.connection.error.id", converted)?;
                }
            }

            if event.has_value("cef.extensions.PanOSConnectionMethod") {
                event.rename(
                    "cef.extensions.PanOSConnectionMethod",
                    "prisma_access.event.connection.method",
                )?;
            }

            if event.has_value("cef.extensions.PanOSContainerID") {
                event.rename(
                    "cef.extensions.PanOSContainerID",
                    "prisma_access.event.container.id",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.container.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if event.has_value("cef.extensions.PanOSContainerName") {
                event.rename(
                    "cef.extensions.PanOSContainerName",
                    "prisma_access.event.container.name.value",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.container.name.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSContainerNameSpace") {
                event.rename(
                    "cef.extensions.PanOSContainerNameSpace",
                    "prisma_access.event.container.name.space",
                )?;
            }

            if event.has_value("cef.extensions.PanOSContentVersion") {
                event.rename(
                    "cef.extensions.PanOSContentVersion",
                    "prisma_access.event.content_version",
                )?;
            }

            if event.has_value("cef.extensions.PanOSCortexDataLakeTenantID") {
                event.rename(
                    "cef.extensions.PanOSCortexDataLakeTenantID",
                    "prisma_access.event.cortex_data_lake_tenant_id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.cortex_data_lake_tenant_id") };
            if _cond {
                event.append_unique(
                    "cloud.account.id",
                    json!(
                        event
                            .get("prisma_access.event.cortex_data_lake_tenant_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSCountOfRepeats") {
                    if let Some(val) = event.get("cef.extensions.PanOSCountOfRepeats") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSCountOfRepeats".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.count_of_repeats", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSCountOfRepeats_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSCpadding") {
                event.rename(
                    "cef.extensions.PanOSCpadding",
                    "prisma_access.event.cpadding",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSCPUUsage")
                    && event.get_str("cef.extensions.PanOSCPUUsage") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSCPUUsage") {
                        if let Some(val) = event.get("cef.extensions.PanOSCPUUsage") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSCPUUsage".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.cpu_usage", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSCPUUsage_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.cpu_usage") };
            if _cond {
                // Painless script
                // Source: if (ctx.host == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"host\", hm);\n}\nctx.host.cpu = new HashMap();\nctx.host.cpu.usage = Math.round(ctx.prisma_access?.event?.cpu_usage *10) / 1000.0;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.host == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"host\", hm);\n}\nctx.host.cpu = new HashMap();\nctx.host.cpu.usage = Math.round(ctx.prisma_access?.event?.cpu_usage *10) / 1000.0;\n"#
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSCrashHistory") {
                event.rename(
                    "cef.extensions.PanOSCrashHistory",
                    "prisma_access.event.crash_history",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDebugLogFile") {
                event.rename(
                    "cef.extensions.PanOSDebugLogFile",
                    "prisma_access.event.debug_log_file",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.debug_log_file") };
            if _cond {
                event.append_unique(
                    "file.name",
                    json!(
                        event
                            .get("prisma_access.event.debug_log_file")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDescription") {
                event.rename(
                    "cef.extensions.PanOSDescription",
                    "prisma_access.event.description",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.destinationAddress")
                    && event.get_str("cef.extensions.destinationAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.destinationAddress") {
                        if let Some(val) = event.get("cef.extensions.destinationAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.destinationAddress".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("prisma_access.event.destination.address.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_destinationAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.destination.address.value") };
            if _cond {
                event.append_unique(
                    "destination.ip",
                    json!(
                        event
                            .get("prisma_access.event.destination.address.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.address.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.destination.address.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomIPv6Address2Label") {
                event.rename(
                    "cef.extensions.deviceCustomIPv6Address2Label",
                    "prisma_access.event.label.c6a2",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomIPv6Address2")
                    && event.get_str("cef.extensions.deviceCustomIPv6Address2") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomIPv6Address2") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomIPv6Address2") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomIPv6Address2".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.source.address.v6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomIPv6Address2_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("prisma_access.event.source.address.v6") {
                map_strings(
                    event,
                    "prisma_access.event.source.address.v6",
                    "prisma_access.event.source.address.v6",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.address.v6") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.address.v6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.address.v6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.address.v6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomIPv6Address3Label") {
                event.rename(
                    "cef.extensions.deviceCustomIPv6Address3Label",
                    "prisma_access.event.label.c6a3",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomIPv6Address3")
                    && event.get_str("cef.extensions.deviceCustomIPv6Address3") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomIPv6Address3") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomIPv6Address3") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomIPv6Address3".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.destination.address.v6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomIPv6Address3_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("prisma_access.event.destination.address.v6") {
                map_strings(
                    event,
                    "prisma_access.event.destination.address.v6",
                    "prisma_access.event.destination.address.v6",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.address.v6") };
            if _cond {
                event.append_unique(
                    "destination.ip",
                    json!(
                        event
                            .get("prisma_access.event.destination.address.v6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.address.v6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.destination.address.v6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceCategory") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceCategory",
                    "prisma_access.event.destination.device.category",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceClass") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceClass",
                    "prisma_access.event.destination.device.class",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceHost") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceHost",
                    "prisma_access.event.destination.device.host",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.device.host") };
            if _cond {
                event.append_unique(
                    "destination.domain",
                    json!(
                        event
                            .get("prisma_access.event.destination.device.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.device.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.destination.device.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceMac") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceMac",
                    "prisma_access.event.destination.device.mac",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.destination.device.mac")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.mac", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.mac") {
                    gsub_field(
                        event,
                        "destination.mac",
                        "destination.mac",
                        cached_regex!("(..)(?!$)"),
                        "$1-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_destination_mac")?;
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("destination.mac") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("destination.mac") {
                        map_strings(
                            event,
                            "destination.mac",
                            "destination.mac",
                            str::to_uppercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "uppercase_destination_mac",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSDestinationDeviceModel") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceModel",
                    "prisma_access.event.destination.device.model",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceOS") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceOS",
                    "prisma_access.event.destination.device.os.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceOSFamily") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceOSFamily",
                    "prisma_access.event.destination.device.os.family",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceOSVersion") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceOSVersion",
                    "prisma_access.event.destination.device.os.version",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceProfile") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceProfile",
                    "prisma_access.event.destination.device.profile",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDeviceVendor") {
                event.rename(
                    "cef.extensions.PanOSDestinationDeviceVendor",
                    "prisma_access.event.destination.device.vendor",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.device.vendor") };
            if _cond {
                event.append_unique(
                    "destination.as.organization.name",
                    json!(
                        event
                            .get("prisma_access.event.destination.device.vendor")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationDynamicAddressGroup") {
                event.rename(
                    "cef.extensions.PanOSDestinationDynamicAddressGroup",
                    "prisma_access.event.destination.dynamic_address_group",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationEDL") {
                event.rename(
                    "cef.extensions.PanOSDestinationEDL",
                    "prisma_access.event.destination.edl",
                )?;
            }

            if event.has_value("cef.extensions.destinationHostName") {
                event.rename(
                    "cef.extensions.destinationHostName",
                    "prisma_access.event.destination.host_name",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.destination.host_name")
                    && event.get_str("cef.device.event_class_id") == Some("HIPMATCH")
            };
            if _cond {
                event.append_unique(
                    "host.name",
                    json!(
                        event
                            .get("prisma_access.event.destination.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.destination.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationLocation") {
                event.rename(
                    "cef.extensions.PanOSDestinationLocation",
                    "prisma_access.event.destination.location",
                )?;
            }

            if event.has_value("cef.extensions.destinationNtDomain") {
                event.rename(
                    "cef.extensions.destinationNtDomain",
                    "prisma_access.event.destination.nt_domain",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.nt_domain") };
            if _cond {
                event.append_unique(
                    "destination.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.destination.nt_domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.destinationPort") {
                    if let Some(val) = event.get("cef.extensions.destinationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.destinationPort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_destinationPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("cef.extensions.destinationServiceName") {
                event.rename(
                    "cef.extensions.destinationServiceName",
                    "prisma_access.event.destination.service_name",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.destinationTranslatedAddress")
                    && event.get_str("cef.extensions.destinationTranslatedAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.destinationTranslatedAddress") {
                        if let Some(val) = event.get("cef.extensions.destinationTranslatedAddress")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.destinationTranslatedAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "prisma_access.event.destination.translated.address",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_destinationTranslatedAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.destination.translated.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.ip", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.translated.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.destination.translated.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.destinationTranslatedPort") {
                    if let Some(val) = event.get("cef.extensions.destinationTranslatedPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.destinationTranslatedPort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.destination.translated.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_destinationTranslatedPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.destination.translated.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.port", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDestinationUser")
                    && event
                        .get("cef.extensions.PanOSDestinationUser")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.PanOSDestinationUser") {
                        // Grok pattern: (?P<prisma_access_event_pan_os_value_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_value_destination_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_pan_os_value_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_value_destination_user_name>[^\\\\]*)", [("prisma_access_event_pan_os_value_destination_user_domain", "prisma_access.event.pan_os_value.destination.user.domain"), ("prisma_access_event_pan_os_value_destination_user_name", "prisma_access.event.pan_os_value.destination.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_PanOSDestinationUser",
                    )?;
                    if event.has_value("cef.extensions.PanOSDestinationUser") {
                        event.rename(
                            "cef.extensions.PanOSDestinationUser",
                            "prisma_access.event.pan_os_value.destination.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDestinationUser")
                    && !(event
                        .get("cef.extensions.PanOSDestinationUser")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.PanOSDestinationUser") {
                    event.rename(
                        "cef.extensions.PanOSDestinationUser",
                        "prisma_access.event.pan_os_value.destination.user.name",
                    )?;
                }
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_value.destination.user.domain") };
            if _cond {
                event.append_unique(
                    "destination.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_value.destination.user.name") };
            if _cond {
                event.append_unique(
                    "destination.user.name",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_value.destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDestinationUserName")
                    && event
                        .get("cef.extensions.PanOSDestinationUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.PanOSDestinationUserName")
                    {
                        // Grok pattern: (?P<prisma_access_event_pan_os_data_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_data_destination_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_pan_os_data_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_data_destination_user_name>[^\\\\]*)", [("prisma_access_event_pan_os_data_destination_user_domain", "prisma_access.event.pan_os_data.destination.user.domain"), ("prisma_access_event_pan_os_data_destination_user_name", "prisma_access.event.pan_os_data.destination.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_PanOSDestinationUserName",
                    )?;
                    if event.has_value("cef.extensions.PanOSDestinationUserName") {
                        event.rename(
                            "cef.extensions.PanOSDestinationUserName",
                            "prisma_access.event.pan_os_data.destination.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDestinationUserName")
                    && !(event
                        .get("cef.extensions.PanOSDestinationUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.PanOSDestinationUserName") {
                    event.rename(
                        "cef.extensions.PanOSDestinationUserName",
                        "prisma_access.event.pan_os_data.destination.user.name",
                    )?;
                }
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_data.destination.user.domain") };
            if _cond {
                event.append_unique(
                    "destination.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_value.destination.user.name") };
            if _cond {
                event.append_unique(
                    "destination.user.name",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("prisma_access.event.pan_os_data.destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationUserDomain") {
                event.rename(
                    "cef.extensions.PanOSDestinationUserDomain",
                    "prisma_access.event.pan_os.destination.user.domain",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os.destination.user.domain") };
            if _cond {
                event.append_unique(
                    "destination.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os.destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.destinationUserId") {
                event.rename(
                    "cef.extensions.destinationUserId",
                    "prisma_access.event.destination.user.id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.id") };
            if _cond {
                event.append_unique(
                    "destination.user.id",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.destinationUserName")
                    && event
                        .get("cef.extensions.destinationUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.destinationUserName") {
                        // Grok pattern: (?P<prisma_access_event_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_destination_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_destination_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_destination_user_name>[^\\\\]*)", [("prisma_access_event_destination_user_domain", "prisma_access.event.destination.user.domain"), ("prisma_access_event_destination_user_name", "prisma_access.event.destination.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_destinationUserName",
                    )?;
                    if event.has_value("cef.extensions.destinationUserName") {
                        event.rename(
                            "cef.extensions.destinationUserName",
                            "prisma_access.event.destination.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.destinationUserName")
                    && !(event
                        .get("cef.extensions.destinationUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.destinationUserName") {
                    event.rename(
                        "cef.extensions.destinationUserName",
                        "prisma_access.event.destination.user.name",
                    )?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.domain") };
            if _cond {
                event.append_unique(
                    "destination.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.name") };
            if _cond {
                event.append_unique(
                    "destination.user.name",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationUserUUID") {
                event.rename(
                    "cef.extensions.PanOSDestinationUserUUID",
                    "prisma_access.event.destination.user.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.uuid") };
            if _cond {
                event.append_unique(
                    "destination.user.id",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.user.uuid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.destination.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDestinationUUID") {
                event.rename(
                    "cef.extensions.PanOSDestinationUUID",
                    "prisma_access.event.destination.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.destination.uuid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.destination.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceAction") {
                event.rename(
                    "cef.extensions.deviceAction",
                    "prisma_access.event.device.action",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("cef.extensions.deviceEventCategory") {
                event.rename(
                    "cef.extensions.deviceEventCategory",
                    "prisma_access.event.device.event.category",
                )?;
            }

            if event.has_value("cef.extensions.Device Event Class ID") {
                event.rename(
                    "cef.extensions.Device Event Class ID",
                    "prisma_access.event.device.event.class_id",
                )?;
            }

            if event.has_value("cef.extensions.DeviceEventClassID") {
                event.rename(
                    "cef.extensions.DeviceEventClassID",
                    "prisma_access.event.device.event.class_id",
                )?;
            }

            if event.has_value("cef.extensions.DeviceEventClassId") {
                event.rename(
                    "cef.extensions.DeviceEventClassId",
                    "prisma_access.event.device.event.class_id",
                )?;
            }

            if event.has_value("cef.extensions.deviceExternalId") {
                event.rename(
                    "cef.extensions.deviceExternalId",
                    "prisma_access.event.device.external_id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.device.external_id") };
            if _cond {
                event.append_unique(
                    "observer.serial_number",
                    json!(
                        event
                            .get("prisma_access.event.device.external_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDeviceGroup") {
                event.rename(
                    "cef.extensions.PanOSDeviceGroup",
                    "prisma_access.event.device.group",
                )?;
            }

            if event.has_value("cef.extensions.deviceHostName") {
                event.rename(
                    "cef.extensions.deviceHostName",
                    "prisma_access.event.device.host_name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.device.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.device.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceInboundInterface") {
                event.rename(
                    "cef.extensions.deviceInboundInterface",
                    "prisma_access.event.device.inbound_interface",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.inbound_interface")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.name", v)?;
            }

            if event.has_value("cef.extensions.deviceCustomIPv6Address1Label") {
                event.rename(
                    "cef.extensions.deviceCustomIPv6Address1Label",
                    "prisma_access.event.label.c6a1",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomIPv6Address1")
                    && event.get_str("cef.extensions.deviceCustomIPv6Address1") != Some("")
                    && event.get_str("cef.device.event_class_id") == Some("HIPMATCH")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomIPv6Address1") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomIPv6Address1") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomIPv6Address1".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.device.ipv6_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomIPv6Address1_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("prisma_access.event.device.ipv6_address") {
                map_strings(
                    event,
                    "prisma_access.event.device.ipv6_address",
                    "prisma_access.event.device.ipv6_address",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.device.ipv6_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.device.ipv6_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDeviceName") {
                event.rename(
                    "cef.extensions.PanOSDeviceName",
                    "prisma_access.event.device.name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.device.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.device.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceOutboundInterface") {
                event.rename(
                    "cef.extensions.deviceOutboundInterface",
                    "prisma_access.event.device.outbound_interface",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.outbound_interface")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.interface.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSDeviceSN") {
                event.rename(
                    "cef.extensions.PanOSDeviceSN",
                    "prisma_access.event.device.sn",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.device.sn") };
            if _cond {
                event.append_unique(
                    "observer.serial_number",
                    json!(
                        event
                            .get("prisma_access.event.device.sn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceTimeZone") {
                event.rename(
                    "cef.extensions.deviceTimeZone",
                    "prisma_access.event.device.time_zone",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.time_zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            if event.has_value("cef.extensions.Device Vendor") {
                event.rename(
                    "cef.extensions.Device Vendor",
                    "prisma_access.event.device.vendor",
                )?;
            }

            if event.has_value("cef.extensions.Vendor Name") {
                event.rename(
                    "cef.extensions.Vendor Name",
                    "prisma_access.event.device.vendor",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.device.vendor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDGHierarchyLevel1") {
                    if let Some(val) = event.get("cef.extensions.PanOSDGHierarchyLevel1") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDGHierarchyLevel1".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dg_hierarchy.level1", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDGHierarchyLevel1_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDGHierarchyLevel2") {
                    if let Some(val) = event.get("cef.extensions.PanOSDGHierarchyLevel2") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDGHierarchyLevel2".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dg_hierarchy.level2", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDGHierarchyLevel2_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDGHierarchyLevel3") {
                    if let Some(val) = event.get("cef.extensions.PanOSDGHierarchyLevel3") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDGHierarchyLevel3".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dg_hierarchy.level3", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDGHierarchyLevel3_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDGHierarchyLevel4") {
                    if let Some(val) = event.get("cef.extensions.PanOSDGHierarchyLevel4") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDGHierarchyLevel4".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dg_hierarchy.level4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDGHierarchyLevel4_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSDiamAppID") {
                event.rename(
                    "cef.extensions.PanOSDiamAppID",
                    "prisma_access.event.diam.app_id",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.diam.app_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.iana_number", v)?;
            }

            if event.has_value("cef.extensions.PanOSDiamAvpCode") {
                event.rename(
                    "cef.extensions.PanOSDiamAvpCode",
                    "prisma_access.event.diam.avp_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDiameterCommandCode") {
                event.rename(
                    "cef.extensions.PanOSDiameterCommandCode",
                    "prisma_access.event.diameter_command_code",
                )?;
            }

            if event.has_value("cef.extensions.flexString2") {
                event.rename(
                    "cef.extensions.flexString2",
                    "prisma_access.event.direction_of_attack",
                )?;
            }

            if event.has_value("cef.extensions.flexString2Label") {
                event.rename(
                    "cef.extensions.flexString2Label",
                    "prisma_access.event.label.flex_string",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDisableHistory") {
                event.rename(
                    "cef.extensions.PanOSDisableHistory",
                    "prisma_access.event.disable_history",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDiskAvailable")
                    && event.get_str("cef.extensions.PanOSDiskAvailable") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSDiskAvailable") {
                        if let Some(val) = event.get("cef.extensions.PanOSDiskAvailable") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSDiskAvailable".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.disk_available", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSDiskAvailable_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSDLPVersionFlag") {
                event.rename(
                    "cef.extensions.PanOSDLPVersionFlag",
                    "prisma_access.event.dlp_version_flag",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDLSAstatus") {
                    if let Some(val) = event.get("cef.extensions.PanOSDLSAstatus") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDLSAstatus".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dlsa_status", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDLSAstatus_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSDNSCategory") {
                event.rename(
                    "cef.extensions.PanOSDNSCategory",
                    "prisma_access.event.dns.category",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDNSReachable") {
                    if let Some(val) = event.get("cef.extensions.PanOSDNSReachable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDNSReachable".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dns.reachable", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDNSReachable_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDNSResolverIP") {
                    if let Some(val) = event.get("cef.extensions.PanOSDNSResolverIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDNSResolverIP".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dns.resolver_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDNSResolverIP_to_ip",
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.dns.resolver_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.dns.resolver_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSDNSResponse")
                    && event
                        .get_str("cef.extensions.PanOSDNSResponse")
                        .is_some_and(|s| s.starts_with("["))
                    && event
                        .get_str("cef.extensions.PanOSDNSResponse")
                        .is_some_and(|s| s.ends_with("]"))
            };
            if _cond {
                event.set("_temp.dnsResponseIsArray", json!(true))?;
            }

            let _cond = { event.get_bool("_temp.dnsResponseIsArray") == Some(true) };
            if _cond {
                gsub_field(
                    event,
                    "cef.extensions.PanOSDNSResponse",
                    "cef.extensions.PanOSDNSResponse",
                    cached_regex!("[\\[\\]]"),
                    "",
                )?;
            }

            let _cond = { event.get_bool("_temp.dnsResponseIsArray") == Some(true) };
            if _cond {
                if let Some(s) = event.get_string("cef.extensions.PanOSDNSResponse") {
                    let mut parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("cef.extensions.PanOSDNSResponse", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_bool("_temp.dnsResponseIsArray") == Some(true) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "cef.extensions.PanOSDNSResponse", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_extension_PanOSDNSResponse_to_ip_element",
                            )?;
                            event.append_unique(
                                "dns.answers.data",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest._value");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get_bool("_temp.dnsResponseIsArray") == Some(true) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "cef.extensions.PanOSDNSResponse", |event| {
                        event.append_unique(
                            "prisma_access.event.dns.response.value",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get_bool("_temp.dnsResponseIsArray") != Some(true) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSDNSResponse") {
                        if let Some(val) = event.get("cef.extensions.PanOSDNSResponse") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSDNSResponse".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.dns.response.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSDNSResponse_to_ip",
                    )?;
                    event.append_unique(
                        "dns.answers.data",
                        json!(
                            event
                                .get("cef.extensions.PanOSDNSResponse")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                event
                    .get("prisma_access.event.dns.response.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_access.event.dns.response.value", |event| {
                    event.append_unique(
                        "dns.resolved_ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("prisma_access.event.dns.response.value")
                    && !(event
                        .get("prisma_access.event.dns.response.value")
                        .is_some_and(|v| v.is_array()))
            };
            if _cond {
                event.append_unique(
                    "dns.resolved_ip",
                    json!(
                        event
                            .get("prisma_access.event.dns.response.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("prisma_access.event.dns.response.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_access.event.dns.response.value", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("prisma_access.event.dns.response.value")
                    && !(event
                        .get("prisma_access.event.dns.response.value")
                        .is_some_and(|v| v.is_array()))
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.dns.response.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDNSResponseCode") {
                event.rename(
                    "cef.extensions.PanOSDNSResponseCode",
                    "prisma_access.event.dns.response.code",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.dns.response.code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.response_code", v)?;
            }

            if event.has_value("cef.extensions.PanOSDNSSecuityVersion") {
                event.rename(
                    "cef.extensions.PanOSDNSSecuityVersion",
                    "prisma_access.event.dns.secuity_version",
                )?;
            }

            if event.has_value("cef.extensions.PanOSDomain") {
                event.rename(
                    "cef.extensions.PanOSDomain",
                    "prisma_access.event.domain.value",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.domain.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDomainEDL") {
                event.rename(
                    "cef.extensions.PanOSDomainEDL",
                    "prisma_access.event.domain.edl",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("DNS SECURITY") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString5") {
                    event.rename(
                        "cef.extensions.deviceCustomString5",
                        "prisma_access.event.dst_zone",
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSDualStackTunnelInterface") {
                    if let Some(val) = event.get("cef.extensions.PanOSDualStackTunnelInterface") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSDualStackTunnelInterface".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.dual_stack_tunnel_interface", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSDualStackTunnelInterface_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSDynamicUserGroup") {
                event.rename(
                    "cef.extensions.PanOSDynamicUserGroup",
                    "prisma_access.event.dynamic_user_group.value",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.dynamic_user_group.value") };
            if _cond {
                event.append_unique(
                    "user.group.name",
                    json!(
                        event
                            .get("prisma_access.event.dynamic_user_group.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSDynamicUserGroupName") {
                event.rename(
                    "cef.extensions.PanOSDynamicUserGroupName",
                    "prisma_access.event.dynamic_user_group.name",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.dynamic_user_group.name") };
            if _cond {
                event.append_unique(
                    "user.group.name",
                    json!(
                        event
                            .get("prisma_access.event.dynamic_user_group.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSEllipticCurve") {
                event.rename(
                    "cef.extensions.PanOSEllipticCurve",
                    "prisma_access.event.elliptic_curve",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.elliptic_curve")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.curve", v)?;
            }

            if event.has_value("cef.extensions.PanOSEmailSubject") {
                event.rename(
                    "cef.extensions.PanOSEmailSubject",
                    "prisma_access.event.email_subject",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.email_subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.endTime")
                    && event.get_str("cef.extensions.endTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.endTime") {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("prisma_access.event.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.endTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_extension_endTime")?;
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointAssociationID") {
                event.rename(
                    "cef.extensions.PanOSEndpointAssociationID",
                    "prisma_access.event.endpoint.association_id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEndpointCPUArchitecture") {
                event.rename(
                    "cef.extensions.PanOSEndpointCPUArchitecture",
                    "prisma_access.event.endpoint.cpu_architecture",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.cpu_architecture")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.architecture", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointDeviceDomain") {
                event.rename(
                    "cef.extensions.PanOSEndpointDeviceDomain",
                    "prisma_access.event.endpoint.device.domain",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.device.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.endpoint.device.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.device.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSEndpointDeviceName") {
                event.rename(
                    "cef.extensions.PanOSEndpointDeviceName",
                    "prisma_access.event.endpoint.device.name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.endpoint.device.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.device.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSEndpointIPaddress")
                    && event.get_str("cef.extensions.PanOSEndpointIPaddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSEndpointIPaddress") {
                        if let Some(val) = event.get("cef.extensions.PanOSEndpointIPaddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSEndpointIPaddress".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.endpoint.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSEndpointIPaddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.endpoint.ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.endpoint.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("HIPMATCH") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    event.rename(
                        "cef.extensions.deviceCustomString2",
                        "prisma_access.event.endpoint.os.type",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString2Label") {
                event.rename(
                    "cef.extensions.deviceCustomString2Label",
                    "prisma_access.event.label.cs2",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEndpointOSType") {
                event.rename(
                    "cef.extensions.PanOSEndpointOSType",
                    "prisma_access.event.endpoint.os.type",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.os.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointOSVersion") {
                event.rename(
                    "cef.extensions.PanOSEndpointOSVersion",
                    "prisma_access.event.endpoint.os.version",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointSerialNumber") {
                event.rename(
                    "cef.extensions.PanOSEndpointSerialNumber",
                    "prisma_access.event.endpoint.serial_number",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEndpointSN") {
                event.rename(
                    "cef.extensions.PanOSEndpointSN",
                    "prisma_access.event.endpoint.sn",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.sn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointUserDomain") {
                event.rename(
                    "cef.extensions.PanOSEndpointUserDomain",
                    "prisma_access.event.endpoint.user.domain",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if event.has_value("cef.extensions.PanOSEndpointUserName") {
                event.rename(
                    "cef.extensions.PanOSEndpointUserName",
                    "prisma_access.event.endpoint.user.name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.endpoint.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSEndpointUserUUID") {
                event.rename(
                    "cef.extensions.PanOSEndpointUserUUID",
                    "prisma_access.event.endpoint.user.uuid",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.endpoint.user.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.endpoint.user.uuid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.endpoint.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSEnforcerStatus") {
                    if let Some(val) = event.get("cef.extensions.PanOSEnforcerStatus") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSEnforcerStatus".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.enforcer_status", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSEnforcerStatus_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSErrorDetails") {
                event.rename(
                    "cef.extensions.PanOSErrorDetails",
                    "prisma_access.event.error.details",
                )?;
            }

            if event.has_value("cef.extensions.PanOSErrorIndex") {
                event.rename(
                    "cef.extensions.PanOSErrorIndex",
                    "prisma_access.event.error.index",
                )?;
            }

            if event.has_value("cef.extensions.PanOSErrorMessage") {
                event.rename(
                    "cef.extensions.PanOSErrorMessage",
                    "prisma_access.event.error.message",
                )?;
            }

            if event.has_value("cef.extensions.PanOSErrorStage") {
                event.rename(
                    "cef.extensions.PanOSErrorStage",
                    "prisma_access.event.error.stage",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEventCode") {
                event.rename(
                    "cef.extensions.PanOSEventCode",
                    "prisma_access.event.data.code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEventDescription") {
                event.rename(
                    "cef.extensions.PanOSEventDescription",
                    "prisma_access.event.data.description",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEventDetails") {
                event.rename(
                    "cef.extensions.PanOSEventDetails",
                    "prisma_access.event.data.details",
                )?;
            }

            if event.has_value("cef.extensions.PanOSEventID") {
                event.rename("cef.extensions.PanOSEventID", "prisma_access.event.data.id")?;
            }

            if event.has_value("cef.extensions.eventOutcome") {
                event.rename(
                    "cef.extensions.eventOutcome",
                    "prisma_access.event.data.outcome",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.data.outcome")
                    && event
                        .get_str("prisma_access.event.data.outcome")
                        .is_some_and(|s| s.to_lowercase() == "success")
            };
            if _cond {
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = {
                event.has_value("prisma_access.event.data.outcome")
                    && event
                        .get_str("prisma_access.event.data.outcome")
                        .is_some_and(|s| s.to_lowercase() == "failure")
            };
            if _cond {
                let v = json!("failure");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            if event.has_value("cef.extensions.PanOSEventResult") {
                event.rename(
                    "cef.extensions.PanOSEventResult",
                    "prisma_access.event.data.result",
                )?;
            }

            if event.has_value("cef.extensions.externalId") {
                event.rename(
                    "cef.extensions.externalId",
                    "prisma_access.event.external_id",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.external_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("cef.extensions.PanOSFallbacktoSSLReason") {
                event.rename(
                    "cef.extensions.PanOSFallbacktoSSLReason",
                    "prisma_access.event.fallback_to_ssl_reason",
                )?;
            }

            if event.has_value("cef.extensions.PanOSFileHash") {
                event.rename(
                    "cef.extensions.PanOSFileHash",
                    "prisma_access.event.file.hash",
                )?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("FILE")
                    || event.get_str("cef.extensions.Name") == Some("file")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.file.hash")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && event.get_str("cef.extensions.Name") != Some("file")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.file.hash")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha256", v)?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.file.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("prisma_access.event.file.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.fileId") {
                event.rename("cef.extensions.fileId", "prisma_access.event.file.id")?;
            }

            if event.has_value("cef.extensions.filePath") {
                event.rename("cef.extensions.filePath", "prisma_access.event.file.name")?;
            }

            let _cond = { event.has_value("prisma_access.event.file.name") };
            if _cond {
                event.append_unique(
                    "file.name",
                    json!(
                        event
                            .get("prisma_access.event.file.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSFileType") {
                event.rename(
                    "cef.extensions.PanOSFileType",
                    "prisma_access.event.file.type",
                )?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("FILE")
                    || event.get_str("cef.extensions.Name") == Some("file")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.file.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.type", v)?;
                }
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && event.get_str("cef.extensions.Name") != Some("file")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.file.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.type", v)?;
                }
            }

            if event.has_value("cef.extensions.PanOSFileURL") {
                event.rename(
                    "cef.extensions.PanOSFileURL",
                    "prisma_access.event.file.url",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.file.url")
                    && event.get_str("cef.device.event_class_id") == Some("FILE")
                    || event.get_str("cef.extensions.Name") == Some("file")
            };
            if _cond {
                event.append_unique(
                    "file.path",
                    json!(
                        event
                            .get("prisma_access.event.file.url")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.filename") {
                event.rename("cef.extensions.filename", "prisma_access.event.filename")?;
            }

            if event.has_value("cef.extensions.PanOSFingerprint") {
                event.rename(
                    "cef.extensions.PanOSFingerprint",
                    "prisma_access.event.fingerprint",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.fingerprint")
                    && event
                        .get_as_string("prisma_access.event.fingerprint")
                        .is_some_and(|s| s.len() == 32)
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.fingerprint")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tls.client.hash.md5", v)?;
                }
            }

            let _cond = {
                event.has_value("prisma_access.event.fingerprint")
                    && event
                        .get_as_string("prisma_access.event.fingerprint")
                        .is_some_and(|s| s.len() == 64)
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.fingerprint")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tls.client.hash.sha256", v)?;
                }
            }

            let _cond = {
                event.has_value("prisma_access.event.fingerprint")
                    && event
                        .get_as_string("prisma_access.event.fingerprint")
                        .is_some_and(|s| s.len() == 40)
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.fingerprint")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tls.client.hash.sha1", v)?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.fingerprint") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("prisma_access.event.fingerprint")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSFirewallToClient")
                    && event.get_str("cef.extensions.PanOSFirewallToClient") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSFirewallToClient") {
                        if let Some(val) = event.get("cef.extensions.PanOSFirewallToClient") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSFirewallToClient".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.firewall_to_client", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSFirewallToClient_to_boolean",
                    )?;
                    if event.has_value("cef.extensions.PanOSFirewallToClient") {
                        event.rename(
                            "cef.extensions.PanOSFirewallToClient",
                            "prisma_access.event.firewall_to_client_str",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("cef.extensions.FlowType") {
                event.rename("cef.extensions.FlowType", "prisma_access.event.flow_type")?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "DNS SECURITY",
                        "DECRYPTION",
                        "FILE",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString4") {
                    event.rename(
                        "cef.extensions.deviceCustomString4",
                        "prisma_access.event.from_zone",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSGateway") {
                event.rename(
                    "cef.extensions.PanOSGateway",
                    "prisma_access.event.gateway.value",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSGatewayAddress")
                    && event.get_str("cef.extensions.PanOSGatewayAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSGatewayAddress") {
                        if let Some(val) = event.get("cef.extensions.PanOSGatewayAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSGatewayAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.gateway.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSGatewayAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.gateway.address") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("prisma_access.event.gateway.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.gateway.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.gateway.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSGatewayAuthentication") {
                event.rename(
                    "cef.extensions.PanOSGatewayAuthentication",
                    "prisma_access.event.gateway.authentication",
                )?;
            }

            if event.has_value("cef.extensions.PanOSGatewayConfigurationName") {
                event.rename(
                    "cef.extensions.PanOSGatewayConfigurationName",
                    "prisma_access.event.gateway.configuration_name",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSGatewayLogoutTime")
                    && event.get_str("cef.extensions.PanOSGatewayLogoutTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSGatewayLogoutTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.gateway.logout_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSGatewayLogoutTime".into(),
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
                        "date_extension_PanOSGatewayLogoutTime",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSGatewayPriority") {
                event.rename(
                    "cef.extensions.PanOSGatewayPriority",
                    "prisma_access.event.gateway.priority",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSGatewayReachable") {
                    if let Some(val) = event.get("cef.extensions.PanOSGatewayReachable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSGatewayReachable".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.gateway.reachable", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSGatewayReachable_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSGatewaySelectionType") {
                event.rename(
                    "cef.extensions.PanOSGatewaySelectionType",
                    "prisma_access.event.gateway.selection_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSGatewaySSLCertificateValid") {
                    if let Some(val) = event.get("cef.extensions.PanOSGatewaySSLCertificateValid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSGatewaySSLCertificateValid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.gateway.ssl_certificate_valid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSGatewaySSLCertificateValid_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSGatewayStatus") {
                event.rename(
                    "cef.extensions.PanOSGatewayStatus",
                    "prisma_access.event.gateway.status",
                )?;
            }

            if event.has_value("cef.extensions.PanOSGlobalProtectClientVersion") {
                event.rename(
                    "cef.extensions.PanOSGlobalProtectClientVersion",
                    "prisma_access.event.global_protect.client_version",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSGlobalProtectCPUUsage")
                    && event.get_str("cef.extensions.PanOSGlobalProtectCPUUsage") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSGlobalProtectCPUUsage") {
                        if let Some(val) = event.get("cef.extensions.PanOSGlobalProtectCPUUsage") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSGlobalProtectCPUUsage".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.global_protect.cpu_usage", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSGlobalProtectCPUUsage_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSGlobalProtectGatewayLocation") {
                event.rename(
                    "cef.extensions.PanOSGlobalProtectGatewayLocation",
                    "prisma_access.event.global_protect.gateway_location",
                )?;
            }

            if event.has_value("cef.extensions.PanOSGlobalProtectMemoryUsage") {
                event.rename(
                    "cef.extensions.PanOSGlobalProtectMemoryUsage",
                    "prisma_access.event.global_protect.memory_usage",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSGlobalProtectMTU") {
                    if let Some(val) = event.get("cef.extensions.PanOSGlobalProtectMTU") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSGlobalProtectMTU".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.global_protect.mtu", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSGlobalProtectMTU_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSGlobalProtectVersion") {
                event.rename(
                    "cef.extensions.PanOSGlobalProtectVersion",
                    "prisma_access.event.global_protect.version",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.global_protect.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has_value("cef.extensions.PanOSGPHostID") {
                event.rename(
                    "cef.extensions.PanOSGPHostID",
                    "prisma_access.event.gp_host_id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.gp_host_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.gp_host_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSHASessionOwner") {
                event.rename(
                    "cef.extensions.PanOSHASessionOwner",
                    "prisma_access.event.ha_session_owner",
                )?;
            }

            if event.has_value("cef.extensions.PanOSHipMatchType") {
                event.rename(
                    "cef.extensions.PanOSHipMatchType",
                    "prisma_access.event.hip_match_type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSHostID") {
                event.rename("cef.extensions.PanOSHostID", "prisma_access.event.host_id")?;
            }

            let _cond = { event.has_value("prisma_access.event.host_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.host_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSHTTPHeaders") {
                event.rename(
                    "cef.extensions.PanOSHTTPHeaders",
                    "prisma_access.event.http.headers",
                )?;
            }

            if event.has_value("cef.extensions.PanOSHTTPMethod") {
                event.rename(
                    "cef.extensions.PanOSHTTPMethod",
                    "prisma_access.event.http.method",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.http.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if event.has_value("cef.extensions.PanOSHTTPRefererFQDN") {
                event.rename(
                    "cef.extensions.PanOSHTTPRefererFQDN",
                    "prisma_access.event.http.referer.fqdn",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.http.referer.fqdn") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.http.referer.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSHTTPRefererPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSHTTPRefererPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSHTTPRefererPort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.http.referer.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSHTTPRefererPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSHTTPRefererProtocol") {
                event.rename(
                    "cef.extensions.PanOSHTTPRefererProtocol",
                    "prisma_access.event.http.referer.protocol",
                )?;
            }

            if event.has_value("cef.extensions.PanOSHTTPRefererURLPath") {
                event.rename(
                    "cef.extensions.PanOSHTTPRefererURLPath",
                    "prisma_access.event.http.referer.url_path",
                )?;
            }

            if event.has_value("cef.extensions.PanOSHTTP2Connection") {
                if let Some(val) = event.get("cef.extensions.PanOSHTTP2Connection") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.PanOSHTTP2Connection".into(),
                            message,
                        }
                    })?;
                    event.set("prisma_access.event.http2_connection", converted)?;
                }
            }

            if event.has_value("cef.extensions.PanOSIMEI") {
                event.rename("cef.extensions.PanOSIMEI", "prisma_access.event.imei")?;
            }

            if event.has_value("cef.extensions.PanOSIMSI") {
                event.rename("cef.extensions.PanOSIMSI", "prisma_access.event.imsi")?;
            }

            if event.has_value("cef.extensions.PanOSInboundInterface") {
                event.rename(
                    "cef.extensions.PanOSInboundInterface",
                    "prisma_access.event.inbound_interface.value",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.inbound_interface.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInboundInterfaceDetailsPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSInboundInterfaceDetailsPort")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInboundInterfaceDetailsPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.inbound_interface.details.port",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInboundInterfaceDetailsPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInboundInterfaceDetailsSlot") {
                    if let Some(val) = event.get("cef.extensions.PanOSInboundInterfaceDetailsSlot")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInboundInterfaceDetailsSlot".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.inbound_interface.details.slot",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInboundInterfaceDetailsSlot_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSInboundInterfaceDetailsType") {
                event.rename(
                    "cef.extensions.PanOSInboundInterfaceDetailsType",
                    "prisma_access.event.inbound_interface.details.type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInboundInterfaceDetailsUnit") {
                    if let Some(val) = event.get("cef.extensions.PanOSInboundInterfaceDetailsUnit")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInboundInterfaceDetailsUnit".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.inbound_interface.details.unit",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInboundInterfaceDetailsUnit_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSInlineMLVerdict") {
                event.rename(
                    "cef.extensions.PanOSInlineMLVerdict",
                    "prisma_access.event.inline_ml_verdict",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInstallHistory") {
                    if let Some(val) = event.get("cef.extensions.PanOSInstallHistory") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInstallHistory".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.install_history", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInstallHistory_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInternalNetwork") {
                    if let Some(val) = event.get("cef.extensions.PanOSInternalNetwork") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInternalNetwork".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.internal.network", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInternalNetwork_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSInternetAccess") {
                    if let Some(val) = event.get("cef.extensions.PanOSInternetAccess") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSInternetAccess".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.internet.access", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSInternetAccess_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSIPSubnetRange") {
                event.rename(
                    "cef.extensions.PanOSIPSubnetRange",
                    "prisma_access.event.ip_subnet_range",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIPSecEnabled") {
                    if let Some(val) = event.get("cef.extensions.PanOSIPSecEnabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIPSecEnabled".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.ipsec.enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIPSecEnabled_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSIPSecFailureReason") {
                event.rename(
                    "cef.extensions.PanOSIPSecFailureReason",
                    "prisma_access.event.ipsec.failure_reason",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsCertCNTruncated") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsCertCNTruncated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsCertCNTruncated".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_cert.cn_truncated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsCertCNTruncated_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsCertECDSA") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsCertECDSA") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsCertECDSA".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_cert.ecdsa", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsCertECDSA_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsCertRSA") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsCertRSA") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsCertRSA".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_cert.rsa", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsCertRSA_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsClienttoServer") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsClienttoServer") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsClienttoServer".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_client_to_server", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsClienttoServer_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("prisma_access.event.is_client_to_server")
                    && event.get_bool("prisma_access.event.is_client_to_server") == Some(true)
            };
            if _cond {
                event.append_unique("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.has_value("prisma_access.event.is_client_to_server")
                    && event.get_bool("prisma_access.event.is_client_to_server") == Some(false)
            };
            if _cond {
                event.append_unique("network.direction", json!("outbound"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsContainer") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsContainer") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsContainer".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_container", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsContainer_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDecryptMirror") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDecryptMirror") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDecryptMirror".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_decrypt_mirror", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDecryptMirror_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDecrypted") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDecrypted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDecrypted".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_decrypted.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDecrypted_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDecryptedLog") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDecryptedLog") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDecryptedLog".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_decrypted.log", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDecryptedLog_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDecryptedPayloadForward") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDecryptedPayloadForward") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDecryptedPayloadForward".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.is_decrypted.payload_forward",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDecryptedPayloadForward_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDuplicateLog") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDuplicateLog") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDuplicateLog".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_duplicate.log", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDuplicateLog_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsDuplicateUser") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsDuplicateUser") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsDuplicateUser".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_duplicate.user", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsDuplicateUser_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsEncrypted") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsEncrypted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsEncrypted".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_encrypted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsEncrypted_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsForwarded") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsForwarded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsForwarded".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_forwarded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsForwarded_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsInspectionBeforeSession") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsInspectionBeforeSession") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsInspectionBeforeSession".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.is_inspection_before_session",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsInspectionBeforeSession_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsIPV6") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsIPV6") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsIPV6".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsIPV6_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("prisma_access.event.is_ipv6")
                    && event.get_bool("prisma_access.event.is_ipv6") == Some(true)
            };
            if _cond {
                let v = json!("ipv6");
                if !painless_is_empty_value(&v) {
                    event.set("network.type", v)?;
                }
            }

            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsIssuerCNTruncated") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsIssuerCNTruncated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsIssuerCNTruncated".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_issuer_cn_truncated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsIssuerCNTruncated_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsMptcpOn") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsMptcpOn") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsMptcpOn".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_mptcp_on", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsMptcpOn_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsNAT") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsNAT") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsNAT".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_nat", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsNAT_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsNonStandardDestinationPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsNonStandardDestinationPort")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsNonStandardDestinationPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.is_non_standard_destination_port",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsNonStandardDestinationPort_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsOffloaded") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsOffloaded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsOffloaded".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_offloaded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsOffloaded_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsPacketCapture") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsPacketCapture") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsPacketCapture".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_packet_capture", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsPacketCapture_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsPhishing") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsPhishing") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsPhishing".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_phishing", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsPhishing_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsPrismaNetwork") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsPrismaNetwork") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsPrismaNetwork".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_prisma.network", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsPrismaNetwork_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsPrismaNetworks") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsPrismaNetworks") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsPrismaNetworks".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_prisma.network", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsPrismaNetworks_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsPrismaUsers") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsPrismaUsers") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsPrismaUsers".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_prisma.users", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsPrismaUsers_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsProxy") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsProxy") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsProxy".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_proxy", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsProxy_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsReconExcluded") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsReconExcluded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsReconExcluded".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_recon_excluded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsReconExcluded_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsResumeSession") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsResumeSession") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsResumeSession".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_resume_session", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsResumeSession_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.is_resume_session")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.resumed", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsRootCNTruncated") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsRootCNTruncated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsRootCNTruncated".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_root_cn_truncated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsRootCNTruncated_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsSaaSApplication") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsSaaSApplication") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsSaaSApplication".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_saas_application", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsSaaSApplication_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsServertoClient") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsServertoClient") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsServertoClient".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_server_to_client", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsServertoClient_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("prisma_access.event.is_server_to_client")
                    && event.get_bool("prisma_access.event.is_server_to_client") == Some(false)
            };
            if _cond {
                event.append_unique("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.has_value("prisma_access.event.is_server_to_client")
                    && event.get_bool("prisma_access.event.is_server_to_client") == Some(true)
            };
            if _cond {
                event.append_unique("network.direction", json!("outbound"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsSNITruncated") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsSNITruncated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsSNITruncated".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_sni_truncated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsSNITruncated_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsSourceXForwarded") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsSourceXForwarded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsSourceXForwarded".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_source_x_forwarded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsSourceXForwarded_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsSystemReturn") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsSystemReturn") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsSystemReturn".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_system_return", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsSystemReturn_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsTransaction") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsTransaction") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsTransaction".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_transaction", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsTransaction_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsTunnelInspected") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsTunnelInspected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsTunnelInspected".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_tunnel_inspected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsTunnelInspected_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIsURLDenied") {
                    if let Some(val) = event.get("cef.extensions.PanOSIsURLDenied") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIsURLDenied".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.is_url_denied", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIsURLDenied_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSIssuerCommonName") {
                event.rename(
                    "cef.extensions.PanOSIssuerCommonName",
                    "prisma_access.event.issuer.common_name",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.issuer.common_name") };
            if _cond {
                event.append_unique(
                    "tls.client.x509.issuer.common_name",
                    json!(
                        event
                            .get("prisma_access.event.issuer.common_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSIssuerNameLength") {
                    if let Some(val) = event.get("cef.extensions.PanOSIssuerNameLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSIssuerNameLength".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.issuer.name_length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSIssuerNameLength_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSJailbrokenStatus") {
                    if let Some(val) = event.get("cef.extensions.PanOSJailbrokenStatus") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSJailbrokenStatus".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.jail_broken_status", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSJailbrokenStatus_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSJitter") {
                    if let Some(val) = event.get("cef.extensions.PanOSJitter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSJitter".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.jitter", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSJitter_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSJustification") {
                event.rename(
                    "cef.extensions.PanOSJustification",
                    "prisma_access.event.justification",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSLastHIPReportTime")
                    && event.get_str("cef.extensions.PanOSLastHIPReportTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSLastHIPReportTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.last.hip_report_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSLastHIPReportTime".into(),
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
                        "date_extension_PanOSLastHIPReportTime",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSLastLogoutTime")
                    && event.get_str("cef.extensions.PanOSLastLogoutTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSLastLogoutTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.last.logout_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSLastLogoutTime".into(),
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
                        "date_extension_PanOSLastLogoutTime",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSLatency") {
                    if let Some(val) = event.get("cef.extensions.PanOSLatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSLatency".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.latency", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSLatency_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSLinkChangeCount") {
                    if let Some(val) = event.get("cef.extensions.PanOSLinkChangeCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSLinkChangeCount".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.link.change_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSLinkChangeCount_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSLinkSwitches") {
                event.rename(
                    "cef.extensions.PanOSLinkSwitches",
                    "prisma_access.event.link.switches",
                )?;
            }

            if event.has_value("cef.extensions.PanOSLocale") {
                event.rename("cef.extensions.PanOSLocale", "prisma_access.event.locale")?;
            }

            if event.has_value("cef.extensions.PanOSLocation") {
                event.rename(
                    "cef.extensions.PanOSLocation",
                    "prisma_access.event.location",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSLogExported") {
                    if let Some(val) = event.get("cef.extensions.PanOSLogExported") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSLogExported".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.log.exported", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSLogExported_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSLogForwarded") {
                    if let Some(val) = event.get("cef.extensions.PanOSLogForwarded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSLogForwarded".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.log.forwarded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSLogForwarded_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.device.event_class_id")
                    && [
                        "AUTH",
                        "FILE",
                        "DECRYPTION",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString6") {
                    event.rename(
                        "cef.extensions.deviceCustomString6",
                        "prisma_access.event.log.setting",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString6Label") {
                event.rename(
                    "cef.extensions.deviceCustomString6Label",
                    "prisma_access.event.label.cs6",
                )?;
            }

            if event.has_value("cef.extensions.PanOSLogSetting") {
                event.rename(
                    "cef.extensions.PanOSLogSetting",
                    "prisma_access.event.log.setting",
                )?;
            }

            if event.has_value("cef.extensions.PanOSLogSource") {
                event.rename(
                    "cef.extensions.PanOSLogSource",
                    "prisma_access.event.log.source.value",
                )?;
            }

            if event.has_value("cef.extensions.LogSourceGroupID") {
                event.rename(
                    "cef.extensions.LogSourceGroupID",
                    "prisma_access.event.log.source.group_id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSLogSourceTimeZoneOffset") {
                event.rename(
                    "cef.extensions.PanOSLogSourceTimeZoneOffset",
                    "prisma_access.event.log.source.timezone_offset",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.log.source.timezone_offset")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSEventTime")
                    && event.get_str("cef.extensions.PanOSEventTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.PanOSEventTime") {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSEventTime".into(),
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
                        "date_timestamp_event_timezone",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSEventTime")
                    && event.get_str("cef.extensions.PanOSEventTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.PanOSEventTime") {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("prisma_access.event.data.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSEventTime".into(),
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
                        "date_extension_PanOSEventTime_event_timezone",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSLogSubtype") {
                event.rename(
                    "cef.extensions.PanOSLogSubtype",
                    "prisma_access.event.log.subtype",
                )?;
            }

            if event.has_value("cef.extensions.PanOSLoggingServiceID") {
                event.rename(
                    "cef.extensions.PanOSLoggingServiceID",
                    "prisma_access.event.logging_service_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSLoginDuration") {
                    if let Some(val) = event.get("cef.extensions.PanOSLoginDuration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSLoginDuration".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.login_duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSLoginDuration_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.login_duration")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            if event.has_value("cef.extensions.PanOSMapAppCode") {
                event.rename(
                    "cef.extensions.PanOSMapAppCode",
                    "prisma_access.event.map_app_code",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("USERID") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString5") {
                    event.rename(
                        "cef.extensions.deviceCustomString5",
                        "prisma_access.event.mapping.data_source.value",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSMappingDataSource") {
                event.rename(
                    "cef.extensions.PanOSMappingDataSource",
                    "prisma_access.event.mapping.data_source.value",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("USERID") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString4") {
                    event.rename(
                        "cef.extensions.deviceCustomString4",
                        "prisma_access.event.mapping.data_source.name",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSMappingDataSourceSubType") {
                event.rename(
                    "cef.extensions.PanOSMappingDataSourceSubType",
                    "prisma_access.event.mapping.data_source.subtype",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("USERID") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString6") {
                    event.rename(
                        "cef.extensions.deviceCustomString6",
                        "prisma_access.event.mapping.data_source.type",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSMappingDataSourceType") {
                event.rename(
                    "cef.extensions.PanOSMappingDataSourceType",
                    "prisma_access.event.mapping.data_source.type",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("USERID") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomNumber3") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomNumber3") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomNumber3".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.mapping.timeout", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomNumber3_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSMappingTimeout") {
                    if let Some(val) = event.get("cef.extensions.PanOSMappingTimeout") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSMappingTimeout".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.mapping.timeout", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSMappingTimeout_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSMemoryUsage") {
                    if let Some(val) = event.get("cef.extensions.PanOSMemoryUsage") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSMemoryUsage".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.memory_usage", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSMemoryUsage_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.message") {
                event.rename("cef.extensions.message", "prisma_access.event.message")?;
            }

            if let Some(v) = event
                .get("prisma_access.event.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("AUTH") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomNumber2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber2") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber2".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.mfa.authentication_id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("USERID") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString1") {
                    event.rename(
                        "cef.extensions.deviceCustomString1",
                        "prisma_access.event.mfa.factor_type",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSMFAVendor") {
                event.rename(
                    "cef.extensions.PanOSMFAVendor",
                    "prisma_access.event.mfa.vendor",
                )?;
            }

            if event.has_value("cef.extensions.PanOSMobileAreaCode") {
                event.rename(
                    "cef.extensions.PanOSMobileAreaCode",
                    "prisma_access.event.mobile.area_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSMobileBaseStationCode") {
                event.rename(
                    "cef.extensions.PanOSMobileBaseStationCode",
                    "prisma_access.event.mobile.base_station_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSMobileCountryCode") {
                event.rename(
                    "cef.extensions.PanOSMobileCountryCode",
                    "prisma_access.event.mobile.country_code",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSMobileIP")
                    && event.get_str("cef.extensions.PanOSMobileIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSMobileIP") {
                        if let Some(val) = event.get("cef.extensions.PanOSMobileIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSMobileIP".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.mobile.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSMobileIP_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.mobile.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.mobile.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSMobileNetworkCode") {
                event.rename(
                    "cef.extensions.PanOSMobileNetworkCode",
                    "prisma_access.event.mobile.network_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSMobileSubscriberISDN") {
                event.rename(
                    "cef.extensions.PanOSMobileSubscriberISDN",
                    "prisma_access.event.mobile.subscriber_isdn",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSNAT") {
                    if let Some(val) = event.get("cef.extensions.PanOSNAT") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSNAT".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.nat.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSNAT_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSNATDestination")
                    && event.get_str("cef.extensions.PanOSNATDestination") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSNATDestination") {
                        if let Some(val) = event.get("cef.extensions.PanOSNATDestination") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSNATDestination".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.nat.destination.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSNATDestination_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.nat.destination.value") };
            if _cond {
                event.append_unique(
                    "destination.nat.ip",
                    json!(
                        event
                            .get("prisma_access.event.nat.destination.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.nat.destination.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.nat.destination.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSNATDestinationPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSNATDestinationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSNATDestinationPort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.nat.destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSNATDestinationPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.nat.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.port", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSNATSource")
                    && event.get_str("cef.extensions.PanOSNATSource") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSNATSource") {
                        if let Some(val) = event.get("cef.extensions.PanOSNATSource") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSNATSource".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.nat.source.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSNATSource_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.nat.source.value") };
            if _cond {
                event.append_unique(
                    "source.nat.ip",
                    json!(
                        event
                            .get("prisma_access.event.nat.source.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.nat.source.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.nat.source.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSNATSourcePort") {
                    if let Some(val) = event.get("cef.extensions.PanOSNATSourcePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSNATSourcePort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.nat.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSNATSourcePort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.nat.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSNetworkAccess") {
                    if let Some(val) = event.get("cef.extensions.PanOSNetworkAccess") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSNetworkAccess".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.network_access", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSNetworkAccess_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSNonStandardDestinationPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSNonStandardDestinationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSNonStandardDestinationPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.non_standard_destination_port",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSNonStandardDestinationPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.get_str("cef.device.event_class_id") == Some("AUTH")
                    && event.has_value("cef.extensions.deviceCustomString2")
                    && event
                        .get("cef.extensions.deviceCustomString2")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.deviceCustomString2") {
                        // Grok pattern: (?P<prisma_access_event_normalize_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_normalize_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_normalize_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_normalize_user_name>[^\\\\]*)", [("prisma_access_event_normalize_user_domain", "prisma_access.event.normalize_user.domain"), ("prisma_access_event_normalize_user_name", "prisma_access.event.normalize_user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_deviceCustomString2",
                    )?;
                    if event.has_value("cef.extensions.deviceCustomString2") {
                        event.rename(
                            "cef.extensions.deviceCustomString2",
                            "prisma_access.event.normalize_user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("AUTH")
                    && event.has_value("cef.extensions.deviceCustomString2")
                    && !(event
                        .get("cef.extensions.deviceCustomString2")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    event.rename(
                        "cef.extensions.deviceCustomString2",
                        "prisma_access.event.normalize_user.name",
                    )?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.normalize_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.normalize_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSNSSAINetworkSliceDifferentiator") {
                event.rename(
                    "cef.extensions.PanOSNSSAINetworkSliceDifferentiator",
                    "prisma_access.event.nssai_network_slice.differentiator",
                )?;
            }

            if event.has_value("cef.extensions.PanOSNSSAINetworkSliceType") {
                event.rename(
                    "cef.extensions.PanOSNSSAINetworkSliceType",
                    "prisma_access.event.nssai_network_slice.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSOperatingSystem") {
                event.rename(
                    "cef.extensions.PanOSOperatingSystem",
                    "prisma_access.event.operating_system",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.operating_system") };
            if _cond {
                event.append_unique(
                    "host.os.full",
                    json!(
                        event
                            .get("prisma_access.event.operating_system")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSOutboundInterfaceDetailsPort") {
                    if let Some(val) = event.get("cef.extensions.PanOSOutboundInterfaceDetailsPort")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSOutboundInterfaceDetailsPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.outbound_interface_details.port",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSOutboundInterfaceDetailsPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSOutboundInterfaceDetailsSlot") {
                    if let Some(val) = event.get("cef.extensions.PanOSOutboundInterfaceDetailsSlot")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSOutboundInterfaceDetailsSlot".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.outbound_interface_details.slot",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSOutboundInterfaceDetailsSlot_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSOutboundInterfaceDetailsType") {
                event.rename(
                    "cef.extensions.PanOSOutboundInterfaceDetailsType",
                    "prisma_access.event.outbound_interface_details.type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSOutboundInterfaceDetailsUnit") {
                    if let Some(val) = event.get("cef.extensions.PanOSOutboundInterfaceDetailsUnit")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSOutboundInterfaceDetailsUnit".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.outbound_interface_details.unit",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSOutboundInterfaceDetailsUnit_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSPacket") {
                event.rename(
                    "cef.extensions.PanOSPacket",
                    "prisma_access.event.packet.value",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketCapture") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketCapture") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketCapture".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packet.capture", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketCapture_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSPacketLoss")
                    && event.get_str("cef.extensions.PanOSPacketLoss") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPacketLoss") {
                        if let Some(val) = event.get("cef.extensions.PanOSPacketLoss") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPacketLoss".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.packet.loss", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPacketLoss_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketsDroppedMax") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketsDroppedMax") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketsDroppedMax".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packets.dropped.max", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketsDroppedMax_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketsDroppedTunnel") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketsDroppedTunnel") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketsDroppedTunnel".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packets.dropped.tunnel", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketsDroppedTunnel_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketsReceived") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketsReceived") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketsReceived".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packets.received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketsReceived_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.packets.received")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.packets", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketsSent") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketsSent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketsSent".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packets.sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketsSent_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.packets.sent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.packets", v)?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["TRAFFIC", "TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomNumber2") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomNumber2") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomNumber2".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.packets.total", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomNumber2_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPacketsTotal") {
                    if let Some(val) = event.get("cef.extensions.PanOSPacketsTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPacketsTotal".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.packets.total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPacketsTotal_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.packets.total")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.packets", v)?;
            }

            if event.has_value("cef.extensions.PanOSPadding") {
                event.rename("cef.extensions.PanOSPadding", "prisma_access.event.padding")?;
            }

            if event.has_value("cef.extensions.PanOSPadding3") {
                event.rename(
                    "cef.extensions.PanOSPadding3",
                    "prisma_access.event.padding3",
                )?;
            }

            if event.has_value("cef.extensions.PanOSPanoramaSN") {
                event.rename(
                    "cef.extensions.PanOSPanoramaSN",
                    "prisma_access.event.panorama_sn",
                )?;
            }

            if event.has_value("cef.extensions.PanOSParentSessionID") {
                event.rename(
                    "cef.extensions.PanOSParentSessionID",
                    "prisma_access.event.parent.session_id",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSParentStartTime")
                    && event.get_str("cef.extensions.PanOSParentStartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSParentStartTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.parent.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSParentStartTime".into(),
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
                        "date_extension_PanOSParentStartTime",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSParentStarttime")
                    && event.get_str("cef.extensions.PanOSParentStarttime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSParentStarttime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.parent.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSParentStarttime".into(),
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
                        "date_extension_PanOSParentStarttime",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSPartialHash") {
                event.rename(
                    "cef.extensions.PanOSPartialHash",
                    "prisma_access.event.partial_hash",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.partial_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("prisma_access.event.partial_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSPayloadProtocolID") {
                event.rename(
                    "cef.extensions.PanOSPayloadProtocolID",
                    "prisma_access.event.payload_protocol_id",
                )?;
            }

            if event.has_value("cef.extensions.PlatformType") {
                event.rename(
                    "cef.extensions.PlatformType",
                    "prisma_access.event.platform_type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSPolicyName") {
                event.rename(
                    "cef.extensions.PanOSPolicyName",
                    "prisma_access.event.policy_name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSPortal") {
                event.rename(
                    "cef.extensions.PanOSPortal",
                    "prisma_access.event.portal.value",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSPortalAddress")
                    && event.get_str("cef.extensions.PanOSPortalAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPortalAddress") {
                        if let Some(val) = event.get("cef.extensions.PanOSPortalAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPortalAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.portal.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPortalAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.portal.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.portal.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSPortalAuthentication") {
                event.rename(
                    "cef.extensions.PanOSPortalAuthentication",
                    "prisma_access.event.portal.authentication",
                )?;
            }

            if event.has_value("cef.extensions.PanOSPortalConfigurationName") {
                event.rename(
                    "cef.extensions.PanOSPortalConfigurationName",
                    "prisma_access.event.portal.configuration_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPortalGatewayLatency") {
                    if let Some(val) = event.get("cef.extensions.PanOSPortalGatewayLatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPortalGatewayLatency".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.portal.gateway_latency", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPortalGatewayLatency_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.flexDate1")
                    && event.get_str("cef.extensions.flexDate1") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.flexDate1") {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.portal.last_connect_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.flexDate1".into(),
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
                        "date_extension_flexDate1",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPortalReachable") {
                    if let Some(val) = event.get("cef.extensions.PanOSPortalReachable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPortalReachable".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.portal.reachable", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPortalReachable_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPortalSSLCertificateValid") {
                    if let Some(val) = event.get("cef.extensions.PanOSPortalSSLCertificateValid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPortalSSLCertificateValid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_access.event.portal.ssl_certificate_valid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPortalSSLCertificateValid_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSPortalStatus") {
                event.rename(
                    "cef.extensions.PanOSPortalStatus",
                    "prisma_access.event.portal.status",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSPrivateIPv4")
                    && event.get_str("cef.extensions.PanOSPrivateIPv4") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPrivateIPv4") {
                        if let Some(val) = event.get("cef.extensions.PanOSPrivateIPv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPrivateIPv4".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.private.ipv4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPrivateIPv4_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.private.ipv4") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.private.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.private.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.private.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSPublicIPv4")
                    && event.get_str("cef.extensions.PanOSPublicIPv4") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPublicIPv4") {
                        if let Some(val) = event.get("cef.extensions.PanOSPublicIPv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPublicIPv4".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.public.ipv4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPublicIPv4_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.public.ipv4") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.public.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.public.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.public.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSPrivateIPv6")
                    && event.get_str("cef.extensions.PanOSPrivateIPv6") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPrivateIPv6") {
                        if let Some(val) = event.get("cef.extensions.PanOSPrivateIPv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPrivateIPv6".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.private.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPrivateIPv6_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("prisma_access.event.private.ipv6") {
                map_strings(
                    event,
                    "prisma_access.event.private.ipv6",
                    "prisma_access.event.private.ipv6",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.private.ipv6") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.private.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.private.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.private.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSPublicIPv6")
                    && event.get_str("cef.extensions.PanOSPublicIPv6") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSPublicIPv6") {
                        if let Some(val) = event.get("cef.extensions.PanOSPublicIPv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSPublicIPv6".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.public.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSPublicIPv6_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("prisma_access.event.public.ipv6") {
                map_strings(
                    event,
                    "prisma_access.event.public.ipv6",
                    "prisma_access.event.public.ipv6",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.public.ipv6") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.public.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.public.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.public.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSPrivileges") {
                    if let Some(val) = event.get("cef.extensions.PanOSPrivileges") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSPrivileges".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.privileges", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSPrivileges_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSProfileName") {
                event.rename(
                    "cef.extensions.PanOSProfileName",
                    "prisma_access.event.profile.name",
                )?;
            }

            if event.has_value("cef.extensions.ProfileToken") {
                event.rename(
                    "cef.extensions.ProfileToken",
                    "prisma_access.event.profile.token",
                )?;
            }

            if event.has_value("cef.extensions.ProjectName") {
                event.rename(
                    "cef.extensions.ProjectName",
                    "prisma_access.event.project_name",
                )?;
            }

            if event.has_value("cef.extensions.transportProtocol") {
                event.rename(
                    "cef.extensions.transportProtocol",
                    "prisma_access.event.transport_protocol",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.transport_protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("cef.extensions.PanOSProtocolDataUnitsessionID") {
                event.rename(
                    "cef.extensions.PanOSProtocolDataUnitsessionID",
                    "prisma_access.event.protocol_data_unitsession_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSProxyServer") {
                    if let Some(val) = event.get("cef.extensions.PanOSProxyServer") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSProxyServer".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.proxy.server", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSProxyServer_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSProxyType") {
                event.rename(
                    "cef.extensions.PanOSProxyType",
                    "prisma_access.event.proxy.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSQuarantineReason") {
                event.rename(
                    "cef.extensions.PanOSQuarantineReason",
                    "prisma_access.event.quarantine_reason",
                )?;
            }

            if event.has_value("cef.extensions.PanOSRadioAccessTechnology") {
                event.rename(
                    "cef.extensions.PanOSRadioAccessTechnology",
                    "prisma_access.event.radio_access_technology",
                )?;
            }

            if event.has_value("cef.extensions.Reason") {
                event.rename("cef.extensions.Reason", "prisma_access.event.reason")?;
            }

            if let Some(v) = event
                .get("prisma_access.event.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has_value("cef.extensions.PanOSReasonForDataFilteringAction") {
                event.rename(
                    "cef.extensions.PanOSReasonForDataFilteringAction",
                    "prisma_access.event.reason_for_data_filtering_action",
                )?;
            }

            if event.has_value("cef.extensions.PanOSRecipientEmail") {
                event.rename(
                    "cef.extensions.PanOSRecipientEmail",
                    "prisma_access.event.recipient_email",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.recipient_email") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("prisma_access.event.recipient_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.recipient_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.recipient_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSRecordType") {
                event.rename(
                    "cef.extensions.PanOSRecordType",
                    "prisma_access.event.record_type",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.record_type")
                    && event
                        .get("prisma_access.event.record_type")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("prisma_access.event.record_type") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_temp.dns_answers", Value::Array(parts))?;
                }
            }

            let _cond = { event.get("_temp.dns_answers").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "_temp.dns_answers", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "dns.answers.type",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("dns.answers.type") };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.record_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("dns.answers.type", v)?;
                }
            }

            if event.has_value("cef.extensions.PanOSReferer") {
                event.rename("cef.extensions.PanOSReferer", "prisma_access.event.referer")?;
            }

            if let Some(v) = event
                .get("prisma_access.event.referer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
            }

            if event.has_value("cef.extensions.PanOSReportID") {
                event.rename(
                    "cef.extensions.PanOSReportID",
                    "prisma_access.event.report_id",
                )?;
            }

            if event.has_value("cef.extensions.requestClientApplication") {
                event.rename(
                    "cef.extensions.requestClientApplication",
                    "prisma_access.event.request.client_application",
                )?;
            }

            if event.has_value("prisma_access.event.request.client_application") {
                if let Some(ua_str) =
                    event.get_string("prisma_access.event.request.client_application")
                {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("cef.extensions.requestContext") {
                event.rename(
                    "cef.extensions.requestContext",
                    "prisma_access.event.request.context",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.request.context") };
            if _cond {
                event.append_unique(
                    "http.response.mime_type",
                    json!(
                        event
                            .get("prisma_access.event.request.context")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.requestMethod") {
                event.rename(
                    "cef.extensions.requestMethod",
                    "prisma_access.event.request.method",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if event.has_value("cef.extensions.requestUrl") {
                event.rename(
                    "cef.extensions.requestUrl",
                    "prisma_access.event.request.url",
                )?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && (["wildfire", "wildfire-virus"]
                        .contains(&event.get_str("cef.extensions.Name").unwrap_or(""))
                        || ["wildfire", "wildfire-virus"]
                            .contains(&event.get_str("cef.name").unwrap_or("")))
                    && event.has_value("prisma_access.event.request.url")
            };
            if _cond {
                event.append_unique(
                    "file.name",
                    json!(
                        event
                            .get("prisma_access.event.request.url")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && (["wildfire", "wildfire-virus"]
                        .contains(&event.get_str("cef.extensions.Name").unwrap_or(""))
                        || ["wildfire", "wildfire-virus"]
                            .contains(&event.get_str("cef.name").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.request.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.name", v)?;
                }
            }

            let _cond = {
                event.has_value("prisma_access.event.request.url")
                    && event.get_str("prisma_access.event.request.url") != Some("")
                    && !(event.get_str("cef.device.event_class_id") == Some("THREAT")
                        && (["wildfire", "wildfire-virus"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or(""))
                            || ["wildfire", "wildfire-virus"]
                                .contains(&event.get_str("cef.name").unwrap_or(""))))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "prisma_access.event.request.url", "url", true, false)?;
                    Ok(())
                })();
            }

            let _cond = {
                !(event.get_str("cef.device.event_class_id") == Some("THREAT")
                    && (["wildfire", "wildfire-virus"]
                        .contains(&event.get_str("cef.extensions.Name").unwrap_or(""))
                        || ["wildfire", "wildfire-virus"]
                            .contains(&event.get_str("cef.name").unwrap_or(""))))
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_access.event.request.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSRootCNLength") {
                    if let Some(val) = event.get("cef.extensions.PanOSRootCNLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSRootCNLength".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.root.cn_length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSRootCNLength_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSRootCommonName") {
                event.rename(
                    "cef.extensions.PanOSRootCommonName",
                    "prisma_access.event.root.common_name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSRootStatus") {
                event.rename(
                    "cef.extensions.PanOSRootStatus",
                    "prisma_access.event.root.status",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.device.receipt_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_extension__rt")?;
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.device.receipt_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "DECRYPTION",
                        "FILE",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString1") {
                    event.rename(
                        "cef.extensions.deviceCustomString1",
                        "prisma_access.event.rule.value",
                    )?;
                }
            }

            if let Some(v) = event
                .get("prisma_access.event.rule.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSRuleMatched") {
                event.rename(
                    "cef.extensions.PanOSRuleMatched",
                    "prisma_access.event.rule.matched",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.rule.matched")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("cef.extensions.PanOSRuleMatchedUUID") {
                event.rename(
                    "cef.extensions.PanOSRuleMatchedUUID",
                    "prisma_access.event.rule.matched_uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.rule.matched_uuid") };
            if _cond {
                event.append_unique(
                    "rule.uuid",
                    json!(
                        event
                            .get("prisma_access.event.rule.matched_uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSRuleUUID") {
                event.rename(
                    "cef.extensions.PanOSRuleUUID",
                    "prisma_access.event.rule.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.rule.uuid") };
            if _cond {
                event.append_unique(
                    "rule.uuid",
                    json!(
                        event
                            .get("prisma_access.event.rule.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSanctionedStateofApp") {
                    if let Some(val) = event.get("cef.extensions.PanOSSanctionedStateofApp") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSanctionedStateofApp".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.sanctioned_state_of_app", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSanctionedStateofApp_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSanctionedStateOfApp") {
                    if let Some(val) = event.get("cef.extensions.PanOSSanctionedStateOfApp") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSanctionedStateOfApp".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.sanctioned_state_of_app", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSanctionedStateOfApp_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSccpCallingGt") {
                event.rename(
                    "cef.extensions.PanOSSccpCallingGt",
                    "prisma_access.event.sccp_calling.gt",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSccpCallingSSN") {
                    if let Some(val) = event.get("cef.extensions.PanOSSccpCallingSSN") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSccpCallingSSN".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.sccp_calling.ssn", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSccpCallingSSN_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSctpCauseCode") {
                event.rename(
                    "cef.extensions.PanOSSctpCauseCode",
                    "prisma_access.event.sctp.cause_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSctpChunkType") {
                event.rename(
                    "cef.extensions.PanOSSctpChunkType",
                    "prisma_access.event.sctp.chunk_type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSCTPEventType") {
                event.rename(
                    "cef.extensions.PanOSSCTPEventType",
                    "prisma_access.event.sctp.event_type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSctpFilter") {
                event.rename(
                    "cef.extensions.PanOSSctpFilter",
                    "prisma_access.event.sctp.filter",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSDWANCluster") {
                event.rename(
                    "cef.extensions.PanOSSDWANCluster",
                    "prisma_access.event.sdwan.cluster.name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSDWANClusterType") {
                event.rename(
                    "cef.extensions.PanOSSDWANClusterType",
                    "prisma_access.event.sdwan.cluster.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSDWANDeviceType") {
                event.rename(
                    "cef.extensions.PanOSSDWANDeviceType",
                    "prisma_access.event.sdwan.device_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSDWANFECRatio") {
                    if let Some(val) = event.get("cef.extensions.PanOSSDWANFECRatio") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSDWANFECRatio".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.sdwan.fec_ratio", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSDWANFECRatio_to_double",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSDWANPolicyName") {
                event.rename(
                    "cef.extensions.PanOSSDWANPolicyName",
                    "prisma_access.event.sdwan.policy_name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSDWANSite") {
                event.rename(
                    "cef.extensions.PanOSSDWANSite",
                    "prisma_access.event.sdwan.site",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSenderEmail") {
                event.rename(
                    "cef.extensions.PanOSSenderEmail",
                    "prisma_access.event.sender_email",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.sender_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.email.address", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.sender_email") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("prisma_access.event.sender_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.sender_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.sender_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSequenceNo") {
                if let Some(val) = event.get("cef.extensions.PanOSSequenceNo") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.PanOSSequenceNo".into(),
                            message,
                        }
                    })?;
                    event.set("prisma_access.event.sequence_no", converted)?;
                }
            }

            if let Some(v) = event
                .get("prisma_access.event.sequence_no")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("cef.extensions.PanOSServerNameIndication") {
                event.rename(
                    "cef.extensions.PanOSServerNameIndication",
                    "prisma_access.event.server.name_indication",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.server.name_indication")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.server_name", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.server.name_indication") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.server.name_indication")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSServerPerformance") {
                event.rename(
                    "cef.extensions.PanOSServerPerformance",
                    "prisma_access.event.server.perfomance",
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["TRAFFIC", "TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomNumber3") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomNumber3") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomNumber3".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.session.duration", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomNumber3_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.session.duration")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            if event.has_value("cef.extensions.PanOSSessionEndReason") {
                event.rename(
                    "cef.extensions.PanOSSessionEndReason",
                    "prisma_access.event.session.end_reason",
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "DECRYPTION",
                        "FILE",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomNumber1") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber1") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber1".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.session.id", converted)?;
                    }
                }
            }

            if event.has_value("cef.extensions.PanOSSessionID") {
                event.rename(
                    "cef.extensions.PanOSSessionID",
                    "prisma_access.event.session.id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSessionOwnerMidx") {
                    if let Some(val) = event.get("cef.extensions.PanOSSessionOwnerMidx") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSessionOwnerMidx".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.session.owner_midx", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSessionOwnerMidx_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSSessionStartTime")
                    && event.get_str("cef.extensions.PanOSSessionStartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSSessionStartTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.session.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSSessionStartTime".into(),
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
                        "date_extension_PanOSSessionStartTime",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.session.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("cef.extensions.PanOSSessionTracker") {
                event.rename(
                    "cef.extensions.PanOSSessionTracker",
                    "prisma_access.event.session.tracker",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSeverity") {
                event.rename(
                    "cef.extensions.PanOSSeverity",
                    "prisma_access.event.severity",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            if event.has_value("cef.extensions.PanOSSigFlags") {
                event.rename(
                    "cef.extensions.PanOSSigFlags",
                    "prisma_access.event.sig_flags",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSNILength") {
                    if let Some(val) = event.get("cef.extensions.PanOSSNILength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSNILength".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.sni_length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSNILength_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSource") {
                event.rename(
                    "cef.extensions.PanOSSource",
                    "prisma_access.event.source.value",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.sourceAddress")
                    && event.get_str("cef.extensions.sourceAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.sourceAddress") {
                        if let Some(val) = event.get("cef.extensions.sourceAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.sourceAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.source.address.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_sourceAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("prisma_access.event.source.address.value")
                    && event.has_value("cef.device.event_class_id")
                    && !(["GLOBALPROTECT"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or("")))
            };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.address.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.source.address.value")
                    && event.has_value("cef.device.event_class_id")
                    && ["GLOBALPROTECT"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                event.append_unique(
                    "source.nat.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.address.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.address.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.address.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceCategory") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceCategory",
                    "prisma_access.event.source.device.category",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceClass") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceClass",
                    "prisma_access.event.source.device.class",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceHost") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceHost",
                    "prisma_access.event.source.device.host",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.device.host") };
            if _cond {
                event.append_unique(
                    "source.domain",
                    json!(
                        event
                            .get("prisma_access.event.source.device.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.device.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.source.device.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceMac") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceMac",
                    "prisma_access.event.source.device.mac",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.source.device.mac")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.mac", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.mac") {
                    gsub_field(
                        event,
                        "source.mac",
                        "source.mac",
                        cached_regex!("(..)(?!$)"),
                        "$1-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_source_mac")?;
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("source.mac") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("source.mac") {
                        map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set("_ingest.on_failure_processor_tag", "uppercase_source_mac")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSourceDeviceModel") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceModel",
                    "prisma_access.event.source.device.model",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceOS") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceOS",
                    "prisma_access.event.source.device.os.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceOSFamily") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceOSFamily",
                    "prisma_access.event.source.device.os.family",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceOSVersion") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceOSVersion",
                    "prisma_access.event.source.device.os.version",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceProfile") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceProfile",
                    "prisma_access.event.source.device.profile",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDeviceVendor") {
                event.rename(
                    "cef.extensions.PanOSSourceDeviceVendor",
                    "prisma_access.event.source.device.vendor",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceDynamicAddressGroup") {
                event.rename(
                    "cef.extensions.PanOSSourceDynamicAddressGroup",
                    "prisma_access.event.source.dynamic_address_group",
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceEDL") {
                event.rename(
                    "cef.extensions.PanOSSourceEDL",
                    "prisma_access.event.source.edl",
                )?;
            }

            if event.has_value("cef.extensions.sourceHostName") {
                event.rename(
                    "cef.extensions.sourceHostName",
                    "prisma_access.event.source.host_name",
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.source.host_name")
                    && event.get_str("cef.device.event_class_id") == Some("GLOBALPROTECT")
            };
            if _cond {
                event.append_unique(
                    "source.domain",
                    json!(
                        event
                            .get("prisma_access.event.source.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("prisma_access.event.source.host_name")
                    && event.get_str("cef.device.event_class_id") == Some("HIPMATCH")
            };
            if _cond {
                event.append_unique(
                    "host.name",
                    json!(
                        event
                            .get("prisma_access.event.source.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.source.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceLocation") {
                event.rename(
                    "cef.extensions.PanOSSourceLocation",
                    "prisma_access.event.source.location",
                )?;
            }

            if event.has_value("cef.extensions.sourceNtDomain") {
                event.rename(
                    "cef.extensions.sourceNtDomain",
                    "prisma_access.event.source.nt_domain",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.nt_domain") };
            if _cond {
                event.append_unique(
                    "source.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.source.nt_domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.sourcePort") {
                    if let Some(val) = event.get("cef.extensions.sourcePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.sourcePort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_sourcePort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("cef.extensions.PanOSSourceRegion") {
                event.rename(
                    "cef.extensions.PanOSSourceRegion",
                    "prisma_access.event.source.region",
                )?;
            }

            if event.has_value("cef.extensions.sourceServiceName") {
                event.rename(
                    "cef.extensions.sourceServiceName",
                    "prisma_access.event.source.service_name",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.sourceTranslatedAddress")
                    && event.get_str("cef.extensions.sourceTranslatedAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.sourceTranslatedAddress") {
                        if let Some(val) = event.get("cef.extensions.sourceTranslatedAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.sourceTranslatedAddress".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("prisma_access.event.source.translated.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_sourceTranslatedAddress_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.source.translated.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.ip", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.source.translated.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.source.translated.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.sourceTranslatedPort") {
                    if let Some(val) = event.get("cef.extensions.sourceTranslatedPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.sourceTranslatedPort".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.source.translated.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_sourceTranslatedPort_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if let Some(v) = event
                .get("prisma_access.event.source.translated.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.port", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSSourceUser")
                    && event
                        .get("cef.extensions.PanOSSourceUser")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.PanOSSourceUser") {
                        // Grok pattern: (?P<prisma_access_event_pan_os_value_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_value_source_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_pan_os_value_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_value_source_user_name>[^\\\\]*)", [("prisma_access_event_pan_os_value_source_user_domain", "prisma_access.event.pan_os_value.source.user.domain"), ("prisma_access_event_pan_os_value_source_user_name", "prisma_access.event.pan_os_value.source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_PanOSSourceUser")?;
                    if event.has_value("cef.extensions.PanOSSourceUser") {
                        event.rename(
                            "cef.extensions.PanOSSourceUser",
                            "prisma_access.event.pan_os_value.source.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSSourceUser")
                    && !(event
                        .get("cef.extensions.PanOSSourceUser")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.PanOSSourceUser") {
                    event.rename(
                        "cef.extensions.PanOSSourceUser",
                        "prisma_access.event.pan_os_value.source.user.name",
                    )?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_value.source.user.domain") };
            if _cond {
                event.append_unique(
                    "source.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_value.source.user.name") };
            if _cond {
                event.append_unique(
                    "source.user.name",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_value.source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_value.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceUserDomain") {
                event.rename(
                    "cef.extensions.PanOSSourceUserDomain",
                    "prisma_access.event.pan_os.source.user.domain",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os.source.user.domain") };
            if _cond {
                event.append_unique(
                    "source.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os.source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.sourceUserId") {
                event.rename(
                    "cef.extensions.sourceUserId",
                    "prisma_access.event.source.user.id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.id") };
            if _cond {
                event.append_unique(
                    "source.user.id",
                    json!(
                        event
                            .get("prisma_access.event.source.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.source.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSSourceUserName")
                    && event
                        .get("cef.extensions.PanOSSourceUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.PanOSSourceUserName") {
                        // Grok pattern: (?P<prisma_access_event_pan_os_data_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_data_source_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_pan_os_data_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_pan_os_data_source_user_name>[^\\\\]*)", [("prisma_access_event_pan_os_data_source_user_domain", "prisma_access.event.pan_os_data.source.user.domain"), ("prisma_access_event_pan_os_data_source_user_name", "prisma_access.event.pan_os_data.source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_PanOSSourceUserName",
                    )?;
                    if event.has_value("cef.extensions.PanOSSourceUserName") {
                        event.rename(
                            "cef.extensions.PanOSSourceUserName",
                            "prisma_access.event.pan_os_data.source.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSSourceUserName")
                    && !(event
                        .get("cef.extensions.PanOSSourceUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.PanOSSourceUserName") {
                    event.rename(
                        "cef.extensions.PanOSSourceUserName",
                        "prisma_access.event.pan_os_data.source.user.name",
                    )?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_data.source.user.domain") };
            if _cond {
                event.append_unique(
                    "source.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_data.source.user.name") };
            if _cond {
                event.append_unique(
                    "source.user.name",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.pan_os_data.source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.pan_os_data.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.suser0") {
                event.rename("cef.extensions.suser0", "prisma_access.event.s_user_0")?;
            }

            if event.has_value("cef.extensions.duser0") {
                event.rename("cef.extensions.duser0", "prisma_access.event.d_user_0")?;
            }

            let _cond = {
                event.has_value("cef.extensions.sourceUserName")
                    && event
                        .get("cef.extensions.sourceUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cef.extensions.sourceUserName") {
                        // Grok pattern: (?P<prisma_access_event_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_source_user_name>[^\\\\]*)
                        if !cached_grok_mapped!("(?P<prisma_access_event_source_user_domain>[^\\\\]*)[\\\\]*(?P<prisma_access_event_source_user_name>[^\\\\]*)", [("prisma_access_event_source_user_domain", "prisma_access.event.source.user.domain"), ("prisma_access_event_source_user_name", "prisma_access.event.source.user.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_sourceUserName")?;
                    if event.has_value("cef.extensions.sourceUserName") {
                        event.rename(
                            "cef.extensions.sourceUserName",
                            "prisma_access.event.source.user.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cef.extensions.sourceUserName")
                    && !(event
                        .get("cef.extensions.sourceUserName")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename(
                        "cef.extensions.sourceUserName",
                        "prisma_access.event.source.user.name",
                    )?;
                }
            }

            let _cond = { event.has_value("prisma_access.event.source.user.domain") };
            if _cond {
                event.append_unique(
                    "source.user.domain",
                    json!(
                        event
                            .get("prisma_access.event.source.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.name") };
            if _cond {
                event.append_unique(
                    "source.user.name",
                    json!(
                        event
                            .get("prisma_access.event.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceUserUUID") {
                event.rename(
                    "cef.extensions.PanOSSourceUserUUID",
                    "prisma_access.event.source.user.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.uuid") };
            if _cond {
                event.append_unique(
                    "source.user.id",
                    json!(
                        event
                            .get("prisma_access.event.source.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.user.uuid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.source.user.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSSourceUUID") {
                event.rename(
                    "cef.extensions.PanOSSourceUUID",
                    "prisma_access.event.source.uuid",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.source.uuid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_access.event.source.uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSplitTunnelconfiguration") {
                    if let Some(val) = event.get("cef.extensions.PanOSSplitTunnelconfiguration") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSplitTunnelconfiguration".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.split_tunnel_configuration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSplitTunnelconfiguration_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSSSLFailureReason") {
                event.rename(
                    "cef.extensions.PanOSSSLFailureReason",
                    "prisma_access.event.ssl.failure_reason",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSSSLResponseTime") {
                    if let Some(val) = event.get("cef.extensions.PanOSSSLResponseTime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSSSLResponseTime".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.ssl.response_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSSSLResponseTime_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSStage") {
                event.rename("cef.extensions.PanOSStage", "prisma_access.event.stage")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSStandardPortsOfApp") {
                    if let Some(val) = event.get("cef.extensions.PanOSStandardPortsOfApp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSStandardPortsOfApp".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.standard_ports_of_app", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSStandardPortsOfApp_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.startTime")
                    && event.get_str("cef.extensions.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.startTime") {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("prisma_access.event.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.startTime".into(),
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
                        "date_extension_startTime",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("cef.extensions.PanOSStreamID") {
                event.rename(
                    "cef.extensions.PanOSStreamID",
                    "prisma_access.event.stream_id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTag") {
                event.rename("cef.extensions.PanOSTag", "prisma_access.event.tag.value")?;
            }

            if event.has_value("cef.extensions.PanOSTagName") {
                event.rename(
                    "cef.extensions.PanOSTagName",
                    "prisma_access.event.tag.name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTemplate") {
                event.rename(
                    "cef.extensions.PanOSTemplate",
                    "prisma_access.event.template",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTenantID") {
                event.rename(
                    "cef.extensions.PanOSTenantID",
                    "prisma_access.event.tenant_id",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.tenant_id") };
            if _cond {
                event.append_unique(
                    "cloud.account.id",
                    json!(
                        event
                            .get("prisma_access.event.tenant_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSThreatCategory") {
                event.rename(
                    "cef.extensions.PanOSThreatCategory",
                    "prisma_access.event.threat.category",
                )?;
            }

            if event.has_value("cef.extensions.PanOSThreatID") {
                event.rename(
                    "cef.extensions.PanOSThreatID",
                    "prisma_access.event.threat.id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSThreatNameFirewall") {
                event.rename(
                    "cef.extensions.PanOSThreatNameFirewall",
                    "prisma_access.event.threat.name_firewall",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSTimeGeneratedHighResolution")
                    && event.get_str("cef.extensions.PanOSTimeGeneratedHighResolution") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSTimeGeneratedHighResolution")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "prisma_access.event.time.generated_high_resolution",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSTimeGeneratedHighResolution".into(),
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
                        "date_extension_PanOSTimeGeneratedHighResolution",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSTimeNotAfter")
                    && event.get_str("cef.extensions.PanOSTimeNotAfter") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.PanOSTimeNotAfter")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601", "UNIX", "UNIX_MS"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.time.not_after", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSTimeNotAfter".into(),
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
                        "date_extension_PanOSTimeNotAfter",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.time.not_after")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.not_after", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSTimeNotBefore")
                    && event.get_str("cef.extensions.PanOSTimeNotBefore") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.PanOSTimeNotBefore")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601", "UNIX", "UNIX_MS"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("prisma_access.event.time.not_before", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSTimeNotBefore".into(),
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
                        "date_extension_PanOSTimeNotBefore",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.time.not_before")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.not_before", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSTimeReceivedManagementPlane")
                    && event.get_str("cef.extensions.PanOSTimeReceivedManagementPlane") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSTimeReceivedManagementPlane")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "prisma_access.event.time.received_management_plane",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSTimeReceivedManagementPlane".into(),
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
                        "date_extension_PanOSTimeReceivedManagementPlane",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.PanOSTimestampDeviceIdentification")
                    && event.get_str("cef.extensions.PanOSTimestampDeviceIdentification")
                        != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cef.extensions.PanOSTimestampDeviceIdentification")
                    {
                        match parse_date_out(
                            &date_str,
                            &["MMM dd yyyy HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "prisma_access.event.timestamp_device_identification",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.PanOSTimestampDeviceIdentification"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("cef.extensions.PanOSTLSAuth") {
                event.rename(
                    "cef.extensions.PanOSTLSAuth",
                    "prisma_access.event.tls.auth",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.tls.auth")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.x509.signature_algorithm", v)?;
            }

            if event.has_value("cef.extensions.PanOSTLSEncryptionAlgorithm") {
                event.rename(
                    "cef.extensions.PanOSTLSEncryptionAlgorithm",
                    "prisma_access.event.tls.encryption_algorithm",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTLSKeyExchange") {
                event.rename(
                    "cef.extensions.PanOSTLSKeyExchange",
                    "prisma_access.event.tls.key_exchange",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTLSVersion") {
                event.rename(
                    "cef.extensions.PanOSTLSVersion",
                    "prisma_access.event.tls.version",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.tls.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.version", v)?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "DECRYPTION",
                        "FILE",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString5") {
                    event.rename(
                        "cef.extensions.deviceCustomString5",
                        "prisma_access.event.to_zone",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSTotalDiskSpace")
                    && event.get_str("cef.extensions.PanOSTotalDiskSpace") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSTotalDiskSpace") {
                        if let Some(val) = event.get("cef.extensions.PanOSTotalDiskSpace") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSTotalDiskSpace".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.total.disk_space", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSTotalDiskSpace_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSTotalMemory") {
                    if let Some(val) = event.get("cef.extensions.PanOSTotalMemory") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSTotalMemory".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.total.memory", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSTotalMemory_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.get_str("cef.device.event_class_id") == Some("DNS SECURITY") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomNumber3") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomNumber3") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomNumber3".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.total.time_elapsed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomNumber3_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.total.time_elapsed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            if event.has_value("cef.extensions.PanOSTpadding") {
                event.rename(
                    "cef.extensions.PanOSTpadding",
                    "prisma_access.event.tpadding",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("TUNNEL") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    event.rename(
                        "cef.extensions.deviceCustomString2",
                        "prisma_access.event.tunnel.value",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSTunnel") {
                event.rename(
                    "cef.extensions.PanOSTunnel",
                    "prisma_access.event.tunnel.value",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelCauseCode") {
                event.rename(
                    "cef.extensions.PanOSTunnelCauseCode",
                    "prisma_access.event.tunnel.cause_code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelEndpointID1") {
                event.rename(
                    "cef.extensions.PanOSTunnelEndpointID1",
                    "prisma_access.event.tunnel.endpoint.id1",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelEndpointID2") {
                event.rename(
                    "cef.extensions.PanOSTunnelEndpointID2",
                    "prisma_access.event.tunnel.endpoint.id2",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelEventCode") {
                event.rename(
                    "cef.extensions.PanOSTunnelEventCode",
                    "prisma_access.event.tunnel.event.code",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelEventType") {
                event.rename(
                    "cef.extensions.PanOSTunnelEventType",
                    "prisma_access.event.tunnel.event.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelInspectionRule") {
                event.rename(
                    "cef.extensions.PanOSTunnelInspectionRule",
                    "prisma_access.event.tunnel.inspection_rule",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelInterface") {
                event.rename(
                    "cef.extensions.PanOSTunnelInterface",
                    "prisma_access.event.tunnel.interface",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelMessageType") {
                event.rename(
                    "cef.extensions.PanOSTunnelMessageType",
                    "prisma_access.event.tunnel.message_type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunnelRemoteIMSIID") {
                event.rename(
                    "cef.extensions.PanOSTunnelRemoteIMSIID",
                    "prisma_access.event.tunnel.remote.imsi_id",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSTunnelRemoteUserIP")
                    && event.get_str("cef.extensions.PanOSTunnelRemoteUserIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSTunnelRemoteUserIP") {
                        if let Some(val) = event.get("cef.extensions.PanOSTunnelRemoteUserIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSTunnelRemoteUserIP".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.tunnel.remote.user_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSTunnelRemoteUserIP_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.tunnel.remote.user_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.tunnel.remote.user_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSTunnelRename") {
                    if let Some(val) = event.get("cef.extensions.PanOSTunnelRename") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSTunnelRename".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.tunnel.rename", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSTunnelRename_to_boolean",
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
                            .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.deviceCustomFloatingPoint1")
                    && event.get_str("cef.extensions.deviceCustomFloatingPoint1") != Some("")
                    && event.has_value("cef.device.event_class_id")
                    && ["TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomFloatingPoint1") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomFloatingPoint1") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomFloatingPoint1".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.packets.dropped.protocol", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomFloatingPoint1_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.deviceCustomFloatingPoint2")
                    && event.get_str("cef.extensions.deviceCustomFloatingPoint2") != Some("")
                    && event.has_value("cef.device.event_class_id")
                    && ["TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomFloatingPoint2") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomFloatingPoint2") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomFloatingPoint2".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.packets.dropped.strict", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomFloatingPoint2_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.deviceCustomFloatingPoint4")
                    && event.get_str("cef.extensions.deviceCustomFloatingPoint4") != Some("")
                    && event.has_value("cef.device.event_class_id")
                    && ["TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomFloatingPoint4") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomFloatingPoint4") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomFloatingPoint4".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.tunnel.sessions.closed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomFloatingPoint4_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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
                event.has_value("cef.extensions.deviceCustomFloatingPoint3")
                    && event.get_str("cef.extensions.deviceCustomFloatingPoint3") != Some("")
                    && event.has_value("cef.device.event_class_id")
                    && ["TUNNEL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceCustomFloatingPoint3") {
                        if let Some(val) = event.get("cef.extensions.deviceCustomFloatingPoint3") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceCustomFloatingPoint3".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.tunnel.sessions.created", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_deviceCustomFloatingPoint3_to_double",
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
                                .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSTunnelType") {
                event.rename(
                    "cef.extensions.PanOSTunnelType",
                    "prisma_access.event.tunnel.type",
                )?;
            }

            if event.has_value("cef.extensions.PanOSTunneledApplication") {
                event.rename(
                    "cef.extensions.PanOSTunneledApplication",
                    "prisma_access.event.tunneled_application",
                )?;
            }

            if event.has_value("cef.extensions.PanOSType") {
                event.rename("cef.extensions.PanOSType", "prisma_access.event.type")?;
            }

            if event.has_value("cef.extensions.PanOSUGFlags") {
                event.rename(
                    "cef.extensions.PanOSUGFlags",
                    "prisma_access.event.ug_flags",
                )?;
            }

            if event.has_value("cef.extensions.PanOSURL") {
                event.rename("cef.extensions.PanOSURL", "prisma_access.event.url.value")?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["FILE", "TRAFFIC", "URL"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    event.rename(
                        "cef.extensions.deviceCustomString2",
                        "prisma_access.event.url.category.value",
                    )?;
                }
            }

            if event.has_value("cef.extensions.PanOSURLCategory") {
                event.rename(
                    "cef.extensions.PanOSURLCategory",
                    "prisma_access.event.url.category.value",
                )?;
            }

            if event.has_value("cef.extensions.PanOSURLCategoryList") {
                event.rename(
                    "cef.extensions.PanOSURLCategoryList",
                    "prisma_access.event.url.category.list",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.PanOSURLCounter") {
                    if let Some(val) = event.get("cef.extensions.PanOSURLCounter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.PanOSURLCounter".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.url.counter", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extension_PanOSURLCounter_to_long",
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
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("cef.extensions.PanOSURLDomain") {
                event.rename(
                    "cef.extensions.PanOSURLDomain",
                    "prisma_access.event.url.domain",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.url.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.domain", v)?;
            }

            if event.has_value("cef.extensions.PanOSUserAgentString") {
                event.rename(
                    "cef.extensions.PanOSUserAgentString",
                    "prisma_access.event.user.agent_string",
                )?;
            }

            if event.has_value("prisma_access.event.user.agent_string") {
                if let Some(ua_str) = event.get_string("prisma_access.event.user.agent_string") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("cef.extensions.PanOSUserComment") {
                event.rename(
                    "cef.extensions.PanOSUserComment",
                    "prisma_access.event.user.comment",
                )?;
            }

            if event.has_value("cef.extensions.PanOSUserGroupFound") {
                event.rename(
                    "cef.extensions.PanOSUserGroupFound",
                    "prisma_access.event.user.group_found",
                )?;
            }

            if event.has_value("cef.extensions.PanOSUserIdentifiedBySource") {
                event.rename(
                    "cef.extensions.PanOSUserIdentifiedBySource",
                    "prisma_access.event.user.identified_by_source",
                )?;
            }

            let _cond = { event.has_value("prisma_access.event.user.identified_by_source") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.user.identified_by_source")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSUsername") {
                event.rename(
                    "cef.extensions.PanOSUsername",
                    "prisma_access.event.username",
                )?;
            }

            if let Some(v) = event
                .get("prisma_access.event.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSUsers")
                    && event.get_str("cef.extensions.PanOSUsers") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSUsers") {
                        if let Some(val) = event.get("cef.extensions.PanOSUsers") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSUsers".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.users.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSUsers_to_ip",
                    )?;
                    if event.has_value("cef.extensions.PanOSUsers") {
                        event.rename(
                            "cef.extensions.PanOSUsers",
                            "prisma_access.event.users.name",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("prisma_access.event.users.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("prisma_access.event.users.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.PanOSUUID") {
                event.rename("cef.extensions.PanOSUUID", "prisma_access.event.uuid")?;
            }

            if event.has_value("cef.extensions.PanOSVDIEndpoint") {
                if let Some(val) = event.get("cef.extensions.PanOSVDIEndpoint") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.PanOSVDIEndpoint".into(),
                            message,
                        }
                    })?;
                    event.set("prisma_access.event.vdi_endpoint", converted)?;
                }
            }

            let _cond = {
                event.has_value("prisma_access.event.vdi_endpoint")
                    && event.get_str("prisma_access.event.vdi_endpoint") == Some("1")
            };
            if _cond {
                let v = json!("virtual desktop infrastructure");
                if !painless_is_empty_value(&v) {
                    event.set("host.type", v)?;
                }
            }

            if event.has_value("cef.extensions.PanOSVendorSeverity") {
                event.rename(
                    "cef.extensions.PanOSVendorSeverity",
                    "prisma_access.event.vendor_severity",
                )?;
            }

            if event.has_value("cef.extensions.PanOSVerdict") {
                event.rename("cef.extensions.PanOSVerdict", "prisma_access.event.verdict")?;
            }

            if event.has_value("cef.extensions.PanOSVerificationTag1") {
                event.rename(
                    "cef.extensions.PanOSVerificationTag1",
                    "prisma_access.event.verification.tag1",
                )?;
            }

            if event.has_value("cef.extensions.PanOSVerificationTag2") {
                event.rename(
                    "cef.extensions.PanOSVerificationTag2",
                    "prisma_access.event.verification.tag2",
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && [
                        "CONFIG",
                        "SYSTEM",
                        "AUTH",
                        "DECRYPTION",
                        "FILE",
                        "HIPMATCH",
                        "IPTAG",
                        "SCTP",
                        "THREAT",
                        "TRAFFIC",
                        "TUNNEL",
                        "URL",
                        "USERID",
                    ]
                    .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
                    || (event.has_value("cef.extensions")
                        && ["url", "file"]
                            .contains(&event.get_str("cef.extensions.Name").unwrap_or("")))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString3") {
                    event.rename(
                        "cef.extensions.deviceCustomString3",
                        "prisma_access.event.virtual.location",
                    )?;
                }
            }

            if event.has_value("cef.extensions.deviceCustomString3Label") {
                event.rename(
                    "cef.extensions.deviceCustomString3Label",
                    "prisma_access.event.label.cs3",
                )?;
            }

            if event.has_value("cef.extensions.PanOSVirtualSystem") {
                event.rename(
                    "cef.extensions.PanOSVirtualSystem",
                    "prisma_access.event.virtual.system.value",
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && ["HIPMATCH", "IPTAG", "USERID"]
                        .contains(&event.get_str("cef.device.event_class_id").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomNumber2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber2") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber2".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_access.event.virtual.system.id", converted)?;
                    }
                }
            }

            if event.has_value("cef.extensions.PanOSVirtualSystemID") {
                event.rename(
                    "cef.extensions.PanOSVirtualSystemID",
                    "prisma_access.event.virtual.system.id",
                )?;
            }

            if event.has_value("cef.extensions.PanOSVirtualSystemName") {
                event.rename(
                    "cef.extensions.PanOSVirtualSystemName",
                    "prisma_access.event.virtual.system.name",
                )?;
            }

            if event.has_value("cef.extensions.PanOSVpadding") {
                event.rename(
                    "cef.extensions.PanOSVpadding",
                    "prisma_access.event.vpadding",
                )?;
            }

            let _cond = { event.get_str("cef.device.event_class_id") == Some("GLOBALPROTECT") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString3") {
                    event.rename(
                        "cef.extensions.deviceCustomString3",
                        "prisma_access.event.v.sys_name",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSXForwardedFor")
                    && event.get_str("cef.extensions.PanOSXForwardedFor") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSXForwardedFor") {
                        if let Some(val) = event.get("cef.extensions.PanOSXForwardedFor") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSXForwardedFor".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.x_forwarded_for.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSXForwardedFor_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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
                .get("prisma_access.event.x_forwarded_for.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.forwarded_ip", v)?;
            }

            let _cond = { event.has_value("prisma_access.event.x_forwarded_for.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.x_forwarded_for.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.PanOSXForwardedForIP")
                    && event.get_str("cef.extensions.PanOSXForwardedForIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.PanOSXForwardedForIP") {
                        if let Some(val) = event.get("cef.extensions.PanOSXForwardedForIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.PanOSXForwardedForIP".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.x_forwarded_for.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extension_PanOSXForwardedForIP_to_ip",
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
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("prisma_access.event.x_forwarded_for.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("prisma_access.event.x_forwarded_for.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.Vendor Severity") {
                event.rename(
                    "cef.extensions.Vendor Severity",
                    "prisma_access.event.vendor_severity",
                )?;
            }

            if event.has_value("cef.extensions.Name") {
                event.rename("cef.extensions.Name", "prisma_access.event.name")?;
            }

            if event.has_value("cef.extensions.Subtype") {
                event.rename("cef.extensions.Subtype", "prisma_access.event.name")?;
            }

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
                event.remove("prisma_access.event.application.protocol");
                event.remove("prisma_access.event.assocation_end_reason");
                event.remove("prisma_access.event.authenticated.user.domain");
                event.remove("prisma_access.event.authenticated.user.name");
                event.remove("prisma_access.event.authenticated.user.uuid");
                event.remove("prisma_access.event.bytes.total");
                event.remove("prisma_access.event.bytes.in");
                event.remove("prisma_access.event.bytes.out");
                event.remove("prisma_access.event.certificate.serial");
                event.remove("prisma_access.event.certificate.version");
                event.remove("prisma_access.event.cloud.hostname");
                event.remove("prisma_access.event.common.name.value");
                event.remove("prisma_access.event.config_version");
                event.remove("prisma_access.event.container.id");
                event.remove("prisma_access.event.container.name.value");
                event.remove("prisma_access.event.cortex_data_lake_tenant_id");
                event.remove("prisma_access.event.debug_log_file");
                event.remove("prisma_access.event.description");
                event.remove("prisma_access.event.destination.address.value");
                event.remove("prisma_access.event.destination.address.v6");
                event.remove("prisma_access.event.destination.device.host");
                event.remove("prisma_access.event.destination.device.mac");
                event.remove("prisma_access.event.destination.device.vendor");
                event.remove("prisma_access.event.destination.nt_domain");
                event.remove("prisma_access.event.destination.port");
                event.remove("prisma_access.event.destination.translated.address");
                event.remove("prisma_access.event.destination.translated.port");
                event.remove("prisma_access.event.pan_os_value.destination.user.domain");
                event.remove("prisma_access.event.pan_os_value.destination.user.name");
                event.remove("prisma_access.event.pan_os_data.destination.user.domain");
                event.remove("prisma_access.event.pan_os_data.destination.user.name");
                event.remove("prisma_access.event.pan_os.destination.user.domain");
                event.remove("prisma_access.event.destination.user.domain");
                event.remove("prisma_access.event.destination.user.id");
                event.remove("prisma_access.event.destination.user.name");
                event.remove("prisma_access.event.destination.user.uuid");
                event.remove("prisma_access.event.device.action");
                event.remove("prisma_access.event.device.sn");
                event.remove("prisma_access.event.device.external_id");
                event.remove("prisma_access.event.device.host_name");
                event.remove("prisma_access.event.device.inbound_interface");
                event.remove("prisma_access.event.device.name");
                event.remove("prisma_access.event.device.outbound_interface");
                event.remove("prisma_access.event.device.time_zone");
                event.remove("prisma_access.event.device.vendor");
                event.remove("prisma_access.event.diam.app_id");
                event.remove("prisma_access.event.dns.response.value");
                event.remove("prisma_access.event.dns.response.code");
                event.remove("prisma_access.event.domain.value");
                event.remove("prisma_access.event.dynamic_user_group.name");
                event.remove("prisma_access.event.dynamic_user_group.value");
                event.remove("prisma_access.event.elliptic_curve");
                event.remove("prisma_access.event.email_subject");
                event.remove("prisma_access.event.end_time");
                event.remove("prisma_access.event.endpoint.cpu_architecture");
                event.remove("prisma_access.event.endpoint.device.domain");
                event.remove("prisma_access.event.endpoint.device.name");
                event.remove("prisma_access.event.endpoint.ip_address");
                event.remove("prisma_access.event.endpoint.os.type");
                event.remove("prisma_access.event.endpoint.os.version");
                event.remove("prisma_access.event.endpoint.sn");
                event.remove("prisma_access.event.endpoint.user.domain");
                event.remove("prisma_access.event.endpoint.user.name");
                event.remove("prisma_access.event.endpoint.user.uuid");
                event.remove("prisma_access.event.data.time");
                event.remove("prisma_access.event.external_id");
                event.remove("prisma_access.event.file.hash");
                event.remove("prisma_access.event.file.name");
                event.remove("prisma_access.event.fingerprint");
                event.remove("prisma_access.event.gateway.address");
                event.remove("prisma_access.event.http.method");
                event.remove("prisma_access.event.inbound_interface.value");
                event.remove("prisma_access.event.is_resume_session");
                event.remove("prisma_access.event.log.source.timezone_offset");
                event.remove("prisma_access.event.login_duration");
                event.remove("prisma_access.event.message");
                event.remove("prisma_access.event.nat.destination.port");
                event.remove("prisma_access.event.nat.destination.value");
                event.remove("prisma_access.event.nat.source.port");
                event.remove("prisma_access.event.nat.source.value");
                event.remove("prisma_access.event.packets.received");
                event.remove("prisma_access.event.packets.sent");
                event.remove("prisma_access.event.packets.total");
                event.remove("prisma_access.event.private.ipv4");
                event.remove("prisma_access.event.private.ipv6");
                event.remove("prisma_access.event.public.ipv6");
                event.remove("prisma_access.event.public.ipv4");
                event.remove("prisma_access.event.transport_protocol");
                event.remove("prisma_access.event.reason");
                event.remove("prisma_access.event.recipient_email");
                event.remove("prisma_access.event.referer");
                event.remove("prisma_access.event.request.context");
                event.remove("prisma_access.event.request.method");
                event.remove("prisma_access.event.device.receipt_time");
                event.remove("prisma_access.event.issuer.common_name");
                event.remove("prisma_access.event.rule.value");
                event.remove("prisma_access.event.rule.matched");
                event.remove("prisma_access.event.rule.matched_uuid");
                event.remove("prisma_access.event.rule.uuid");
                event.remove("prisma_access.event.sender_email");
                event.remove("prisma_access.event.sequence_no");
                event.remove("prisma_access.event.server.name_indication");
                event.remove("prisma_access.event.session.duration");
                event.remove("prisma_access.event.session.start_time");
                event.remove("prisma_access.event.severity");
                event.remove("prisma_access.event.source.address.v6");
                event.remove("prisma_access.event.source.device.host");
                event.remove("prisma_access.event.source.device.mac");
                event.remove("prisma_access.event.source.port");
                event.remove("prisma_access.event.source.translated.address");
                event.remove("prisma_access.event.source.translated.port");
                event.remove("prisma_access.event.source.nt_domain");
                event.remove("prisma_access.event.pan_os_value.source.user.domain");
                event.remove("prisma_access.event.pan_os_value.source.user.name");
                event.remove("prisma_access.event.pan_os_data.source.user.domain");
                event.remove("prisma_access.event.pan_os_data.source.user.name");
                event.remove("prisma_access.event.pan_os.source.user.domain");
                event.remove("prisma_access.event.source.user.domain");
                event.remove("prisma_access.event.source.user.id");
                event.remove("prisma_access.event.source.user.name");
                event.remove("prisma_access.event.source.user.uuid");
                event.remove("prisma_access.event.start_time");
                event.remove("prisma_access.event.tenant_id");
                event.remove("prisma_access.event.time.not_after");
                event.remove("prisma_access.event.time.not_before");
                event.remove("prisma_access.event.tls.auth");
                event.remove("prisma_access.event.tls.version");
                event.remove("prisma_access.event.total.time_elapsed");
                event.remove("prisma_access.event.username");
                event.remove("prisma_access.event.url.domain");
                event.remove("prisma_access.event.x_forwarded_for.value");
                event.remove("prisma_access.event.request.url");
                event.remove("prisma_access.event.global_protect.version");
                event.remove("prisma_access.event.operating_system");
                event.remove("prisma_access.event.source.address.value");
            }

            if event.has_value("cef.device.event_class_id") {
                event.rename("cef.device.event_class_id", "prisma_access.event.class_id")?;
            }

            if event.has_value("cef.name") {
                event.rename("cef.name", "prisma_access.event.cef.name")?;
            }

            if event.has_value("cef.device.vendor") {
                event.rename("cef.device.vendor", "prisma_access.event.cef.device.vendor")?;
            }

            if event.has_value("cef.device.product") {
                event.rename(
                    "cef.device.product",
                    "prisma_access.event.cef.device.product",
                )?;
            }

            if event.has_value("cef.device.version") {
                event.rename(
                    "cef.device.version",
                    "prisma_access.event.cef.device.version",
                )?;
            }

            if event.has_value("cef.version") {
                event.rename("cef.version", "prisma_access.event.cef.version")?;
            }

            let _cond =
                { event.has_value("cef.severity") && event.get_str("cef.severity") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.severity") {
                        if let Some(val) = event.get("cef.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_access.event.cef.severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cef_severity")?;
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
                                .get("_ingest.on_failure_pipeline")
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

            event.remove("cef");
            event.remove("_temp");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                            .get("_ingest.on_failure_pipeline")
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
