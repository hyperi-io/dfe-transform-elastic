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

            event.set("observer.vendor", json!("Microsoft"))?;

            event.set("observer.product", json!("Defender for Endpoint"))?;

            event.set("event.kind", json!("event"))?;

            if event.has("json.type") {
                event.rename(
                    "json.type",
                    "microsoft_defender_endpoint.machine_action.type",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("microsoft_defender_endpoint.machine_action.type")
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("microsoft_defender_endpoint.machine_action.type") };
            if _cond {
                // Painless script
                // Source: if (params.get(ctx.microsoft_defender_endpoint.machine_action.type) == null) {\n  return;\n} params.get(ctx.microsoft_defender_endpoint.machine_action.type).forEach((k, v) -> {\n  ctx.event[k] = v\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"if (params.get(ctx.microsoft_defender_endpoint.machine_action.type) == null) {\n  return;\n} params.get(ctx.microsoft_defender_endpoint.machine_action.type).forEach((k, v) -> {\n  ctx.event[k] = v\n});"#
                    ),
                    cached_params!(
                        "{\"CollectInvestigationPackage\":{\"type\":[\"info\"]},\"Isolate\":{\"category\":[\"network\"],\"type\":[\"end\"]},\"LiveResponse\":{\"type\":[\"info\"]},\"Offboard\":{\"type\":[\"info\"]},\"RestrictCodeExecution\":{\"category\":[\"package\"],\"type\":[\"access\"]},\"RunAntiVirusScan\":{\"type\":[\"info\"]},\"StopAndQuarantineFile\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"Unisolate\":{\"category\":[\"network\"],\"type\":[\"start\"]},\"UnrestrictCodeExecution\":{\"category\":[\"package\"],\"type\":[\"access\"]}}"
                    ),
                )?;
            }

            if event.has("json.status") {
                event.rename(
                    "json.status",
                    "microsoft_defender_endpoint.machine_action.status",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_defender_endpoint.machine_action.status")
                    && event
                        .get_str("microsoft_defender_endpoint.machine_action.status")
                        .is_some_and(|s| s.to_lowercase() == "succeeded")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("microsoft_defender_endpoint.machine_action.status")
                    && event
                        .get_str("microsoft_defender_endpoint.machine_action.status")
                        .is_some_and(|s| s.to_lowercase() == "failed")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            if event.has("json.cancellationComment") {
                event.rename(
                    "json.cancellationComment",
                    "microsoft_defender_endpoint.machine_action.cancellation_comment",
                )?;
            }

            let _cond = {
                event.has_value("json.cancellationDateTimeUtc")
                    && event.get_str("json.cancellationDateTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.cancellationDateTimeUtc") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("microsoft_defender_endpoint.machine_action.cancellation_date_time_utc", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_cancellationDateTimeUtc",
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

            if event.has("json.cancellationRequestor") {
                event.rename(
                    "json.cancellationRequestor",
                    "microsoft_defender_endpoint.machine_action.cancellation_requestor",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_defender_endpoint.machine_action.cancellation_requestor")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get(
                                "microsoft_defender_endpoint.machine_action.cancellation_requestor"
                            )
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.commands") {
                event.rename(
                    "json.commands",
                    "microsoft_defender_endpoint.machine_action.commands",
                )?;
            }

            if event.has("json.computerDnsName") {
                event.rename(
                    "json.computerDnsName",
                    "microsoft_defender_endpoint.machine_action.computer_dns_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.computer_dns_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.computer_dns_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine_action.computer_dns_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine_action.computer_dns_name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.creationDateTimeUtc")
                    && event.get_str("json.creationDateTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.creationDateTimeUtc") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set(
                                "microsoft_defender_endpoint.machine_action.creation_date_time_utc",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_creationDateTimeUtc",
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

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.creation_date_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has("json.errorHResult") {
                event.rename(
                    "json.errorHResult",
                    "microsoft_defender_endpoint.machine_action.error_h_result",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.errorHResult") {
                    if let Some(val) = event.get("json.errorHResult") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errorHResult".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_defender_endpoint.machine.error_h_result",
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
                    "convert_errorHResult_to_long",
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

            if event.has("json.externalID") {
                event.rename(
                    "json.externalID",
                    "microsoft_defender_endpoint.machine_action.external_id",
                )?;
            }

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine_action.external_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine_action.external_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.id") {
                event.rename("json.id", "microsoft_defender_endpoint.machine_action.id")?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("json.lastUpdateDateTimeUtc")
                    && event.get_str("json.lastUpdateDateTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastUpdateDateTimeUtc") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("microsoft_defender_endpoint.machine_action.last_update_date_time_utc", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_lastUpdateDateTimeUtc",
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

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.last_update_date_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has("json.machineId") {
                event.rename(
                    "json.machineId",
                    "microsoft_defender_endpoint.machine_action.machine_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_endpoint.machine_action.machine_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond =
                { event.has_value("microsoft_defender_endpoint.machine_action.machine_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_defender_endpoint.machine_action.machine_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.relatedFileInfo.fileIdentifier") {
                event.rename(
                    "json.relatedFileInfo.fileIdentifier",
                    "microsoft_defender_endpoint.machine_action.related_file_info.file_identifier",
                )?;
            }

            if event.has("json.relatedFileInfo.fileIdentifierType") {
                event.rename("json.relatedFileInfo.fileIdentifierType", "microsoft_defender_endpoint.machine_action.related_file_info.file_identifier_type")?;
            }

            let _cond = {
                event.has_value(
                    "microsoft_defender_endpoint.machine_action.related_file_info.file_identifier",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("microsoft_defender_endpoint.machine_action.related_file_info.file_identifier").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("microsoft_defender_endpoint.machine_action.related_file_info.file_identifier_type")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.file = ctx.file ?: [:]; ctx.file.hash = ctx.file.hash ?: [:]; String fileType = ctx.microsoft_defender_endpoint.machine_action.related_file_info.file_identifier_type.toLowerCase(); String fileHash = ctx.microsoft_defender_endpoint.machine_action.related_file_info.file_identifier; if (fileType.contains('sha1')) {\n  ctx.file.hash.sha1 = fileHash;\n} else if (fileType.contains('md5')) {\n  ctx.file.hash.md5 = fileHash;\n} else if (fileType.contains('sha256')) {\n  ctx.file.hash.sha256 = fileHash;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx.file = ctx.file ?: [:]; ctx.file.hash = ctx.file.hash ?: [:]; String fileType = ctx.microsoft_defender_endpoint.machine_action.related_file_info.file_identifier_type.toLowerCase(); String fileHash = ctx.microsoft_defender_endpoint.machine_action.related_file_info.file_identifier; if (fileType.contains('sha1')) {\n  ctx.file.hash.sha1 = fileHash;\n} else if (fileType.contains('md5')) {\n  ctx.file.hash.md5 = fileHash;\n} else if (fileType.contains('sha256')) {\n  ctx.file.hash.sha256 = fileHash;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_set_file_hash_*")?;
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

            if event.has("json.requestSource") {
                event.rename(
                    "json.requestSource",
                    "microsoft_defender_endpoint.machine_action.request_source",
                )?;
            }

            if event.has("json.requestor") {
                event.rename(
                    "json.requestor",
                    "microsoft_defender_endpoint.machine_action.requestor",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_defender_endpoint.machine_action.requestor")
                    && event
                        .get_str("microsoft_defender_endpoint.machine_action.requestor")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_endpoint.machine_action.requestor")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { !event.has_value("user.email") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_endpoint.machine_action.requestor")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.requestorComment") {
                event.rename(
                    "json.requestorComment",
                    "microsoft_defender_endpoint.machine_action.requestor_comment",
                )?;
            }

            if event.has("json.scope") {
                event.rename(
                    "json.scope",
                    "microsoft_defender_endpoint.machine_action.scope",
                )?;
            }

            if event.has("json.title") {
                event.rename(
                    "json.title",
                    "microsoft_defender_endpoint.machine_action.title",
                )?;
            }

            if event.has("json.troubleshootInfo") {
                event.rename(
                    "json.troubleshootInfo",
                    "microsoft_defender_endpoint.machine_action.troubleshoot_info",
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
                event.remove("microsoft_defender_endpoint.machine_action.computer_dns_name");
                event.remove("microsoft_defender_endpoint.machine_action.creation_date_time_utc");
                event.remove("microsoft_defender_endpoint.machine_action.id");
                event
                    .remove("microsoft_defender_endpoint.machine_action.last_update_date_time_utc");
                event.remove("microsoft_defender_endpoint.machine_action.machine_id");
                event.remove("microsoft_defender_endpoint.machine_action.requestor");
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
