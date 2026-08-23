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

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("malware")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.alertInfo.createdAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.alertInfo.updatedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.alertInfo.alertId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has("json.agentDetectionInfo.machineType") {
                event.rename("json.agentDetectionInfo.machineType", "host.type")?;
            }

            if event.has("json.agentDetectionInfo.name") {
                event.rename("json.agentDetectionInfo.name", "host.name")?;
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

            if event.has("json.agentDetectionInfo.osFamily") {
                event.rename("json.agentDetectionInfo.osFamily", "host.os.family")?;
            }

            if event.has("json.agentDetectionInfo.osRevision") {
                event.rename("json.agentDetectionInfo.osRevision", "host.os.version")?;
            }

            if event.has("json.agentDetectionInfo.siteId") {
                event.rename("json.agentDetectionInfo.siteId", "sentinel_one.site.id")?;
            }

            if event.has("json.agentDetectionInfo.uuid") {
                event.rename("json.agentDetectionInfo.uuid", "observer.serial_number")?;
            }

            if event.has("json.agentDetectionInfo.osName") {
                event.rename("json.agentDetectionInfo.osName", "host.os.name")?;
            }

            if event.has("json.agentDetectionInfo.version") {
                event.rename("json.agentDetectionInfo.version", "observer.version")?;
            }

            if event.has("json.agentRealtimeInfo.id") {
                event.rename("json.agentRealtimeInfo.id", "sentinel_one.alert.agent.id")?;
            }

            let _cond = { event.has_value("sentinel_one.alert.agent.id") };
            if _cond {
                if let Some(v) = event.get("sentinel_one.alert.agent.id").cloned() {
                    event.set("host.id", v)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentRealtimeInfo.infected") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.infected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.infected".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.alert.agent.infected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_infected_to_sentinel_one_alert_agent_infected_0737d0ab")?;
                event.remove("json.agentRealtimeInfo.infected");
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
                if event.has_value("json.agentRealtimeInfo.isActive") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.isActive") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.isActive".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.alert.agent.is_active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_isActive_to_sentinel_one_alert_agent_is_active_8d5d2602")?;
                event.remove("json.agentRealtimeInfo.isActive");
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
                if event.has_value("json.agentRealtimeInfo.isDecommissioned") {
                    if let Some(val) = event.get("json.agentRealtimeInfo.isDecommissioned") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentRealtimeInfo.isDecommissioned".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.alert.agent.is_decommissioned", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_agentRealtimeInfo_isDecommissioned_to_sentinel_one_alert_agent_is_decommissioned_464efbad")?;
                event.remove("json.agentRealtimeInfo.isDecommissioned");
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

            if event.has("json.agentRealtimeInfo.machineType") {
                event.rename(
                    "json.agentRealtimeInfo.machineType",
                    "sentinel_one.alert.agent.machine_type",
                )?;
            }

            if event.has("json.agentRealtimeInfo.name") {
                event.rename(
                    "json.agentRealtimeInfo.name",
                    "sentinel_one.alert.agent.computer_name",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.alert.agent.computer_name") };
            if _cond {
                if let Some(v) = event.get("sentinel_one.alert.agent.computer_name").cloned() {
                    event.set("host.name", v)?;
                }
            }

            if event.has("json.agentRealtimeInfo.os") {
                event.rename(
                    "json.agentRealtimeInfo.os",
                    "sentinel_one.alert.agent.os.type",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.alert.agent.os.type") };
            if _cond {
                // Painless script
                // Source: ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString os_type = ctx.sentinel_one.alert.agent.os.type.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString os_type = ctx.sentinel_one.alert.agent.os.type.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
            }

            let _cond = { event.has_value("json.alertInfo.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alertInfo.createdAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_alertInfo_createdAt_c1f4ab8d",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alertInfo.srcIp") {
                    if let Some(val) = event.get("json.alertInfo.srcIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alertInfo.srcIp".into(),
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
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_alertInfo_srcIp_to_source_ip_ed3052c4",
                )?;
                event.remove("json.alertInfo.srcIp");
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.alertInfo.incidentStatus") {
                event.rename(
                    "json.alertInfo.incidentStatus",
                    "sentinel_one.alert.info.status",
                )?;
            }

            if event.has("json.alertInfo.registryOldValue") {
                event.rename(
                    "json.alertInfo.registryOldValue",
                    "sentinel_one.alert.info.registry.old_value",
                )?;
            }

            if event.has("json.alertInfo.alertId") {
                event.rename("json.alertInfo.alertId", "event.id")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alertInfo.dstPort") {
                    if let Some(val) = event.get("json.alertInfo.dstPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alertInfo.dstPort".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_alertInfo_dstPort_to_destination_port_77ede9c4",
                )?;
                event.remove("json.alertInfo.dstPort");
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

            if event.has("json.alertInfo.indicatorName") {
                event.rename(
                    "json.alertInfo.indicatorName",
                    "sentinel_one.alert.info.indicator.name",
                )?;
            }

            if event.has("json.alertInfo.registryPath") {
                event.rename("json.alertInfo.registryPath", "registry.path")?;
            }

            if event.has("json.alertInfo.loginType") {
                event.rename(
                    "json.alertInfo.loginType",
                    "sentinel_one.alert.info.login.type",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alertInfo.dstIp") {
                    if let Some(val) = event.get("json.alertInfo.dstIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alertInfo.dstIp".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_alertInfo_dstIp_to_destination_ip_3d288503",
                )?;
                event.remove("json.alertInfo.dstIp");
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.alertInfo.updatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alertInfo.updatedAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.alert.info.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_alertInfo_updatedAt_to_sentinel_one_alert_info_updated_at_d3017bdb")?;
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

            if event.has("json.alertInfo.indicatorDescription") {
                event.rename(
                    "json.alertInfo.indicatorDescription",
                    "sentinel_one.alert.info.indicator.description",
                )?;
            }

            if event.has("json.alertInfo.loginsUserName") {
                event.rename("json.alertInfo.loginsUserName", "user.name")?;
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

            if event.has("json.alertInfo.loginIsSuccessful") {
                event.rename(
                    "json.alertInfo.loginIsSuccessful",
                    "sentinel_one.alert.info.login.is_successful",
                )?;
            }

            if event.has("json.alertInfo.indicatorCategory") {
                event.rename(
                    "json.alertInfo.indicatorCategory",
                    "sentinel_one.alert.info.indicator.category",
                )?;
            }

            if event.has("json.alertInfo.modulePath") {
                event.rename("json.alertInfo.modulePath", "dll.path")?;
            }

            if event.has("json.alertInfo.loginAccountSid") {
                event.rename(
                    "json.alertInfo.loginAccountSid",
                    "sentinel_one.alert.info.login.account.sid",
                )?;
            }

            if event.has("json.alertInfo.dnsResponse") {
                event.rename(
                    "json.alertInfo.dnsResponse",
                    "sentinel_one.alert.info.dns.response",
                )?;
            }

            let _cond = {
                event.has_value("json.alertInfo.netEventDirection")
                    && [
                        "ingress", "egress", "inbound", "outbound", "internal", "external",
                        "unknown",
                    ]
                    .contains(
                        &event
                            .get_str("json.alertInfo.netEventDirection")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                if event.has("json.alertInfo.netEventDirection") {
                    event.rename("json.alertInfo.netEventDirection", "network.direction")?;
                }
            }

            if event.has("json.alertInfo.registryValue") {
                event.rename("json.alertInfo.registryValue", "registry.value")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alertInfo.srcMachineIp") {
                    if let Some(val) = event.get("json.alertInfo.srcMachineIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alertInfo.srcMachineIp".into(),
                                message,
                            }
                        })?;
                        event.set("json.alertInfo.srcMachineIp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_alertInfo_srcMachineIp_83a5b96a",
                )?;
                event.remove("json.alertInfo.srcMachineIp");
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

            let _cond = { event.has_value("json.alertInfo.srcMachineIp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("json.alertInfo.srcMachineIp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.alertInfo.srcMachineIp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.alertInfo.srcMachineIp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.alertInfo.registryOldValueType") {
                event.rename(
                    "json.alertInfo.registryOldValueType",
                    "sentinel_one.alert.info.registry.old_value_type",
                )?;
            }

            if event.has("json.alertInfo.eventType") {
                event.rename(
                    "json.alertInfo.eventType",
                    "sentinel_one.alert.info.event_type",
                )?;
            }

            if event.has("json.alertInfo.analystVerdict") {
                event.rename(
                    "json.alertInfo.analystVerdict",
                    "sentinel_one.alert.analyst_verdict",
                )?;
            }

            if event.has("json.alertInfo.dvEventId") {
                event.rename("json.alertInfo.dvEventId", "sentinel_one.alert.dv_event.id")?;
            }

            if event.has("json.alertInfo.dnsRequest") {
                event.rename("json.alertInfo.dnsRequest", "dns.question.name")?;
            }

            if event.has("json.alertInfo.loginIsAdministratorEquivalent") {
                event.rename(
                    "json.alertInfo.loginIsAdministratorEquivalent",
                    "sentinel_one.alert.info.login.is_administrator",
                )?;
            }

            if event.has("json.alertInfo.loginAccountDomain") {
                event.rename("json.alertInfo.loginAccountDomain", "user.domain")?;
            }

            if event.has("json.alertInfo.tiIndicatorType") {
                event.rename(
                    "json.alertInfo.tiIndicatorType",
                    "sentinel_one.alert.info.ti_indicator.type",
                )?;
            }

            if event.has("json.alertInfo.moduleSha1") {
                event.rename("json.alertInfo.moduleSha1", "dll.hash.sha1")?;
            }

            let _cond = { event.has_value("dll.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("dll.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.alertInfo.source") {
                event.rename("json.alertInfo.source", "sentinel_one.alert.info.source")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alertInfo.srcPort") {
                    if let Some(val) = event.get("json.alertInfo.srcPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alertInfo.srcPort".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_alertInfo_srcPort_to_source_port_9411029b",
                )?;
                event.remove("json.alertInfo.srcPort");
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

            if event.has("json.alertInfo.tiIndicatorValue") {
                event.rename(
                    "json.alertInfo.tiIndicatorValue",
                    "sentinel_one.alert.info.ti_indicator.value",
                )?;
            }

            if event.has("json.alertInfo.tiIndicatorSource") {
                event.rename(
                    "json.alertInfo.tiIndicatorSource",
                    "sentinel_one.alert.info.ti_indicator.source",
                )?;
            }

            let _cond = { event.has_value("json.alertInfo.reportedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alertInfo.reportedAt") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.alert.info.reported_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_alertInfo_reportedAt_to_sentinel_one_alert_info_reported_at_f02885b7")?;
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

            if event.has("json.alertInfo.registryKeyPath") {
                event.rename("json.alertInfo.registryKeyPath", "registry.key")?;
            }

            if event.has("json.alertInfo.tiIndicatorComparisonMethod") {
                event.rename(
                    "json.alertInfo.tiIndicatorComparisonMethod",
                    "sentinel_one.alert.info.ti_indicator.comparison_method",
                )?;
            }

            if event.has("json.alertInfo.hitType") {
                event.rename("json.alertInfo.hitType", "sentinel_one.alert.info.hit.type")?;
            }

            if event.has("json.containerInfo.id") {
                event.rename("json.containerInfo.id", "container.id")?;
            }

            if event.has("json.containerInfo.image") {
                event.rename("json.containerInfo.image", "container.image.name")?;
            }

            if event.has("json.containerInfo.labels") {
                event.rename(
                    "json.containerInfo.labels",
                    "sentinel_one.alert.container.info.labels",
                )?;
            }

            let _cond = { !event.has_value("container.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("json.containerInfo.name") {
                        event.rename("json.containerInfo.name", "container.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("orchestrator.cluster.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("json.kubernetesInfo.cluster") {
                        event.rename("json.kubernetesInfo.cluster", "orchestrator.cluster.name")?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.kubernetesInfo.controllerKind") {
                event.rename(
                    "json.kubernetesInfo.controllerKind",
                    "sentinel_one.alert.kubernetes.controller.kind",
                )?;
            }

            if event.has("json.kubernetesInfo.controllerLabels") {
                event.rename(
                    "json.kubernetesInfo.controllerLabels",
                    "sentinel_one.alert.kubernetes.controller.labels",
                )?;
            }

            if event.has("json.kubernetesInfo.controllerName") {
                event.rename(
                    "json.kubernetesInfo.controllerName",
                    "sentinel_one.alert.kubernetes.controller.name",
                )?;
            }

            let _cond = { !event.has_value("orchestrator.namespace") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("json.kubernetesInfo.namespace") {
                        event.rename("json.kubernetesInfo.namespace", "orchestrator.namespace")?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.kubernetesInfo.namespaceLabels") {
                event.rename(
                    "json.kubernetesInfo.namespaceLabels",
                    "sentinel_one.alert.kubernetes.namespace.labels",
                )?;
            }

            if event.has("json.kubernetesInfo.node") {
                event.rename(
                    "json.kubernetesInfo.node",
                    "sentinel_one.alert.kubernetes.node",
                )?;
            }

            if event.has("json.kubernetesInfo.pod") {
                event.rename(
                    "json.kubernetesInfo.pod",
                    "sentinel_one.alert.kubernetes.pod.name",
                )?;
            }

            if event.has("json.kubernetesInfo.podLabels") {
                event.rename(
                    "json.kubernetesInfo.podLabels",
                    "sentinel_one.alert.kubernetes.pod.labels",
                )?;
            }

            if event.has("json.osName") {
                event.rename("json.osName", "observer.os.name")?;
            }

            if event.has("json.ruleInfo.type") {
                event.rename("json.ruleInfo.type", "rule.category")?;
            }

            if event.has("json.ruleInfo.description") {
                event.rename("json.ruleInfo.description", "rule.description")?;
            }

            if event.has("json.ruleInfo.id") {
                event.rename("json.ruleInfo.id", "rule.id")?;
            }

            if event.has("json.ruleInfo.name") {
                event.rename("json.ruleInfo.name", "rule.name")?;
            }

            let _cond = { event.has_value("rule.name") };
            if _cond {
                if let Some(v) = event.get("rule.name").cloned() {
                    event.set("message", v)?;
                }
            }

            if event.has("json.ruleInfo.scopeLevel") {
                event.rename(
                    "json.ruleInfo.scopeLevel",
                    "sentinel_one.alert.rule.scope_level",
                )?;
            }

            if event.has("json.ruleInfo.severity") {
                event.rename("json.ruleInfo.severity", "sentinel_one.alert.rule.severity")?;
            }

            let _cond = {
                event
                    .get("sentinel_one.alert.rule.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.sentinel_one.alert.rule.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.sentinel_one.alert.rule.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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

            if event.has("json.ruleInfo.treatAsThreat") {
                event.rename(
                    "json.ruleInfo.treatAsThreat",
                    "sentinel_one.alert.rule.treat_as_threat",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.commandline") {
                event.rename(
                    "json.sourceParentProcessInfo.commandline",
                    "process.parent.command_line",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.fileHashMd5") {
                event.rename(
                    "json.sourceParentProcessInfo.fileHashMd5",
                    "process.parent.hash.md5",
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceParentProcessInfo.fileHashSha1") {
                event.rename(
                    "json.sourceParentProcessInfo.fileHashSha1",
                    "process.parent.hash.sha1",
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceParentProcessInfo.fileHashSha256") {
                event.rename(
                    "json.sourceParentProcessInfo.fileHashSha256",
                    "process.parent.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceParentProcessInfo.filePath") {
                event.rename(
                    "json.sourceParentProcessInfo.filePath",
                    "process.parent.executable",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.fileSignerIdentity") {
                event.rename(
                    "json.sourceParentProcessInfo.fileSignerIdentity",
                    "process.parent.code_signature.signing_id",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.integrityLevel") {
                event.rename(
                    "json.sourceParentProcessInfo.integrityLevel",
                    "sentinel_one.alert.process.parent.integrity_level",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.name") {
                event.rename("json.sourceParentProcessInfo.name", "process.parent.name")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sourceParentProcessInfo.pid") {
                    if let Some(val) = event.get("json.sourceParentProcessInfo.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sourceParentProcessInfo.pid".into(),
                                message,
                            }
                        })?;
                        event.set("process.parent.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_sourceParentProcessInfo_pid_to_process_parent_pid_fb5c17d9",
                )?;
                event.remove("json.sourceParentProcessInfo.pid");
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

            let _cond = { event.has_value("json.sourceParentProcessInfo.pidStarttime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.sourceParentProcessInfo.pidStarttime")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("process.parent.start", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_sourceParentProcessInfo_pidStarttime_to_process_parent_start_2ababe18")?;
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

            if event.has("json.sourceParentProcessInfo.storyline") {
                event.rename(
                    "json.sourceParentProcessInfo.storyline",
                    "sentinel_one.alert.process.parent.storyline",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.subsystem") {
                event.rename(
                    "json.sourceParentProcessInfo.subsystem",
                    "sentinel_one.alert.process.parent.subsystem",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.uniqueId") {
                event.rename(
                    "json.sourceParentProcessInfo.uniqueId",
                    "process.parent.entity_id",
                )?;
            }

            if event.has("json.sourceParentProcessInfo.user") {
                event.rename(
                    "json.sourceParentProcessInfo.user",
                    "process.parent.user.name",
                )?;
            }

            if event.has("json.sourceProcessInfo.commandline") {
                event.rename("json.sourceProcessInfo.commandline", "process.command_line")?;
            }

            if event.has("json.sourceProcessInfo.fileHashMd5") {
                event.rename("json.sourceProcessInfo.fileHashMd5", "process.hash.md5")?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceProcessInfo.fileHashSha1") {
                event.rename("json.sourceProcessInfo.fileHashSha1", "process.hash.sha1")?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceProcessInfo.fileHashSha256") {
                event.rename(
                    "json.sourceProcessInfo.fileHashSha256",
                    "process.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.sourceProcessInfo.filePath") {
                event.rename("json.sourceProcessInfo.filePath", "process.executable")?;
            }

            if event.has("json.sourceProcessInfo.fileSignerIdentity") {
                event.rename(
                    "json.sourceProcessInfo.fileSignerIdentity",
                    "process.code_signature.signing_id",
                )?;
            }

            if event.has("json.sourceProcessInfo.integrityLevel") {
                event.rename(
                    "json.sourceProcessInfo.integrityLevel",
                    "sentinel_one.alert.process.integrity_level",
                )?;
            }

            if event.has("json.sourceProcessInfo.name") {
                event.rename("json.sourceProcessInfo.name", "process.name")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sourceProcessInfo.pid") {
                    if let Some(val) = event.get("json.sourceProcessInfo.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sourceProcessInfo.pid".into(),
                                message,
                            }
                        })?;
                        event.set("process.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_sourceProcessInfo_pid_to_process_pid_2e887a3b",
                )?;
                event.remove("json.sourceProcessInfo.pid");
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

            let _cond = { event.has_value("json.sourceProcessInfo.pidStarttime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.sourceProcessInfo.pidStarttime")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("process.start", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_sourceProcessInfo_pidStarttime_to_process_start_c53c3d9a",
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

            if event.has("json.sourceProcessInfo.storyline") {
                event.rename(
                    "json.sourceProcessInfo.storyline",
                    "sentinel_one.alert.process.storyline",
                )?;
            }

            if event.has("json.sourceProcessInfo.subsystem") {
                event.rename(
                    "json.sourceProcessInfo.subsystem",
                    "sentinel_one.alert.process.subsystem",
                )?;
            }

            if event.has("json.sourceProcessInfo.uniqueId") {
                event.rename("json.sourceProcessInfo.uniqueId", "process.entity_id")?;
            }

            if event.has("json.sourceProcessInfo.user") {
                event.rename("json.sourceProcessInfo.user", "process.user.name")?;
            }

            let _cond = { event.has_value("json.targetProcessInfo.tgtFileCreatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.targetProcessInfo.tgtFileCreatedAt")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("file.created", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_targetProcessInfo_tgtFileCreatedAt_to_file_created_443ebea1",
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

            if event.has("json.targetProcessInfo.tgtFileIsSigned") {
                event.rename(
                    "json.targetProcessInfo.tgtFileIsSigned",
                    "sentinel_one.alert.target.process.file.is_signed",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtFileOldPath") {
                event.rename(
                    "json.targetProcessInfo.tgtFileOldPath",
                    "sentinel_one.alert.target.process.file.old_path",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtProcImagePath") {
                event.rename(
                    "json.targetProcessInfo.tgtProcImagePath",
                    "sentinel_one.alert.target.process.proc.image_path",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtProcSignedStatus") {
                event.rename(
                    "json.targetProcessInfo.tgtProcSignedStatus",
                    "sentinel_one.alert.target.process.proc.signed_status",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtFileHashSha256") {
                event.rename(
                    "json.targetProcessInfo.tgtFileHashSha256",
                    "sentinel_one.alert.target.process.file.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.alert.target.process.file.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("sentinel_one.alert.target.process.file.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.targetProcessInfo.tgtProcStorylineId") {
                event.rename(
                    "json.targetProcessInfo.tgtProcStorylineId",
                    "sentinel_one.alert.target.process.proc.storyline_id",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.targetProcessInfo.tgtProcPid") {
                    if let Some(val) = event.get("json.targetProcessInfo.tgtProcPid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.targetProcessInfo.tgtProcPid".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.alert.target.process.proc.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_targetProcessInfo_tgtProcPid_to_sentinel_one_alert_target_process_proc_pid_169aa7c5")?;
                event.remove("json.targetProcessInfo.tgtProcPid");
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

            if event.has("json.targetProcessInfo.tgtProcCmdLine") {
                event.rename(
                    "json.targetProcessInfo.tgtProcCmdLine",
                    "sentinel_one.alert.target.process.proc.cmdline",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtProcName") {
                event.rename(
                    "json.targetProcessInfo.tgtProcName",
                    "sentinel_one.alert.target.process.proc.name",
                )?;
            }

            let _cond = { event.has_value("json.targetProcessInfo.tgtFileModifiedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.targetProcessInfo.tgtFileModifiedAt")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("file.mtime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_targetProcessInfo_tgtFileModifiedAt_to_file_mtime_009addf7",
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

            if event.has("json.targetProcessInfo.tgtFileId") {
                event.rename(
                    "json.targetProcessInfo.tgtFileId",
                    "sentinel_one.alert.target.process.file.id",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtProcIntegrityLevel") {
                event.rename(
                    "json.targetProcessInfo.tgtProcIntegrityLevel",
                    "sentinel_one.alert.target.process.proc.integrity_level",
                )?;
            }

            if event.has("json.targetProcessInfo.tgtFileHashSha1") {
                event.rename(
                    "json.targetProcessInfo.tgtFileHashSha1",
                    "sentinel_one.alert.target.process.file.hash.sha1",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.alert.target.process.file.hash.sha1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("sentinel_one.alert.target.process.file.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.targetProcessInfo.tgtProcUid") {
                event.rename(
                    "json.targetProcessInfo.tgtProcUid",
                    "sentinel_one.alert.target.process.proc.uid",
                )?;
            }

            let _cond = { event.has_value("json.targetProcessInfo.tgtProcessStartTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.targetProcessInfo.tgtProcessStartTime")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("sentinel_one.alert.target.process.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_targetProcessInfo_tgtProcessStartTime_to_sentinel_one_alert_target_process_start_time_37556602")?;
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

            if event.has("json.targetProcessInfo.tgtFilePath") {
                event.rename(
                    "json.targetProcessInfo.tgtFilePath",
                    "sentinel_one.alert.target.process.file.path",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.alert.target.process.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if event.remove("json").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json".into(),
                });
            }

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
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
