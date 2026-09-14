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

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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
                            .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("json.data")
                    && event.get("json.data").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("json.links.self") {
                event.rename("json.links.self", "trellix_epo_cloud.device.links.self")?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.links.self")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.links.self")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reference", v)?;
                }
            }

            if event.has_value("json.attributes.domainName") {
                event.rename(
                    "json.attributes.domainName",
                    "trellix_epo_cloud.device.attributes.domain_name",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.domain_name")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.domain_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.domain", v)?;
                }
            }

            let _cond = { event.has_value("host.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "trellix_epo_cloud.device.id")?;
            }

            let _cond = {
                !(["(none)", "N/A"]
                    .contains(&event.get_str("trellix_epo_cloud.device.id").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
            }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !(["", "(none)", "N/A"]
                    .contains(&event.get_str("json.attributes.ipAddress").unwrap_or("")))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.ipAddress") {
                        if let Some(val) = event.get("json.attributes.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.device.attributes.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ipAddress_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.device.attributes.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.device.attributes.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trellix_epo_cloud.device.attributes.ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.device.attributes.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"]
                    .contains(&event.get_str("json.attributes.macAddress").unwrap_or("")))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.macAddress") {
                        gsub_field(
                            event,
                            "json.attributes.macAddress",
                            "trellix_epo_cloud.device.attributes.mac_address",
                            cached_regex!("(..)(?!$)"),
                            "$1-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_macAddress")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("trellix_epo_cloud.device.attributes.mac_address") {
                map_strings(
                    event,
                    "trellix_epo_cloud.device.attributes.mac_address",
                    "trellix_epo_cloud.device.attributes.mac_address",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.has_value("trellix_epo_cloud.device.attributes.mac_address") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("trellix_epo_cloud.device.attributes.mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attributes.computerName") {
                event.rename(
                    "json.attributes.computerName",
                    "trellix_epo_cloud.device.attributes.computer_name",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.computer_name")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.computer_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attributes.osPlatform") {
                event.rename(
                    "json.attributes.osPlatform",
                    "trellix_epo_cloud.device.attributes.os.platform",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.os.platform")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.os.platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.platform", v)?;
                }
            }

            if event.has_value("json.attributes.osType") {
                event.rename(
                    "json.attributes.osType",
                    "trellix_epo_cloud.device.attributes.os.type",
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("windows"))
            };
            if _cond {
                event.set("host.os.type", json!("windows"))?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("linux"))
            };
            if _cond {
                event.set("host.os.type", json!("linux"))?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("macos"))
            };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("unix"))
            };
            if _cond {
                event.set("host.os.type", json!("unix"))?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("ios"))
            };
            if _cond {
                event.set("host.os.type", json!("ios"))?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.device.attributes.os.type")
                    && !(["(none)", "N/A"].contains(
                        &event
                            .get_str("trellix_epo_cloud.device.attributes.os.type")
                            .unwrap_or(""),
                    ))
                    && event
                        .get_str("trellix_epo_cloud.device.attributes.os.type")
                        .is_some_and(|s| s.to_lowercase().contains("android"))
            };
            if _cond {
                event.set("host.os.type", json!("android"))?;
            }

            if event.has_value("json.attributes.osVersion") {
                event.rename(
                    "json.attributes.osVersion",
                    "trellix_epo_cloud.device.attributes.os.version",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.os.version")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.os.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
            }

            if event.has_value("json.attributes.systemSerialNumber") {
                event.rename(
                    "json.attributes.systemSerialNumber",
                    "trellix_epo_cloud.device.attributes.system.serial_number",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.system.serial_number")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.system.serial_number")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.serial_number", v)?;
                }
            }

            if event.has_value("json.attributes.userName") {
                event.rename(
                    "json.attributes.userName",
                    "trellix_epo_cloud.device.attributes.user_name",
                )?;
            }

            let _cond = {
                !(["(none)", "N/A"].contains(
                    &event
                        .get_str("trellix_epo_cloud.device.attributes.user_name")
                        .unwrap_or(""),
                ))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.device.attributes.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
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

            if event.has_value("json.attributes.agentGuid") {
                event.rename(
                    "json.attributes.agentGuid",
                    "trellix_epo_cloud.device.attributes.agent.guid",
                )?;
            }

            if event.has_value("json.attributes.agentPlatform") {
                event.rename(
                    "json.attributes.agentPlatform",
                    "trellix_epo_cloud.device.attributes.agent.platform",
                )?;
            }

            let _cond = { event.get_str("json.attributes.agentState") != Some("1") };
            if _cond {
                event.set("json.attributes.agentState", json!(true))?;
            }

            let _cond = { event.get_str("json.attributes.agentState") != Some("0") };
            if _cond {
                event.set("json.attributes.agentState", json!(false))?;
            }

            let _cond = { event.get_str("json.attributes.agentState") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.agentState") {
                        if let Some(val) = event.get("json.attributes.agentState") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.agentState".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.device.attributes.agent.state",
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
                        "convert_agentState_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.agentVersion") {
                event.rename(
                    "json.attributes.agentVersion",
                    "trellix_epo_cloud.device.attributes.agent.version",
                )?;
            }

            let _cond = { event.get_str("json.attributes.cpuSpeed") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.cpuSpeed") {
                        if let Some(val) = event.get("json.attributes.cpuSpeed") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.cpuSpeed".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.device.attributes.cpu.speed", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cpuSpeed_to_long",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.cpuType") {
                event.rename(
                    "json.attributes.cpuType",
                    "trellix_epo_cloud.device.attributes.cpu.type",
                )?;
            }

            if event.has_value("json.attributes.excludedTags") {
                event.rename(
                    "json.attributes.excludedTags",
                    "trellix_epo_cloud.device.attributes.excluded_tags",
                )?;
            }

            if event.has_value("json.attributes.ipHostName") {
                event.rename(
                    "json.attributes.ipHostName",
                    "trellix_epo_cloud.device.attributes.ip_host_name",
                )?;
            }

            let _cond = { event.has_value("trellix_epo_cloud.device.attributes.ip_host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trellix_epo_cloud.device.attributes.ip_host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attributes.isPortable") {
                event.rename(
                    "json.attributes.isPortable",
                    "trellix_epo_cloud.device.attributes.is_portable",
                )?;
            }

            let _cond = { event.get_str("json.attributes.lastUpdate") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.lastUpdate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("trellix_epo_cloud.device.attributes.last_update", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.lastUpdate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastUpdate")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.managed") {
                event.rename(
                    "json.attributes.managed",
                    "trellix_epo_cloud.device.attributes.managed",
                )?;
            }

            let _cond = { event.get_str("json.attributes.managedState") != Some("1") };
            if _cond {
                event.set("json.attributes.managedState", json!(true))?;
            }

            let _cond = { event.get_str("json.attributes.managedState") != Some("0") };
            if _cond {
                event.set("json.attributes.managedState", json!(false))?;
            }

            let _cond = { event.get_str("json.attributes.managedState") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.managedState") {
                        if let Some(val) = event.get("json.attributes.managedState") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.managedState".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.device.attributes.managed_state",
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
                        "convert_managedState_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.name") {
                event.rename(
                    "json.attributes.name",
                    "trellix_epo_cloud.device.attributes.name",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.nodeCreatedDate")
                    && event.get_str("json.attributes.nodeCreatedDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.nodeCreatedDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "trellix_epo_cloud.device.attributes.node.created_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.nodeCreatedDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_nodeCreatedDate")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.nodePath") {
                event.rename(
                    "json.attributes.nodePath",
                    "trellix_epo_cloud.device.attributes.node.path",
                )?;
            }

            let _cond = { event.get_str("json.attributes.numOfCpu") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.numOfCpu") {
                        if let Some(val) = event.get("json.attributes.numOfCpu") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.numOfCpu".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.device.attributes.num_of_cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_numOfCpu_to_long",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.osBuildNumber") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.osBuildNumber") {
                        if let Some(val) = event.get("json.attributes.osBuildNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.osBuildNumber".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.device.attributes.os.build_number",
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
                        "convert_osBuildNumber_to_long",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.parentId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.parentId") {
                        if let Some(val) = event.get("json.attributes.parentId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.parentId".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.device.attributes.parent.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_parentId_to_string",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.subnetAddress") {
                event.rename(
                    "json.attributes.subnetAddress",
                    "trellix_epo_cloud.device.attributes.subnet_address",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.systemBootTime")
                    && event.get_str("json.attributes.systemBootTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.systemBootTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "trellix_epo_cloud.device.attributes.system.boot_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.systemBootTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_systemBootTime")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.systemManufacturer") {
                event.rename(
                    "json.attributes.systemManufacturer",
                    "trellix_epo_cloud.device.attributes.system.manufacturer",
                )?;
            }

            if event.has_value("json.attributes.systemModel") {
                event.rename(
                    "json.attributes.systemModel",
                    "trellix_epo_cloud.device.attributes.system.model",
                )?;
            }

            let _cond = { event.get_str("json.attributes.systemRebootPending") != Some("1") };
            if _cond {
                event.set("json.attributes.systemRebootPending", json!(true))?;
            }

            let _cond = { event.get_str("json.attributes.systemRebootPending") != Some("0") };
            if _cond {
                event.set("json.attributes.systemRebootPending", json!(false))?;
            }

            let _cond = { event.get_str("json.attributes.systemRebootPending") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.systemRebootPending") {
                        if let Some(val) = event.get("json.attributes.systemRebootPending") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.systemRebootPending".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.device.attributes.system.reboot_pending",
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
                        "convert_systemRebootPending_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.tags") {
                if let Some(s) = event.get_string("json.attributes.tags") {
                    let mut parts: Vec<Value> = s.split(", ").map(|p| json!(p)).collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "trellix_epo_cloud.device.attributes.tags",
                        Value::Array(parts),
                    )?;
                }
            }

            let _cond = { event.get_str("json.attributes.tenantId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.tenantId") {
                        if let Some(val) = event.get("json.attributes.tenantId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.tenantId".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.device.attributes.tenant.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tenantId_to_string",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.totalPhysicalMemory") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.totalPhysicalMemory") {
                        if let Some(val) = event.get("json.attributes.totalPhysicalMemory") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.totalPhysicalMemory".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.device.attributes.total_physical_memory",
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
                        "convert_totalPhysicalMemory_to_long",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.relationships.devices.data.id") {
                event.rename(
                    "json.relationships.devices.data.id",
                    "trellix_epo_cloud.device.relationships.devices.data.id",
                )?;
            }

            if event.has_value("json.relationships.devices.data.type") {
                event.rename(
                    "json.relationships.devices.data.type",
                    "trellix_epo_cloud.device.relationships.devices.data.type",
                )?;
            }

            if event.has_value("json.relationships.devices.links.related") {
                event.rename(
                    "json.relationships.devices.links.related",
                    "trellix_epo_cloud.device.relationships.devices.links.related",
                )?;
            }

            if event.has_value("json.relationships.devices.links.self") {
                event.rename(
                    "json.relationships.devices.links.self",
                    "trellix_epo_cloud.device.relationships.devices.links.self",
                )?;
            }

            if event.has_value("json.relationships.installedProducts.links.related") {
                event.rename(
                    "json.relationships.installedProducts.links.related",
                    "trellix_epo_cloud.device.relationships.installed_products.links.related",
                )?;
            }

            if event.has_value("json.relationships.installedProducts.links.self") {
                event.rename(
                    "json.relationships.installedProducts.links.self",
                    "trellix_epo_cloud.device.relationships.installed_products.links.self",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "trellix_epo_cloud.type")?;
            }

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
                event.remove("trellix_epo_cloud.device.links.self");
                event.remove("trellix_epo_cloud.device.attributes.domain_name");
                event.remove("trellix_epo_cloud.device.id");
                event.remove("trellix_epo_cloud.device.attributes.ip_address");
                event.remove("trellix_epo_cloud.device.attributes.mac_address");
                event.remove("trellix_epo_cloud.device.attributes.computer_name");
                event.remove("trellix_epo_cloud.device.attributes.os.platform");
                event.remove("trellix_epo_cloud.device.attributes.os.type");
                event.remove("trellix_epo_cloud.device.attributes.os.version");
                event.remove("trellix_epo_cloud.device.attributes.system.serial_number");
                event.remove("trellix_epo_cloud.device.attributes.user_name");
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
