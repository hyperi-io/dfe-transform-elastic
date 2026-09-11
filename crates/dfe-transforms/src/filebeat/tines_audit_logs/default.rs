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
            parse_json_field(event, "message", "json")?;

            let _cond =
                { !event.has_value("json") || !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("missing json object in input document").to_string(),
                });
            }

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

            if event.has_value("json") {
                event.rename("json", "tines.audit_log")?;
            }

            if event.has_value("_tmp.tenant_url") {
                event.rename("_tmp.tenant_url", "tines.tenant_url")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("tines.audit_log.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("tines.audit_log.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("tines.audit_log.updated_at") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "_id",
                        json!(
                            fingerprint_with(&values, "MurmurHash3", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?
                        ),
                    )?;
                }
            }

            let _cond = {
                event.has_value("tines.audit_log.created_at")
                    && event.get_str("tines.event.created_at") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("tines.audit_log.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tines.audit_log.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("tines.audit_log.updated_at")
                    && event.get_str("tines.event.updated_at") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("tines.audit_log.updated_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tines.audit_log.updated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("tines.audit_log.id") {
                if let Some(val) = event.get("tines.audit_log.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "tines.audit_log.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            event.append_unique("event.category", json!("configuration"))?;

            event.append_unique("event.type", json!("info"))?;

            if let Some(v) = event
                .get("tines.audit_log.operation_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("tines.audit_log.user_id") {
                if let Some(val) = event.get("tines.audit_log.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "tines.audit_log.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("tines.audit_log.user_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("tines.audit_log.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
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

            if event.has_value("tines.audit_log.request_ip") {
                if let Some(val) = event.get("tines.audit_log.request_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "tines.audit_log.request_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("tines.audit_log.request_user_agent") {
                    if let Some(ua_str) = event.get_string("tines.audit_log.request_user_agent") {
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
                Ok(())
            })();

            let _cond = { event.has_value("tines.audit_log.inputs.fieldId") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("tines.audit_log.inputs.fieldId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "tines.audit_log.inputs.fieldId".into(),
                                message,
                            }
                        })?;
                        event.set("tines.audit_log.inputs.fieldId", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_field_id_long")?;
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
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.remove("tines.audit_log.inputs.fieldId").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "tines.audit_log.inputs.fieldId".into(),
                            });
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
            }

            let _cond = { event.has_value("tines.audit_log.inputs.inputs.multiSelect") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("tines.audit_log.inputs.inputs.multiSelect") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "tines.audit_log.inputs.inputs.multiSelect".into(),
                                message,
                            }
                        })?;
                        event.set("tines.audit_log.inputs.inputs.multiSelect", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_field_id_long")?;
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
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .remove("tines.audit_log.inputs.inputs.multiSelect")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "tines.audit_log.inputs.inputs.multiSelect".into(),
                            });
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
            }

            let _cond = { event.has_value("tines.audit_log.inputs.inputs.required") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("tines.audit_log.inputs.inputs.required") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "tines.audit_log.inputs.inputs.required".into(),
                                message,
                            }
                        })?;
                        event.set("tines.audit_log.inputs.inputs.required", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_field_id_long")?;
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
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .remove("tines.audit_log.inputs.inputs.required")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "tines.audit_log.inputs.inputs.required".into(),
                            });
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
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            event.remove("_tmp");
            event.remove("message");
            event.remove("json");

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
                event.remove("tines.audit_log.id");
                event.remove("tines.audit_log.operation_name");
                event.remove("tines.audit_log.request_ip");
                event.remove("tines.audit_log.request_user_agent");
                event.remove("tines.audit_log.user_email");
                event.remove("tines.audit_log.user_id");
                event.remove("tines.audit_log.user_name");
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
