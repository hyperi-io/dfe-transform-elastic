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
            event.set("ecs.version", json!("8.0.0"))?;

            if event.has("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.message = ctx.message.replace(params.empty_field_name, '')
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"ctx.message = ctx.message.replace(params.empty_field_name, '')"#
                    ),
                    cached_params!("{\"empty_field_name\":\"\\\"\\\":\\\"\\\",\"}"),
                )?;
                Ok(())
            })();

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
                event.set("azure.activitylogs", parsed)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.activitylogs.time") {
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

            event.remove("azure.activitylogs.time");

            if event.has("azure.activitylogs.resourceId") {
                event.rename("azure.activitylogs.resourceId", "azure.resource_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("azure.activitylogs.callerIpAddress") {
                    if let Some(s) = event.get_string("azure.activitylogs.callerIpAddress") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "azure.activitylogs.callerIpAddress".into(),
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
                    if event.has("azure.activitylogs.callerIpAddress") {
                        event.rename("azure.activitylogs.callerIpAddress", "source.address")?;
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.remove("azure.activitylogs.callerIpAddress");
            }

            event.set(
                "client.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("source.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            if event.has("azure.activitylogs.level") {
                event.rename("azure.activitylogs.level", "log.level")?;
            }

            if event.has("azure.activitylogs.durationMs") {
                event.rename("azure.activitylogs.durationMs", "event.duration")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}"#
                    ),
                    cached_params!("{\"param_nano\":1000000}"),
                )?;
                Ok(())
            })();

            if event.has("azure.activitylogs.location") {
                event.rename("azure.activitylogs.location", "geo.name")?;
            }

            let _cond = {
                event
                    .get("azure.activitylogs.identity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has("azure.activitylogs.identity") {
                    event.rename(
                        "azure.activitylogs.identity",
                        "azure.activitylogs.identity_name",
                    )?;
                }
            }

            let _cond = {
                event
                    .get("azure.activitylogs.identity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("azure.activitylogs.identity") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "azure.activitylogs.identity".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("azure.activitylogs.identity", parsed)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("azure.activitylogs.properties")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("azure.activitylogs.properties") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "azure.activitylogs.properties".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("azure.activitylogs.properties", parsed)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx?.azure?.activitylogs?.properties?.eventCategory != null) {\n  ctx.azure.activitylogs.event_category = ctx.azure.activitylogs.properties.eventCategory;\n} else if (ctx?.azure?.activitylogs?.properties?.policies != null)  {\n  ctx.azure.activitylogs.event_category = 'Policy';\n} else {\n  ctx.azure.activitylogs.event_category = 'Administrative';\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx?.azure?.activitylogs?.properties?.eventCategory != null) {\n  ctx.azure.activitylogs.event_category = ctx.azure.activitylogs.properties.eventCategory;\n} else if (ctx?.azure?.activitylogs?.properties?.policies != null)  {\n  ctx.azure.activitylogs.event_category = 'Policy';\n} else {\n  ctx.azure.activitylogs.event_category = 'Administrative';\n}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("azure.activitylogs.event_category") };
            if _cond {
                event.remove("azure.activitylogs.properties.eventCategory");
            }

            if event.has("azure.activitylogs.resultType") {
                event.rename(
                    "azure.activitylogs.resultType",
                    "azure.activitylogs.result_type",
                )?;
            }

            let _cond = {
                event.has_value("azure.activitylogs.result_type")
                    && event
                        .get("azure.activitylogs.result_type")
                        .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("azure.activitylogs.result_type")
                        .is_some_and(|s| s.to_lowercase() == "success")
                        || event
                            .get_str("azure.activitylogs.result_type")
                            .is_some_and(|s| s.to_lowercase() == "failure"))
            };
            if _cond {
                if let Some(val) = event.get("azure.activitylogs.result_type") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.outcome", converted)?;
                }
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("azure.activitylogs.properties.result")
                    && event
                        .get("azure.activitylogs.properties.result")
                        .is_some_and(|v| v.is_string())
                    && ["success", "failure", "unknown"].contains(
                        &event
                            .get_str("azure.activitylogs.properties.result")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                if let Some(val) = event.get("azure.activitylogs.properties.result") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.outcome", converted)?;
                }
            }

            if event.has("azure.activitylogs.operationName") {
                event.rename(
                    "azure.activitylogs.operationName",
                    "azure.activitylogs.operation_name",
                )?;
            }

            if event.has("azure.activitylogs.operation_name") {
                if let Some(val) = event.get("azure.activitylogs.operation_name") {
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

            if event.has("azure.activitylogs.operationVersion") {
                event.rename(
                    "azure.activitylogs.operationVersion",
                    "azure.activitylogs.operation_version",
                )?;
            }

            if event.has("azure.activitylogs.tenantId") {
                event.rename(
                    "azure.activitylogs.tenantId",
                    "azure.activitylogs.tenant_id",
                )?;
            }

            if event.has("azure.activitylogs.Level") {
                event.rename("azure.activitylogs.Level", "azure.activitylogs.level")?;
            }

            if event.has("azure.activitylogs.resultSignature") {
                event.rename(
                    "azure.activitylogs.resultSignature",
                    "azure.activitylogs.result_signature",
                )?;
            }

            if event.has("azure.activitylogs.identity.authorization.evidence.roleAssignmentScope") {
                event.rename(
                    "azure.activitylogs.identity.authorization.evidence.roleAssignmentScope",
                    "azure.activitylogs.identity.authorization.evidence.role_assignment_scope",
                )?;
            }

            if event.has("azure.activitylogs.identity.authorization.evidence.roleDefinitionId") {
                event.rename(
                    "azure.activitylogs.identity.authorization.evidence.roleDefinitionId",
                    "azure.activitylogs.identity.authorization.evidence.role_definition_id",
                )?;
            }

            if event.has("azure.activitylogs.identity.authorization.evidence.roleAssignmentId") {
                event.rename(
                    "azure.activitylogs.identity.authorization.evidence.roleAssignmentId",
                    "azure.activitylogs.identity.authorization.evidence.role_assignment_id",
                )?;
            }

            if event.has("azure.activitylogs.identity.authorization.evidence.principalId") {
                event.rename(
                    "azure.activitylogs.identity.authorization.evidence.principalId",
                    "azure.activitylogs.identity.authorization.evidence.principal_id",
                )?;
            }

            if event.has("azure.activitylogs.identity.authorization.evidence.principalType") {
                event.rename(
                    "azure.activitylogs.identity.authorization.evidence.principalType",
                    "azure.activitylogs.identity.authorization.evidence.principal_type",
                )?;
            }

            if event.has("azure.activitylogs.correlationId") {
                event.rename("azure.activitylogs.correlationId", "azure.correlation_id")?;
            }

            if event.has("azure.activitylogs.properties.serviceRequestId") {
                event.rename(
                    "azure.activitylogs.properties.serviceRequestId",
                    "azure.activitylogs.properties.service_request_id",
                )?;
            }

            if event.has("azure.activitylogs.properties.statusMessage") {
                event.rename("azure.activitylogs.properties.statusMessage", "message")?;
            }

            if event.has("azure.activitylogs.properties.statusCode") {
                event.rename(
                    "azure.activitylogs.properties.statusCode",
                    "azure.activitylogs.properties.status_code",
                )?;
            }

            if event.has("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has("azure.activitylogs.identity.claims.name") {
                event.rename(
                    "azure.activitylogs.identity.claims.name",
                    "azure.activitylogs.identity.claims_initiated_by_user.fullname",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.surname = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.surname = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname'];\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.name = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.name = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name'];\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.givenname = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname'] != null) {\n  ctx.azure.activitylogs.identity.claims_initiated_by_user.givenname = ctx.azure.activitylogs.identity.claims['http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname'];\n}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("azure.activitylogs.identity")
                    && event.has_value("azure.activitylogs.identity.claims_initiated_by_user")
                    && event.has_value("azure.activitylogs.identity.claims_initiated_by_user.name")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "azure.activitylogs.identity.claims_initiated_by_user.schema",
                        json!("http://schemas.xmlsoap.org/ws/2005/05/identity/claims"),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.azure.activitylogs.identity.claims != null) {\n  ctx.temp_claims = new HashMap();\n  for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {\n    ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);\n  }\n  ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx.azure.activitylogs.identity.claims != null) {\n  ctx.temp_claims = new HashMap();\n  for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {\n    ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);\n  }\n  ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');\n}"#
                    ),
                )?;
                Ok(())
            })();

            // Painless script
            // Source: if (ctx?.azure?.activitylogs?.category == null) {\n  return;\n} def category = ctx.azure.activitylogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"if (ctx?.azure?.activitylogs?.category == null) {\n  return;\n} def category = ctx.azure.activitylogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"action\":{\"type\":[\"change\"]},\"delete\":{\"type\":[\"deletion\"]},\"read\":{\"type\":[\"access\"]},\"write\":{\"type\":[\"change\"]}}"
                ),
            )?;

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("azure.activitylogs.identity.claims_initiated_by_user.name") {
                    if let Some(input) = event
                        .get_string("azure.activitylogs.identity.claims_initiated_by_user.name")
                    {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                            .extract_into(&input, event)?
                        {}
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.set(
                    "user.email",
                    event
                        .get("azure.activitylogs.identity.claims_initiated_by_user.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                event.set(
                    "user.name",
                    event
                        .get("azure.activitylogs.identity.claims_initiated_by_user.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.name").cloned().unwrap_or(Value::Null),
                )?;
            }

            if event.has("azure.activitylogs.identity.claims_initiated_by_user.fullname") {
                if let Some(val) =
                    event.get("azure.activitylogs.identity.claims_initiated_by_user.fullname")
                {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("user.full_name", converted)?;
                }
            }

            event.set("event.kind", json!("event"))?;

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

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
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
                event.set("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
