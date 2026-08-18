// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.0.0"))?;

        if event.has("azure") {
            event.rename("azure", "azure-eventhub")?;
        }

        let cond = { !event.has("event.original") };
        if cond {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        let cond = { event.has("event.original") };
        if cond {
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

        // ctx.azure.auditlogs.category != 'AuditLogs'
        let cond = event
            .get_as_string("azure.auditlogs.category")
            .is_none_or(|c| c != "AuditLogs");
        if cond {
            return Ok(TransformResult::Drop);
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("azure.auditlogs.time") {
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                    })
                {
                    event.set(
                        "@timestamp",
                        dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                    )?;
                }
            }
            Ok(())
        })();

        if event.has("azure.auditlogs.resourceId") {
            event.rename("azure.auditlogs.resourceId", "azure.resource_id")?;
        }

        if event.has("azure.auditlogs.durationMs") {
            event.rename("azure.auditlogs.durationMs", "event.duration")?;
        }

        // Painless script
        // Source: ctx.event.duration = ctx.event.duration * params.param_nano
        painless_exec(
            event,
            r#"ctx.event.duration = ctx.event.duration * params.param_nano"#,
        )?;

        // properties.result is a String whose lowercase form is 'success' or 'failure'
        let cond = event
            .get_str("azure.auditlogs.properties.result")
            .is_some_and(|r| {
                let r = r.to_lowercase();
                r == "success" || r == "failure"
            });
        if cond {
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

        if event.has("azure.auditlogs.properties.activityDisplayName") {
            event.rename(
                "azure.auditlogs.properties.activityDisplayName",
                "azure.auditlogs.properties.activity_display_name",
            )?;
        }

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

        let cond = { event.has("source.ip") };
        if cond {
            event.remove("azure.auditlogs.callerIpAddress");
        }

        let cond = { !event.has("azure.auditlogs.properties.initiatedBy.app.appId") };
        if cond {
            event.remove("azure.auditlogs.properties.initiatedBy.app.appId");
        }

        let cond =
            { !event.has("azure.auditlogs.properties.initiatedBy.app.servicePrincipalName") };
        if cond {
            event.remove("azure.auditlogs.properties.initiatedBy.app.servicePrincipalName");
        }

        let cond = { !event.has("azure.auditlogs.properties.userAgent") };
        if cond {
            event.remove("azure.auditlogs.properties.userAgent");
        }

        event.set(
            "client.ip",
            event.get("source.ip").cloned().unwrap_or(Value::Null),
        )?;

        let cond = { event.has("source.ip") };
        if cond {
            event.append(
                "related.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        if event.has("azure.auditlogs.properties.resultReason") {
            event.rename(
                "azure.auditlogs.properties.resultReason",
                "azure.auditlogs.properties.result_reason",
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Painless script
            // Source: if (ctx.azure.auditlogs.properties.targetResources != null) {\n  ctx.azure.auditlogs.properties.target_resources = new HashMap();\n  for (def i = 0; i < ctx.azure.auditlogs.properties.targetResources.length; i++) {\n    String index = String.valueOf(i);\n    ctx.azure.auditlogs.properties.target_resources[index] = new HashMap();\n    if(ctx.azure.auditlogs.properties.targetResources[i].displayName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].display_name = ctx.azure.auditlogs.properties.targetResources[i].displayName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].id = ctx.azure.auditlogs.properties.targetResources[i].id;\n    ctx.azure.auditlogs.properties.target_resources[index].type = ctx.azure.auditlogs.properties.targetResources[i].type;\n    if (ctx.azure.auditlogs.properties.targetResources[i].ipAddress != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].ip_address = ctx.azure.auditlogs.properties.targetResources[i].ipAddress;\n    }\n    if (ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].user_principal_name = ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].modified_properties = new HashMap();\n    for (def j = 0; j < ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties.length; j++) {\n      String n = String.valueOf(j);\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n] = new HashMap();\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].display_name = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].displayName;\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].new_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue;\n      }\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].old_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue;\n      }\n    }\n  }\n  ctx.azure.auditlogs.properties.remove('targetResources');\n}
            painless_exec(
                event,
                r#"if (ctx.azure.auditlogs.properties.targetResources != null) {\n  ctx.azure.auditlogs.properties.target_resources = new HashMap();\n  for (def i = 0; i < ctx.azure.auditlogs.properties.targetResources.length; i++) {\n    String index = String.valueOf(i);\n    ctx.azure.auditlogs.properties.target_resources[index] = new HashMap();\n    if(ctx.azure.auditlogs.properties.targetResources[i].displayName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].display_name = ctx.azure.auditlogs.properties.targetResources[i].displayName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].id = ctx.azure.auditlogs.properties.targetResources[i].id;\n    ctx.azure.auditlogs.properties.target_resources[index].type = ctx.azure.auditlogs.properties.targetResources[i].type;\n    if (ctx.azure.auditlogs.properties.targetResources[i].ipAddress != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].ip_address = ctx.azure.auditlogs.properties.targetResources[i].ipAddress;\n    }\n    if (ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].user_principal_name = ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].modified_properties = new HashMap();\n    for (def j = 0; j < ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties.length; j++) {\n      String n = String.valueOf(j);\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n] = new HashMap();\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].display_name = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].displayName;\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].new_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue;\n      }\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].old_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue;\n      }\n    }\n  }\n  ctx.azure.auditlogs.properties.remove('targetResources');\n}"#,
            )?;
            Ok(())
        })();

        if event.has("azure.auditlogs.properties.initiatedBy") {
            event.rename(
                "azure.auditlogs.properties.initiatedBy",
                "azure.auditlogs.properties.initiated_by",
            )?;
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

        // Begin nested pipeline: "azure-shared-pipeline"
        event.set("cloud.provider", json!("azure"))?;
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Pattern definitions for grok
            // PROVIDERNAME = .+
            // RULE = .+
            // GROUPID = .+
            // NAMESPACE = .+
            // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
            if let Some(input) = event.get_string("azure.resource_id") {
                // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let cached = cached_grok(
                    "/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}",
                );
                let grok_re = &cached.regex;
                let grok_field_map = &cached.field_map;
                if let Some(caps) = grok_re.captures(&input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            let field_path =
                                grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                            event.set(field_path, m.as_str())?;
                        }
                    }
                }
                // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/namespaces/%{NAMESPACE:azure.resource.namespace}/authorizationRules/%{RULE:azure.resource.authorization_rule}
            }
            Ok(())
        })();
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+/([A-Za-z])\w+.
                // GROUPID = .+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // NAME = ((?!AUTHORIZATIONRULES).)*$
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok(
                        "/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}",
                    );
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // NAME = .+
                // GROUPID = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok(
                        "/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}",
                    );
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDER = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /providers/%{PROVIDER:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok("/providers/%{PROVIDER:azure.resource.provider}");
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /PROVIDERS/%{PROVIDER:azure.resource.provider}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok(
                        "/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}",
                    );
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/providers/%{PROVIDERNAME:azure.resource.provider}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // GROUPID = .+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok(
                        "/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}",
                    );
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let cached = cached_grok("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}");
                    let grok_re = &cached.regex;
                    let grok_field_map = &cached.field_map;
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}
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

        let cond = {
            !event.has("tags")
                || !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_original_event")),
                    serde_json::Value::String(s) => s.contains("preserve_original_event"),
                    _ => false,
                }))
        };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        // --- Post-processing ---
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
