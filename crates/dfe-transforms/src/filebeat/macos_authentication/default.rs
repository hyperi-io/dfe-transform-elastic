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
            event.append("event.category", json!("authentication"))?;

            event.append("event.type", json!("info"))?;

            // Begin nested pipeline: "common-pipeline"
            event.set("ecs.version", json!("9.2.0"))?;
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
            event.set("event.kind", json!("event"))?;
            if event.has_value("json.activityIdentifier") {
                if let Some(val) = event.get("json.activityIdentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.activityIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set("macos.activity_identifier", converted)?;
                }
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    if event.has_value("_ingest._value.imageOffset") {
                        if let Some(val) = event.get("_ingest._value.imageOffset") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.imageOffset".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.image.offset", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    event.remove("_ingest._value.imageOffset");
                    Ok(())
                })?;
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    if event.has_value("_ingest._value.imageUUID") {
                        event.rename("_ingest._value.imageUUID", "_ingest._value.image.uuid")?;
                    }
                    Ok(())
                })?;
            }
            if event.has_value("json.backtrace.frames") {
                event.rename("json.backtrace.frames", "macos.backtrace.frames")?;
            }
            if event.has_value("json.bootUUID") {
                event.rename("json.bootUUID", "macos.boot_uuid")?;
            }
            if event.has_value("json.category") {
                event.rename("json.category", "macos.category")?;
            }
            if event.has_value("json.eventMessage") {
                event.rename("json.eventMessage", "macos.event.message.description")?;
            }
            if let Some(v) = event
                .get("macos.event.message.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }
            if event.has_value("json.eventType") {
                event.rename("json.eventType", "macos.event.type")?;
            }
            if event.has_value("json.formatString") {
                event.rename("json.formatString", "macos.format_string")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.machTimestamp") {
                    if let Some(val) = event.get("json.machTimestamp") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.machTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("macos.mach_timestamp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_machTimestamp_to_string",
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
            let _cond = { event.has_value("json.messageType") };
            if _cond {
                // Painless script
                // Source: ctx.log = ctx.log ?: [:];\nctx.log.put(\"level\", params.get(ctx.json.messageType.toLowerCase()));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.log = ctx.log ?: [:];\nctx.log.put(\"level\", params.get(ctx.json.messageType.toLowerCase()));"#
                    ),
                    cached_params!(
                        "{\"default\":\"info\",\"error\":\"error\",\"debug\":\"debug\",\"info\":\"info\",\"fault\":\"warning\"}"
                    ),
                )?;
            }
            if event.has_value("json.parentActivityIdentifier") {
                if let Some(val) = event.get("json.parentActivityIdentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.parentActivityIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set("macos.parent_activity_identifier", converted)?;
                }
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.processID") {
                    if let Some(val) = event.get("json.processID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.processID".into(),
                                message,
                            }
                        })?;
                        event.set("json.processID", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_processID_to_string",
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
                .get("json.processID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }
            if event.has_value("json.processImagePath") {
                event.rename("json.processImagePath", "macos.process.image_path")?;
            }
            if event.has_value("json.processImageUUID") {
                event.rename("json.processImageUUID", "macos.process.image_uuid")?;
            }
            if event.has_value("json.senderImagePath") {
                event.rename("json.senderImagePath", "macos.sender.image_path")?;
            }
            if event.has_value("json.senderImageUUID") {
                event.rename("json.senderImageUUID", "macos.sender.image_uuid")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.senderProgramCounter") {
                    if let Some(val) = event.get("json.senderProgramCounter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.senderProgramCounter".into(),
                                message,
                            }
                        })?;
                        event.set("macos.sender.program_counter", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_senderProgramCounter_to_long",
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
            if event.has_value("json.source") {
                event.rename("json.source", "macos.source")?;
            }
            if event.has_value("json.subsystem") {
                event.rename("json.subsystem", "macos.subsystem")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threadID") {
                    if let Some(val) = event.get("json.threadID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threadID".into(),
                                message,
                            }
                        })?;
                        event.set("json.threadID", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_threadID_to_long",
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
                .get("json.threadID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.id", v)?;
            }
            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSZ",
                                "yyyy-MM-dd HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
            if event.has_value("json.timezoneName") {
                event.rename("json.timezoneName", "macos.timezone_name")?;
            }
            if event.has_value("json.traceID") {
                if let Some(val) = event.get("json.traceID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.traceID".into(),
                            message,
                        }
                    })?;
                    event.set("macos.trace_id", converted)?;
                }
            }
            if event.has_value("json.userID") {
                if let Some(val) = event.get("json.userID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.userID".into(),
                            message,
                        }
                    })?;
                    event.set("json.userID", converted)?;
                }
            }
            if let Some(v) = event
                .get("json.userID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }
            let _cond = { event.has_value("json.userID") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.userID")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
            // End nested pipeline: "common-pipeline"

            if event.has_value("macos.event.message.description") {
                if let Some(input) = event.get_string("macos.event.message.description") {
                    // Grok pattern: ^-\\[%{WORD} %{WORD}\\] \\|%{SPACE}final sessionDictionary:\\{(?:%{SPACE}DirectLogoutType = %{NUMBER:macos.event.message.direct_logout_type:int};)?(?:%{SPACE}GroupID = %{NUMBER:group.id};)?(?:%{SPACE}GuestAccount = %{NUMBER:macos.event.message.guest_account:int};)?(?:%{SPACE}HomeDirectoryPath = %{DATA:macos.event.message.home_directory_path};)?(?:%{SPACE}SessionAgentPID = %{NUMBER:macos.event.message.session_agent_pid};)?(?:%{SPACE}UserGUID = %{DATA:user.group.id};)?(?:%{SPACE}UserID = %{NUMBER:user.id};)?(?:%{SPACE}UserLongName = %{DATA:user.full_name};)?(?:%{SPACE}UserName = %{DATA:user.name};)?\\n\\}
                    // Grok pattern: ^-\\[%{WORD} %{WORD}\\] \\|(?:%{SPACE}shortUsername = %{WORD:user.name},)?(?:%{SPACE}userID = %{NUMBER:user.id},)?(?:%{SPACE}groupID = %{NUMBER:group.id})
                    // Grok pattern: %{GREEDYDATA:macos.event.message.original}
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^-\\[%{WORD} %{WORD}\\] \\|%{SPACE}final sessionDictionary:\\{(?:%{SPACE}DirectLogoutType = %{NUMBER:macos.event.message.direct_logout_type:int};)?(?:%{SPACE}GroupID = %{NUMBER:group.id};)?(?:%{SPACE}GuestAccount = %{NUMBER:macos.event.message.guest_account:int};)?(?:%{SPACE}HomeDirectoryPath = %{DATA:macos.event.message.home_directory_path};)?(?:%{SPACE}SessionAgentPID = %{NUMBER:macos.event.message.session_agent_pid};)?(?:%{SPACE}UserGUID = %{DATA:user.group.id};)?(?:%{SPACE}UserID = %{NUMBER:user.id};)?(?:%{SPACE}UserLongName = %{DATA:user.full_name};)?(?:%{SPACE}UserName = %{DATA:user.name};)?\\n\\}"
                            ),
                            cached_grok!(
                                "^-\\[%{WORD} %{WORD}\\] \\|(?:%{SPACE}shortUsername = %{WORD:user.name},)?(?:%{SPACE}userID = %{NUMBER:user.id},)?(?:%{SPACE}groupID = %{NUMBER:group.id})"
                            ),
                            cached_grok!("%{GREEDYDATA:macos.event.message.original}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("user.group.id") {
                    gsub_field(
                        event,
                        "user.group.id",
                        "user.group.id",
                        cached_regex!("\\\""),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
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
                if event.has_value("user.name") {
                    gsub_field(event, "user.name", "user.name", cached_regex!("\\\""), "")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
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
                if event.has_value("user.full_name") {
                    gsub_field(
                        event,
                        "user.full_name",
                        "user.full_name",
                        cached_regex!("\\\""),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
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

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.has_value("user.group.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.group.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("macos.event.message.original");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
