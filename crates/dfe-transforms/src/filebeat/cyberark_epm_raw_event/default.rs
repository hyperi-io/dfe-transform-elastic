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
                if let Some(v) = event.get("json.arrivalTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.displayName") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.eventType") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.fileName") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.fileQualifier") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.hash") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastEventDate") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.eventType") {
                event.rename("json.eventType", "cyberark_epm.raw_event.type")?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
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
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Installation")))
            };
            if _cond {
                event.append("event.category", json!("package"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Launch")))
            };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Ransomware")))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("ElevationRequest"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Trust"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("ManualRequest"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Block"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("RestrictAccess"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("DetectAccess"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Access"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("StartElevated")))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("CyberArk"))?;

            event.set("observer.product", json!("Endpoint Privilege Manager"))?;

            if event.has_value("json.accessAction") {
                event.rename("json.accessAction", "cyberark_epm.raw_event.access_action")?;
            }

            if event.has_value("json.accessTargetName") {
                event.rename(
                    "json.accessTargetName",
                    "cyberark_epm.raw_event.access_target_name",
                )?;
            }

            if event.has_value("json.accessTargetType") {
                event.rename(
                    "json.accessTargetType",
                    "cyberark_epm.raw_event.access_target_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.agentEventCount") {
                    if let Some(val) = event.get("json.agentEventCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.agentEventCount".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.agent_event_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_agentEventCount_to_long",
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
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.raw_event.agent_event_count")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.sightings", v)?;
                }
            }

            if event.has_value("json.agentId") {
                event.rename("json.agentId", "cyberark_epm.raw_event.agent_id")?;
            }

            if event.has_value("json.applicationSubType") {
                event.rename(
                    "json.applicationSubType",
                    "cyberark_epm.raw_event.application_sub_type",
                )?;
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
                                event.set("cyberark_epm.raw_event.arrival_time", parsed)?
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
                .get("cyberark_epm.raw_event.arrival_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.authorizationRight") {
                event.rename(
                    "json.authorizationRight",
                    "cyberark_epm.raw_event.authorization_right",
                )?;
            }

            if event.has_value("json.authorizationRights") {
                event.rename(
                    "json.authorizationRights",
                    "cyberark_epm.raw_event.authorization_rights",
                )?;
            }

            if event.has_value("json.bundleId") {
                event.rename("json.bundleId", "cyberark_epm.raw_event.bundle_id")?;
            }

            if event.has_value("json.bundleName") {
                event.rename("json.bundleName", "cyberark_epm.raw_event.bundle_name")?;
            }

            if event.has_value("json.bundleVersion") {
                event.rename(
                    "json.bundleVersion",
                    "cyberark_epm.raw_event.bundle_version",
                )?;
            }

            if event.has_value("json.commandInfo") {
                event.rename("json.commandInfo", "cyberark_epm.raw_event.command_info")?;
            }

            if event.has_value("json.company") {
                event.rename("json.company", "cyberark_epm.raw_event.company")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.company")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has_value("json.computerName") {
                event.rename("json.computerName", "cyberark_epm.raw_event.computer_name")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.computer_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("cyberark_epm.raw_event.computer_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.eventCount") {
                    if let Some(val) = event.get("json.eventCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.eventCount".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_eventCount_to_long",
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
                if event.has_value("json.deceptionType") {
                    if let Some(val) = event.get("json.deceptionType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deceptionType".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.deception_type", converted)?;
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

            let _cond = { event.has_value("cyberark_epm.raw_event.deception_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = (int) ctx.cyberark_epm.raw_event.deception_type;\nif (value >= 1 && value <= params.DeceptionType.length) {\n  ctx.cyberark_epm.raw_event.put('deception_type_value', params['DeceptionType'][value - 1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = (int) ctx.cyberark_epm.raw_event.deception_type;\nif (value >= 1 && value <= params.DeceptionType.length) {\n  ctx.cyberark_epm.raw_event.put('deception_type_value', params['DeceptionType'][value - 1]);\n}"#
                        ),
                        cached_params!("{\"DeceptionType\":[\"Lsass lure\",\"Browser lure\"]}"),
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
                        event.set("cyberark_epm.raw_event.defence_action_id", converted)?;
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

            let _cond = { event.has_value("cyberark_epm.raw_event.defence_action_id") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = (int) ctx.cyberark_epm.raw_event.defence_action_id;\nif (value >= 0 && value < params.DefenceAction.length) {\n  ctx.cyberark_epm.raw_event.put('defence_action_value', params['DefenceAction'][value]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = (int) ctx.cyberark_epm.raw_event.defence_action_id;\nif (value >= 0 && value < params.DefenceAction.length) {\n  ctx.cyberark_epm.raw_event.put('defence_action_value', params['DefenceAction'][value]);\n}"#
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

            if event.has_value("json.displayName") {
                event.rename("json.displayName", "cyberark_epm.raw_event.display_name")?;
            }

            if event.has_value("json.evidences") {
                event.rename("json.evidences", "cyberark_epm.raw_event.evidences")?;
            }

            if event.has_value("json.exposedUsers") {
                event.rename("json.exposedUsers", "cyberark_epm.raw_event.exposed_users")?;
            }

            let _cond = {
                event
                    .get("cyberark_epm.raw_event.exposed_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "cyberark_epm.raw_event.exposed_users", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.fatherProcess") {
                event.rename(
                    "json.fatherProcess",
                    "cyberark_epm.raw_event.father_process",
                )?;
            }

            if event.has_value("json.fileAccessPermission") {
                event.rename(
                    "json.fileAccessPermission",
                    "cyberark_epm.raw_event.file_access_permission",
                )?;
            }

            let _cond = { event.has_value("cyberark_epm.raw_event.file_access_permission") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def getOctalValue(String permissions) {\n  def value = 0;\n  if (permissions.charAt(0) == (char) 'r') value += 4;\n  if (permissions.charAt(1) == (char) 'w') value += 2;\n  if (permissions.charAt(2) == (char) 'x') value += 1;\n  return value;\n}\nString permissionString = ctx.cyberark_epm.raw_event.file_access_permission;\nif (permissionString.length() != 10) {\n  return;\n}\nint owner = getOctalValue(permissionString.substring(1, 4));\nint group = getOctalValue(permissionString.substring(4, 7));\nint other = getOctalValue(permissionString.substring(7, 10));\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nctx.file.put('mode', Integer.toString(owner) + Integer.toString(group) + Integer.toString(other));
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def getOctalValue(String permissions) {\n  def value = 0;\n  if (permissions.charAt(0) == (char) 'r') value += 4;\n  if (permissions.charAt(1) == (char) 'w') value += 2;\n  if (permissions.charAt(2) == (char) 'x') value += 1;\n  return value;\n}\nString permissionString = ctx.cyberark_epm.raw_event.file_access_permission;\nif (permissionString.length() != 10) {\n  return;\n}\nint owner = getOctalValue(permissionString.substring(1, 4));\nint group = getOctalValue(permissionString.substring(4, 7));\nint other = getOctalValue(permissionString.substring(7, 10));\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nctx.file.put('mode', Integer.toString(owner) + Integer.toString(group) + Integer.toString(other));"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_file_mode")?;
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

            if event.has_value("json.fileDescription") {
                event.rename(
                    "json.fileDescription",
                    "cyberark_epm.raw_event.file_description",
                )?;
            }

            if event.has_value("json.fileName") {
                event.rename("json.fileName", "cyberark_epm.raw_event.file_name")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if event.has_value("json.filePath") {
                event.rename("json.filePath", "cyberark_epm.raw_event.file_path")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if event.has_value("json.filePathWithoutFilename") {
                event.rename(
                    "json.filePathWithoutFilename",
                    "cyberark_epm.raw_event.file_path_without_filename",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_path_without_filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.directory", v)?;
            }

            if event.has_value("json.fileQualifier") {
                event.rename(
                    "json.fileQualifier",
                    "cyberark_epm.raw_event.file_qualifier",
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
                        event.set("cyberark_epm.raw_event.file_size", converted)?;
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
                .get("cyberark_epm.raw_event.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.size", v)?;
            }

            if event.has_value("json.fileVersion") {
                event.rename("json.fileVersion", "cyberark_epm.raw_event.file_version")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.version", v)?;
            }

            let _cond = {
                event.has_value("json.firstEventDate")
                    && event.get_str("json.firstEventDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstEventDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("cyberark_epm.raw_event.first_event_date", parsed)?
                            }
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
                .get("cyberark_epm.raw_event.first_event_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.raw_event.first_event_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.first_seen", v)?;
                }
            }

            if event.has_value("json.hash") {
                event.rename("json.hash", "cyberark_epm.raw_event.hash")?;
            }

            let _cond = { event.has_value("cyberark_epm.raw_event.hash") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def hash = ctx.cyberark_epm.raw_event.hash;\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nif (ctx.file.hash == null) {\n  ctx.file.put('hash', new HashMap());\n}\nif (hash.length() == 40) {\n  ctx.file.hash.sha1 = hash;\n} else if (hash.startsWith('sha1##') || hash.startsWith('SHA1##')) {\n  ctx.file.hash.sha1 = hash.substring(6);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hash = ctx.cyberark_epm.raw_event.hash;\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nif (ctx.file.hash == null) {\n  ctx.file.put('hash', new HashMap());\n}\nif (hash.length() == 40) {\n  ctx.file.hash.sha1 = hash;\n} else if (hash.startsWith('sha1##') || hash.startsWith('SHA1##')) {\n  ctx.file.hash.sha1 = hash.substring(6);\n}"#
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
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
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
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
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

            if event.has_value("json.interpreter") {
                event.rename("json.interpreter", "cyberark_epm.raw_event.interpreter")?;
            }

            if event.has_value("json.justification") {
                event.rename("json.justification", "cyberark_epm.raw_event.justification")?;
            }

            if event.has_value("json.justificationEmail") {
                event.rename(
                    "json.justificationEmail",
                    "cyberark_epm.raw_event.justification_email",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.justification_email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.lastEventComputerName") {
                event.rename(
                    "json.lastEventComputerName",
                    "cyberark_epm.raw_event.last_event_computer_name",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("cyberark_epm.raw_event.last_event_computer_name")
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
                            Some(parsed) => {
                                event.set("cyberark_epm.raw_event.last_event_date", parsed)?
                            }
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
                .get("cyberark_epm.raw_event.last_event_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.raw_event.last_event_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.last_seen", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.logonAttemptTypeId") {
                    if let Some(val) = event.get("json.logonAttemptTypeId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.logonAttemptTypeId".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.logon_attempt_type_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_logonAttemptTypeId_to_long",
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

            let _cond = { event.has_value("cyberark_epm.raw_event.logon_attempt_type_id") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = Long.toString(ctx.cyberark_epm.raw_event.logon_attempt_type_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_attempt_value', params[value]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = Long.toString(ctx.cyberark_epm.raw_event.logon_attempt_type_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_attempt_value', params[value]);\n}"#
                        ),
                        cached_params!(
                            "{\"2\":\"Interactive (logon at keyboard and screen of system)\",\"3\":\"Network (i.e. connection to shared folder on this computer from elsewhere on network)\",\"4\":\"Batch (i.e. scheduled task)\",\"5\":\"Service (Service startup)\",\"7\":\"Unlock (i.e. unnattended workstation with password protected screen saver)\",\"8\":\"NetworkCleartext (Logon with credentials sent in the clear text.)\",\"9\":\"NewCredentials such as with RunAs or mapping a network drive with alternate credentials.\",\"10\":\"RemoteInteractive (Terminal Services, Remote Desktop or Remote Assistance)\",\"11\":\"CachedInteractive (logon with cached domain credentials, such as when logging on to a laptop when away from the network)\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_logon_attempt")?;
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
                if event.has_value("json.logonStatusId") {
                    if let Some(val) = event.get("json.logonStatusId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.logonStatusId".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.logon_status_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_logonStatusId_to_long",
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

            let _cond = { event.has_value("cyberark_epm.raw_event.logon_status_id") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = Long.toString(ctx.cyberark_epm.raw_event.logon_status_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_status_value', params[value]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def value = Long.toString(ctx.cyberark_epm.raw_event.logon_status_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_status_value', params[value]);\n}"#
                        ),
                        cached_params!(
                            "{\"3221225566\":\"There are currently no logon servers available to service the logon request.\",\"3221225572\":\"User logon with misspelled or bad user account\",\"3221225578\":\"User logon with misspelled or bad password\",\"3221225581\":\"The cause is either a bad username or authentication information\",\"3221225582\":\"Indicates a referenced user name and authentication information are valid, but some user account restriction has prevented successful authentication (such as time-of-day restrictions).\",\"3221225583\":\"User logon outside authorized hours\",\"3221225584\":\"User logon from unauthorized workstation\",\"3221225585\":\"User logon with expired password\",\"3221225586\":\"User logon to account disabled by administrator\",\"3221225692\":\"Indicates the Sam Server was in the wrong state to perform the desired operation.\",\"3221225779\":\"Clocks between DC and other computer too far out of sync\",\"3221225819\":\"The user has not been granted the requested logon type (also called the logon right) at this machine\",\"3221225868\":\"The logon request failed because the trust relationship between the primary domain and the trusted domain failed.\",\"3221225874\":\"An attempt was made to logon, but the Netlogon service was not started.\",\"3221225875\":\"User logon with expired account\",\"3221226020\":\"User is required to change password at next logon\",\"3221226021\":\"Evidently a bug in Windows and not a risk\",\"3221226036\":\"User logon with account locked\",\"3221226222\":\"Failure Reason: An Error occurred during Logon\",\"3221226515\":\"Logon Failure: The machine you are logging on to is protected by an authentication firewall. The specified account is not allowed to authenticate to the machine.\",\"0\":\"Status OK.\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_logon_status")?;
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

            if event.has_value("json.lureUser") {
                event.rename("json.lureUser", "cyberark_epm.raw_event.lure_user")?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.lure_user")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.modificationTime")
                    && event.get_str("json.modificationTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.modificationTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("cyberark_epm.raw_event.modification_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.modificationTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_modificationTime")?;
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
                .get("cyberark_epm.raw_event.modification_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mtime", v)?;
            }

            if event.has_value("json.operatingSystemType") {
                event.rename(
                    "json.operatingSystemType",
                    "cyberark_epm.raw_event.operating_system_type",
                )?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.operating_system_type")
                    && (event
                        .get_str("cyberark_epm.raw_event.operating_system_type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Windows"))
                        || event
                            .get_str("cyberark_epm.raw_event.operating_system_type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("macOS"))
                        || event
                            .get_str("cyberark_epm.raw_event.operating_system_type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Linux")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.raw_event.operating_system_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.type", v)?;
                }
            }

            if event.has_value("host.os.type") {
                map_strings(event, "host.os.type", "host.os.type", str::to_lowercase)?;
            }

            if event.has_value("json.originUserUID") {
                event.rename(
                    "json.originUserUID",
                    "cyberark_epm.raw_event.origin_user_uid",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.origin_user_uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.origin_user_uid")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.originalFileName") {
                event.rename(
                    "json.originalFileName",
                    "cyberark_epm.raw_event.original_file_name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.owner") {
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.file_owner_domain}\\\\%{DATA:cyberark_epm.raw_event.file_owner_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.file_owner_domain}\\\\\\\\%{DATA:cyberark_epm.raw_event.file_owner_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.file_owner_name}@%{DATA:cyberark_epm.raw_event.file_owner_domain}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.file_owner_name}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.file_owner_domain}\\\\%{DATA:cyberark_epm.raw_event.file_owner_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.file_owner_domain}\\\\\\\\%{DATA:cyberark_epm.raw_event.file_owner_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.file_owner_name}@%{DATA:cyberark_epm.raw_event.file_owner_domain}$"
                            ),
                            cached_grok!("^%{DATA:cyberark_epm.raw_event.file_owner_name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("cyberark_epm.raw_event.file_owner_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.owner", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.file_owner_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.packageName") {
                event.rename("json.packageName", "cyberark_epm.raw_event.package_name")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.package_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.name", v)?;
            }

            if event.has_value("json.policyCategory") {
                event.rename(
                    "json.policyCategory",
                    "cyberark_epm.raw_event.policy_category",
                )?;
            }

            if event.has_value("json.policyName") {
                event.rename("json.policyName", "cyberark_epm.raw_event.policy_name")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.policy_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.processCertificateIssuer") {
                event.rename(
                    "json.processCertificateIssuer",
                    "cyberark_epm.raw_event.process_certificate_issuer",
                )?;
            }

            if event.has_value("json.processCommandLine") {
                event.rename(
                    "json.processCommandLine",
                    "cyberark_epm.raw_event.process_command_line",
                )?;
            }

            if event.has_value("json.productCode") {
                event.rename("json.productCode", "cyberark_epm.raw_event.product_code")?;
            }

            if event.has_value("json.productName") {
                event.rename("json.productName", "cyberark_epm.raw_event.product_name")?;
            }

            let _cond = {
                event.has_value("cyberark_epm.raw_event.type")
                    && (event
                        .get_str("cyberark_epm.raw_event.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("AttackAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("AttackBlock"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityAttempt"))
                        || event
                            .get_str("cyberark_epm.raw_event.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("SuspiciousActivityBlock")))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.raw_event.product_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.software.name", v)?;
                }
            }

            if event.has_value("json.productVersion") {
                event.rename(
                    "json.productVersion",
                    "cyberark_epm.raw_event.product_version",
                )?;
            }

            if event.has_value("json.publisher") {
                event.rename("json.publisher", "cyberark_epm.raw_event.publisher")?;
            }

            if event.has_value("json.runAsUsername") {
                event.rename(
                    "json.runAsUsername",
                    "cyberark_epm.raw_event.run_as_username",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.run_as_username")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

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
                        event.set("cyberark_epm.raw_event.skipped_count", converted)?;
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

            if event.has_value("json.sourceName") {
                event.rename("json.sourceName", "cyberark_epm.raw_event.source_name")?;
            }

            if event.has_value("json.sourceProcessCertificateIssuer") {
                event.rename(
                    "json.sourceProcessCertificateIssuer",
                    "cyberark_epm.raw_event.source_process_certificate_issuer",
                )?;
            }

            if event.has_value("json.sourceProcessCommandLine") {
                event.rename(
                    "json.sourceProcessCommandLine",
                    "cyberark_epm.raw_event.source_process_command_line",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.source_process_command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if event.has_value("json.sourceProcessHash") {
                event.rename(
                    "json.sourceProcessHash",
                    "cyberark_epm.raw_event.source_process_hash",
                )?;
            }

            let _cond = { event.has_value("cyberark_epm.raw_event.source_process_hash") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = ctx.cyberark_epm.raw_event.source_process_hash;\nif (ctx.process == null) {\n  ctx.put('process', new HashMap());\n}\nif (ctx.process.hash == null) {\n  ctx.process.put('hash', new HashMap());\n}\nif (ctx.related == null) {\n  ctx.put('related', new HashMap());\n}\nif (ctx.related.hash == null) {\n  ctx.related.put('hash', new ArrayList());\n}\nif (value.length() == 32) {\n  ctx.process.hash.md5 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 40) {\n  ctx.process.hash.sha1 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 64) {\n  ctx.process.hash.sha256 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 96) {\n  ctx.process.hash.sha384 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 128) {\n  ctx.process.hash.sha512 = value;\n  ctx.related.hash.add(value);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def value = ctx.cyberark_epm.raw_event.source_process_hash;\nif (ctx.process == null) {\n  ctx.put('process', new HashMap());\n}\nif (ctx.process.hash == null) {\n  ctx.process.put('hash', new HashMap());\n}\nif (ctx.related == null) {\n  ctx.put('related', new HashMap());\n}\nif (ctx.related.hash == null) {\n  ctx.related.put('hash', new ArrayList());\n}\nif (value.length() == 32) {\n  ctx.process.hash.md5 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 40) {\n  ctx.process.hash.sha1 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 64) {\n  ctx.process.hash.sha256 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 96) {\n  ctx.process.hash.sha384 = value;\n  ctx.related.hash.add(value);\n} else if (value.length() == 128) {\n  ctx.process.hash.sha512 = value;\n  ctx.related.hash.add(value);\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_process_hash")?;
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

            if event.has_value("json.sourceProcessPublisher") {
                event.rename(
                    "json.sourceProcessPublisher",
                    "cyberark_epm.raw_event.source_process_publisher",
                )?;
            }

            if event.has_value("json.sourceProcessSigner") {
                event.rename(
                    "json.sourceProcessSigner",
                    "cyberark_epm.raw_event.source_process_signer",
                )?;
            }

            if event.has_value("json.sourceProcessUsername") {
                event.rename(
                    "json.sourceProcessUsername",
                    "cyberark_epm.raw_event.source_process_username",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.source_process_username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.real_user.name", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.source_process_username")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.sourceType") {
                event.rename("json.sourceType", "cyberark_epm.raw_event.source_type")?;
            }

            let _cond = { event.get_str("json.sourceWSIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.sourceWSIp") {
                        if let Some(val) = event.get("json.sourceWSIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.sourceWSIp".into(),
                                    message,
                                }
                            })?;
                            event.set("cyberark_epm.raw_event.source_ws_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sourceWSIp_to_ip",
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
                .get("cyberark_epm.raw_event.source_ws_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("cyberark_epm.raw_event.source_ws_ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.sourceWSName") {
                event.rename("json.sourceWSName", "cyberark_epm.raw_event.source_ws_name")?;
            }

            if event.has_value("json.symLink") {
                event.rename("json.symLink", "cyberark_epm.raw_event.sym_link")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.sym_link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.target_path", v)?;
            }

            if event.has_value("json.threatProtectionAction") {
                event.rename(
                    "json.threatProtectionAction",
                    "cyberark_epm.raw_event.threat_protection_action",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.threat_protection_action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("json.threatProtectionActionId") {
                if let Some(val) = event.get("json.threatProtectionActionId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.threatProtectionActionId".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cyberark_epm.raw_event.threat_protection_action_id",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.upgradeCode") {
                event.rename("json.upgradeCode", "cyberark_epm.raw_event.upgrade_code")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.userIsAdmin") {
                    if let Some(val) = event.get("json.userIsAdmin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.userIsAdmin".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.user_is_admin", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_userIsAdmin_to_boolean",
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.userName") {
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.user_domain}\\\\%{DATA:cyberark_epm.raw_event.user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.user_domain}\\\\\\\\%{DATA:cyberark_epm.raw_event.user_name}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.user_name}@%{DATA:cyberark_epm.raw_event.user_domain}$
                    // Grok pattern: ^%{DATA:cyberark_epm.raw_event.user_name}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.user_domain}\\\\%{DATA:cyberark_epm.raw_event.user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.user_domain}\\\\\\\\%{DATA:cyberark_epm.raw_event.user_name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cyberark_epm.raw_event.user_name}@%{DATA:cyberark_epm.raw_event.user_domain}$"
                            ),
                            cached_grok!("^%{DATA:cyberark_epm.raw_event.user_name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("cyberark_epm.raw_event.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.user_domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("cyberark_epm.raw_event.user_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.winEventRecordId") {
                    if let Some(val) = event.get("json.winEventRecordId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.winEventRecordId".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.win_event_record_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_winEventRecordId_to_long",
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
                if event.has_value("json.winEventType") {
                    if let Some(val) = event.get("json.winEventType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.winEventType".into(),
                                message,
                            }
                        })?;
                        event.set("cyberark_epm.raw_event.win_event_type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_winEventType_to_long",
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

            if event.has_value("json.workingDirectory") {
                event.rename(
                    "json.workingDirectory",
                    "cyberark_epm.raw_event.working_directory",
                )?;
            }

            if let Some(v) = event
                .get("cyberark_epm.raw_event.working_directory")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
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
                event.remove("cyberark_epm.raw_event.arrival_time");
                event.remove("cyberark_epm.raw_event.company");
                event.remove("cyberark_epm.raw_event.computer_name");
                event.remove("cyberark_epm.raw_event.file_path");
                event.remove("cyberark_epm.raw_event.file_path_without_filename");
                event.remove("cyberark_epm.raw_event.file_size");
                event.remove("cyberark_epm.raw_event.file_version");
                event.remove("cyberark_epm.raw_event.first_event_date");
                event.remove("cyberark_epm.raw_event.last_event_date");
                event.remove("cyberark_epm.raw_event.modification_time");
                event.remove("cyberark_epm.raw_event.origin_user_uid");
                event.remove("cyberark_epm.raw_event.file_name");
                event.remove("cyberark_epm.raw_event.file_owner_name");
                event.remove("cyberark_epm.raw_event.package_name");
                event.remove("cyberark_epm.raw_event.policy_name");
                event.remove("cyberark_epm.raw_event.source_process_command_line");
                event.remove("cyberark_epm.raw_event.source_process_username");
                event.remove("cyberark_epm.raw_event.source_ws_ip");
                event.remove("cyberark_epm.raw_event.sym_link");
                event.remove("cyberark_epm.raw_event.threat_protection_action");
                event.remove("cyberark_epm.raw_event.user_name");
                event.remove("cyberark_epm.raw_event.working_directory");
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
