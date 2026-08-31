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

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("connection")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("cloud.provider", json!("azure"))?;

            let _cond = {
                event.has_value("event.original")
                    && event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("records"))
                        }
                        serde_json::Value::String(s) => s.contains("records"),
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            parse_json_field(event, "event.original", "azure.frontdoor.waf")?;

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            if event.has_value("azure.frontdoor.waf.resourceId") {
                event.rename(
                    "azure.frontdoor.waf.resourceId",
                    "azure.frontdoor.resource_id",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.operationName") {
                event.rename(
                    "azure.frontdoor.waf.operationName",
                    "azure.frontdoor.operation_name",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.properties.trackingReference") {
                event.rename(
                    "azure.frontdoor.waf.properties.trackingReference",
                    "azure.frontdoor.tracking_reference",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.category") {
                event.rename("azure.frontdoor.waf.category", "azure.frontdoor.category")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.clientIP") {
                event.rename("azure.frontdoor.waf.properties.clientIP", "client.ip")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.clientPort") {
                event.rename("azure.frontdoor.waf.properties.clientPort", "client.port")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.action") {
                event.rename("azure.frontdoor.waf.properties.action", "event.action")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.socketIP") {
                event.rename("azure.frontdoor.waf.properties.socketIP", "client.address")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.ruleName") {
                event.rename("azure.frontdoor.waf.properties.ruleName", "rule.name")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.details") {
                event.rename(
                    "azure.frontdoor.waf.properties.details",
                    "azure.frontdoor.waf.details",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.properties.host") {
                event.rename("azure.frontdoor.waf.properties.host", "url.domain")?;
            }

            if event.has_value("azure.frontdoor.waf.properties.policy") {
                event.rename(
                    "azure.frontdoor.waf.properties.policy",
                    "azure.frontdoor.waf.policy",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.properties.policyMode") {
                event.rename(
                    "azure.frontdoor.waf.properties.policyMode",
                    "azure.frontdoor.waf.policy_mode",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.properties.requestUri") {
                event.rename("azure.frontdoor.waf.properties.requestUri", "url.original")?;
            }

            if event.has_value("url.original") {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            if let Some(v) = event
                .get("url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            let _cond = {
                event
                    .get("azure.frontdoor.waf.identity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("azure.frontdoor.waf.identity") {
                    event.rename(
                        "azure.frontdoor.waf.identity",
                        "azure.frontdoor.waf.identity_name",
                    )?;
                }
            }

            let _cond = {
                event
                    .get("azure.frontdoor.waf.identity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "azure.frontdoor.waf.identity",
                        "azure.frontdoor.waf.identity",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_identity")?;
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

            if event.has_value(
                "azure.frontdoor.waf.identity.authorization.evidence.roleAssignmentScope",
            ) {
                event.rename(
                    "azure.frontdoor.waf.identity.authorization.evidence.roleAssignmentScope",
                    "azure.frontdoor.waf.identity.authorization.evidence.role_assignment_scope",
                )?;
            }

            if event
                .has_value("azure.frontdoor.waf.identity.authorization.evidence.roleDefinitionId")
            {
                event.rename(
                    "azure.frontdoor.waf.identity.authorization.evidence.roleDefinitionId",
                    "azure.frontdoor.waf.identity.authorization.evidence.role_definition_id",
                )?;
            }

            if event
                .has_value("azure.frontdoor.waf.identity.authorization.evidence.roleAssignmentId")
            {
                event.rename(
                    "azure.frontdoor.waf.identity.authorization.evidence.roleAssignmentId",
                    "azure.frontdoor.waf.identity.authorization.evidence.role_assignment_id",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.identity.authorization.evidence.principalId") {
                event.rename(
                    "azure.frontdoor.waf.identity.authorization.evidence.principalId",
                    "azure.frontdoor.waf.identity.authorization.evidence.principal_id",
                )?;
            }

            if event.has_value("azure.frontdoor.waf.identity.authorization.evidence.principalType")
            {
                event.rename(
                    "azure.frontdoor.waf.identity.authorization.evidence.principalType",
                    "azure.frontdoor.waf.identity.authorization.evidence.principal_type",
                )?;
            }

            let _cond = { event.has_value("azure.frontdoor.waf.identity.claims") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: Map convertDotsToUnderscore(Map m) {\n  def out = new HashMap();\n  for (entry in m.entrySet()) {\n    def k = entry.getKey().replace('.', '_');\n    def v = entry.getValue();\n    out.put(k, v);\n  }\n  return out;\n}\nctx.azure.frontdoor.waf.identity.claims = convertDotsToUnderscore(ctx.azure.frontdoor.waf.identity.claims);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Map convertDotsToUnderscore(Map m) {\n  def out = new HashMap();\n  for (entry in m.entrySet()) {\n    def k = entry.getKey().replace('.', '_');\n    def v = entry.getValue();\n    out.put(k, v);\n  }\n  return out;\n}\nctx.azure.frontdoor.waf.identity.claims = convertDotsToUnderscore(ctx.azure.frontdoor.waf.identity.claims);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_claims_cleanup")?;
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
                event
                    .get("azure.frontdoor.waf.identity.claims")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def claims = ctx.azure.frontdoor.waf.identity.claims;\ndef claims_initiated_by_user = new HashMap();\nif (claims.name != null) {\n    claims_initiated_by_user.fullname = claims.name;\n}\nfor (entry in params.entrySet()) {\n    if (claims[entry.getValue()] != null) {\n        claims_initiated_by_user[entry.getKey()] = claims[entry.getValue()];\n    }\n}\nif (claims_initiated_by_user.size() > 0) {\n    claims_initiated_by_user.schema = \"http://schemas.xmlsoap.org/ws/2005/05/identity/claims\";\n    ctx.azure.frontdoor.waf.identity.claims_initiated_by_user = claims_initiated_by_user;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def claims = ctx.azure.frontdoor.waf.identity.claims;\ndef claims_initiated_by_user = new HashMap();\nif (claims.name != null) {\n    claims_initiated_by_user.fullname = claims.name;\n}\nfor (entry in params.entrySet()) {\n    if (claims[entry.getValue()] != null) {\n        claims_initiated_by_user[entry.getKey()] = claims[entry.getValue()];\n    }\n}\nif (claims_initiated_by_user.size() > 0) {\n    claims_initiated_by_user.schema = \"http://schemas.xmlsoap.org/ws/2005/05/identity/claims\";\n    ctx.azure.frontdoor.waf.identity.claims_initiated_by_user = claims_initiated_by_user;\n}"#
                        ),
                        cached_params!(
                            "{\"surname\":\"http://schemas_xmlsoap_org/ws/2005/05/identity/claims/surname\",\"name\":\"http://schemas_xmlsoap_org/ws/2005/05/identity/claims/name\",\"givenname\":\"http://schemas_xmlsoap_org/ws/2005/05/identity/claims/givenname\",\"objectidentifier\":\"http://schemas_microsoft_com/identity/claims/objectidentifier\",\"tenantid\":\"http://schemas_microsoft_com/identity/claims/tenantid\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_claims_user")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("azure.frontdoor.waf.identity.claims_initiated_by_user.name") {
                    if let Some(input) = event
                        .get_string("azure.frontdoor.waf.identity.claims_initiated_by_user.name")
                    {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("user.name") };
            if _cond {
                let v = json!(
                    event
                        .get("azure.frontdoor.waf.identity.claims_initiated_by_user.name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                let v = json!(
                    event
                        .get("azure.frontdoor.waf.identity.claims_initiated_by_user.name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.name", v)?;
                }
            }

            if event.has_value("azure.frontdoor.waf.identity.claims_initiated_by_user.fullname") {
                event.rename(
                    "azure.frontdoor.waf.identity.claims_initiated_by_user.fullname",
                    "user.full_name",
                )?;
            }

            if event
                .has_value("azure.frontdoor.waf.identity.claims_initiated_by_user.objectidentifier")
            {
                event.rename(
                    "azure.frontdoor.waf.identity.claims_initiated_by_user.objectidentifier",
                    "user.id",
                )?;
            }

            let _cond =
                { event.has_value("azure.frontdoor.waf.identity.authorization.evidence.role") };
            if _cond {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("azure.frontdoor.waf.identity.authorization.evidence.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("azure.frontdoor.waf.identity.claims_initiated_by_user.tenantid") {
                event.rename(
                    "azure.frontdoor.waf.identity.claims_initiated_by_user.tenantid",
                    "cloud.account.id",
                )?;
            }

            if event.has_value("client.port") {
                if let Some(val) = event.get("client.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "client.port".into(),
                            message,
                        }
                    })?;
                    event.set("client.port", converted)?;
                }
            }

            if let Some(date_str) = event.get_as_string("azure.frontdoor.waf.time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "azure.frontdoor.waf.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.remove("azure.frontdoor.waf.time");
            event.remove("azure.frontdoor.waf.properties");
            event.remove("azure.frontdoor.waf.details.matches");

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
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

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
