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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.eventId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.eventTime") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.endpoint_activity = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.endpoint_activity = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.dst") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.dst",
                    "trend_micro_vision_one.endpoint_activity.destination.address",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.dpt") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.dpt",
                    "trend_micro_vision_one.endpoint_activity.destination.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.endpoint_guid") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.endpoint_guid",
                    "trend_micro_vision_one.endpoint_activity.endpoint.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.endpoint_host_name") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.endpoint_host_name",
                    "trend_micro_vision_one.endpoint_activity.endpoint.host_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.endpoint_ip") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.endpoint_ip",
                    "trend_micro_vision_one.endpoint_activity.endpoint.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.event_id") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.event_id",
                    "trend_micro_vision_one.endpoint_activity.event.id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.event_sub_id") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.event_sub_id",
                    "trend_micro_vision_one.endpoint_activity.event.sub_id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.event_time") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.event_time",
                    "trend_micro_vision_one.endpoint_activity.event.time",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.event_time_dt") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.event_time_dt",
                    "trend_micro_vision_one.endpoint_activity.event.time_dt",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_cmd") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_cmd",
                    "trend_micro_vision_one.endpoint_activity.object.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_file_hash_sha1",
                    "trend_micro_vision_one.endpoint_activity.object.file.hash_sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_file_path") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_file_path",
                    "trend_micro_vision_one.endpoint_activity.object.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_host_name") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_host_name",
                    "trend_micro_vision_one.endpoint_activity.object.host_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_integrity_level") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_integrity_level",
                    "trend_micro_vision_one.endpoint_activity.object.integrity_level",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_ip") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_ip",
                    "trend_micro_vision_one.endpoint_activity.object.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_ips") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_ips",
                    "trend_micro_vision_one.endpoint_activity.object.ips",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_port") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_port",
                    "trend_micro_vision_one.endpoint_activity.object.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_registry_data") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_registry_data",
                    "trend_micro_vision_one.endpoint_activity.object.registry.data",
                )?;
            }

            if event
                .has_value("trend_micro_vision_one.endpoint_activity.object_registry_key_handle")
            {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_registry_key_handle",
                    "trend_micro_vision_one.endpoint_activity.object.registry.key_handle",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_registry_value") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_registry_value",
                    "trend_micro_vision_one.endpoint_activity.object.registry.value",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_signer") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_signer",
                    "trend_micro_vision_one.endpoint_activity.object.signer",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_signer_valid") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_signer_valid",
                    "trend_micro_vision_one.endpoint_activity.object.signer_valid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_sub_true_type") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_sub_true_type",
                    "trend_micro_vision_one.endpoint_activity.object.sub_true_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_true_type") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_true_type",
                    "trend_micro_vision_one.endpoint_activity.object.true_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.object_user") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.object_user",
                    "trend_micro_vision_one.endpoint_activity.object.user",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.parent_cmd") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.parent_cmd",
                    "trend_micro_vision_one.endpoint_activity.parent.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.parent_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.parent_file_hash_sha1",
                    "trend_micro_vision_one.endpoint_activity.parent.file.hash_sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.parent_file_path") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.parent_file_path",
                    "trend_micro_vision_one.endpoint_activity.parent.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.process_cmd") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.process_cmd",
                    "trend_micro_vision_one.endpoint_activity.process.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.process_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.process_file_hash_sha1",
                    "trend_micro_vision_one.endpoint_activity.process.file.hash_sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.process_file_path") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.process_file_path",
                    "trend_micro_vision_one.endpoint_activity.process.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.spt") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.spt",
                    "trend_micro_vision_one.endpoint_activity.source.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.src") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.src",
                    "trend_micro_vision_one.endpoint_activity.source.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.src_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.src_file_hash_sha1",
                    "trend_micro_vision_one.endpoint_activity.source_file.hash_sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.src_file_path") {
                event.rename(
                    "trend_micro_vision_one.endpoint_activity.src_file_path",
                    "trend_micro_vision_one.endpoint_activity.source_file.path",
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.event.time")
                    && event.get_str("trend_micro_vision_one.endpoint_activity.event.time")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventTime = Long.parseLong(ctx.trend_micro_vision_one.endpoint_activity.event.time.toString()); if (eventTime < 10000000000L) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.time = eventTime * 1000L;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventTime = Long.parseLong(ctx.trend_micro_vision_one.endpoint_activity.event.time.toString()); if (eventTime < 10000000000L) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.time = eventTime * 1000L;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_normalize_event_time_to_unix_ms",
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

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.event.time")
                    && event.get_str("trend_micro_vision_one.endpoint_activity.event.time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.endpoint_activity.event.time")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.endpoint_activity.event.time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.endpoint_activity.event.time"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_time")?;
                    event.remove("trend_micro_vision_one.endpoint_activity.event.time");
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

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.event.time_dt")
                    && event.get_str("trend_micro_vision_one.endpoint_activity.event.time_dt")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.endpoint_activity.event.time_dt")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss[.SSSSSS][.SSS]XXX"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.endpoint_activity.event.time_dt",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.endpoint_activity.event.time_dt"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_time_dt")?;
                    event.remove("trend_micro_vision_one.endpoint_activity.event.time_dt");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.destination.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.destination.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.destination.port"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.destination.port",
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
                    "convert_destination_port_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.destination.port");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.event.sub_id") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.event.sub_id")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.event.sub_id"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.event.sub_id",
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
                    "convert_event_sub_id_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.event.sub_id");
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
                if event
                    .has_value("trend_micro_vision_one.endpoint_activity.object.integrity_level")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.object.integrity_level")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "trend_micro_vision_one.endpoint_activity.object.integrity_level".into(),
                            message,
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.object.integrity_level",
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
                    "convert_object_integrity_level_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.object.integrity_level");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.object.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.object.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.object.port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.object.port",
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
                    "convert_object_port_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.object.port");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.object.sub_true_type")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.object.sub_true_type")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.endpoint_activity.object.sub_true_type"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.object.sub_true_type",
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
                    "convert_object_sub_true_type_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.object.sub_true_type");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.object.true_type") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.object.true_type")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.object.true_type"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.object.true_type",
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
                    "convert_object_true_type_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.object.true_type");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.source.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.source.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.source.port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.source.port",
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
                    "convert_source_port_to_long",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.source.port");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.destination.address") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.destination.address")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.endpoint_activity.destination.address"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.destination.address",
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
                    "convert_destination_address_to_ip",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.destination.address");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.source.ip") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.source.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.source.ip",
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
                    "convert_source_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.source.ip");
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
                if event.has_value("trend_micro_vision_one.endpoint_activity.object.ip") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.endpoint_activity.object.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.endpoint_activity.object.ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.endpoint_activity.object.ip",
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
                    "convert_object_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.endpoint_activity.object.ip");
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
                event
                    .get("trend_micro_vision_one.endpoint_activity.endpoint.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.endpoint_activity.endpoint.ip")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_endpoint_ip_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
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
                            "trend_micro_vision_one.endpoint_activity.endpoint.ip",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.object.ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.endpoint_activity.object.ips")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_object_ips_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
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
                            "trend_micro_vision_one.endpoint_activity.object.ips",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.object.signer_valid")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.endpoint_activity.object.signer_valid")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_object_signer_valid_to_boolean",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
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
                            "trend_micro_vision_one.endpoint_activity.object.signer_valid",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.endpoint_activity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventId = ctx.trend_micro_vision_one.endpoint_activity.event?.id;\nif (eventId != null && params.eventId.containsKey(eventId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.id_value = params.eventId[eventId.toString()];\n}\ndef eventSubId = ctx.trend_micro_vision_one.endpoint_activity.event?.sub_id;\nif (eventSubId != null && params.eventSubId.containsKey(eventSubId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.sub_id_value = params.eventSubId[eventSubId.toString()];\n}\ndef objectIntegrityLevel = ctx.trend_micro_vision_one.endpoint_activity.object?.integrity_level;\nif (objectIntegrityLevel != null && params.objectIntegrityLevel.containsKey(objectIntegrityLevel.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.integrity_level_value = params.objectIntegrityLevel[objectIntegrityLevel.toString()];\n}\ndef objectTrueType = ctx.trend_micro_vision_one.endpoint_activity.object?.true_type;\nif (objectTrueType != null && params.objectTrueType.containsKey(objectTrueType.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.true_type_value = params.objectTrueType[objectTrueType.toString()];\n}\ndef objectSubTrueType = ctx.trend_micro_vision_one.endpoint_activity.object?.sub_true_type;\nif (objectSubTrueType != null && params.objectSubTrueType.containsKey(objectSubTrueType.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.sub_true_type_value = params.objectSubTrueType[objectSubTrueType.toString()];\n}\ndef winEventId = ctx.trend_micro_vision_one.endpoint_activity.win_event_id;\nif (winEventId != null && params.winEventId.containsKey(winEventId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.win_event_id_value = params.winEventId[winEventId.toString()];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def eventId = ctx.trend_micro_vision_one.endpoint_activity.event?.id;\nif (eventId != null && params.eventId.containsKey(eventId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.id_value = params.eventId[eventId.toString()];\n}\ndef eventSubId = ctx.trend_micro_vision_one.endpoint_activity.event?.sub_id;\nif (eventSubId != null && params.eventSubId.containsKey(eventSubId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.event.sub_id_value = params.eventSubId[eventSubId.toString()];\n}\ndef objectIntegrityLevel = ctx.trend_micro_vision_one.endpoint_activity.object?.integrity_level;\nif (objectIntegrityLevel != null && params.objectIntegrityLevel.containsKey(objectIntegrityLevel.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.integrity_level_value = params.objectIntegrityLevel[objectIntegrityLevel.toString()];\n}\ndef objectTrueType = ctx.trend_micro_vision_one.endpoint_activity.object?.true_type;\nif (objectTrueType != null && params.objectTrueType.containsKey(objectTrueType.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.true_type_value = params.objectTrueType[objectTrueType.toString()];\n}\ndef objectSubTrueType = ctx.trend_micro_vision_one.endpoint_activity.object?.sub_true_type;\nif (objectSubTrueType != null && params.objectSubTrueType.containsKey(objectSubTrueType.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.object.sub_true_type_value = params.objectSubTrueType[objectSubTrueType.toString()];\n}\ndef winEventId = ctx.trend_micro_vision_one.endpoint_activity.win_event_id;\nif (winEventId != null && params.winEventId.containsKey(winEventId.toString())) {\n  ctx.trend_micro_vision_one.endpoint_activity.win_event_id_value = params.winEventId[winEventId.toString()];\n}"#
                        ),
                        cached_params!(
                            "{\"eventId\":{\"1\":\"EVENT_PROCESS\",\"2\":\"EVENT_FILE\",\"3\":\"EVENT_CONNECTIO\",\"4\":\"EVENT_DNS\",\"5\":\"EVENT_REGISTRY\",\"6\":\"EVENT_ACCOUNT\",\"7\":\"EVENT_INTERNET\",\"8\":\"XDR_EVENT_MODIFIED_PROCESS\",\"9\":\"EVENT_WINDOWS_HOOK\",\"10\":\"EVENT_WINDOWS_EVENT\",\"11\":\"EVENT_AMSI\",\"12\":\"EVENT_WMI\",\"13\":\"TELEMETRY_MEMORY\",\"14\":\"TELEMETRY_BM\"},\"eventSubId\":{\"0\":\"TELEMETRY_NONE\",\"1\":\"XDR_PROCESS_OPEN\",\"2\":\"XDR_PROCESS_CREATE\",\"3\":\"XDR_PROCESS_TERMINATE\",\"4\":\"XDR_PROCESS_LOAD_IMAGE\",\"5\":\"TELEMETRY_PROCESS_EXECUTE\",\"6\":\"TELEMETRY_PROCESS_CONNECT\",\"7\":\"TELEMETRY_PROCESS_TRACME\",\"101\":\"XDR_FILE_CREATE\",\"102\":\"XDR_FILE_OPEN\",\"103\":\"XDR_FILE_DELETE\",\"104\":\"XDR_FILE_SET_SECURITY\",\"105\":\"XDR_FILE_COPY\",\"106\":\"XDR_FILE_MOVE\",\"107\":\"XDR_FILE_CLOSE\",\"108\":\"TELEMETRY_FILE_MODIFY_TIMESTAMP\",\"109\":\"TELEMETRY_FILE_MODIFY\",\"201\":\"XDR_CONNECTION_CONNECT\",\"202\":\"XDR_CONNECTION_LISTEN\",\"203\":\"XDR_CONNECTION_CONNECT_INBOUND\",\"204\":\"XDR_CONNECTION_CONNECT_OUTBOUND\",\"301\":\"XDR_DNS_QUERY\",\"401\":\"XDR_REGISTRY_CREATE\",\"402\":\"XDR_REGISTRY_SET\",\"403\":\"XDR_REGISTRY_DELETE\",\"404\":\"XDR_REGISTRY_RENAME\",\"501\":\"XDR_ACCOUNT_ADD\",\"502\":\"XDR_ACCOUNT_DELETE\",\"503\":\"XDR_ACCOUNT_IMPERSONATE\",\"504\":\"XDR_ACCOUNT_MODIFY\",\"601\":\"XDR_INTERNET_OPEN\",\"602\":\"XDR_INTERNET_CONNECT\",\"603\":\"XDR_INTERNET_DOWNLOAD\",\"701\":\"XDR_MODIFIED_PROCESS_CREATE_REMOTETHREAD\",\"702\":\"XDR_MODIFIED_PROCESS_WRITE_MEMORY\",\"703\":\"TELEMETRY_MODIFIED_PROCESS_WRITE_PROCESS\",\"704\":\"TELEMETRY_MODIFIED_PROCESS_READ_PROCESS\",\"705\":\"TELEMETRY_MODIFIED_PROCESS_WRITE_PROCESS_NAME\",\"801\":\"XDR_WINDOWS_HOOK_SET\",\"901\":\"XDR_AMSI_EXECUTE\",\"1001\":\"TELEMETRY_MEMORY_MODIFY\",\"1002\":\"TELEMETRY_MEMORY_MODIFY_PERMISSION\",\"1003\":\"TELEMETRY_MEMORY_READ\",\"1101\":\"TELEMETRY_BM_INVOKE\",\"1102\":\"TELEMETRY_BM_INVOKE_API\"},\"objectIntegrityLevel\":{\"0\":\"Untrusted\",\"4096\":\"Low\",\"8192\":\"Medium\",\"12288\":\"High\",\"16384\":\"System\"},\"objectTrueType\":{\"-1\":\"Uninitialized type\",\"-2\":\"Unknown type\",\"1\":\"Word for Windows\",\"2\":\"Windows PowerPoint\",\"3\":\"Windows Font\",\"4\":\"Excel for Windows\",\"5\":\"COM\",\"6\":\"Windows Icon\",\"7\":\"EXE\",\"8\":\"SUN GKS\",\"9\":\"MSCOMP\",\"10\":\"PCX\",\"11\":\"unix cpio archive\",\"12\":\"PPM image\",\"13\":\"LHA\",\"14\":\"unix ar archive\",\"15\":\"ARC\",\"16\":\"Windows Write\",\"17\":\"Windows Calendar\",\"18\":\"ASCII text\",\"19\":\"ELF\",\"20\":\"TAR\",\"21\":\"TeleDisk Image\",\"22\":\"AutoDesk Animator(FLI or FLC)\",\"23\":\"empty file(size 0)\",\"24\":\"NT/95 shortcut(*.lnk)\",\"25\":\"RAR\",\"26\":\"Microsoft Access(MDB)\",\"27\":\"MAC\",\"28\":\"VBScript\",\"29\":\"Script File Type match\",\"30\":\"Project for Windows\",\"31\":\"Advanced Streaming Format\",\"32\":\"Quick Time Media\",\"33\":\"MPEG\",\"34\":\"Portable Network Graphics\",\"35\":\"Pain Shop Pro\",\"36\":\"Targa Image\",\"37\":\"Macintosh Bitmap\",\"38\":\"Apple Sound\",\"39\":\"Encapsulated Postscript\",\"40\":\"Audio InterChange File Format from Apple/SGI\",\"41\":\"Animated Cursor USELESS since moving to VSDT_RIFF_ANI\",\"42\":\"TerraGen ATMosphere\",\"43\":\"Nullsoft AVS Files\",\"44\":\"SGI Image\",\"45\":\"Cinema 4D\",\"46\":\"BAR CDA Music Track File Format USELESS since moving to VSDT_RIFF_CDA\",\"47\":\"Computer Graphics Metafiles\",\"48\":\"CHL File\",\"49\":\"Corel Presentation Exchange USELESS since moving to VSDT_RIFF_CMX\",\"51\":\"Caligari TrueSpace File\",\"52\":\"Visual C Obj File\",\"53\":\"Macromedia Director Cast\",\"54\":\"Macromedia Director Shockwave Movie\",\"57\":\"Diamondware Dgitized Sound\",\"58\":\"AutoCAD DWG\",\"61\":\"Free Hand Document\",\"65\":\"SoftImage\",\"66\":\"Amiga 8SVX Audio InterChange File Format\",\"68\":\"Interleaf Image\",\"69\":\"GEM Image\",\"70\":\"Imagine 3D Object\",\"71\":\"Uninstall Scripts\",\"72\":\"InterVoice Files\",\"74\":\"LightWave 3D Object\",\"75\":\"Matlab Sound\",\"76\":\"MAUD Sample Format\",\"78\":\"Magick Image File Format\",\"79\":\"Media Catalog\",\"80\":\"Multiple-image Network Graphics\",\"81\":\"Atari Neochrome\",\"83\":\"Gravis Patch Files\",\"84\":\"PalmPilot Image\",\"85\":\"Adobe Font File\",\"86\":\"Shortcut to Microsoft Program\",\"87\":\"Real Audio\",\"90\":\"WaveFront RLA\",\"92\":\"Sculpt 3D/4D Scene\",\"94\":\"Lotus ScreenCam Movie\",\"95\":\"MIDI Sample Sound\",\"96\":\"IRCAM\",\"97\":\"Sonic Foundry File\",\"98\":\"Solitaire Image Recorder\",\"99\":\"SampleVision Sound\",\"100\":\"Sndtool Sound File\",\"101\":\"TerraGen Surface\",\"102\":\"TerraGen Terrain\",\"103\":\"TerraGen World\",\"104\":\"Yamaha tx-16w\",\"106\":\"Convox V8 File\",\"107\":\"Bitmap Image YUV12\",\"109\":\"Webshots Collection\",\"110\":\"Windows Metafile\",\"112\":\"Psion Audio Files\",\"115\":\"Macintosh MacBinary\",\"116\":\"Mail Box (MicroSoft Outlook 4.x or Unix-based)\",\"117\":\"Script User-Defined Type match (Reyer\",\"118\":\"Script Customer-Defined Type match (Reyer\",\"119\":\"Corel Global Macro (aldous\",\"120\":\"Corel PhotoPaint (aldous\",\"121\":\"GNU BZIP2\",\"122\":\"WordPro\",\"123\":\"Windows Installer\",\"124\":\"JP Government file\",\"125\":\"ACE compression file\",\"126\":\"EPOC file\",\"127\":\"Windows Process Memory\",\"128\":\"EPOC sis file (sis)\",\"129\":\"Dalvik VM DEX file (VSAPI 9.700)\",\"200\":\"LAMA file\",\"201\":\".xz file\",\"202\":\"7-ZIP file\",\"203\":\"MS Enhanced Metafile format (VSAPI 9.700)\",\"204\":\"Binary Property List\",\"205\":\"UHarc Compressed Archive\",\"206\":\"Hangul Document (Korean) (ATSE 9.740)\",\"207\":\"Microsoft Windows Imaging Format file\",\"208\":\"ALZ .alz file\",\"209\":\"ALZ .egg file\",\"4000\":\"unix core file\",\"4001\":\"Windows Group\",\"4002\":\"JPEG\",\"4003\":\"PKZIP\",\"4004\":\"Audio\",\"4005\":\"JAVA Aplet\",\"4006\":\"PA-RISC executable\",\"4007\":\"PA-RISC demand-load executable\",\"4008\":\"PA-RISC shared executable\",\"4009\":\"PA-RISC dynamic load library\",\"4010\":\"PA-RISC shared library\",\"4011\":\"Compiled LISP\",\"4012\":\"HP-WINDOWS font\",\"4013\":\"MMDF mail box\",\"4014\":\"HP s800 executable\",\"4015\":\"HP s800 shared executable\",\"4016\":\"HP s800 demand-load executable\",\"4017\":\"HP s800 shared library\",\"4018\":\"HP s800 dynamic load library\",\"4019\":\"PA-RISC relocatable object\",\"4020\":\"Microsoft RIFF\",\"4021\":\"Microsoft Paint v1.x\",\"4022\":\"Microsoft Paint v2.x\",\"4023\":\"Creative Lab CMF\",\"4024\":\"TIFF\",\"4025\":\"WordPerfect\",\"4026\":\"Sun Raster(RAS)\",\"4027\":\"Adobe PhotoShop(PSD)\",\"4028\":\"MIDI\",\"4029\":\"MS word/DOS 4.0/5.0\",\"4030\":\"MS Cabinet\",\"4031\":\"MP3\",\"4032\":\"MSFT(TLB\",\"4033\":\"HLP\",\"4034\":\"BND\",\"4035\":\"Trend backup file\",\"4036\":\"Real Media\",\"4037\":\"True Type Collection\",\"4038\":\"Macromedia Flash\",\"4039\":\"Compiled HTML (CHM)\",\"4040\":\"Corel Draw file\",\"4041\":\"IBM AS400 saving file\",\"4042\":\"Lotus Notes Database\",\"4043\":\"Encapsulated Postcript (EPS)\",\"4044\":\"QuarkXPress Document (QXD)\",\"4045\":\"added by Lucy Office12\",\"4046\":\"Microsoft Document Imaging\",\"4047\":\"Macromedia Flash FLV Video\",\"4048\":\"Open Document\",\"4049\":\"Java Archive (JAR) file (VSAPI 9.700)\",\"4050\":\"Android Application Package file (APK) (VSAPI 9.700)\",\"4051\":\"Fat binary file\",\"4052\":\"Mach object file format (Mach-O)\",\"4053\":\"Reserved for VSAPI 9.718 - Binary XML (AndroidManifest.xml) (VSAPI 9.718)\",\"4054\":\"PST Outlook PST File\",\"4055\":\"MS OneNote File\",\"4056\":\"Extensible Archiver (XAR)\",\"4057\":\"Apple Disk Image\",\"4058\":\"Reserved for VSAPI 9.718 - PKCS7 Certificate file\",\"4059\":\"Chrome Extension Format (CRX)\",\"6000\":\"UUENCODE\",\"6001\":\"Adobe Font\",\"6002\":\"BINHEX\",\"6003\":\"Windows Cardfile\",\"6004\":\"Frame Maker\",\"6005\":\"GIF\",\"6006\":\"Netware Loadable Module\",\"6007\":\"Postscript\",\"6008\":\"Microsoft RTF\",\"6010\":\"Mime base 64\",\"6011\":\"Novell system PrinfDef Device Definition\",\"6012\":\"Novell Help Librarian data file\",\"6013\":\"NetWare Unicode Rule Table file\",\"6014\":\"Creative Voice Format(VOC)\",\"6015\":\"Adobe Portable Document Format file\",\"6016\":\"Macors in MS Office compressed by ActiveMime\",\"6017\":\"Aladdin StuffIt Archive\",\"6018\":\"YEncode\",\"6019\":\"Microsoft Shell Link\",\"6020\":\"ICalendar\",\"2000\":\"ARJ\",\"2001\":\"Windows BMP\",\"2002\":\"Windows Clipboard\",\"2003\":\"GNU ZIP\",\"2004\":\"LZW\",\"2005\":\"Compiled Terminfo entry\",\"1000\":\"Fujitsu AMG compressed type\",\"1001\":\"Outlook Item(TNEF)\",\"1002\":\"ZLib format\",\"1003\":\"Office XML\",\"1004\":\"Hancom Office 2014\",\"1005\":\"ISO image\",\"1006\":\"True Type Font\"},\"objectSubTrueType\":{\"1000\":\"MS Office or Unknown OLE\",\"1001\":\"Winword 2.0\",\"1002\":\"Winword 1.0\",\"1003\":\"Hangul Word Processor (Korean)\",\"1004\":\"Ichitaro (Japan)\",\"1005\":\"JungUm Global (Korean) (ATSE 9.740)\",\"1006\":\"Outlook Item (.msg) (ATSE 9.755)\",\"1007\":\"MS Publisher (.pub) (ATSE 9.862)\",\"4000\":\"MS Office\",\"4001\":\"Hancom Hancell (ATSE 9.740)\",\"4045000\":\"Unknown type (MS Office 2007)\",\"4045001\":\"WORD (MS Office 2007)\",\"4045002\":\"Excel (MS Office 2007)\",\"4045003\":\"PTT (MS Office2007)\",\"4045004\":\"XPS (Office 2007)\",\"1004000\":\"Hwp Standard OWPML Document\",\"1003000\":\"Unknown XML Document\",\"1003001\":\"Word 2003 XML Document\",\"1003002\":\"XML Spreadsheet 2003\",\"1003003\":\"PowerPoint XML Presentation\",\"22000\":\".FLI: AutoDesk Animator\",\"22001\":\".FLC: AutoDesk 3D studio\",\"22002\":\".FLIC:AutoDesk Animator Pro\",\"16000\":\"Windows Write\",\"16001\":\"Word for DOS\",\"7000\":\"DOS EXE\",\"7001\":\"WIN16 EXE\",\"7002\":\"WIN32 EXE\",\"7003\":\"OS2 EXE\",\"7004\":\"WIN16 DLL\",\"7005\":\"Win32 DLL\",\"7006\":\"Windows VxD\",\"7007\":\"OS/2 2.x VxD\",\"7008\":\"NT/MIPS EXE\",\"7009\":\"PKLITE EXE\",\"7010\":\"LZEXE\",\"7011\":\"DIET EXE\",\"7012\":\"PKZIP EXE\",\"7013\":\"ARJ EXE\",\"7014\":\"LZH EXE\",\"7015\":\"LZH EXE used by ZipMail\",\"7016\":\"ASPACK\",\"7017\":\"UPX EXE\",\"7018\":\"MSIL\",\"7019\":\"ASPACK 2.x\",\"7020\":\"WWPACK\",\"7021\":\"PETITE\",\"7022\":\"PEPACK\",\"7023\":\"MEW 1.1\",\"7024\":\"MEW 0.5\",\"7025\":\"MEW 1.0\",\"7026\":\"AMD64 EXE\",\"7027\":\"AMD64 DLL\",\"7028\":\"ARM EXE\",\"7029\":\"THUNB EXE\",\"7030\":\"Miscellaneous EXE\",\"7031\":\"UPX64\",\"7032\":\"DLL Not Program\",\"5000\":\"DOS COM\",\"5001\":\"PKLITE COM\",\"5002\":\"DIET COM\",\"5003\":\"LZH COM\",\"19000\":\"ELF File\",\"19001\":\"ELF Reloactable File\",\"19002\":\"ELF EXE File\",\"19003\":\"ELF Library File\",\"19004\":\"ELF Coredump file\",\"126000\":\"EPOC Binary File\",\"126001\":\"EPOC EXE File\",\"126002\":\"EPOC Library File\",\"126003\":\"EPOC Compressed file\",\"128000\":\"SIS File\",\"128001\":\"SIS File\",\"6015000\":\"PDF\",\"6015001\":\"PDF\",\"6015002\":\"PDF\",\"6015003\":\"PDF\",\"6015004\":\"PDF\",\"6015005\":\"PDF\",\"1005000\":\"ISO 9660 File\",\"1005001\":\"UDF File\",\"6004000\":\"Frame Maker documentation file\",\"6004001\":\"Frame Maker MIF file\",\"6004002\":\"Frame Maker MML file\",\"6004003\":\"Frame Maker Book file\",\"6004004\":\"Frame Maker dictionary file\",\"6004005\":\"Frame Maker font file\",\"6004006\":\"Frame Maker IPL\",\"6001000\":\"Adobe font metrics\",\"6001001\":\"Adobe font bits\",\"4020000\":\"AVI\",\"4020001\":\"WAV\",\"4020002\":\"BND\",\"4020003\":\"RMI\",\"4020004\":\"RDI\",\"4020005\":\"CDA\",\"4020006\":\"ANI\",\"4020007\":\"CMX\",\"2004000\":\"compressed 16 bits\",\"2004001\":\"packed data\",\"2004002\":\"compacked data\",\"2004003\":\"SCO compressed\",\"28000\":\"Script - VB Script/Javascript\",\"28001\":\"HTML File\",\"28002\":\"PALM Resource Code File\",\"28003\":\"ASP file\",\"28004\":\"General Text File\",\"28005\":\"Action Script\",\"28006\":\"XML Data Package\",\"115000\":\"MACBIN\",\"115001\":\"MACBIN\",\"115002\":\"MACBIN\",\"58000\":\"AutoCAD DWG\",\"58001\":\"AutoCAD R2000\",\"116000\":\"MBX_OUTLOOK4\",\"116001\":\"MBX_Unix\",\"116002\":\"MBX_FOXMAIL\",\"26000\":\"MS Access(MDB)\",\"26001\":\"MS Access 2000/XP\",\"26002\":\"MS Access(MDB)2.0\",\"26003\":\"MS Access 2007\",\"6016000\":\"Outlook MSO File\",\"6016001\":\"Exchange MSO DATA\",\"4048000\":\"Unknown OpenDocument File\",\"4048001\":\"OpenDocument Text File\",\"4048002\":\"OpenDocument Graphic File\",\"4048003\":\"OpenDocument Presentation File\",\"4048004\":\"OpenDocument Spreadsheet File\",\"4048005\":\"OpenDocument Formula File\",\"4048006\":\"OpenDocument Database File\",\"4038000\":\"Uncompressed SWF\",\"4038001\":\"Compressed SWF\",\"4038002\":\"LZMA Compressed SWF\",\"4052000\":\"Unnown Match Object File\",\"4052001\":\"X86 Match Object File\",\"4052002\":\"X64 Match Object File\",\"4055000\":\"ONENOTE_ONE\",\"4055001\":\"ONENOTE_ONETOC2\",\"4055002\":\"ONENOTE_ONE_FSSHTTP\",\"4055003\":\"ONENOTE_ONETOC2_FSSHTTP\",\"8000001\":\"FILESS_REG\",\"8000002\":\"FILESS_WMI\",\"8000003\":\"FILESS_SCHEDULE_TASK\",\"8000004\":\"FILESS_AMSI\",\"8000005\":\"FILESS_MIP3\",\"8000006\":\"FILESS_CMD\",\"8000007\":\"FILESS_MIP3_X86_64\"},\"winEventId\":{\"3\":\"The BITS service created a new job\",\"11\":\"WMI operation new Trigger\",\"12\":\"WMI operation created operational\",\"21\":\"Remote Desktop Services: Session logon succeeded\",\"25\":\"Remote Desktop Services: Session reconnection succeeded\",\"100\":\"Task started\",\"104\":\"The system audit log was cleared\",\"106\":\"Task registered\",\"110\":\"Task triggered by user\",\"119\":\"Scheduled Task is triggered at logon\",\"129\":\"Created task process\",\"151\":\"DomainModuleLoad_V1\",\"152\":\"ModuleLoad_V2\",\"154\":\"AssemblyLoad_V1\",\"200\":\"Action started\",\"201\":\"Action completed\",\"300\":\"Microsoft Office Alerts\",\"1001\":\"RegisterRawInputDevices is invoked\",\"1003\":\"Multiple GetAsyncKeyState are invoked\",\"1034\":\"TcpConnectTcbFailure\",\"1074\":\"System has initiated shutdown/restart\",\"1102\":\"The audit log was cleared\",\"1149\":\"Remote Desktop Services: User authentication succeeded\",\"2004\":\"A rule has been added to the Windows Firewall exception list\",\"4104\":\"Powershell Script Block Logging\",\"4624\":\"An account was successfully logged on\",\"4625\":\"An account failed to log on\",\"4634\":\"An account was logged off\",\"4648\":\"A logon was attempted using explicit credentials\",\"4661\":\"A handle to an object was requested\",\"4663\":\"An attempt was made to access an object\",\"4670\":\"Permissions on an object were changed\",\"4672\":\"Special privileges assigned to new logon\",\"4695\":\"Unprotection of auditable protected data was attempted.\",\"4697\":\"A service was installed in the system\",\"4698\":\"A scheduled task was created\",\"4699\":\"A scheduled task was deleted\",\"4700\":\"A scheduled task was enabled\",\"4701\":\"A scheduled task was disabled\",\"4702\":\"A scheduled task was updated\",\"4703\":\"A user right was adjusted\",\"4720\":\"A user account was created\",\"4722\":\"A user account was enabled\",\"4723\":\"An attempt was made to change an account's password\",\"4724\":\"An attempt was made to reset an account's password\",\"4738\":\"A user account was changed\",\"4740\":\"A user account was locked out\",\"4768\":\"A Kerberos authentication ticket (TGT) was requested\",\"4769\":\"A Kerberos service ticket was requested\",\"4778\":\"A session was reconnected to a Window Station\",\"4779\":\"A session was disconnected from a Window Station\",\"5140\":\"A network share object was accessed\",\"5156\":\"The Windows Filtering Platform has permitted a connection.\",\"5379\":\"Credential Manager credentials were read.\",\"5381\":\"Vault credentials were read.\",\"5827\":\"NetLogon Connection Denied (machine account)\",\"5828\":\"NetLogon Connection Denied (trust account)\",\"5829\":\"NetLogon Connection Allowed\",\"5830\":\"NetLogon Connection Allowed by Group Policy (machine account)\",\"5831\":\"NetLogon Connection Allowed by Group Policy (trust account)\",\"5857\":\"WMI operation started operational\",\"5859\":\"WMI operation ESS(Event SubSystem) started\",\"5860\":\"WMI operation temporary ESS started\",\"5861\":\"WMI operation ESS to consumer binding\",\"7040\":\"The start type of a service has been changed\",\"7045\":\"A service was installed in the system\",\"8225\":\"VSS (Volume Shadow Copy Service) operation\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_resolve_numeric_field_values",
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
                .get("trend_micro_vision_one.endpoint_activity.event.time_dt")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("trend_micro_vision_one.endpoint_activity.win_event_id") {
                if let Some(val) =
                    event.get("trend_micro_vision_one.endpoint_activity.win_event_id")
                {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.endpoint_activity.win_event_id".into(),
                            message,
                        }
                    })?;
                    event.set("event.code", converted)?;
                }
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.address", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.endpoint.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.endpoint.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.endpoint_activity.endpoint.ip",
                    |event| {
                        event.append_unique(
                            "host.ip",
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

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.source.ip")
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

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.logon_user")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.endpoint_activity.logon_user",
                    |event| {
                        event.append_unique(
                            "user.name",
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

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.object.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.process.cmd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.process.file.hash_sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.process.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.parent.cmd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.parent.file.hash_sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.parent.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.object.file.hash_sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.object.file.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.request")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.object.registry.key_handle")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.path", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.endpoint_activity.object.registry.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("registry.value", v)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.endpoint.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.endpoint_activity.endpoint.ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
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

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.object.ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.endpoint_activity.object.ips",
                    |event| {
                        event.append_unique(
                            "related.ip",
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

            let _cond = { event.has_value("trend_micro_vision_one.endpoint_activity.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.endpoint_activity.destination.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.destination.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.endpoint_activity.object.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.object.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.endpoint_activity.logon_user")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.endpoint_activity.logon_user",
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

            let _cond = { event.has_value("trend_micro_vision_one.endpoint_activity.object.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.object.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.object.file.hash_sha1")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.object.file.hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.process.file.hash_sha1")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.process.file.hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.parent.file.hash_sha1")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.parent.file.hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.endpoint_activity.source_file.hash_sha1")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.source_file.hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.endpoint_activity.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.endpoint_activity.endpoint.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.endpoint.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.endpoint_activity.object.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.endpoint_activity.object.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                event.remove("trend_micro_vision_one.endpoint_activity.destination.address");
                event.remove("trend_micro_vision_one.endpoint_activity.destination.port");
                event.remove("trend_micro_vision_one.endpoint_activity.endpoint.host_name");
                event.remove("trend_micro_vision_one.endpoint_activity.endpoint.ip");
                event.remove("trend_micro_vision_one.endpoint_activity.event.id");
                event.remove("trend_micro_vision_one.endpoint_activity.event.time_dt");
                event.remove("trend_micro_vision_one.endpoint_activity.host_name");
                event.remove("trend_micro_vision_one.endpoint_activity.logon_user");
                event.remove("trend_micro_vision_one.endpoint_activity.object.file.hash_sha1");
                event.remove("trend_micro_vision_one.endpoint_activity.object.file.path");
                event.remove("trend_micro_vision_one.endpoint_activity.object.registry.key_handle");
                event.remove("trend_micro_vision_one.endpoint_activity.object.registry.value");
                event.remove("trend_micro_vision_one.endpoint_activity.object.user");
                event.remove("trend_micro_vision_one.endpoint_activity.os");
                event.remove("trend_micro_vision_one.endpoint_activity.parent.cmd");
                event.remove("trend_micro_vision_one.endpoint_activity.parent.file.hash_sha1");
                event.remove("trend_micro_vision_one.endpoint_activity.parent.file.path");
                event.remove("trend_micro_vision_one.endpoint_activity.process.cmd");
                event.remove("trend_micro_vision_one.endpoint_activity.process.file.hash_sha1");
                event.remove("trend_micro_vision_one.endpoint_activity.process.file.path");
                event.remove("trend_micro_vision_one.endpoint_activity.request");
                event.remove("trend_micro_vision_one.endpoint_activity.source.ip");
                event.remove("trend_micro_vision_one.endpoint_activity.source.port");
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
