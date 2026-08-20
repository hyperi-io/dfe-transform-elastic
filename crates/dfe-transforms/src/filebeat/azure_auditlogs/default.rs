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

            if event.has("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            event.remove("routing.category");

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if let Some(s) = event.get_string("event.original") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "event.original".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("azure.auditlogs", parsed)?;
            }

            let _cond = { event.get_str("azure.auditlogs.category") != Some("AuditLogs") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.auditlogs.time") {
                    if let Some(parsed) = parse_date_out(
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
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            if event.has("azure.auditlogs.resourceId") {
                event.rename("azure.auditlogs.resourceId", "azure.resource_id")?;
            }

            let _cond = {
                event.has_value("azure.auditlogs.durationMs")
                    && event
                        .get("azure.auditlogs.durationMs")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("azure.auditlogs.durationMs") {
                        if let Some(val) = event.get("azure.auditlogs.durationMs") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "azure.auditlogs.durationMs".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "azure.auditlogs.durationMs".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    }
                                }
                                Value::Number(n) => {
                                    json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                                }
                                Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                                _ => {
                                    return Err(TransformError::ParseError {
                                        path: "azure.auditlogs.durationMs".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("event.duration", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("azure.auditlogs.durationMs")
                    && !(event
                        .get("azure.auditlogs.durationMs")
                        .is_some_and(|v| v.is_string()))
            };
            if _cond {
                if event.has("azure.auditlogs.durationMs") {
                    event.rename("azure.auditlogs.durationMs", "event.duration")?;
                }
            }

            event.remove("azure.auditlogs.durationMs");

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.event.duration * params.param_nano
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"ctx.event.duration = ctx.event.duration * params.param_nano"#
                    ),
                    cached_params!("{\"param_nano\":1000000}"),
                )?;
            }

            let _cond = {
                event.has_value("azure.auditlogs.properties.result")
                    && event
                        .get("azure.auditlogs.properties.result")
                        .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("azure.auditlogs.properties.result")
                        .is_some_and(|s| s.to_lowercase() == "success")
                        || event
                            .get_str("azure.auditlogs.properties.result")
                            .is_some_and(|s| s.to_lowercase() == "failure"))
            };
            if _cond {
                event.rename("azure.auditlogs.properties.result", "event.outcome")?;
            }

            if event.has("azure.auditlogs.level") {
                event.rename("azure.auditlogs.level", "log.level")?;
            }

            event.remove("azure.auditlogs.time");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("azure.auditlogs.operationName") {
                    if let Some(val) = event.get("azure.auditlogs.operationName") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("event.action", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("azure.auditlogs.operationName") {
                event.rename(
                    "azure.auditlogs.operationName",
                    "azure.auditlogs.operation_name",
                )?;
            }

            if event.has("azure.auditlogs.resultSignature") {
                event.rename(
                    "azure.auditlogs.resultSignature",
                    "azure.auditlogs.result_signature",
                )?;
            }

            if event.has("azure.auditlogs.resultDescription") {
                event.rename(
                    "azure.auditlogs.resultDescription",
                    "azure.auditlogs.result_description",
                )?;
            }

            if event.has("azure.auditlogs.operationVersion") {
                event.rename(
                    "azure.auditlogs.operationVersion",
                    "azure.auditlogs.operation_version",
                )?;
            }

            if event.has("azure.auditlogs.tenantId") {
                event.rename("azure.auditlogs.tenantId", "azure.tenant_id")?;
            }

            if event.has("azure.auditlogs.correlationId") {
                event.rename("azure.auditlogs.correlationId", "azure.correlation_id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("azure.auditlogs.properties.activityDisplayName") {
                    event.rename(
                        "azure.auditlogs.properties.activityDisplayName",
                        "azure.auditlogs.properties.activity_display_name",
                    )?;
                }
                Ok(())
            })();

            if event.has("azure.auditlogs.properties.activityDateTime") {
                event.rename(
                    "azure.auditlogs.properties.activityDateTime",
                    "azure.auditlogs.properties.activity_datetime",
                )?;
            }

            if event.has("azure.auditlogs.properties.additionalDetails") {
                event.rename(
                    "azure.auditlogs.properties.additionalDetails",
                    "azure.auditlogs.properties.additional_details",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("azure.auditlogs.callerIpAddress") {
                    if let Some(s) = event.get_string("azure.auditlogs.callerIpAddress") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "azure.auditlogs.callerIpAddress".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("azure.auditlogs.callerIpAddress") {
                        event.rename("azure.auditlogs.callerIpAddress", "source.address")?;
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.remove("azure.auditlogs.callerIpAddress");
            }

            let _cond = { !event.has_value("azure.auditlogs.properties.initiatedBy.app.appId") };
            if _cond {
                event.remove("azure.auditlogs.properties.initiatedBy.app.appId");
            }

            let _cond = {
                !event.has_value("azure.auditlogs.properties.initiatedBy.app.servicePrincipalName")
            };
            if _cond {
                event.remove("azure.auditlogs.properties.initiatedBy.app.servicePrincipalName");
            }

            let _cond = { !event.has_value("azure.auditlogs.properties.userAgent") };
            if _cond {
                event.remove("azure.auditlogs.properties.userAgent");
            }

            let v = json!(
                event
                    .get("source.ip")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("azure.auditlogs.properties.resultReason") {
                event.rename(
                    "azure.auditlogs.properties.resultReason",
                    "azure.auditlogs.properties.result_reason",
                )?;
            }

            if event.has("azure.auditlogs.properties.resultDescription") {
                event.rename(
                    "azure.auditlogs.properties.resultDescription",
                    "azure.auditlogs.properties.result_description",
                )?;
            }

            if event.has("azure.auditlogs.properties.correlationId") {
                event.rename(
                    "azure.auditlogs.properties.correlationId",
                    "azure.auditlogs.properties.correlation_id",
                )?;
            }

            if event.has("azure.auditlogs.properties.loggedByService") {
                event.rename(
                    "azure.auditlogs.properties.loggedByService",
                    "azure.auditlogs.properties.logged_by_service",
                )?;
            }

            if event.has("azure.auditlogs.properties.operationType") {
                event.rename(
                    "azure.auditlogs.properties.operationType",
                    "azure.auditlogs.properties.operation_type",
                )?;
            }

            if event.has("azure.auditlogs.Level") {
                event.rename("azure.auditlogs.Level", "azure.auditlogs.level")?;
            }

            if event.has("azure.auditlogs.properties.additional_details.userAgent") {
                event.rename(
                    "azure.auditlogs.properties.additional_details.userAgent",
                    "azure.auditlogs.properties.additional_details.user_agent",
                )?;
            }

            let _cond =
                { !event.has_value("azure.auditlogs.properties.initiatedBy.user.displayName") };
            if _cond {
                event.remove("azure.auditlogs.properties.initiatedBy.user.displayName");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.azure.auditlogs.properties?.targetResources != null) {\n  ctx.azure.auditlogs.properties.target_resources = new HashMap();\n  for (def i = 0; i < ctx.azure.auditlogs.properties.targetResources.length; i++) {\n    String index = String.valueOf(i);\n    ctx.azure.auditlogs.properties.target_resources[index] = new HashMap();\n    if(ctx.azure.auditlogs.properties.targetResources[i].displayName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].display_name = ctx.azure.auditlogs.properties.targetResources[i].displayName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].id = ctx.azure.auditlogs.properties.targetResources[i].id;\n    ctx.azure.auditlogs.properties.target_resources[index].type = ctx.azure.auditlogs.properties.targetResources[i].type;\n    if (ctx.azure.auditlogs.properties.targetResources[i].ipAddress != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].ip_address = ctx.azure.auditlogs.properties.targetResources[i].ipAddress;\n    }\n    if (ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].user_principal_name = ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].modified_properties = new HashMap();\n    for (def j = 0; j < ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties.length; j++) {\n      String n = String.valueOf(j);\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n] = new HashMap();\n\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].display_name = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].displayName;\n      \n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].new_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue;\n      }\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].old_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue;\n      }\n    }\n  }\n  ctx.azure.auditlogs.properties.remove('targetResources');\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.azure.auditlogs.properties?.targetResources != null) {\n  ctx.azure.auditlogs.properties.target_resources = new HashMap();\n  for (def i = 0; i < ctx.azure.auditlogs.properties.targetResources.length; i++) {\n    String index = String.valueOf(i);\n    ctx.azure.auditlogs.properties.target_resources[index] = new HashMap();\n    if(ctx.azure.auditlogs.properties.targetResources[i].displayName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].display_name = ctx.azure.auditlogs.properties.targetResources[i].displayName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].id = ctx.azure.auditlogs.properties.targetResources[i].id;\n    ctx.azure.auditlogs.properties.target_resources[index].type = ctx.azure.auditlogs.properties.targetResources[i].type;\n    if (ctx.azure.auditlogs.properties.targetResources[i].ipAddress != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].ip_address = ctx.azure.auditlogs.properties.targetResources[i].ipAddress;\n    }\n    if (ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].user_principal_name = ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].modified_properties = new HashMap();\n    for (def j = 0; j < ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties.length; j++) {\n      String n = String.valueOf(j);\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n] = new HashMap();\n\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].display_name = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].displayName;\n      \n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].new_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue;\n      }\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].old_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue;\n      }\n    }\n  }\n  ctx.azure.auditlogs.properties.remove('targetResources');\n}"#
                    ),
                )?;
                Ok(())
            })();

            if event.has("azure.auditlogs.properties.initiatedBy") {
                event.rename(
                    "azure.auditlogs.properties.initiatedBy",
                    "azure.auditlogs.properties.initiated_by",
                )?;
            }

            if let Some(v) = event
                .get("azure.auditlogs.properties.initiated_by.user.userPrincipalName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has("azure.auditlogs.properties.initiated_by.user.id") {
                if let Some(val) = event.get("azure.auditlogs.properties.initiated_by.user.id") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("user.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("azure.auditlogs.properties.initiated_by.user.displayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if event.has("source.ip") {
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

            if event.has("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("azure.auditlogs.properties.initiated_by.user.id")
                    && event
                        .get_str("azure.auditlogs.properties.initiated_by.user.id")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "related.entity",
                    json!(
                        event
                            .get("azure.auditlogs.properties.initiated_by.user.id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("azure.auditlogs.properties.initiated_by.app.servicePrincipalId")
                    && event
                        .get_str("azure.auditlogs.properties.initiated_by.app.servicePrincipalId")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "related.entity",
                    json!(
                        event
                            .get("azure.auditlogs.properties.initiated_by.app.servicePrincipalId")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.related = ctx.related ?: [:];\nctx.related.entity = ctx.related.entity ?: [];\nif (ctx.azure.auditlogs.properties?.target_resources != null) {\n    for (String k : ctx.azure.auditlogs.properties.target_resources.keySet()) {\n        def resource = ctx.azure.auditlogs.properties.target_resources[k];\n        if (resource?.id != null && resource.id != '' && !ctx.related.entity.contains(resource.id)) {\n                ctx.related.entity.add(resource.id);\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.related = ctx.related ?: [:];\nctx.related.entity = ctx.related.entity ?: [];\nif (ctx.azure.auditlogs.properties?.target_resources != null) {\n    for (String k : ctx.azure.auditlogs.properties.target_resources.keySet()) {\n        def resource = ctx.azure.auditlogs.properties.target_resources[k];\n        if (resource?.id != null && resource.id != '' && !ctx.related.entity.contains(resource.id)) {\n                ctx.related.entity.add(resource.id);\n        }\n    }\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                    if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_namespace", "azure.resource.namespace"), ("azure_resource_authorization_rule", "azure.resource.authorization_rule")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_namespace", "azure.resource.namespace"), ("azure_resource_authorization_rule", "azure.resource.authorization_rule")]).extract_into(&input, event)? {
                }
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
                        if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                }
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
                        if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                }
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
                        if !cached_grok_mapped!(
                            "/providers/(?P<azure_resource_provider>(?:.+))",
                            [("azure_resource_provider", "azure.resource.provider")]
                        )
                        .extract_into(&input, event)?
                        {
                            // Grok pattern: /PROVIDERS/(?P<azure_resource_provider>(?:.+))
                            if !cached_grok_mapped!(
                                "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                [("azure_resource_provider", "azure.resource.provider")]
                            )
                            .extract_into(&input, event)?
                            {}
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
                        if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_provider", "azure.resource.provider")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_provider", "azure.resource.provider")]).extract_into(&input, event)? {
                }
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
                        if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group")]).extract_into(&input, event)? {
                }
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
                        if !cached_grok_mapped!("/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))", [("azure_subscription_id", "azure.subscription_id")]).extract_into(&input, event)? {
                // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                if !cached_grok_mapped!("/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))", [("azure_subscription_id", "azure.subscription_id")]).extract_into(&input, event)? {
                }
                }
                    }
                    Ok(())
                })();
            }
            if event.has("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has("event.outcome") {
                if let Some(s) = event.get_string("event.outcome") {
                    let lowered = s.to_lowercase();
                    event.set("event.outcome", lowered)?;
                }
            }
            // End nested pipeline: "azure-shared-pipeline"

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
