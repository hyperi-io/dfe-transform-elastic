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

            let _cond = { event.has_value("message") && event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.aggregatedBy") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.arrivalTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.eventType") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.fileQualifier") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastEventDate") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastEventDisplayName") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastEventFileName") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastEventId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.eventType") {
                event.rename("json.eventType", "cyberark_epm.aggregated_event.type")?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Installation")))
            };
            if _cond {
                event.append("event.category", json!("package"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Launch")))
            };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Ransomware")))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("ElevationRequest"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Trust"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("ManualRequest"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Block"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("RestrictAccess"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("DetectAccess"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("StartElevated")))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("CyberArk"))?;

            event.set("observer.product", json!("Endpoint Privilege Manager"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.affectedComputers") {
                    if let Some(val) = event.get("json.affectedComputers") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.affectedComputers".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyberark_epm.aggregated_event.affected_computers",
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
                    "convert_affectedComputers_to_long",
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
                if event.has_value("json.affectedUsers") {
                    if let Some(val) = event.get("json.affectedUsers") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.affectedUsers".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.affected_users", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_affectedUsers_to_long",
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

            if event.has_value("json.agentId") {
                event.rename("json.agentId", "cyberark_epm.aggregated_event.agent_id")?;
            }

            if event.has_value("json.aggregatedBy") {
                event.rename(
                    "json.aggregatedBy",
                    "cyberark_epm.aggregated_event.aggregated_by",
                )?;
            }

            if event.has_value("json.appPackageDisplayName") {
                event.rename(
                    "json.appPackageDisplayName",
                    "cyberark_epm.aggregated_event.app_package_display_name",
                )?;
            }

            if event.has_value("json.applicationType") {
                event.rename(
                    "json.applicationType",
                    "cyberark_epm.aggregated_event.application_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.applicationTypeId") {
                    if let Some(val) = event.get("json.applicationTypeId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.applicationTypeId".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyberark_epm.aggregated_event.application_type_id",
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
                    "convert_applicationTypeId_to_long",
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

            let _cond = {
                event.has_value("json.arrivalTime") && event.get_str("json.arrivalTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.arrivalTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("cyberark_epm.aggregated_event.arrival_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.arrivalTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_arrivalTime")?;
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
                .get("cyberark_epm.aggregated_event.arrival_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.authorizationRight") {
                event.rename(
                    "json.authorizationRight",
                    "cyberark_epm.aggregated_event.authorization_right",
                )?;
            }

            if event.has_value("json.CLSID") {
                event.rename("json.CLSID", "cyberark_epm.aggregated_event.clsid")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.deceptionType") {
                    if let Some(val) = event.get("json.deceptionType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deceptionType".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.deception_type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_deceptionType_to_long",
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

            let _cond = { event.has_value("cyberark_epm.aggregated_event.deception_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = (int) ctx.cyberark_epm.aggregated_event.deception_type;\nif (value >= 1 && value <= params.DeceptionType.length) {\n  ctx.cyberark_epm.aggregated_event.put('deception_type_value', params['DeceptionType'][value - 1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = (int) ctx.cyberark_epm.aggregated_event.deception_type;\nif (value >= 1 && value <= params.DeceptionType.length) {\n  ctx.cyberark_epm.aggregated_event.put('deception_type_value', params['DeceptionType'][value - 1]);\n}"#
                        ),
                        cached_params!(
                            "{\"DeceptionType\":[\"\\\"Local User LSASS\\\" honeypot\",\"\\\"Browsers\\\" (IE, Chrome, Firefox, Edge) honeypot\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_deception_type")?;
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
                if event.has_value("json.defenceActionId") {
                    if let Some(val) = event.get("json.defenceActionId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.defenceActionId".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.defence_action_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_defenceActionId_to_long",
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

            let _cond = { event.has_value("cyberark_epm.aggregated_event.defence_action_id") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = (int) ctx.cyberark_epm.aggregated_event.defence_action_id;\nif (value >= 0 && value < params.DefenceAction.length) {\n  ctx.cyberark_epm.aggregated_event.put('defence_action_value', params['DefenceAction'][value]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = (int) ctx.cyberark_epm.aggregated_event.defence_action_id;\nif (value >= 0 && value < params.DefenceAction.length) {\n  ctx.cyberark_epm.aggregated_event.put('defence_action_value', params['DefenceAction'][value]);\n}"#
                        ),
                        cached_params!("{\"DefenceAction\":[\"No action\",\"Detect\",\"Block\"]}"),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_defence_action")?;
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
                if event.has_value("json.eventTypeId") {
                    if let Some(val) = event.get("json.eventTypeId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.eventTypeId".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.type_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_eventTypeId_to_long",
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
                if event.has_value("json.exposedUsers") {
                    if let Some(val) = event.get("json.exposedUsers") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.exposedUsers".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.exposed_users", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_exposedUsers_to_long",
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

            if event.has_value("json.fileLocation") {
                event.rename(
                    "json.fileLocation",
                    "cyberark_epm.aggregated_event.file_location",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.file_location")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.directory", v)?;
            }

            if event.has_value("json.fileQualifier") {
                event.rename(
                    "json.fileQualifier",
                    "cyberark_epm.aggregated_event.file_qualifier",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.fileSize") {
                    if let Some(val) = event.get("json.fileSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.fileSize".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.file_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_fileSize_to_long",
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
                .get("cyberark_epm.aggregated_event.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.size", v)?;
            }

            if event.has_value("json.firstEventComputerName") {
                event.rename(
                    "json.firstEventComputerName",
                    "cyberark_epm.aggregated_event.first_event_computer_name",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("cyberark_epm.aggregated_event.first_event_computer_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.firstEventDate")
                    && event.get_str("json.firstEventDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstEventDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("cyberark_epm.aggregated_event.first_event_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.firstEventDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstEventDate")?;
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
                .get("cyberark_epm.aggregated_event.first_event_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.aggregated_event.first_event_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.first_seen", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.firstEventUserName") {
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}\\\\%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}\\\\\\\\%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.first_event_user_name}@%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}\\\\%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}\\\\\\\\%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.first_event_user_name}@%{DATA:cyberark_epm.aggregated_event.first_event_user_domain}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.first_event_user_name}$"
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

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.aggregated_event.first_event_user_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.hash") {
                event.rename("json.hash", "cyberark_epm.aggregated_event.hash")?;
            }

            let _cond = { event.has_value("cyberark_epm.aggregated_event.hash") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def hash = ctx.cyberark_epm.aggregated_event.hash;\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nif (ctx.file.hash == null) {\n  ctx.file.put('hash', new HashMap());\n}\nif (hash.length() == 40) {\n  ctx.file.hash.sha1 = hash;\n} else if (hash.startsWith('sha1##') || hash.startsWith('SHA1##')) {\n  ctx.file.hash.sha1 = hash.substring(6);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hash = ctx.cyberark_epm.aggregated_event.hash;\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nif (ctx.file.hash == null) {\n  ctx.file.put('hash', new HashMap());\n}\nif (hash.length() == 40) {\n  ctx.file.hash.sha1 = hash;\n} else if (hash.startsWith('sha1##') || hash.startsWith('SHA1##')) {\n  ctx.file.hash.sha1 = hash.substring(6);\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_map_file_hash_sha1_field",
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
                .get("file.hash.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.checksum", v)?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("file.hash.sha1")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("file.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.lastAgentId") {
                event.rename(
                    "json.lastAgentId",
                    "cyberark_epm.aggregated_event.last_agent_id",
                )?;
            }

            if event.has_value("json.lastEventAccessTargetName") {
                event.rename(
                    "json.lastEventAccessTargetName",
                    "cyberark_epm.aggregated_event.last_event_access_target_name",
                )?;
            }

            if event.has_value("json.lastEventAccessTargetType") {
                event.rename(
                    "json.lastEventAccessTargetType",
                    "cyberark_epm.aggregated_event.last_event_access_target_type",
                )?;
            }

            if event.has_value("json.lastEventAgentId") {
                event.rename(
                    "json.lastEventAgentId",
                    "cyberark_epm.aggregated_event.last_event_agent_id",
                )?;
            }

            if event.has_value("json.lastEventAuthorizationRights") {
                event.rename(
                    "json.lastEventAuthorizationRights",
                    "cyberark_epm.aggregated_event.last_event_authorization_rights",
                )?;
            }

            if event.has_value("json.lastEventComputerName") {
                event.rename(
                    "json.lastEventComputerName",
                    "cyberark_epm.aggregated_event.last_event_computer_name",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("cyberark_epm.aggregated_event.last_event_computer_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.lastEventDate")
                    && event.get_str("json.lastEventDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastEventDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("cyberark_epm.aggregated_event.last_event_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastEventDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastEventDate")?;
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
                .get("cyberark_epm.aggregated_event.last_event_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.aggregated_event.last_event_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.last_seen", v)?;
                }
            }

            if event.has_value("json.lastEventDisplayName") {
                event.rename(
                    "json.lastEventDisplayName",
                    "cyberark_epm.aggregated_event.last_event_display_name",
                )?;
            }

            if event.has_value("json.lastEventExposedUsers") {
                event.rename(
                    "json.lastEventExposedUsers",
                    "cyberark_epm.aggregated_event.last_event_exposed_users",
                )?;
            }

            let _cond = {
                event
                    .get("cyberark_epm.aggregated_event.last_event_exposed_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cyberark_epm.aggregated_event.last_event_exposed_users",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.lastEventExposedUsersCount") {
                    if let Some(val) = event.get("json.lastEventExposedUsersCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.lastEventExposedUsersCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyberark_epm.aggregated_event.last_event_exposed_users_count",
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
                    "convert_lastEventExposedUsersCount_to_long",
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

            if event.has_value("json.lastEventFileName") {
                event.rename(
                    "json.lastEventFileName",
                    "cyberark_epm.aggregated_event.last_event_file_name",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.last_event_file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if event.has_value("json.lastEventId") {
                event.rename(
                    "json.lastEventId",
                    "cyberark_epm.aggregated_event.last_event_id",
                )?;
            }

            if event.has_value("json.lastEventInitiatedProcess") {
                event.rename(
                    "json.lastEventInitiatedProcess",
                    "cyberark_epm.aggregated_event.last_event_initiated_process",
                )?;
            }

            if event.has_value("json.lastEventInitiatedProcessLocation") {
                event.rename(
                    "json.lastEventInitiatedProcessLocation",
                    "cyberark_epm.aggregated_event.last_event_initiated_process_location",
                )?;
            }

            if event.has_value("json.lastEventJustification") {
                event.rename(
                    "json.lastEventJustification",
                    "cyberark_epm.aggregated_event.last_event_justification",
                )?;
            }

            if event.has_value("json.lastEventOriginalFileName") {
                event.rename(
                    "json.lastEventOriginalFileName",
                    "cyberark_epm.aggregated_event.last_event_original_file_name",
                )?;
            }

            if event.has_value("json.lastEventPackageName") {
                event.rename(
                    "json.lastEventPackageName",
                    "cyberark_epm.aggregated_event.last_event_package_name",
                )?;
            }

            if event.has_value("json.lastEventSourceName") {
                event.rename(
                    "json.lastEventSourceName",
                    "cyberark_epm.aggregated_event.last_event_source_name",
                )?;
            }

            if event.has_value("json.lastEventSourceType") {
                event.rename(
                    "json.lastEventSourceType",
                    "cyberark_epm.aggregated_event.last_event_source_type",
                )?;
            }

            if event.has_value("json.lastEventSymLink") {
                event.rename(
                    "json.lastEventSymLink",
                    "cyberark_epm.aggregated_event.last_event_sym_link",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.last_event_sym_link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.target_path", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.lastEventUserName") {
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}\\\\%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}\\\\\\\\%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.last_event_user_name}@%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}$
                    // Grok pattern: ^%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}\\\\%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}\\\\\\\\%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.last_event_user_name}@%{DATA:cyberark_epm.aggregated_event.last_event_user_domain}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.aggregated_event.last_event_user_name}$"
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

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.aggregated_event.last_event_user_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.mimeType") {
                event.rename("json.mimeType", "cyberark_epm.aggregated_event.mime_type")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.mime_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mime_type", v)?;
            }

            if event.has_value("json.operatingSystemType") {
                event.rename(
                    "json.operatingSystemType",
                    "cyberark_epm.aggregated_event.operating_system_type",
                )?;
            }

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.operating_system_type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.operating_system_type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Windows"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.operating_system_type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("macOS"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.operating_system_type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Linux")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.aggregated_event.operating_system_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.type", v)?;
                }
            }

            if event.has_value("host.os.type") {
                map_strings(event, "host.os.type", "host.os.type", str::to_lowercase)?;
            }

            if event.has_value("json.packageName") {
                event.rename(
                    "json.packageName",
                    "cyberark_epm.aggregated_event.package_name",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.package_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.name", v)?;
            }

            if event.has_value("json.policyName") {
                event.rename(
                    "json.policyName",
                    "cyberark_epm.aggregated_event.policy_name",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.aggregated_event.policy_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.productCode") {
                event.rename(
                    "json.productCode",
                    "cyberark_epm.aggregated_event.product_code",
                )?;
            }

            if event.has_value("json.publisher") {
                event.rename("json.publisher", "cyberark_epm.aggregated_event.publisher")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.skipped") {
                    if let Some(val) = event.get("json.skipped") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.skipped".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.skipped", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_skipped_to_boolean",
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
                if event.has_value("json.skippedCount") {
                    if let Some(val) = event.get("json.skippedCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.skippedCount".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.skipped_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_skippedCount_to_long",
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

            if event.has_value("json.threatDetectionAction") {
                event.rename(
                    "json.threatDetectionAction",
                    "cyberark_epm.aggregated_event.threat_detection_action",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.totalEvents") {
                    if let Some(val) = event.get("json.totalEvents") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalEvents".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.aggregated_event.total_events", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_totalEvents_to_long",
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

            let _cond = {
                event.has_value("cyberark_epm.aggregated_event.type")
                    && (event
                        .get_str("cyberark_epm.aggregated_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.aggregated_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.aggregated_event.total_events")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.sightings", v)?;
                }
            }

            if event.has_value("json.upgradeCode") {
                event.rename(
                    "json.upgradeCode",
                    "cyberark_epm.aggregated_event.upgrade_code",
                )?;
            }

            if event.has_value("json.url") {
                event.rename("json.url", "cyberark_epm.aggregated_event.url")?;
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
                event.remove("cyberark_epm.aggregated_event.arrival_time");
                event.remove("cyberark_epm.aggregated_event.file_location");
                event.remove("cyberark_epm.aggregated_event.file_size");
                event.remove("cyberark_epm.aggregated_event.first_event_date");
                event.remove("cyberark_epm.aggregated_event.last_event_date");
                event.remove("cyberark_epm.aggregated_event.last_event_file_name");
                event.remove("cyberark_epm.aggregated_event.last_event_sym_link");
                event.remove("cyberark_epm.aggregated_event.mime_type");
                event.remove("cyberark_epm.aggregated_event.package_name");
                event.remove("cyberark_epm.aggregated_event.policy_name");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
