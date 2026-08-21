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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let v = json!("Microsoft");
            if !painless_is_empty_value(&v) {
                event.set("observer.vendor", v)?;
            }

            let v = json!("Defender for Endpoint");
            if !painless_is_empty_value(&v) {
                event.set("observer.product", v)?;
            }

            let v = json!("event");
            if !painless_is_empty_value(&v) {
                event.set("event.kind", v)?;
            }

            event.append_unique("event.category", json!("host"))?;

            event.append_unique("event.type", json!("info"))?;

            if event.has("json.aadDeviceId") {
                event.rename(
                    "json.aadDeviceId",
                    "microsoft_defender_endpoint.machine.aad_device_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.aad_device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            let _cond = { event.has_value("microsoft_defender_endpoint.machine.aad_device_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.aad_device_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.agentVersion") {
                event.rename(
                    "json.agentVersion",
                    "microsoft_defender_endpoint.machine.agent_version",
                )?;
            }

            if event.has("json.computerDnsName") {
                event.rename(
                    "json.computerDnsName",
                    "microsoft_defender_endpoint.machine.computer_dns_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.computer_dns_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.computer_dns_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine.computer_dns_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.computer_dns_name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.deviceValue") {
                event.rename(
                    "json.deviceValue",
                    "microsoft_defender_endpoint.machine.device_value",
                )?;
            }

            if event.has("json.exclusionReason") {
                event.rename(
                    "json.exclusionReason",
                    "microsoft_defender_endpoint.machine.exclusion_reason",
                )?;
            }

            if event.has("json.exposureLevel") {
                event.rename(
                    "json.exposureLevel",
                    "microsoft_defender_endpoint.machine.exposure_level",
                )?;
            }

            let _cond = {
                event.has_value("json.firstSeen") && event.get_str("json.firstSeen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstSeen") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("microsoft_defender_endpoint.machine.first_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstSeen")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.healthStatus") {
                event.rename(
                    "json.healthStatus",
                    "microsoft_defender_endpoint.machine.health_status",
                )?;
            }

            if event.has("json.id") {
                event.rename("json.id", "microsoft_defender_endpoint.machine.id")?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("microsoft_defender_endpoint.machine.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.ipAddresses") };
            if _cond {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx.json.ipAddresses);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"boolean drop(Object object) {\n  if (object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx.json.ipAddresses);"#
                    ),
                )?;
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ipAddress") {
                                if let Some(val) = event.get("_ingest._value.ipAddress") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.ipAddress".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.ip_address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ipAddresses_ipAddress_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.ip_address")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.macAddress") {
                            if let Some(s) = event.get_string("_ingest._value.macAddress") {
                                let uppered = s.to_uppercase();
                                event.set("_ingest._value.mac_address", uppered)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.mac_address") {
                            if let Some(s) = event.get_string("_ingest._value.mac_address") {
                                let re = cached_regex!("(..)(?!$)");
                                let replaced = re.replace_all(&s, "$1-").into_owned();
                                event.set("_ingest._value.mac_address", replaced)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.operationalStatus") {
                            event.rename(
                                "_ingest._value.operationalStatus",
                                "_ingest._value.operational_status",
                            )?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.ipAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.ipAddresses").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.remove("_ingest._value.ipAddress");
                        event.remove("_ingest._value.macAddress");
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.ipAddresses", Value::Array(out))?;
                }
            }

            if event.has("json.ipAddresses") {
                event.rename(
                    "json.ipAddresses",
                    "microsoft_defender_endpoint.machine.ip_addresses",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.isAadJoined") {
                    if let Some(val) = event.get("json.isAadJoined") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isAadJoined".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_defender_endpoint.machine.is_aad_joined",
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
                    "convert_isAadJoined_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                if event.has_value("json.isExcluded") {
                    if let Some(val) = event.get("json.isExcluded") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isExcluded".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_defender_endpoint.machine.is_excluded", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isExcluded_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                if event.has_value("json.isPotentialDuplication") {
                    if let Some(val) = event.get("json.isPotentialDuplication") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isPotentialDuplication".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_defender_endpoint.machine.is_potential_duplication",
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
                    "convert_isPotentialDuplication_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("json.lastExternalIpAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.lastExternalIpAddress") {
                        if let Some(val) = event.get("json.lastExternalIpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.lastExternalIpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_endpoint.machine.last_external_ip_address",
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
                        "convert_lastExternalIpAddress_to_ip",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine.last_external_ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.last_external_ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine.last_external_ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.last_external_ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.lastIpAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.lastIpAddress") {
                        if let Some(val) = event.get("json.lastIpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.lastIpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_endpoint.machine.last_ip_address",
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
                        "convert_lastIpAddress_to_ip",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.has_value("microsoft_defender_endpoint.machine.last_ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine.last_ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.lastSeen") && event.get_str("json.lastSeen") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastSeen") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("microsoft_defender_endpoint.machine.last_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastSeen")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.machineTags") {
                event.rename(
                    "json.machineTags",
                    "microsoft_defender_endpoint.machine.machine_tags",
                )?;
            }

            if event.has("json.managedBy") {
                event.rename(
                    "json.managedBy",
                    "microsoft_defender_endpoint.machine.managed_by",
                )?;
            }

            if event.has("json.managedByStatus") {
                event.rename(
                    "json.managedByStatus",
                    "microsoft_defender_endpoint.machine.managed_by_status",
                )?;
            }

            if event.has_value("json.mergedIntoMachineId") {
                if let Some(val) = event.get("json.mergedIntoMachineId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.mergedIntoMachineId".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "microsoft_defender_endpoint.machine.merged_into_machine_id",
                        converted,
                    )?;
                }
            }

            if event.has("json.onboardingStatus") {
                event.rename(
                    "json.onboardingStatus",
                    "microsoft_defender_endpoint.machine.onboarding_status",
                )?;
            }

            if event.has("json.osArchitecture") {
                event.rename(
                    "json.osArchitecture",
                    "microsoft_defender_endpoint.machine.os_architecture",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.osBuild") {
                    if let Some(val) = event.get("json.osBuild") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.osBuild".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_defender_endpoint.machine.os_build", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_osBuild_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.osPlatform") {
                event.rename(
                    "json.osPlatform",
                    "microsoft_defender_endpoint.machine.os_platform",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.os_platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            let _cond = { event.has_value("microsoft_defender_endpoint.machine.os_platform") };
            if _cond {
                // Painless script
                // Source: String os_platform = ctx.microsoft_defender_endpoint.machine.os_platform.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\nif (os_platform.contains('centos') || os_platform.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"String os_platform = ctx.microsoft_defender_endpoint.machine.os_platform.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\nif (os_platform.contains('centos') || os_platform.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
            }

            if event.has("json.osProcessor") {
                event.rename(
                    "json.osProcessor",
                    "microsoft_defender_endpoint.machine.os_processor",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.os_processor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.architecture", v)?;
            }

            if event.has_value("json.osVersion") {
                if let Some(val) = event.get("json.osVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.osVersion".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft_defender_endpoint.machine.os_version", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.rbacGroupId") {
                    if let Some(val) = event.get("json.rbacGroupId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rbacGroupId".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_defender_endpoint.machine.rbac_group_id",
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
                    "convert_group_id_to_string",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                .get("microsoft_defender_endpoint.machine.rbac_group_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if event.has("json.rbacGroupName") {
                event.rename(
                    "json.rbacGroupName",
                    "microsoft_defender_endpoint.machine.rbac_group_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.rbac_group_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            if event.has("json.riskScore") {
                event.rename(
                    "json.riskScore",
                    "microsoft_defender_endpoint.machine.risk_score",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.risk_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.risk.calculated_level", v)?;
            }

            if event.has("json.version") {
                event.rename(
                    "json.version",
                    "microsoft_defender_endpoint.machine.version",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            let _cond =
                { event.has_value("host.os.platform") && event.has_value("host.os.version") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "host.os.name",
                        json!(format!(
                            "{} {}",
                            event
                                .get("host.os.platform")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("host.os.version")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.vm_metadata.cloudProvider") {
                event.rename(
                    "json.vm_metadata.cloudProvider",
                    "microsoft_defender_endpoint.machine.vm_metadata.cloud_provider",
                )?;
            }

            if event.has("json.vm_metadata.resourceId") {
                event.rename(
                    "json.vm_metadata.resourceId",
                    "microsoft_defender_endpoint.machine.vm_metadata.resource_id",
                )?;
            }

            if event.has("json.vm_metadata.subscriptionId") {
                event.rename(
                    "json.vm_metadata.subscriptionId",
                    "microsoft_defender_endpoint.machine.vm_metadata.subscription_id",
                )?;
            }

            if event.has("json.vm_metadata.vmId") {
                event.rename(
                    "json.vm_metadata.vmId",
                    "microsoft_defender_endpoint.machine.vm_metadata.vm_id",
                )?;
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
                event.remove("microsoft_defender_endpoint.machine.aad_device_id");
                event.remove("microsoft_defender_endpoint.machine.computer_dns_name");
                event.remove("microsoft_defender_endpoint.machine.id");
                event.remove("microsoft_defender_endpoint.machine.last_external_ip_address");
                event.remove("microsoft_defender_endpoint.machine.os_platform");
                event.remove("microsoft_defender_endpoint.machine.rbac_group_id");
                event.remove("microsoft_defender_endpoint.machine.rbac_group_name");
                event.remove("microsoft_defender_endpoint.machine.risk_score");
                event.remove("microsoft_defender_endpoint.machine.version");
                event.remove("microsoft_defender_endpoint.machine.os_processor");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
