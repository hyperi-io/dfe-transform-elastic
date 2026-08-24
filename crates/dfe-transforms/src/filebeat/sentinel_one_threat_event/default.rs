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
            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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

            event.set("event.kind", json!("event"))?;

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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.createdAt") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has("json.id") {
                event.rename("json.id", "sentinel_one.threat_event.id")?;
            }

            if event.has("json.objectType") {
                event.rename("json.objectType", "sentinel_one.threat_event.object_type")?;
            }

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat_event.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdAt")?;
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

            if event.has("json.processName") {
                event.rename("json.processName", "sentinel_one.threat_event.process.name")?;
            }

            if event.has("json.agentName") {
                event.rename("json.agentName", "sentinel_one.threat_event.agent.name")?;
            }

            if event.has("json.agentGroupId") {
                event.rename(
                    "json.agentGroupId",
                    "sentinel_one.threat_event.agent.group_id",
                )?;
            }

            if event.has("json.agentId") {
                event.rename("json.agentId", "sentinel_one.threat_event.agent.id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentIsActive") {
                    if let Some(val) = event.get("json.agentIsActive") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentIsActive".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.agent.is_active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_agentIsActive_to_boolean",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentIsDecommissioned") {
                    if let Some(val) = event.get("json.agentIsDecommissioned") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentIsDecommissioned".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sentinel_one.threat_event.agent.is_decommissioned",
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
                    "convert_agentIsDecommissioned_to_boolean",
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

            if event.has("json.agentMachineType") {
                event.rename(
                    "json.agentMachineType",
                    "sentinel_one.threat_event.agent.machine_type",
                )?;
            }

            if event.has("json.agentNetworkStatus") {
                event.rename(
                    "json.agentNetworkStatus",
                    "sentinel_one.threat_event.agent.network_status",
                )?;
            }

            if event.has("json.agentOs") {
                event.rename("json.agentOs", "sentinel_one.threat_event.agent.os")?;
            }

            if event.has("json.agentVersion") {
                event.rename(
                    "json.agentVersion",
                    "sentinel_one.threat_event.agent.version",
                )?;
            }

            if event.has("json.agentUuid") {
                event.rename("json.agentUuid", "sentinel_one.threat_event.agent.uuid")?;
            }

            if event.has("json.siteId") {
                event.rename("json.siteId", "sentinel_one.site.id")?;
            }

            if event.has("json.siteName") {
                event.rename("json.siteName", "sentinel_one.site.name")?;
            }

            if event.has("json.pid") {
                event.rename("json.pid", "sentinel_one.threat_event.pid")?;
            }

            let _cond = { event.get_str("json.srcIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.srcIp") {
                        if let Some(val) = event.get("json.srcIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.srcIp".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one.threat_event.src.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_srcIp_to_ip")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.srcPort") {
                    if let Some(val) = event.get("json.srcPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.srcPort".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.src.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_srcPort_to_long",
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

            let _cond = { event.get_str("json.dstIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dstIp") {
                        if let Some(val) = event.get("json.dstIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dstIp".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one.threat_event.dst.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dstIp_to_ip")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.dstPort") {
                    if let Some(val) = event.get("json.dstPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.dstPort".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.dst.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_dstPort_to_long",
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

            if event.has("json.fileSha1") {
                event.rename("json.fileSha1", "sentinel_one.threat_event.file.sha1")?;
            }

            if event.has("json.fileSha256") {
                event.rename("json.fileSha256", "sentinel_one.threat_event.file.sha256")?;
            }

            if event.has("json.fileMd5") {
                event.rename("json.fileMd5", "sentinel_one.threat_event.file.md5")?;
            }

            if event.has("json.oldFileSha1") {
                event.rename(
                    "json.oldFileSha1",
                    "sentinel_one.threat_event.old_file.sha1",
                )?;
            }

            if event.has("json.oldFileSha256") {
                event.rename(
                    "json.oldFileSha256",
                    "sentinel_one.threat_event.old_file.sha256",
                )?;
            }

            if event.has("json.oldFileMd5") {
                event.rename("json.oldFileMd5", "sentinel_one.threat_event.old_file.md5")?;
            }

            if event.has("json.signatureSignedInvalidReason") {
                event.rename(
                    "json.signatureSignedInvalidReason",
                    "sentinel_one.threat_event.signature_signed_invalid_reason",
                )?;
            }

            if event.has("json.verifiedStatus") {
                event.rename(
                    "json.verifiedStatus",
                    "sentinel_one.threat_event.verified_status",
                )?;
            }

            if event.has("json.signedStatus") {
                event.rename(
                    "json.signedStatus",
                    "sentinel_one.threat_event.signed_status",
                )?;
            }

            if event.has("json.sha256") {
                event.rename("json.sha256", "sentinel_one.threat_event.sha256")?;
            }

            if event.has("json.sha1") {
                event.rename("json.sha1", "sentinel_one.threat_event.sha1")?;
            }

            if event.has("json.md5") {
                event.rename("json.md5", "sentinel_one.threat_event.md5")?;
            }

            if event.has("json.fileFullName") {
                event.rename(
                    "json.fileFullName",
                    "sentinel_one.threat_event.file.full_name",
                )?;
            }

            if event.has("json.oldFileName") {
                event.rename(
                    "json.oldFileName",
                    "sentinel_one.threat_event.old_file.name",
                )?;
            }

            if event.has("json.tid") {
                event.rename("json.tid", "sentinel_one.threat_event.tid")?;
            }

            if event.has("json.rpid") {
                event.rename("json.rpid", "sentinel_one.threat_event.rpid")?;
            }

            if event.has("json.dnsRequest") {
                event.rename("json.dnsRequest", "sentinel_one.threat_event.dns_request")?;
            }

            if event.has("json.dnsResponse") {
                event.rename("json.dnsResponse", "sentinel_one.threat_event.dns_response")?;
            }

            if event.has("json.processCmd") {
                event.rename("json.processCmd", "sentinel_one.threat_event.process.cmd")?;
            }

            if event.has("json.processGroupId") {
                event.rename(
                    "json.processGroupId",
                    "sentinel_one.threat_event.process.group_id",
                )?;
            }

            if event.has("json.processImagePath") {
                event.rename(
                    "json.processImagePath",
                    "sentinel_one.threat_event.process.image_path",
                )?;
            }

            if event.has("json.processUserName") {
                event.rename(
                    "json.processUserName",
                    "sentinel_one.threat_event.process.user_name",
                )?;
            }

            if event.has("json.processImageSha1Hash") {
                event.rename(
                    "json.processImageSha1Hash",
                    "sentinel_one.threat_event.process.image_sha1_hash",
                )?;
            }

            if event.has("json.processUniqueKey") {
                event.rename(
                    "json.processUniqueKey",
                    "sentinel_one.threat_event.process.unique_key",
                )?;
            }

            let _cond = {
                event.has_value("json.processStartTime")
                    && event.get_str("json.processStartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.processStartTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one.threat_event.process.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.processStartTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_processStartTime")?;
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

            if event.has("json.processSubSystem") {
                event.rename(
                    "json.processSubSystem",
                    "sentinel_one.threat_event.process.sub_system",
                )?;
            }

            if event.has("json.processSessionId") {
                event.rename(
                    "json.processSessionId",
                    "sentinel_one.threat_event.process.session_id",
                )?;
            }

            if event.has("json.processIntegrityLevel") {
                event.rename(
                    "json.processIntegrityLevel",
                    "sentinel_one.threat_event.process.integrity_level",
                )?;
            }

            if event.has("json.processDisplayName") {
                event.rename(
                    "json.processDisplayName",
                    "sentinel_one.threat_event.process.display_name",
                )?;
            }

            if event.has("json.processIsWow64") {
                event.rename(
                    "json.processIsWow64",
                    "sentinel_one.threat_event.process.is_wow64",
                )?;
            }

            if event.has("json.processIsRedirectedCommandProcessor") {
                event.rename(
                    "json.processIsRedirectedCommandProcessor",
                    "sentinel_one.threat_event.process.is_redirected_command_processor",
                )?;
            }

            if event.has("json.processRoot") {
                event.rename("json.processRoot", "sentinel_one.threat_event.process.root")?;
            }

            if event.has("json.parentProcessName") {
                event.rename(
                    "json.parentProcessName",
                    "sentinel_one.threat_event.parent_process.name",
                )?;
            }

            if event.has("json.parentPid") {
                event.rename("json.parentPid", "sentinel_one.threat_event.parent_pid")?;
            }

            if event.has("json.parentProcessUniqueKey") {
                event.rename(
                    "json.parentProcessUniqueKey",
                    "sentinel_one.threat_event.parent_process.unique_key",
                )?;
            }

            if event.has("json.networkSource") {
                event.rename(
                    "json.networkSource",
                    "sentinel_one.threat_event.network.source",
                )?;
            }

            if event.has("json.networkUrl") {
                event.rename("json.networkUrl", "sentinel_one.threat_event.network.url")?;
            }

            if event.has("json.networkMethod") {
                event.rename(
                    "json.networkMethod",
                    "sentinel_one.threat_event.network.method",
                )?;
            }

            if event.has("json.direction") {
                event.rename("json.direction", "sentinel_one.threat_event.direction")?;
            }

            if event.has("json.eventType") {
                event.rename("json.eventType", "sentinel_one.threat_event.event_type")?;
            }

            if event.has("json.registryPath") {
                event.rename(
                    "json.registryPath",
                    "sentinel_one.threat_event.registry.path",
                )?;
            }

            if event.has("json.registryId") {
                event.rename("json.registryId", "sentinel_one.threat_event.registry.id")?;
            }

            if event.has("json.registryClassification") {
                event.rename(
                    "json.registryClassification",
                    "sentinel_one.threat_event.registry.classification",
                )?;
            }

            if event.has("json.taskName") {
                event.rename("json.taskName", "sentinel_one.threat_event.task_name")?;
            }

            if event.has("json.taskPath") {
                event.rename("json.taskPath", "sentinel_one.threat_event.task_path")?;
            }

            if event.has("json.trueContext") {
                event.rename("json.trueContext", "sentinel_one.threat_event.true_context")?;
            }

            if event.has("json.storyline") {
                event.rename("json.storyline", "sentinel_one.threat_event.storyline")?;
            }

            if event.has("json.fileId") {
                event.rename("json.fileId", "sentinel_one.threat_event.file.id")?;
            }

            if event.has("json.loginsUserName") {
                event.rename(
                    "json.loginsUserName",
                    "sentinel_one.threat_event.logins_user_name",
                )?;
            }

            let _cond = { event.has_value("sentinel_one.threat_event.logins_user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one.threat_event.logins_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.loginsBaseType") {
                event.rename(
                    "json.loginsBaseType",
                    "sentinel_one.threat_event.logins_base_type",
                )?;
            }

            if event.has("json.indicatorCategory") {
                event.rename(
                    "json.indicatorCategory",
                    "sentinel_one.threat_event.indicator.category",
                )?;
            }

            if event.has("json.indicatorDescription") {
                event.rename(
                    "json.indicatorDescription",
                    "sentinel_one.threat_event.indicator.description",
                )?;
            }

            if event.has("json.indicatorMetadata") {
                event.rename(
                    "json.indicatorMetadata",
                    "sentinel_one.threat_event.indicator.metadata",
                )?;
            }

            if event.has("json.indicatorName") {
                event.rename(
                    "json.indicatorName",
                    "sentinel_one.threat_event.indicator.name",
                )?;
            }

            if event.has("json.connectionStatus") {
                event.rename(
                    "json.connectionStatus",
                    "sentinel_one.threat_event.connection_status",
                )?;
            }

            if event.has("json.publisher") {
                event.rename("json.publisher", "sentinel_one.threat_event.publisher")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentInfected") {
                    if let Some(val) = event.get("json.agentInfected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentInfected".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.agent.infected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_agentInfected_to_boolean",
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

            if event.has("json.agentDomain") {
                event.rename("json.agentDomain", "sentinel_one.threat_event.agent.domain")?;
            }

            let _cond = { event.get_str("json.agentIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.agentIp") {
                        if let Some(val) = event.get("json.agentIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.agentIp".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one.threat_event.agent.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_agentIp_to_ip")?;
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

            let _cond = { event.has_value("sentinel_one.threat_event.agent.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sentinel_one.threat_event.agent.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.user") {
                event.rename("json.user", "sentinel_one.threat_event.user")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.relatedToThreat") {
                    if let Some(val) = event.get("json.relatedToThreat") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.relatedToThreat".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.related_to_threat", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_relatedToThreat_to_boolean",
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

            if event.has("json.threatStatus") {
                event.rename(
                    "json.threatStatus",
                    "sentinel_one.threat_event.threat_status",
                )?;
            }

            if event.has("json.protocol") {
                event.rename("json.protocol", "sentinel_one.threat_event.protocol")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.hasActiveContent") {
                    if let Some(val) = event.get("json.hasActiveContent") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.hasActiveContent".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.has_active_content", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_hasActiveContent_to_boolean",
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

            if event.has("json.activeContentFileId") {
                event.rename(
                    "json.activeContentFileId",
                    "sentinel_one.threat_event.active_content.file_id",
                )?;
            }

            if event.has("json.activeContentPath") {
                event.rename(
                    "json.activeContentPath",
                    "sentinel_one.threat_event.active_content.path",
                )?;
            }

            if event.has("json.activeContentHash") {
                event.rename(
                    "json.activeContentHash",
                    "sentinel_one.threat_event.active_content.hash",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.processIsMalicious") {
                    if let Some(val) = event.get("json.processIsMalicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.processIsMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.threat_event.process.is_malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_processIsMalicious_to_boolean",
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

            if event.has("json.parentProcessGroupId") {
                event.rename(
                    "json.parentProcessGroupId",
                    "sentinel_one.threat_event.parent_process.group_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.parentProcessIsMalicious") {
                    if let Some(val) = event.get("json.parentProcessIsMalicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.parentProcessIsMalicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sentinel_one.threat_event.parent_process.is_malicious",
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
                    "convert_parentProcessIsMalicious_to_boolean",
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

            if event.has("json.fileSize") {
                event.rename("json.fileSize", "sentinel_one.threat_event.file.size")?;
            }

            if event.has("json.fileType") {
                event.rename("json.fileType", "sentinel_one.threat_event.file.type")?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sentinel_one.threat_event.pid") {
                    if let Some(val) = event.get("sentinel_one.threat_event.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sentinel_one.threat_event.pid".into(),
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
                    "convert_event_pid_to_long",
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

            if let Some(v) = event
                .get("sentinel_one.threat_event.src.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            if let Some(v) = event
                .get("sentinel_one.threat_event.src.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.dst.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.dst.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.file.full_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.cmd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.image_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            let _cond = { event.has_value("process.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.image_sha1_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.process.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.parent_process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sentinel_one.threat_event.parent_pid") {
                    if let Some(val) = event.get("sentinel_one.threat_event.parent_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sentinel_one.threat_event.parent_pid".into(),
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
                    "convert_event_parent_pid_to_long",
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

            if let Some(v) = event
                .get("sentinel_one.threat_event.network.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.registry.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.path", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.indicator.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.description", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.indicator.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.threat_event.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
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
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sentinel_one.threat_event.protocol") {
                map_strings(
                    event,
                    "sentinel_one.threat_event.protocol",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("sentinel_one.threat_event.file.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sentinel_one.threat_event.file.size") {
                        if let Some(val) = event.get("sentinel_one.threat_event.file.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sentinel_one.threat_event.file.size".into(),
                                    message,
                                }
                            })?;
                            event.set("file.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_size_to_long",
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

            if let Some(v) = event
                .get("sentinel_one.threat_event.file.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
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
                event.remove("sentinel_one.threat_event.created_at");
                event.remove("sentinel_one.threat_event.dst.ip");
                event.remove("sentinel_one.threat_event.dst.port");
                event.remove("sentinel_one.threat_event.file.full_name");
                event.remove("sentinel_one.threat_event.file.md5");
                event.remove("sentinel_one.threat_event.file.sha1");
                event.remove("sentinel_one.threat_event.file.sha256");
                event.remove("sentinel_one.threat_event.file.size");
                event.remove("sentinel_one.threat_event.file.type");
                event.remove("sentinel_one.threat_event.id");
                event.remove("sentinel_one.threat_event.indicator.description");
                event.remove("sentinel_one.threat_event.indicator.name");
                event.remove("sentinel_one.threat_event.network.url");
                event.remove("sentinel_one.threat_event.parent_pid");
                event.remove("sentinel_one.threat_event.parent_process.name");
                event.remove("sentinel_one.threat_event.pid");
                event.remove("sentinel_one.threat_event.process.cmd");
                event.remove("sentinel_one.threat_event.process.image_path");
                event.remove("sentinel_one.threat_event.process.image_sha1_hash");
                event.remove("sentinel_one.threat_event.process.name");
                event.remove("sentinel_one.threat_event.process.start_time");
                event.remove("sentinel_one.threat_event.process.user_name");
                event.remove("sentinel_one.threat_event.protocol");
                event.remove("sentinel_one.threat_event.registry.path");
                event.remove("sentinel_one.threat_event.src.ip");
                event.remove("sentinel_one.threat_event.src.port");
                event.remove("sentinel_one.threat_event.user");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
