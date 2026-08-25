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

            event.remove("routing.category");

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

            parse_json_field(event, "event.original", "azure.identityprotection")?;

            let _cond = {
                !event.has_value("azure.identityprotection.category")
                    || event.get_str("azure.identityprotection.category") != Some("RiskyUsers")
                        && event.get_str("azure.identityprotection.category")
                            != Some("UserRiskEvents")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.identityprotection.time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "ISO8601",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.identityprotection.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.identityprotection.time");

            if event.has_value("azure.identityprotection.resourceId") {
                event.rename("azure.identityprotection.resourceId", "azure.resource_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.identityprotection.callerIpAddress") {
                    if let Some(val) = event.get("azure.identityprotection.callerIpAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.identityprotection.callerIpAddress".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("azure.identityprotection.callerIpAddress") {
                        event
                            .rename("azure.identityprotection.callerIpAddress", "source.address")?;
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
                event.remove("azure.identityprotection.callerIpAddress");
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("azure.identityprotection.durationMs") {
                event.rename("azure.identityprotection.durationMs", "event.duration")?;
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

            if event.has_value("azure.identityprotection.operationName") {
                event.rename(
                    "azure.identityprotection.operationName",
                    "azure.identityprotection.operation_name",
                )?;
            }

            if event.has_value("azure.identityprotection.operation_name") {
                if let Some(val) = event.get("azure.identityprotection.operation_name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.identityprotection.operation_name".into(),
                            message,
                        }
                    })?;
                    event.set("event.action", converted)?;
                }
            }

            if event.has_value("azure.identityprotection.operationVersion") {
                event.rename(
                    "azure.identityprotection.operationVersion",
                    "azure.identityprotection.operation_version",
                )?;
            }

            if event.has_value("azure.identityprotection.tenantId") {
                event.rename("azure.identityprotection.tenantId", "azure.tenant_id")?;
            }

            event.remove("azure.identityprotection.identity");

            event.remove("azure.identityprotection.Level");

            event.remove("azure.identityprotection.location");

            if event.has_value("azure.identityprotection.resultSignature") {
                event.rename(
                    "azure.identityprotection.resultSignature",
                    "azure.identityprotection.result_signature",
                )?;
            }

            if event.has_value("azure.identityprotection.correlationId") {
                event.rename(
                    "azure.identityprotection.correlationId",
                    "azure.correlation_id",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.isDeleted") {
                event.rename(
                    "azure.identityprotection.properties.isDeleted",
                    "azure.identityprotection.properties.is_deleted",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.isGuest") {
                event.rename(
                    "azure.identityprotection.properties.isGuest",
                    "azure.identityprotection.properties.is_guest",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.isProcessing") {
                event.rename(
                    "azure.identityprotection.properties.isProcessing",
                    "azure.identityprotection.properties.is_processing",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.riskType") {
                event.rename(
                    "azure.identityprotection.properties.riskType",
                    "azure.identityprotection.properties.risk_type",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.riskEventType") {
                event.rename(
                    "azure.identityprotection.properties.riskEventType",
                    "azure.identityprotection.properties.risk_event_type",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.riskState") {
                event.rename(
                    "azure.identityprotection.properties.riskState",
                    "azure.identityprotection.properties.risk_state",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.riskLevel") {
                event.rename(
                    "azure.identityprotection.properties.riskLevel",
                    "azure.identityprotection.properties.risk_level",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.riskDetail") {
                event.rename(
                    "azure.identityprotection.properties.riskDetail",
                    "azure.identityprotection.properties.risk_detail",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("azure.identityprotection.properties.lastUpdatedDateTime")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "azure.identityprotection.properties.last_updated_datetime",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.identityprotection.properties.lastUpdatedDateTime"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.identityprotection.properties.lastUpdatedDateTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event
                    .get_as_string("azure.identityprotection.properties.riskLastUpdatedDateTime")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "azure.identityprotection.properties.risk_last_updated_datetime",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.identityprotection.properties.riskLastUpdatedDateTime"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.identityprotection.properties.riskLastUpdatedDateTime");

            if event.has_value("azure.identityprotection.properties.userId") {
                event.rename(
                    "azure.identityprotection.properties.userId",
                    "azure.identityprotection.properties.user_id",
                )?;
            }

            let _cond = { !event.has_value("azure.identityprotection.properties.user_id") };
            if _cond {
                event.remove("azure.identityprotection.properties.user_id");
            }

            if let Some(v) = event
                .get("azure.identityprotection.properties.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("azure.identityprotection.properties.userDisplayName") {
                event.rename(
                    "azure.identityprotection.properties.userDisplayName",
                    "azure.identityprotection.properties.user_display_name",
                )?;
            }

            if let Some(v) = event
                .get("azure.identityprotection.properties.user_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if event.has_value("azure.identityprotection.properties.userPrincipalName") {
                event.rename(
                    "azure.identityprotection.properties.userPrincipalName",
                    "azure.identityprotection.properties.user_principal_name",
                )?;
            }

            if let Some(v) = event
                .get("azure.identityprotection.properties.user_principal_name")
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

            if event.has_value("azure.identityprotection.properties.userType") {
                event.rename(
                    "azure.identityprotection.properties.userType",
                    "azure.identityprotection.properties.user_type",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("azure.identityprotection.properties.activityDateTime")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "azure.identityprotection.properties.activity_datetime",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.identityprotection.properties.activityDateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.identityprotection.properties.activityDateTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("azure.identityprotection.properties.detectedDateTime")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set(
                            "azure.identityprotection.properties.detected_datetime",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.identityprotection.properties.detectedDateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.identityprotection.properties.detectedDateTime");

            if event.has_value("azure.identityprotection.properties.detectionTimingType") {
                event.rename(
                    "azure.identityprotection.properties.detectionTimingType",
                    "azure.identityprotection.properties.detection_timing_type",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.requestId") {
                event.rename(
                    "azure.identityprotection.properties.requestId",
                    "azure.identityprotection.properties.request_id",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.correlationId") {
                event.rename(
                    "azure.identityprotection.properties.correlationId",
                    "azure.identityprotection.properties.correlation_id",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.tokenIssuerType") {
                event.rename(
                    "azure.identityprotection.properties.tokenIssuerType",
                    "azure.identityprotection.properties.token_issuer_type",
                )?;
            }

            if event.has_value("azure.identityprotection.properties.ipAddress") {
                event.rename(
                    "azure.identityprotection.properties.ipAddress",
                    "azure.identityprotection.properties.ip_address",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "azure.identityprotection.properties.additionalInfo",
                    "azure.identityprotection.properties.additional_info",
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("azure.identityprotection.properties.additional_info") };
            if _cond {
                event.remove("azure.identityprotection.properties.additionalInfo");
            }

            event.remove("azure.identityprotection.properties.resourceTenantId");

            event.remove("azure.identityprotection.properties.homeTenantId");

            event.remove("azure.identityprotection.properties.crossTenantAccessType");

            event.set("event.kind", json!("event"))?;

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /TENANTS/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:.+))
                    // Grok pattern: /tenants/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:.+))
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "/TENANTS/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                [
                                    ("azure_tenant_id", "azure.tenant_id"),
                                    ("azure_resource_provider", "azure.resource.provider")
                                ]
                            ),
                            cached_grok_mapped!(
                                "/tenants/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:.+))",
                                [
                                    ("azure_tenant_id", "azure.tenant_id"),
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
            if event.has_value("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }
            // End nested pipeline: "azure-shared-pipeline"

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
