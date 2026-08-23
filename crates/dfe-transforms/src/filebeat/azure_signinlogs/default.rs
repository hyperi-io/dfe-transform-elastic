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

            parse_json_field(event, "message", "azure.signinlogs")?;

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\n\nctx.azure['signinlogs'] = keysToSnakeCase(ctx.azure.signinlogs);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\n\nctx.azure['signinlogs'] = keysToSnakeCase(ctx.azure.signinlogs);\n"#
                ),
            )?;

            let _cond = {
                !event.has_value("azure.signinlogs.category")
                    || !(event
                        .get_str("azure.signinlogs.category")
                        .is_some_and(|s| s.ends_with("SignInLogs")))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { !event.has_value("azure.signinlogs.time") };
            if _cond {
                if event.has("azure.signinlogs.created_date_time") {
                    event.rename(
                        "azure.signinlogs.created_date_time",
                        "azure.signinlogs.time",
                    )?;
                }
            }

            if let Some(date_str) = event.get_as_string("azure.signinlogs.time") {
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

            if event.remove("azure.signinlogs.time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "azure.signinlogs.time".into(),
                });
            }

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

            if event.has("azure.signinlogs.resource_id") {
                event.rename("azure.signinlogs.resource_id", "azure.resource_id")?;
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                let v = json!(
                    event
                        .get("azure.signinlogs.properties.ipaddress")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.address", v)?;
                }
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                let v = json!(
                    event
                        .get("azure.signinlogs.properties.ip_address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.address", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("azure.signinlogs.caller_ip_address") {
                    if let Some(val) = event.get("azure.signinlogs.caller_ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.signinlogs.caller_ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("azure.signinlogs.caller_ip_address", converted)?;
                    }
                }
                Ok(())
            })();

            event.remove("azure.signinlogs.properties.ipaddress");
            event.remove("azure.signinlogs.properties.ip_address");

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("source.ip")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.ip", v)?;
            }

            if event.has_value("azure.signinlogs.level") {
                if let Some(val) = event.get("azure.signinlogs.level") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.signinlogs.level".into(),
                            message,
                        }
                    })?;
                    event.set("log.level", converted)?;
                }
            }

            event.remove("azure.signinlogs.level");

            let _cond = {
                event.has_value("azure.signinlogs.duration_ms")
                    && event
                        .get("azure.signinlogs.duration_ms")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("azure.signinlogs.duration_ms") {
                        if let Some(val) = event.get("azure.signinlogs.duration_ms") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "azure.signinlogs.duration_ms".into(),
                                    message,
                                }
                            })?;
                            event.set("event.duration", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("azure.signinlogs.duration_ms")
                    && !(event
                        .get("azure.signinlogs.duration_ms")
                        .is_some_and(|v| v.is_string()))
            };
            if _cond {
                if event.has("azure.signinlogs.duration_ms") {
                    event.rename("azure.signinlogs.duration_ms", "event.duration")?;
                }
            }

            event.remove("azure.signinlogs.duration_ms");

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.event.duration * 1000000
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event.duration = ctx.event.duration * 1000000"#),
                )?;
            }

            if event.has("azure.signinlogs.location") {
                event.rename("azure.signinlogs.location", "geo.country_iso_code")?;
            }

            if event.has_value("azure.signinlogs.operation_name") {
                if let Some(val) = event.get("azure.signinlogs.operation_name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.signinlogs.operation_name".into(),
                            message,
                        }
                    })?;
                    event.set("event.action", converted)?;
                }
            }

            if event.has("azure.signinlogs.tenant_id") {
                event.rename("azure.signinlogs.tenant_id", "azure.tenant_id")?;
            }

            if event.has("azure.signinlogs.correlation_id") {
                event.rename("azure.signinlogs.correlation_id", "azure.correlation_id")?;
            }

            if event.has("azure.signinlogs.properties.created_date_time") {
                event.rename(
                    "azure.signinlogs.properties.created_date_time",
                    "azure.signinlogs.properties.created_at",
                )?;
            }

            if event.has("azure.signinlogs.properties.processing_time_in_milliseconds") {
                event.rename(
                    "azure.signinlogs.properties.processing_time_in_milliseconds",
                    "azure.signinlogs.properties.processing_time_ms",
                )?;
            }

            if event.has("azure.signinlogs.properties.risk_level_during_sign_in") {
                event.rename(
                    "azure.signinlogs.properties.risk_level_during_sign_in",
                    "azure.signinlogs.properties.risk_level_during_signin",
                )?;
            }

            // Painless script
            // Source: String reason = ctx?.azure?.signinlogs?.properties?.status?.failure_reason; String details = ctx?.azure?.signinlogs?.properties?.status?.additional_details; if (reason != null && details != null) { ctx['message'] = reason + ' (' + details + ')'; } else if (reason != null) { ctx['message'] = reason; } else if (details != null) { ctx['message'] = details; }
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String reason = ctx?.azure?.signinlogs?.properties?.status?.failure_reason; String details = ctx?.azure?.signinlogs?.properties?.status?.additional_details; if (reason != null && details != null) { ctx['message'] = reason + ' (' + details + ')'; } else if (reason != null) { ctx['message'] = reason; } else if (details != null) { ctx['message'] = details; }"#
                ),
            )?;

            event.remove("azure.signinlogs.properties.status.failure_reason");

            event.remove("azure.signinlogs.properties.status.additional_details");

            if event.has("azure.signinlogs.properties.location.city") {
                event.rename("azure.signinlogs.properties.location.city", "geo.city_name")?;
            }

            if event.has("azure.signinlogs.properties.location.state") {
                event.rename(
                    "azure.signinlogs.properties.location.state",
                    "geo.region_name",
                )?;
            }

            if event.has("azure.signinlogs.properties.location.geo_coordinates.latitude") {
                event.rename(
                    "azure.signinlogs.properties.location.geo_coordinates.latitude",
                    "geo.location.lat",
                )?;
            }

            if event.has("azure.signinlogs.properties.location.geo_coordinates.longitude") {
                event.rename(
                    "azure.signinlogs.properties.location.geo_coordinates.longitude",
                    "geo.location.lon",
                )?;
            }

            event.remove("azure.signinlogs.properties.location");

            let _cond = {
                event.has_value("azure.signinlogs.properties.authentication_processing_details")
                    && event
                        .get("azure.signinlogs.properties.authentication_processing_details")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def tmp = [:];\nfor (item in ctx.azure.signinlogs.properties.authentication_processing_details) {\n    tmp[item.key] = item.value;\n}\nctx.azure.signinlogs.properties.authentication_processing_details = tmp;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def tmp = [:];\nfor (item in ctx.azure.signinlogs.properties.authentication_processing_details) {\n    tmp[item.key] = item.value;\n}\nctx.azure.signinlogs.properties.authentication_processing_details = tmp;\n"#
                    ),
                )?;
            }

            event.set("event.kind", json!("event"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("authentication")]),
            )?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                !event.has_value("azure.signinlogs.properties.status.error_code")
                    || event.get_i64("azure.signinlogs.properties.status.error_code") == Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.status.error_code")
                    && event
                        .get_i64("azure.signinlogs.properties.status.error_code")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let v = json!(
                event
                    .get("azure.signinlogs.properties.id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("azure.signinlogs.properties.user_principal_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.name").cloned() {
                    event.set("user.email", v)?;
                }
            }

            if event.has_value("azure.signinlogs.properties.user_display_name") {
                if let Some(val) = event.get("azure.signinlogs.properties.user_display_name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.signinlogs.properties.user_display_name".into(),
                            message,
                        }
                    })?;
                    event.set("user.full_name", converted)?;
                }
            }

            let _cond = { !event.has_value("azure.signinlogs.properties.user_id") };
            if _cond {
                event.remove("azure.signinlogs.properties.user_id");
            }

            if event.has_value("azure.signinlogs.properties.user_id") {
                if let Some(val) = event.get("azure.signinlogs.properties.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.signinlogs.properties.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.full_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
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

            if event.has_value("source.ip") {
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

            let _cond = { !event.has_value("user_agent.original") };
            if _cond {
                if event.has("azure.signinlogs.properties.user_agent") {
                    event.rename(
                        "azure.signinlogs.properties.user_agent",
                        "user_agent.original",
                    )?;
                }
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                event.remove("azure.signinlogs.properties.user_agent");
            }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
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

            let _cond = {
                event.has_value("azure.signinlogs.properties.app_id")
                    && event.get_str("azure.signinlogs.properties.app_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.app_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.resource_id")
                    && event.get_str("azure.signinlogs.properties.resource_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.resource_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("azure.signinlogs.properties.service_principal_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.name", v)?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.service_principal_id")
                    && event.get_str("azure.signinlogs.properties.service_principal_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.service_principal_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.service_principal_credential_key_id")
                    && event
                        .get_str("azure.signinlogs.properties.service_principal_credential_key_id")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.service_principal_credential_key_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.user_id")
                    && event.get_str("azure.signinlogs.properties.user_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("azure.signinlogs.properties.device_detail.device_id")
                    && event.get_str("azure.signinlogs.properties.device_detail.device_id")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.entity",
                    json!(
                        event
                            .get("azure.signinlogs.properties.device_detail.device_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                    // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))",
                                [
                                    ("azure_subscription_id", "azure.subscription_id"),
                                    ("azure_resource_group", "azure.resource.group"),
                                    ("azure_resource_provider", "azure.resource.provider"),
                                    ("azure_resource_namespace", "azure.resource.namespace"),
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
                                    ("azure_resource_namespace", "azure.resource.namespace"),
                                    (
                                        "azure_resource_authorization_rule",
                                        "azure.resource.authorization_rule"
                                    )
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )?;
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
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )?;
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
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_name", "azure.resource.name")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )?;
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
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/providers/(?P<azure_resource_provider>(?:.+))",
                                    [("azure_resource_provider", "azure.resource.provider")]
                                ),
                                cached_grok_mapped!(
                                    "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                    [("azure_resource_provider", "azure.resource.provider")]
                                ),
                            ],
                            &input,
                            event,
                        )?;
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
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_provider", "azure.resource.provider")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_provider", "azure.resource.provider")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )?;
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
                        let _ = extract_first_match(
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
                        )?;
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
                        let _ = extract_first_match(
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
                        )?;
                    }
                    Ok(())
                })();
            }
            if event.has("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
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
