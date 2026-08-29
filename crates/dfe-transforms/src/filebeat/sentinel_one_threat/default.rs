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
            let _cond = {
                event.get_bool("retry") == Some(true)
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Drop);
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

            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("alert"))?;

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx.json);\n"#
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.threatInfo.createdAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.threatInfo.updatedAt") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.timeline.id") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.timeline.createdAt") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.timeline.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.timeline.updatedAt") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = {
                event.has_value("json.threatInfo.threatId")
                    && event.get_str("json.threatInfo.threatId") != Some("")
                    && event
                        .get_str("json.threatInfo.classification")
                        .is_some_and(|s| ["exploit", "pua"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("threat")]))?;
            }

            let _cond = {
                event.has_value("json.threatInfo.threatId")
                    && event.get_str("json.threatInfo.threatId") != Some("")
                    && event
                        .get_str("json.threatInfo.classification")
                        .is_some_and(|s| {
                            ["malware", "ransomware", "trojan", "downloader"]
                                .contains(&s.to_lowercase().as_str())
                        })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("malware")]))?;
            }

            let _cond = {
                event.has_value("json.threatInfo.threatId")
                    && event.get_str("json.threatInfo.threatId") != Some("")
                    && event
                        .get_str("json.threatInfo.classification")
                        .is_some_and(|s| ["exploit", "pua"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("indicator")]))?;
            }

            let _cond = {
                event.has_value("json.threatInfo.threatId")
                    && event.get_str("json.threatInfo.threatId") != Some("")
                    && event
                        .get_str("json.threatInfo.classification")
                        .is_some_and(|s| {
                            ["malware", "ransomware", "trojan", "downloader"]
                                .contains(&s.to_lowercase().as_str())
                        })
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = { event.has_value("json.threatInfo.updatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.threatInfo.updatedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.threatInfo.updatedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_threatInfo_updatedAt_f0b375f7",
                    )?;
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
                let joined = event
                    .get("json.threatInfo.engines")
                    .and_then(|v| join_values(v, ","));
                if let Some(joined) = joined {
                    event.set("event.action", json!(joined))?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("json.threatInfo.originatorProcess")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if event.has_value("json.agentDetectionInfo.accountId") {
                event.rename(
                    "json.agentDetectionInfo.accountId",
                    "sentinel_one.threat.detection.account.id",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.accountName") {
                event.rename(
                    "json.agentDetectionInfo.accountName",
                    "sentinel_one.threat.detection.account.name",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentDetectionState") {
                event.rename(
                    "json.agentDetectionInfo.agentDetectionState",
                    "sentinel_one.threat.detection.state",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentDomain") {
                event.rename(
                    "json.agentDetectionInfo.agentDomain",
                    "sentinel_one.threat.detection.agent.domain",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentIpV4") {
                if let Some(s) = event.get_string("json.agentDetectionInfo.agentIpV4") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("json.agentDetectionInfo.agentIpV4", Value::Array(parts))?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentDetectionInfo.agentIpV4") {
                    if let Some(val) = event.get("json.agentDetectionInfo.agentIpV4") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentDetectionInfo.agentIpV4".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.detection.agent.ipv4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_agentIpV4_to_ip",
                )?;
                event.remove("json.agentDetectionInfo.agentIpV4");
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

            let _cond = { event.has_value("sentinel_one.threat.detection.agent.ipv4") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "sentinel_one.threat.detection.agent.ipv4", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.agentDetectionInfo.agentIpV6") {
                if let Some(s) = event.get_string("json.agentDetectionInfo.agentIpV6") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("json.agentDetectionInfo.agentIpV6", Value::Array(parts))?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentDetectionInfo.agentIpV6") {
                    if let Some(val) = event.get("json.agentDetectionInfo.agentIpV6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentDetectionInfo.agentIpV6".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.detection.agent.ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_agentIpV6_to_ip",
                )?;
                event.remove("json.agentDetectionInfo.agentIpV6");
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

            let _cond = { event.has_value("sentinel_one.threat.detection.agent.ipv6") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "sentinel_one.threat.detection.agent.ipv6", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.agentDetectionInfo.agentLastLoggedInUpn") {
                event.rename(
                    "json.agentDetectionInfo.agentLastLoggedInUpn",
                    "sentinel_one.threat.detection.agent.last_logged_in.upn",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentLastLoggedInUserMail") {
                event.rename(
                    "json.agentDetectionInfo.agentLastLoggedInUserMail",
                    "user.email",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentLastLoggedInUserName") {
                event.rename(
                    "json.agentDetectionInfo.agentLastLoggedInUserName",
                    "user.name",
                )?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("json.threatInfo.processUser")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.get("user.name").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                    serde_json::Value::String(s) => s.contains("\\"),
                    _ => false,
                })
            };
            if _cond {
                if let Some(input) = event.get_string("user.name") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("\\") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\\") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "user.name".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
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

            if event.has_value("json.agentDetectionInfo.agentMitigationMode") {
                event.rename(
                    "json.agentDetectionInfo.agentMitigationMode",
                    "sentinel_one.threat.detection.agent.mitigation_mode",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentOsName") {
                event.rename(
                    "json.agentDetectionInfo.agentOsName",
                    "sentinel_one.threat.detection.agent.os.name",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentOsRevision") {
                event.rename(
                    "json.agentDetectionInfo.agentOsRevision",
                    "sentinel_one.threat.detection.agent.os.version",
                )?;
            }

            let _cond = { event.has_value("json.agentDetectionInfo.agentRegisteredAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.agentDetectionInfo.agentRegisteredAt")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("sentinel_one.threat.detection.agent.registered_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.agentDetectionInfo.agentRegisteredAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_agentDetectionInfo_agentRegisteredAt_to_sentinel_one_threat_detection_agent_registered_at_a7199dfc")?;
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

            if event.has_value("json.agentDetectionInfo.agentUuid") {
                event.rename(
                    "json.agentDetectionInfo.agentUuid",
                    "sentinel_one.threat.detection.agent.uuid",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.agentVersion") {
                event.rename(
                    "json.agentDetectionInfo.agentVersion",
                    "sentinel_one.threat.detection.agent.version",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.cloudProviders") {
                event.rename(
                    "json.agentDetectionInfo.cloudProviders",
                    "sentinel_one.threat.detection.cloud_providers",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentDetectionInfo.externalIp") {
                    if let Some(val) = event.get("json.agentDetectionInfo.externalIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentDetectionInfo.externalIp".into(),
                                message,
                            }
                        })?;
                        event.set("json.agentDetectionInfo.externalIp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_externalIp_to_ip",
                )?;
                event.remove("json.agentDetectionInfo.externalIp");
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

            let _cond = { event.has_value("json.agentDetectionInfo.externalIp") };
            if _cond {
                if event.has_value("json.agentDetectionInfo.externalIp") {
                    if let Some(ip_str) = event.get_string("json.agentDetectionInfo.externalIp") {
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
            }

            let _cond = { event.has_value("json.agentDetectionInfo.externalIp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("json.agentDetectionInfo.externalIp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.agentDetectionInfo.externalIp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.agentDetectionInfo.externalIp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.agentDetectionInfo.groupId") {
                event.rename(
                    "json.agentDetectionInfo.groupId",
                    "sentinel_one.threat.detection.agent.group.id",
                )?;
            }

            if event.has_value("json.agentDetectionInfo.groupName") {
                event.rename(
                    "json.agentDetectionInfo.groupName",
                    "sentinel_one.threat.detection.agent.group.name",
                )?;
            }

            let _cond = { !event.has_value("group.id") };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.threat.detection.agent.group.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
            }

            let _cond = { !event.has_value("group.name") };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.threat.detection.agent.group.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
            }

            if event.has_value("json.agentDetectionInfo.siteId") {
                event.rename("json.agentDetectionInfo.siteId", "sentinel_one.site.id")?;
            }

            if event.has_value("json.agentDetectionInfo.siteName") {
                event.rename("json.agentDetectionInfo.siteName", "sentinel_one.site.name")?;
            }

            if event.has_value("json.agentRealtimeInfo.accountId") {
                event.rename(
                    "json.agentRealtimeInfo.accountId",
                    "sentinel_one.threat.agent.account.id",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.accountName") {
                event.rename(
                    "json.agentRealtimeInfo.accountName",
                    "sentinel_one.account.name",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.activeThreats") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.activeThreats") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.activeThreats".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.active_threats", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_activeThreats_to_sentinel_one_threat_agent_active_threats_d3b4fd39")?;
                event.remove("json.agentRealtimeInfo.activeThreats");
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

            if event.has_value("json.agentRealtimeInfo.agentComputerName") {
                event.rename("json.agentRealtimeInfo.agentComputerName", "host.name")?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.agentDecommissionedAt") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.agentDecommissionedAt") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.agentDecommissionedAt".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.decommissioned_at", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_agentDecommissionedAt_to_sentinel_one_threat_agent_decommissioned_at_a2cf389e")?;
                event.remove("json.agentRealtimeInfo.agentDecommissionedAt");
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

            if event.has_value("json.agentRealtimeInfo.agentDomain") {
                event.rename("json.agentRealtimeInfo.agentDomain", "host.domain")?;
            }

            if event.has_value("json.agentRealtimeInfo.agentId") {
                event.rename(
                    "json.agentRealtimeInfo.agentId",
                    "sentinel_one.threat.agent.id",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.threat.agent.id") };
            if _cond {
                if let Some(v) = event.get("sentinel_one.threat.agent.id").cloned() {
                    event.set("host.id", v)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.agentInfected") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.agentInfected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.agentInfected".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.infected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_agentInfected_to_sentinel_one_threat_agent_infected_e0abe9a1")?;
                event.remove("json.agentRealtimeInfo.agentInfected");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.agentIsActive") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.agentIsActive") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.agentIsActive".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.is_active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_agentIsActive_to_sentinel_one_threat_agent_is_active_4e60f134")?;
                event.remove("json.agentRealtimeInfo.agentIsActive");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.agentIsDecommissioned") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.agentIsDecommissioned") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.agentIsDecommissioned".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.is_decommissioned", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_agentIsDecommissioned_to_sentinel_one_threat_agent_is_decommissioned_9b1e3491")?;
                event.remove("json.agentRealtimeInfo.agentIsDecommissioned");
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

            if event.has_value("json.agentRealtimeInfo.agentMachineType") {
                event.rename(
                    "json.agentRealtimeInfo.agentMachineType",
                    "sentinel_one.threat.agent.machine_type",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.agentMitigationMode") {
                event.rename(
                    "json.agentRealtimeInfo.agentMitigationMode",
                    "sentinel_one.threat.agent.mitigation_mode",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.agentNetworkStatus") {
                event.rename(
                    "json.agentRealtimeInfo.agentNetworkStatus",
                    "sentinel_one.threat.agent.network_status",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.agentOsName") {
                event.rename("json.agentRealtimeInfo.agentOsName", "host.os.name")?;
            }

            if event.has_value("json.agentRealtimeInfo.agentOsRevision") {
                event.rename(
                    "json.agentRealtimeInfo.agentOsRevision",
                    "sentinel_one.threat.agent.os.version",
                )?;
            }

            let _cond = { event.has_value("json.agentRealtimeInfo.agentOsType") };
            if _cond {
                // Painless script
                // Source: ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString agent_os_type = ctx.json.agentRealtimeInfo.agentOsType.toLowerCase();\nfor (String os: params.os_type) {\n  if (agent_os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString agent_os_type = ctx.json.agentRealtimeInfo.agentOsType.toLowerCase();\nfor (String os: params.os_type) {\n  if (agent_os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.agentUuid") {
                event.rename(
                    "json.agentRealtimeInfo.agentUuid",
                    "sentinel_one.threat.agent.uuid",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.agentVersion") {
                event.rename("json.agentRealtimeInfo.agentVersion", "observer.version")?;
            }

            if event.has_value("json.agentRealtimeInfo.groupId") {
                event.rename(
                    "json.agentRealtimeInfo.groupId",
                    "sentinel_one.threat.agent.group.id",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.agent.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if event.has_value("json.agentRealtimeInfo.groupName") {
                event.rename(
                    "json.agentRealtimeInfo.groupName",
                    "sentinel_one.threat.agent.group.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.agent.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("json.agentRealtimeInfo.networkInterfaces")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject = event.get("_ingest._value.inet").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // on_failure: 2 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if event.has_value("_ingest._value") {
                                                        if let Some(val) =
                                                            event.get("_ingest._value")
                                                        {
                                                            let converted = convert_value(
                                                                val, "ip",
                                                            )
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value".into(),
                                                                    message,
                                                                }
                                                            })?;
                                                            event
                                                                .set("_ingest._value", converted)?;
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "convert",
                                                    )?;
                                                    if event.remove("_ingest._value").is_none() {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path: "_ingest._value".into(),
                                                            },
                                                        );
                                                    }
                                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_ingest._value.inet",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.agentRealtimeInfo.networkInterfaces",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.agentRealtimeInfo.networkInterfaces", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.inet", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("json.agentRealtimeInfo.networkInterfaces")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject = event.get("_ingest._value.inet6").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // on_failure: 2 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if event.has_value("_ingest._value") {
                                                        if let Some(val) =
                                                            event.get("_ingest._value")
                                                        {
                                                            let converted = convert_value(
                                                                val, "ip",
                                                            )
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value".into(),
                                                                    message,
                                                                }
                                                            })?;
                                                            event
                                                                .set("_ingest._value", converted)?;
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "convert",
                                                    )?;
                                                    if event.remove("_ingest._value").is_none() {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path: "_ingest._value".into(),
                                                            },
                                                        );
                                                    }
                                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_ingest._value.inet6",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.agentRealtimeInfo.networkInterfaces",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.agentRealtimeInfo.networkInterfaces", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.inet6", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.agentRealtimeInfo.networkInterfaces", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "host.mac",
                                json!(
                                    event
                                        .get("_ingest._value.physical")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.agentRealtimeInfo.networkInterfaces")
                    && event
                        .get("json.agentRealtimeInfo.networkInterfaces")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.agentRealtimeInfo.networkInterfaces", |event| {
                        event.remove("_ingest._value.physical");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.agentRealtimeInfo.networkInterfaces") {
                event.rename(
                    "json.agentRealtimeInfo.networkInterfaces",
                    "sentinel_one.threat.agent.network_interface",
                )?;
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            if event.has_value("json.agentRealtimeInfo.operationalState") {
                event.rename(
                    "json.agentRealtimeInfo.operationalState",
                    "sentinel_one.threat.agent.operational_state",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.rebootRequired") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.rebootRequired") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.rebootRequired".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.agent.reboot_required", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_rebootRequired_to_sentinel_one_threat_agent_reboot_required_ec8a937c")?;
                event.remove("json.agentRealtimeInfo.rebootRequired");
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

            let _cond = { event.has_value("json.agentRealtimeInfo.scanAbortedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.agentRealtimeInfo.scanAbortedAt")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.agent.scan.aborted_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.agentRealtimeInfo.scanAbortedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_agentRealtimeInfo_scanAbortedAt_to_sentinel_one_threat_agent_scan_aborted_at_e4e1a5b9")?;
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

            let _cond = { event.has_value("json.agentRealtimeInfo.scanFinishedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.agentRealtimeInfo.scanFinishedAt")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.agent.scan.finished_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.agentRealtimeInfo.scanFinishedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_agentRealtimeInfo_scanFinishedAt_to_sentinel_one_threat_agent_scan_finished_at_6694d470")?;
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

            let _cond = { event.has_value("json.agentRealtimeInfo.scanStartedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.agentRealtimeInfo.scanStartedAt")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.agent.scan.started_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.agentRealtimeInfo.scanStartedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_agentRealtimeInfo_scanStartedAt_to_sentinel_one_threat_agent_scan_started_at_5e11dc87")?;
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

            if event.has_value("json.agentRealtimeInfo.scanStatus") {
                event.rename(
                    "json.agentRealtimeInfo.scanStatus",
                    "sentinel_one.threat.agent.scan.status",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.siteId") {
                event.rename(
                    "json.agentRealtimeInfo.siteId",
                    "sentinel_one.threat.agent.site.id",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.siteName") {
                event.rename(
                    "json.agentRealtimeInfo.siteName",
                    "sentinel_one.threat.agent.site.name",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.storageName") {
                event.rename(
                    "json.agentRealtimeInfo.storageName",
                    "sentinel_one.threat.agent.storage.name",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.storageType") {
                event.rename(
                    "json.agentRealtimeInfo.storageType",
                    "sentinel_one.threat.agent.storage.type",
                )?;
            }

            if event.has_value("json.agentRealtimeInfo.userActionsNeeded") {
                event.rename(
                    "json.agentRealtimeInfo.userActionsNeeded",
                    "sentinel_one.threat.agent.user_action_needed",
                )?;
            }

            if event.has_value("json.containerInfo.id") {
                event.rename("json.containerInfo.id", "container.id")?;
            }

            if event.has_value("json.containerInfo.image") {
                event.rename("json.containerInfo.image", "container.image.name")?;
            }

            if event.has_value("json.containerInfo.labels") {
                event.rename(
                    "json.containerInfo.labels",
                    "sentinel_one.threat.container.labels",
                )?;
            }

            if event.has_value("json.containerInfo.name") {
                event.rename("json.containerInfo.name", "container.name")?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "sentinel_one.threat.description")?;
            }

            let _cond = { event.has_value("json.id") };
            if _cond {
                if let Some(v) = event.get("json.id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sentinel_one.threat.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    if event.has_value("_ingest._value.category") {
                        event.rename("_ingest._value.category", "_ingest._value.category.name")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    if event.has_value("_ingest._value.categoryId") {
                        event.rename("_ingest._value.categoryId", "_ingest._value.category.id")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.ids", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "threat.tactic.id",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            let _cond = { event.get("json.indicators").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.indicators", |event| {
                    if event.has_value("_ingest._value.ids") {
                        foreach_array(event, "_ingest._value.ids", |event| {
                            event.append_unique(
                                "threat.indicator.id",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.tactics", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "threat.tactic.name",
                                    json!(
                                        event
                                            .get("_ingest._value.name")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.tactics", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "threat.framework",
                                    json!(
                                        event
                                            .get("_ingest._value.source")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.tactics", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(event, "_ingest._value.techniques", |event| {
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        event.append_unique(
                                            "threat.technique.reference",
                                            json!(
                                                event
                                                    .get("_ingest._value.link")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    })();
                                    Ok(())
                                })?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.tactics", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(event, "_ingest._value.techniques", |event| {
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        event.append_unique(
                                            "threat.technique.id",
                                            json!(
                                                event
                                                    .get("_ingest._value.name")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    })();
                                    Ok(())
                                })?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.indicators", |event| {
                    event.remove("_ingest._value.ids");
                    event.remove("_ingest._value.tactics");
                    Ok(())
                })?;
                Ok(())
            })();

            if event.has_value("json.indicators") {
                event.rename("json.indicators", "sentinel_one.threat.indicators")?;
            }

            if event.has_value("json.kubernetesInfo.cluster") {
                event.rename(
                    "json.kubernetesInfo.cluster",
                    "sentinel_one.threat.kubernetes.cluster",
                )?;
            }

            if event.has_value("json.kubernetesInfo.controllerKind") {
                event.rename(
                    "json.kubernetesInfo.controllerKind",
                    "sentinel_one.threat.kubernetes.controller.kind",
                )?;
            }

            if event.has_value("json.kubernetesInfo.controllerLabels") {
                event.rename(
                    "json.kubernetesInfo.controllerLabels",
                    "sentinel_one.threat.kubernetes.controller.labels",
                )?;
            }

            if event.has_value("json.kubernetesInfo.controllerName") {
                event.rename(
                    "json.kubernetesInfo.controllerName",
                    "sentinel_one.threat.kubernetes.controller.name",
                )?;
            }

            if event.has_value("json.kubernetesInfo.namespace") {
                event.rename(
                    "json.kubernetesInfo.namespace",
                    "sentinel_one.threat.kubernetes.namespace.name",
                )?;
            }

            if event.has_value("json.kubernetesInfo.namespaceLabels") {
                event.rename(
                    "json.kubernetesInfo.namespaceLabels",
                    "sentinel_one.threat.kubernetes.namespace.labels",
                )?;
            }

            if event.has_value("json.kubernetesInfo.node") {
                event.rename(
                    "json.kubernetesInfo.node",
                    "sentinel_one.threat.kubernetes.node",
                )?;
            }

            if event.has_value("json.kubernetesInfo.pod") {
                event.rename(
                    "json.kubernetesInfo.pod",
                    "sentinel_one.threat.kubernetes.pod.name",
                )?;
            }

            if event.has_value("json.kubernetesInfo.podLabels") {
                event.rename(
                    "json.kubernetesInfo.podLabels",
                    "sentinel_one.threat.kubernetes.pod.labels",
                )?;
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.actionsCounters.failed") {
                                if let Some(val) =
                                    event.get("_ingest._value.actionsCounters.failed")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.actionsCounters.failed"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event
                                        .set("_ingest._value.action_counters.failed", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.actionsCounters.notFound") {
                                if let Some(val) =
                                    event.get("_ingest._value.actionsCounters.notFound")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.actionsCounters.notFound"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.action_counters.not_found",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) =
                            (|| -> Result<()> {
                                if event.has_value("_ingest._value.actionsCounters.pendingReboot") {
                                    if let Some(val) =
                                        event.get("_ingest._value.actionsCounters.pendingReboot")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.actionsCounters.pendingReboot".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.action_counters.pending_reboot",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })()
                        {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.actionsCounters.success") {
                                if let Some(val) =
                                    event.get("_ingest._value.actionsCounters.success")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.actionsCounters.success"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event
                                        .set("_ingest._value.action_counters.success", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.actionsCounters.total") {
                                if let Some(val) = event.get("_ingest._value.actionsCounters.total")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.actionsCounters.total".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.action_counters.total", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.agentSupportsReport") {
                                if let Some(val) = event.get("_ingest._value.agentSupportsReport") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.agentSupportsReport".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.agent_supports_report", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.groupNotFound") {
                                if let Some(val) = event.get("_ingest._value.groupNotFound") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.groupNotFound".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.group_not_found", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.mitigationStatus").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.lastUpdate")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.last_update", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.lastUpdate".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.mitigationStatus",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        if event.has_value("_ingest._value.latestReport") {
                            event.rename(
                                "_ingest._value.latestReport",
                                "_ingest._value.latest_report",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        if event.has_value("_ingest._value.reportId") {
                            event.rename("_ingest._value.reportId", "_ingest._value.report_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.mitigationStatus").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.mitigationEndedAt")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.mitigation_ended_at",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.mitigationEndedAt".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.mitigationStatus",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.mitigationStatus").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.mitigationStartedAt")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.mitigation_started_at",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.mitigationStartedAt"
                                                        .into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.mitigationStatus",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitigationStatus")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitigationStatus", |event| {
                        event.remove("_ingest._value.actionsCounters");
                        event.remove("_ingest._value.agentSupportsReport");
                        event.remove("_ingest._value.groupNotFound");
                        event.remove("_ingest._value.lastUpdate");
                        event.remove("_ingest._value.mitigationEndedAt");
                        event.remove("_ingest._value.mitigationStartedAt");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.mitigationStatus") {
                event.rename(
                    "json.mitigationStatus",
                    "sentinel_one.threat.mitigation_status",
                )?;
            }

            if event.has_value("json.threatInfo.analystVerdict") {
                event.rename(
                    "json.threatInfo.analystVerdict",
                    "sentinel_one.threat.analysis.verdict",
                )?;
            }

            if event.has_value("json.threatInfo.analystVerdictDescription") {
                event.rename(
                    "json.threatInfo.analystVerdictDescription",
                    "sentinel_one.threat.analysis.description",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.automaticallyResolved") {
                    if let Some(val) = event.get("json.threatInfo.automaticallyResolved") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.automaticallyResolved".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.automatically_resolved", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_automaticallyResolved_to_sentinel_one_threat_automatically_resolved_44bccd84")?;
                event.remove("json.threatInfo.automaticallyResolved");
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

            if event.has_value("json.threatInfo.browserType") {
                event.rename(
                    "json.threatInfo.browserType",
                    "sentinel_one.threat.browser_type",
                )?;
            }

            if event.has_value("json.threatInfo.certificateId") {
                event.rename(
                    "json.threatInfo.certificateId",
                    "sentinel_one.threat.certificate.id",
                )?;
            }

            if event.has_value("json.threatInfo.classification") {
                event.rename(
                    "json.threatInfo.classification",
                    "sentinel_one.threat_classification.name",
                )?;
            }

            if event.has_value("json.threatInfo.classificationSource") {
                event.rename(
                    "json.threatInfo.classificationSource",
                    "sentinel_one.threat_classification.source",
                )?;
            }

            if event.has_value("json.threatInfo.cloudFilesHashVerdict") {
                event.rename(
                    "json.threatInfo.cloudFilesHashVerdict",
                    "sentinel_one.threat.cloudfiles_hash_verdict",
                )?;
            }

            if event.has_value("json.threatInfo.collectionId") {
                event.rename(
                    "json.threatInfo.collectionId",
                    "sentinel_one.threat.collection.id",
                )?;
            }

            if event.has_value("json.threatInfo.confidenceLevel") {
                event.rename(
                    "json.threatInfo.confidenceLevel",
                    "sentinel_one.threat.confidence_level",
                )?;
            }

            let _cond = { event.has_value("json.threatInfo.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.threatInfo.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("sentinel_one.threat.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.threatInfo.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_threatInfo_createdAt_to_sentinel_one_threat_created_at_0042c1e8",
                    )?;
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

            if event.has_value("json.threatInfo.detectionEngines") {
                event.rename(
                    "json.threatInfo.detectionEngines",
                    "sentinel_one.threat.detection.engines",
                )?;
            }

            if event.has_value("json.threatInfo.detectionType") {
                event.rename(
                    "json.threatInfo.detectionType",
                    "sentinel_one.threat.detection.type",
                )?;
            }

            if event.has_value("json.threatInfo.engines") {
                event.rename("json.threatInfo.engines", "sentinel_one.threat.engines")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.externalTicketExists") {
                    if let Some(val) = event.get("json.threatInfo.externalTicketExists") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.externalTicketExists".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.external_ticket.exist", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_externalTicketExists_to_sentinel_one_threat_external_ticket_exist_e30127f5")?;
                event.remove("json.threatInfo.externalTicketExists");
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

            if event.has_value("json.threatInfo.externalTicketId") {
                event.rename(
                    "json.threatInfo.externalTicketId",
                    "sentinel_one.threat.external_ticket.id",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.failedActions") {
                    if let Some(val) = event.get("json.threatInfo.failedActions") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.failedActions".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.failed_actions", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_failedActions_to_sentinel_one_threat_failed_actions_0c60d959")?;
                event.remove("json.threatInfo.failedActions");
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

            if event.has_value("json.threatInfo.fileExtension") {
                event.rename(
                    "json.threatInfo.fileExtension",
                    "threat.indicator.file.extension",
                )?;
            }

            if event.has_value("json.threatInfo.fileExtensionType") {
                event.rename(
                    "json.threatInfo.fileExtensionType",
                    "sentinel_one.threat.file.extension.type",
                )?;
            }

            if event.has_value("json.threatInfo.filePath") {
                event.rename("json.threatInfo.filePath", "threat.indicator.file.path")?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.fileSize") {
                    if let Some(val) = event.get("json.threatInfo.fileSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.fileSize".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.file.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_threatInfo_fileSize_to_threat_indicator_file_size_39950d27",
                )?;
                event.remove("json.threatInfo.fileSize");
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

            if event.has_value("json.threatInfo.fileVerificationType") {
                event.rename(
                    "json.threatInfo.fileVerificationType",
                    "sentinel_one.threat.file.verification_type",
                )?;
            }

            let _cond = { event.has_value("json.threatInfo.identifiedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.threatInfo.identifiedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.file.identified_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.threatInfo.identifiedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_threatInfo_identifiedAt_to_sentinel_one_threat_file_identified_at_6215ffe3")?;
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

            if event.has_value("json.threatInfo.incidentStatus") {
                event.rename(
                    "json.threatInfo.incidentStatus",
                    "sentinel_one.threat.incident.status",
                )?;
            }

            if event.has_value("json.threatInfo.incidentStatusDescription") {
                event.rename(
                    "json.threatInfo.incidentStatusDescription",
                    "sentinel_one.threat.incident.status_description",
                )?;
            }

            if event.has_value("json.threatInfo.initiatedBy") {
                event.rename(
                    "json.threatInfo.initiatedBy",
                    "sentinel_one.threat.initiated.name",
                )?;
            }

            if event.has_value("json.threatInfo.initiatedByDescription") {
                event.rename(
                    "json.threatInfo.initiatedByDescription",
                    "sentinel_one.threat.initiated.description",
                )?;
            }

            if event.has_value("json.threatInfo.initiatingUserId") {
                event.rename(
                    "json.threatInfo.initiatingUserId",
                    "sentinel_one.threat.initiating_user.id",
                )?;
            }

            if event.has_value("json.threatInfo.initiatingUsername") {
                event.rename(
                    "json.threatInfo.initiatingUsername",
                    "sentinel_one.threat.initiating_user.name",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.threat.initiating_user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("sentinel_one.threat.initiating_user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.isFileless") {
                    if let Some(val) = event.get("json.threatInfo.isFileless") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.isFileless".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.is_fileless", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_isFileless_to_sentinel_one_threat_is_fileless_f443788e")?;
                event.remove("json.threatInfo.isFileless");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.isValidCertificate") {
                    if let Some(val) = event.get("json.threatInfo.isValidCertificate") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.isValidCertificate".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.is_valid_certificate", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_isValidCertificate_to_sentinel_one_threat_is_valid_certificate_64cf6ced")?;
                event.remove("json.threatInfo.isValidCertificate");
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

            if event.has_value("json.threatInfo.maliciousProcessArguments") {
                event.rename(
                    "json.threatInfo.maliciousProcessArguments",
                    "sentinel_one.threat.malicious_process_arguments",
                )?;
            }

            if event.has_value("json.threatInfo.md5") {
                event.rename("json.threatInfo.md5", "threat.indicator.file.hash.md5")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.mitigatedPreemptively") {
                    if let Some(val) = event.get("json.threatInfo.mitigatedPreemptively") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.mitigatedPreemptively".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.mitigated_preemptively", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_mitigatedPreemptively_to_sentinel_one_threat_mitigated_preemptively_33d6e67b")?;
                event.remove("json.threatInfo.mitigatedPreemptively");
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

            if event.has_value("json.threatInfo.mitigationStatus") {
                event.rename(
                    "json.threatInfo.mitigationStatus",
                    "sentinel_one.threat.mitigation.status",
                )?;
            }

            if event.has_value("json.threatInfo.mitigationStatusDescription") {
                event.rename(
                    "json.threatInfo.mitigationStatusDescription",
                    "sentinel_one.threat.mitigation.description",
                )?;
            }

            if event.has_value("json.threatInfo.originatorProcess") {
                event.rename(
                    "json.threatInfo.originatorProcess",
                    "sentinel_one.threat.originator_process",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.pendingActions") {
                    if let Some(val) = event.get("json.threatInfo.pendingActions") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.pendingActions".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.pending_actions", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_pendingActions_to_sentinel_one_threat_pending_actions_c419435f")?;
                event.remove("json.threatInfo.pendingActions");
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

            if event.has_value("json.threatInfo.processUser") {
                event.rename(
                    "json.threatInfo.processUser",
                    "sentinel_one.threat.process_user",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.threat.process_user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("sentinel_one.threat.process_user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.threatInfo.publisherName") {
                event.rename(
                    "json.threatInfo.publisherName",
                    "sentinel_one.threat.publisher.name",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.reachedEventsLimit") {
                    if let Some(val) = event.get("json.threatInfo.reachedEventsLimit") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.reachedEventsLimit".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.reached_events_limit", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_reachedEventsLimit_to_sentinel_one_threat_reached_events_limit_3486f536")?;
                event.remove("json.threatInfo.reachedEventsLimit");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threatInfo.rebootRequired") {
                    if let Some(val) = event.get("json.threatInfo.rebootRequired") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threatInfo.rebootRequired".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat.reboot_required", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_threatInfo_rebootRequired_to_sentinel_one_threat_reboot_required_1c5cb85f")?;
                event.remove("json.threatInfo.rebootRequired");
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

            if event.has_value("json.threatInfo.sha1") {
                event.rename("json.threatInfo.sha1", "threat.indicator.file.hash.sha1")?;
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("threat.indicator.file.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.threatInfo.sha256") {
                event.rename(
                    "json.threatInfo.sha256",
                    "threat.indicator.file.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("threat.indicator.file.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.threatInfo.storyline") {
                event.rename("json.threatInfo.storyline", "sentinel_one.threat.storyline")?;
            }

            if event.has_value("json.threatInfo.threatId") {
                event.rename("json.threatInfo.threatId", "sentinel_one.threat.threat_id")?;
            }

            if event.has_value("json.threatInfo.threatName") {
                event.rename("json.threatInfo.threatName", "sentinel_one.threat.name")?;
            }

            let _cond = {
                event.has_value("sentinel_one.threat.name")
                    && event.has_value("sentinel_one.threat.confidence_level")
            };
            if _cond {
                event.set(
                    "message",
                    json!(format!(
                        "Threat Detected: {} ({})",
                        event
                            .get("sentinel_one.threat.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("sentinel_one.threat.confidence_level")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("json.whiteningOptions") {
                event.rename(
                    "json.whiteningOptions",
                    "sentinel_one.threat.whitening_option",
                )?;
            }

            let _cond = { event.has_value("json.timeline.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timeline.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timeline.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_timeline_created_at",
                    )?;
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

            if event.has_value("json.timeline.activityType") {
                event.rename(
                    "json.timeline.activityType",
                    "sentinel_one.threat.timeline.activity_type",
                )?;
            }

            if event.has_value("json.timeline.primaryDescription") {
                event.rename(
                    "json.timeline.primaryDescription",
                    "sentinel_one.threat.timeline.primary_description",
                )?;
            }

            if event.has_value("json.timeline.secondaryDescription") {
                event.rename(
                    "json.timeline.secondaryDescription",
                    "sentinel_one.threat.timeline.secondary_description",
                )?;
            }

            if event.has_value("json.timeline.id") {
                event.rename("json.timeline.id", "sentinel_one.threat.timeline.id")?;
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.threat.timeline.primary_description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
            }

            if event.has_value("json.timeline.data.ruledescription") {
                event.rename(
                    "json.timeline.data.ruledescription",
                    "sentinel_one.threat.rule_description",
                )?;
            }

            if event.has_value("json.timeline.data.ruleid") {
                event.rename("json.timeline.data.ruleid", "sentinel_one.threat.rule_id")?;
            }

            if event.has_value("json.timeline.data.rulename") {
                event.rename(
                    "json.timeline.data.rulename",
                    "sentinel_one.threat.rule_name",
                )?;
            }

            let _cond = { !event.has_value("sentinel_one.threat.rule_description") };
            if _cond {
                if event.has_value("json.timeline.data.ruleDescription") {
                    event.rename(
                        "json.timeline.data.ruleDescription",
                        "sentinel_one.threat.rule_description",
                    )?;
                }
            }

            let _cond = { !event.has_value("sentinel_one.threat.rule_id") };
            if _cond {
                if event.has_value("json.timeline.data.ruleId") {
                    event.rename("json.timeline.data.ruleId", "sentinel_one.threat.rule_id")?;
                }
            }

            let _cond = { !event.has_value("sentinel_one.threat.rule_name") };
            if _cond {
                if event.has_value("json.timeline.data.ruleName") {
                    event.rename(
                        "json.timeline.data.ruleName",
                        "sentinel_one.threat.rule_name",
                    )?;
                }
            }

            if event.has_value("json.timeline.accountId") {
                event.rename(
                    "json.timeline.accountId",
                    "sentinel_one.threat.timeline.account.id",
                )?;
            }

            if event.has_value("json.timeline.agentId") {
                event.rename(
                    "json.timeline.agentId",
                    "sentinel_one.threat.timeline.agent.id",
                )?;
            }

            if event.has_value("json.timeline.agentUpdatedVersion") {
                event.rename(
                    "json.timeline.agentUpdatedVersion",
                    "sentinel_one.threat.timeline.agent_updated_version",
                )?;
            }

            let _cond = { event.has_value("json.timeline.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timeline.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.timeline.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timeline.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_timeline_created_at_field",
                    )?;
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

            if event.has_value("json.timeline.groupId") {
                event.rename(
                    "json.timeline.groupId",
                    "sentinel_one.threat.timeline.group.id",
                )?;
            }

            let _cond = { !event.has_value("group.id") };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.threat.timeline.group.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
            }

            if event.has_value("json.timeline.hash") {
                event.rename("json.timeline.hash", "sentinel_one.threat.timeline.hash")?;
            }

            let _cond = { event.has_value("sentinel_one.threat.timeline.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one.threat.timeline.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.timeline.osFamily") {
                event.rename(
                    "json.timeline.osFamily",
                    "sentinel_one.threat.timeline.os_family",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.timeline.os_family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            if event.has_value("json.timeline.siteId") {
                event.rename(
                    "json.timeline.siteId",
                    "sentinel_one.threat.timeline.site.id",
                )?;
            }

            let _cond = { event.has_value("json.timeline.updatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timeline.updatedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat.timeline.updated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timeline.updatedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_timeline_updated_at",
                    )?;
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

            if event.has_value("json.timeline.userId") {
                event.rename(
                    "json.timeline.userId",
                    "sentinel_one.threat.timeline.user.id",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.timeline.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
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

            if event.has_value("json.timeline.data") {
                event.rename("json.timeline.data", "sentinel_one.threat.timeline.data")?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.rule_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
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
