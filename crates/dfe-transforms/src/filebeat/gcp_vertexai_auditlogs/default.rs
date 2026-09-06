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

            event.set("event.kind", json!("event"))?;

            event.set("cloud.provider", json!("gcp"))?;

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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.protoPayload.@type") {
                event.rename("json.protoPayload.@type", "gcp.vertexai.audit.type")?;
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.type")
                    && event.get_str("gcp.vertexai.audit.type")
                        != Some("type.googleapis.com/google.cloud.audit.AuditLog")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("json.insertId") };
            if _cond {
                if let Some(v) = event.get("json.insertId").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if event.has_value("json.logName") {
                event.rename("json.logName", "log.logger")?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "log.level")?;
            }

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = { event.has_value("json.resource.labels.project_id") };
            if _cond {
                event.rename("json.resource.labels.project_id", "cloud.project.id")?;
            }

            let _cond = { event.has_value("json.resource.labels.instance_id") };
            if _cond {
                event.rename("json.resource.labels.instance_id", "cloud.instance.id")?;
            }

            let _cond = { event.has_value("json.resource.type") };
            if _cond {
                event.rename("json.resource.type", "gcp.vertexai.audit.resource_type")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.authoritySelector",
                    "gcp.vertexai.audit.authentication_info.authority_selector",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalEmail",
                    "gcp.vertexai.audit.authentication_info.principal_email",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalSubject",
                    "gcp.vertexai.audit.authentication_info.principal_subject",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.serviceAccountKeyName",
                    "gcp.vertexai.audit.authentication_info.service_account_key_name",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                    "gcp.vertexai.audit.authentication_info.service_account_delegation_info",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.thirdPartyPrincipal",
                    "gcp.vertexai.audit.authentication_info.third_party_principal",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.serviceName") {
                    event.rename(
                        "json.protoPayload.serviceName",
                        "gcp.vertexai.audit.service_name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.methodName") {
                    event.rename("json.protoPayload.methodName", "event.action")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.resourceName") {
                    event.rename(
                        "json.protoPayload.resourceName",
                        "gcp.vertexai.audit.resource_name",
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.protoPayload.request") };
            if _cond {
                event.rename("json.protoPayload.request", "gcp.vertexai.audit.request")?;
            }

            let _cond = { event.has_value("json.protoPayload.response") };
            if _cond {
                event.rename("json.protoPayload.response", "gcp.vertexai.audit.response")?;
            }

            if event.has_value("json.protoPayload.resourceLocation.currentLocations") {
                event.rename(
                    "json.protoPayload.resourceLocation.currentLocations",
                    "gcp.vertexai.audit.resource_location.current_locations",
                )?;
            }

            if event.has_value("json.protoPayload.numResponseItems") {
                if let Some(val) = event.get("json.protoPayload.numResponseItems") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.protoPayload.numResponseItems".into(),
                            message,
                        }
                    })?;
                    event.set("gcp.vertexai.audit.num_response_items", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.status.code") {
                    event.rename(
                        "json.protoPayload.status.code",
                        "gcp.vertexai.audit.status.code",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.status.message") {
                    event.rename(
                        "json.protoPayload.status.message",
                        "gcp.vertexai.audit.status.message",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.protoPayload.status.details") {
                    event.rename(
                        "json.protoPayload.status.details",
                        "gcp.vertexai.audit.status.details",
                    )?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.vertexai.audit.authorization_info") && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| v.is_array()) && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1)
            };
            if _cond {
                event.append("event.category", json!("network"))?;
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.authorization_info") && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| v.is_array()) && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1) && event.has_value("gcp.vertexai.audit.authorization_info.0.granted") && event.get_bool("gcp.vertexai.audit.authorization_info.0.granted") == Some(true)
            };
            if _cond {
                event.append("event.type", json!("access"))?;
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.authorization_info") && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| v.is_array()) && event.get("gcp.vertexai.audit.authorization_info").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1) && event.has_value("gcp.vertexai.audit.authorization_info.0.granted") && !(event.get_bool("gcp.vertexai.audit.authorization_info.0.granted") == Some(true))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
                event.append("event.type", json!("denied"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo",
                    "gcp.vertexai.audit.policy_violation_info.violations",
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.vertexai.audit.policy_violation_info.violations")
                    && event
                        .get("gcp.vertexai.audit.policy_violation_info.violations")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.vertexai.audit.policy_violation_info.violations") {
                        foreach_array(
                            event,
                            "gcp.vertexai.audit.policy_violation_info.violations",
                            |event| {
                                event.rename(
                                    "_ingest._value.errorMessage",
                                    "_ingest._value.error_message",
                                )?;
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.policy_violation_info.violations")
                    && event
                        .get("gcp.vertexai.audit.policy_violation_info.violations")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.vertexai.audit.policy_violation_info.violations") {
                        foreach_array(
                            event,
                            "gcp.vertexai.audit.policy_violation_info.violations",
                            |event| {
                                event.rename(
                                    "_ingest._value.checkedValue",
                                    "_ingest._value.checked_value",
                                )?;
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.policy_violation_info.violations")
                    && event
                        .get("gcp.vertexai.audit.policy_violation_info.violations")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.vertexai.audit.policy_violation_info.violations") {
                        foreach_array(
                            event,
                            "gcp.vertexai.audit.policy_violation_info.violations",
                            |event| {
                                event.rename(
                                    "_ingest._value.policyType",
                                    "_ingest._value.policy_type",
                                )?;
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.payload",
                    "gcp.vertexai.audit.policy_violation_info.payload",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceType",
                    "gcp.vertexai.audit.policy_violation_info.resource_type",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceTags",
                    "gcp.vertexai.audit.policy_violation_info.resource_tags",
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.protoPayload.requestMetadata.callerIp")
                    && event.get_str("json.protoPayload.requestMetadata.callerIp")
                        != Some("gce-internal-ip")
                    && event.get_str("json.protoPayload.requestMetadata.callerIp")
                        != Some("private")
            };
            if _cond {
                if event.has_value("json.protoPayload.requestMetadata.callerIp") {
                    if let Some(val) = event.get("json.protoPayload.requestMetadata.callerIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.protoPayload.requestMetadata.callerIp".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
            }

            let _cond = { event.has_value("json.protoPayload.requestMetadata.callerIp") };
            if _cond {
                event.remove("json.protoPayload.requestMetadata.callerIp");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.requestMetadata.callerSuppliedUserAgent",
                    "user_agent.original",
                )?;
                Ok(())
            })();

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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("source.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.has_value("client.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("client.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.protoPayload.metadata") };
            if _cond {
                event.rename("json.protoPayload.metadata", "gcp.vertexai.audit.metadata")?;
            }

            let _cond = { !event.has_value("client.user.email") };
            if _cond {
                if event.has_value("gcp.vertexai.audit.authentication_info.principal_email") {
                    event.rename(
                        "gcp.vertexai.audit.authentication_info.principal_email",
                        "client.user.email",
                    )?;
                }
            }

            let _cond = { !event.has_value("client.user.id") };
            if _cond {
                if event.has_value("gcp.vertexai.audit.authentication_info.principal_subject") {
                    event.rename(
                        "gcp.vertexai.audit.authentication_info.principal_subject",
                        "client.user.id",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authorizationInfo",
                    "gcp.vertexai.audit.authorization_info",
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.vertexai.audit.authorization_info")
                    && event
                        .get("gcp.vertexai.audit.authorization_info")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.vertexai.audit.authorization_info") {
                        foreach_array(event, "gcp.vertexai.audit.authorization_info", |event| {
                            event.rename(
                                "_ingest._value.resourceAttributes",
                                "_ingest._value.resource_attributes",
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("gcp.vertexai.audit.authorization_info")
                    && event
                        .get("gcp.vertexai.audit.authorization_info")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.vertexai.audit.authorization_info") {
                        foreach_array(event, "gcp.vertexai.audit.authorization_info", |event| {
                            event.rename(
                                "_ingest._value.permissionType",
                                "_ingest._value.permission_type",
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
