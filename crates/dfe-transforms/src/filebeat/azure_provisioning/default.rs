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

            event.remove("routing.category");

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

            parse_json_field(event, "event.original", "azure.provisioning")?;

            let _cond = {
                !event.has_value("azure.provisioning.category")
                    || event.get_str("azure.provisioning.category") != Some("ProvisioningLogs")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.provisioning.time") {
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
                                path: "azure.provisioning.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.provisioning.time");

            if event.has_value("azure.provisioning.resourceId") {
                event.rename("azure.provisioning.resourceId", "azure.resource_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("azure.provisioning.callerIpAddress") {
                    if let Some(val) = event.get("azure.provisioning.callerIpAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "azure.provisioning.callerIpAddress".into(),
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
                    if event.has_value("azure.provisioning.callerIpAddress") {
                        event.rename("azure.provisioning.callerIpAddress", "source.address")?;
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
                event.remove("azure.provisioning.callerIpAddress");
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

            if event.has_value("azure.provisioning.durationMs") {
                event.rename("azure.provisioning.durationMs", "event.duration")?;
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

            if event.has_value("azure.provisioning.resultType") {
                event.rename(
                    "azure.provisioning.resultType",
                    "azure.provisioning.result_type",
                )?;
            }

            let _cond = {
                event.has_value("azure.provisioning.result_type")
                    && event
                        .get("azure.provisioning.result_type")
                        .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("azure.provisioning.result_type")
                        .is_some_and(|s| s.to_lowercase() == "success")
                        || event
                            .get_str("azure.provisioning.result_type")
                            .is_some_and(|s| s.to_lowercase() == "failure"))
            };
            if _cond {
                if let Some(val) = event.get("azure.provisioning.result_type") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.provisioning.result_type".into(),
                            message,
                        }
                    })?;
                    event.set("event.outcome", converted)?;
                }
            }

            if event.has_value("azure.provisioning.operationName") {
                event.rename(
                    "azure.provisioning.operationName",
                    "azure.provisioning.operation_name",
                )?;
            }

            if event.has_value("azure.provisioning.operation_name") {
                if let Some(val) = event.get("azure.provisioning.operation_name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.provisioning.operation_name".into(),
                            message,
                        }
                    })?;
                    event.set("event.action", converted)?;
                }
            }

            if event.has_value("azure.provisioning.operationVersion") {
                event.rename(
                    "azure.provisioning.operationVersion",
                    "azure.provisioning.operation_version",
                )?;
            }

            if event.has_value("azure.provisioning.tenantId") {
                event.rename("azure.provisioning.tenantId", "azure.tenant_id")?;
            }

            if event.has_value("azure.provisioning.Level") {
                event.rename("azure.provisioning.Level", "azure.provisioning.level")?;
            }

            if event.has_value("azure.provisioning.resultSignature") {
                event.rename(
                    "azure.provisioning.resultSignature",
                    "azure.provisioning.result_signature",
                )?;
            }

            if event.has_value("azure.provisioning.correlationId") {
                event.rename("azure.provisioning.correlationId", "azure.correlation_id")?;
            }

            if event.has_value("azure.provisioning.properties.tenantId") {
                event.rename(
                    "azure.provisioning.properties.tenantId",
                    "azure.provisioning.properties.tenant_id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.activityDateTime") {
                event.rename(
                    "azure.provisioning.properties.activityDateTime",
                    "azure.provisioning.properties.activity_datetime",
                )?;
            }

            if event.has_value("azure.provisioning.properties.changeId") {
                event.rename(
                    "azure.provisioning.properties.changeId",
                    "azure.provisioning.properties.change_id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.cycleId") {
                event.rename(
                    "azure.provisioning.properties.cycleId",
                    "azure.provisioning.properties.cycle_id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.durationInMilliseconds") {
                event.rename(
                    "azure.provisioning.properties.durationInMilliseconds",
                    "azure.provisioning.properties.duration_ms",
                )?;
            }

            if event.has_value("azure.provisioning.properties.initiatedBy") {
                event.rename(
                    "azure.provisioning.properties.initiatedBy",
                    "azure.provisioning.properties.initiated_by",
                )?;
            }

            if event.has_value("azure.provisioning.properties.initiated_by.Id") {
                event.rename(
                    "azure.provisioning.properties.initiated_by.Id",
                    "azure.provisioning.properties.initiated_by.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.initiated_by.Name") {
                event.rename(
                    "azure.provisioning.properties.initiated_by.Name",
                    "azure.provisioning.properties.initiated_by.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.initiated_by.Type") {
                event.rename(
                    "azure.provisioning.properties.initiated_by.Type",
                    "azure.provisioning.properties.initiated_by.type",
                )?;
            }

            if event.has_value("azure.provisioning.properties.jobId") {
                event.rename(
                    "azure.provisioning.properties.jobId",
                    "azure.provisioning.properties.job_id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.modifiedProperties") {
                event.rename(
                    "azure.provisioning.properties.modifiedProperties",
                    "azure.provisioning.properties.modified_properties",
                )?;
            }

            if event.has_value("azure.provisioning.properties.modified_properties.displayName") {
                event.rename(
                    "azure.provisioning.properties.modified_properties.displayName",
                    "azure.provisioning.properties.modified_properties.display_name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.modified_properties.newValue") {
                event.rename(
                    "azure.provisioning.properties.modified_properties.newValue",
                    "azure.provisioning.properties.modified_properties.new_value",
                )?;
            }

            if event.has_value("azure.provisioning.properties.modified_properties.oldValue") {
                event.rename(
                    "azure.provisioning.properties.modified_properties.oldValue",
                    "azure.provisioning.properties.modified_properties.old_value",
                )?;
            }

            if event.has_value("azure.provisioning.properties.provisioningAction") {
                event.rename(
                    "azure.provisioning.properties.provisioningAction",
                    "azure.provisioning.properties.provisioning_action",
                )?;
            }

            if event.has_value("azure.provisioning.properties.provisioningStatusInfo") {
                event.rename(
                    "azure.provisioning.properties.provisioningStatusInfo",
                    "azure.provisioning.properties.provisioning_status_info",
                )?;
            }

            if event.has_value("azure.provisioning.properties.provisioning_status_info.Status") {
                event.rename(
                    "azure.provisioning.properties.provisioning_status_info.Status",
                    "azure.provisioning.properties.provisioning_status_info.status",
                )?;
            }

            let _cond = {
                !event.has_value(
                    "azure.provisioning.properties.provisioning_status_info.errorInformation",
                )
            };
            if _cond {
                event.remove(
                    "azure.provisioning.properties.provisioning_status_info.errorInformation",
                );
            }

            if event.has_value(
                "azure.provisioning.properties.provisioning_status_info.errorInformation",
            ) {
                event.rename(
                    "azure.provisioning.properties.provisioning_status_info.errorInformation",
                    "azure.provisioning.properties.provisioning_status_info.error_information",
                )?;
            }

            if event.has_value("azure.provisioning.properties.provisioning_status_info.error_information.additionalDetails") {
                    event.rename("azure.provisioning.properties.provisioning_status_info.error_information.additionalDetails", "azure.provisioning.properties.provisioning_status_info.error_information.additional_details")?;
                }

            if event.has_value("azure.provisioning.properties.provisioning_status_info.error_information.errorCategory") {
                    event.rename("azure.provisioning.properties.provisioning_status_info.error_information.errorCategory", "azure.provisioning.properties.provisioning_status_info.error_information.error_category")?;
                }

            if event.has_value("azure.provisioning.properties.provisioning_status_info.error_information.errorCode") {
                    event.rename("azure.provisioning.properties.provisioning_status_info.error_information.errorCode", "azure.provisioning.properties.provisioning_status_info.error_information.error_code")?;
                }

            if event.has_value("azure.provisioning.properties.provisioning_status_info.error_information.recommendedAction") {
                    event.rename("azure.provisioning.properties.provisioning_status_info.error_information.recommendedAction", "azure.provisioning.properties.provisioning_status_info.error_information.recommended_action")?;
                }

            if event.has_value("azure.provisioning.properties.servicePrincipal") {
                event.rename(
                    "azure.provisioning.properties.servicePrincipal",
                    "azure.provisioning.properties.service_principal",
                )?;
            }

            if event.has_value("azure.provisioning.properties.service_principal.Id") {
                event.rename(
                    "azure.provisioning.properties.service_principal.Id",
                    "azure.provisioning.properties.service_principal.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.service_principal.Name") {
                event.rename(
                    "azure.provisioning.properties.service_principal.Name",
                    "azure.provisioning.properties.service_principal.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.sourceIdentity") {
                event.rename(
                    "azure.provisioning.properties.sourceIdentity",
                    "azure.provisioning.properties.source_identity",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_identity.Id") {
                event.rename(
                    "azure.provisioning.properties.source_identity.Id",
                    "azure.provisioning.properties.source_identity.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_identity.Name") {
                event.rename(
                    "azure.provisioning.properties.source_identity.Name",
                    "azure.provisioning.properties.source_identity.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_identity.identityType") {
                event.rename(
                    "azure.provisioning.properties.source_identity.identityType",
                    "azure.provisioning.properties.source_identity.identity_type",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_identity.details.DisplayName")
            {
                event.rename(
                    "azure.provisioning.properties.source_identity.details.DisplayName",
                    "azure.provisioning.properties.source_identity.details.display_name",
                )?;
            }

            if event.has_value(
                "azure.provisioning.properties.source_identity.details.UserPrincipalName",
            ) {
                event.rename(
                    "azure.provisioning.properties.source_identity.details.UserPrincipalName",
                    "azure.provisioning.properties.source_identity.details.user_principal_name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.targetIdentity") {
                event.rename(
                    "azure.provisioning.properties.targetIdentity",
                    "azure.provisioning.properties.target_identity",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_identity.Id") {
                event.rename(
                    "azure.provisioning.properties.target_identity.Id",
                    "azure.provisioning.properties.target_identity.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_identity.Name") {
                event.rename(
                    "azure.provisioning.properties.target_identity.Name",
                    "azure.provisioning.properties.target_identity.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_identity.identityType") {
                event.rename(
                    "azure.provisioning.properties.target_identity.identityType",
                    "azure.provisioning.properties.target_identity.identity_type",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_identity.details.DisplayName")
            {
                event.rename(
                    "azure.provisioning.properties.target_identity.details.DisplayName",
                    "azure.provisioning.properties.target_identity.details.display_name",
                )?;
            }

            if event.has_value(
                "azure.provisioning.properties.target_identity.details.UserPrincipalName",
            ) {
                event.rename(
                    "azure.provisioning.properties.target_identity.details.UserPrincipalName",
                    "azure.provisioning.properties.target_identity.details.user_principal_name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.sourceSystem") {
                event.rename(
                    "azure.provisioning.properties.sourceSystem",
                    "azure.provisioning.properties.source_system",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_system.Id") {
                event.rename(
                    "azure.provisioning.properties.source_system.Id",
                    "azure.provisioning.properties.source_system.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_system.Name") {
                event.rename(
                    "azure.provisioning.properties.source_system.Name",
                    "azure.provisioning.properties.source_system.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.source_system.details.ApplicationId")
            {
                event.rename(
                    "azure.provisioning.properties.source_system.details.ApplicationId",
                    "azure.provisioning.properties.source_system.details.application_id",
                )?;
            }

            if event.has_value(
                "azure.provisioning.properties.source_system.details.ServicePrincipalDisplayName",
            ) {
                event.rename("azure.provisioning.properties.source_system.details.ServicePrincipalDisplayName", "azure.provisioning.properties.source_system.details.dervice_principal_display_name")?;
            }

            if event
                .has_value("azure.provisioning.properties.source_system.details.ServicePrincipalId")
            {
                event.rename(
                    "azure.provisioning.properties.source_system.details.ServicePrincipalId",
                    "azure.provisioning.properties.source_system.details.service_principal_id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.targetSystem") {
                event.rename(
                    "azure.provisioning.properties.targetSystem",
                    "azure.provisioning.properties.target_system",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_system.Id") {
                event.rename(
                    "azure.provisioning.properties.target_system.Id",
                    "azure.provisioning.properties.target_system.id",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_system.Name") {
                event.rename(
                    "azure.provisioning.properties.target_system.Name",
                    "azure.provisioning.properties.target_system.name",
                )?;
            }

            if event.has_value("azure.provisioning.properties.target_system.details.ApplicationId")
            {
                event.rename(
                    "azure.provisioning.properties.target_system.details.ApplicationId",
                    "azure.provisioning.properties.target_system.details.application_id",
                )?;
            }

            if event.has_value(
                "azure.provisioning.properties.target_system.details.ServicePrincipalDisplayName",
            ) {
                event.rename("azure.provisioning.properties.target_system.details.ServicePrincipalDisplayName", "azure.provisioning.properties.target_system.details.dervice_principal_display_name")?;
            }

            if event
                .has_value("azure.provisioning.properties.target_system.details.ServicePrincipalId")
            {
                event.rename(
                    "azure.provisioning.properties.target_system.details.ServicePrincipalId",
                    "azure.provisioning.properties.target_system.details.service_principal_id",
                )?;
            }

            event.remove("azure.provisioning.properties.statusInfo");

            if event.has_value("azure.provisioning.properties.provisioningSteps") {
                event.rename(
                    "azure.provisioning.properties.provisioningSteps",
                    "azure.provisioning.properties.provisioning_steps",
                )?;
            }

            if event.has_value("azure.provisioning.properties.provisioning_steps") {
                foreach_array(
                    event,
                    "azure.provisioning.properties.provisioning_steps",
                    |event| {
                        if event.has_value("_ingest._value.provisioningStepType") {
                            event.rename(
                                "_ingest._value.provisioningStepType",
                                "_ingest._value.provisioning_step_type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("azure.provisioning.properties.activityDateTime") {
                event.rename(
                    "azure.provisioning.properties.activityDateTime",
                    "azure.provisioning.properties.activity_datetime",
                )?;
            }

            event.set("event.kind", json!("event"))?;

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /TENANTS/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:.+))
                    // Grok pattern: /tenants/(?P<azure_tenant_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:.+))
                    if !extract_first_match(
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
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
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
