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
            let _cond = { event.get_str("event.reason") == Some("want_more") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("9.3.0"))?;

            event.set("event.kind", json!("event"))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "trend_micro_vision_one.telemetry")?;
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

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.trend_micro_vision_one?.telemetry != null) {\n  ctx.trend_micro_vision_one.telemetry = convertToSnakeCase(ctx.trend_micro_vision_one.telemetry);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.trend_micro_vision_one?.telemetry != null) {\n  ctx.trend_micro_vision_one.telemetry = convertToSnakeCase(ctx.trend_micro_vision_one.telemetry);\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def key = (ctx.trend_micro_vision_one?.telemetry?.event_id ?: \"\").toString();\nif (params.containsKey(key)) {\n  ctx.trend_micro_vision_one.telemetry.event_type = params[key];\n} else {\n  ctx.trend_micro_vision_one.telemetry.event_type = 'Other';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def key = (ctx.trend_micro_vision_one?.telemetry?.event_id ?: \"\").toString();\nif (params.containsKey(key)) {\n  ctx.trend_micro_vision_one.telemetry.event_type = params[key];\n} else {\n  ctx.trend_micro_vision_one.telemetry.event_type = 'Other';\n}\n"#
                ),
                cached_params!(
                    "{\"1\":\"TELEMETRY_PROCESS\",\"2\":\"TELEMETRY_FILE\",\"3\":\"TELEMETRY_CONNECTION\",\"4\":\"TELEMETRY_DNS\",\"5\":\"TELEMETRY_REGISTRY\",\"6\":\"TELEMETRY_ACCOUNT\",\"7\":\"TELEMETRY_INTERNET\",\"8\":\"TELEMETRY_MODIFIED_PROCESS\",\"9\":\"TELEMETRY_WINDOWS_HOOK\",\"10\":\"TELEMETRY_WINDOWS_EVENT\",\"11\":\"TELEMETRY_AMSI\",\"12\":\"TELEMETRY_WMI\",\"13\":\"TELEMETRY_MEMORY\",\"14\":\"TELEMETRY_BM\",\"15\":\"TELEMETRY_APP\",\"16\":\"TELEMETRY_SYSTEM_EVENT\",\"17\":\"TELEMETRY_EVENT_PIPE\",\"18\":\"TELEMETRY_MAC_SYS_LOG\",\"19\":\"TELEMETRY_DDR\",\"101\":\"TELEMETRY_ASSOCIATION\"}"
                ),
            )?;

            // Painless script
            // Source: def key = (ctx.trend_micro_vision_one?.telemetry?.event_sub_id ?: \"\").toString();\nif (params.containsKey(key)) {\n  ctx.trend_micro_vision_one.telemetry.event_subtype = params[key];\n} else {\n  ctx.trend_micro_vision_one.telemetry.event_subtype = 'Other';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def key = (ctx.trend_micro_vision_one?.telemetry?.event_sub_id ?: \"\").toString();\nif (params.containsKey(key)) {\n  ctx.trend_micro_vision_one.telemetry.event_subtype = params[key];\n} else {\n  ctx.trend_micro_vision_one.telemetry.event_subtype = 'Other';\n}\n"#
                ),
                cached_params!(
                    "{\"0\":\"TELEMETRY_NONE\",\"1\":\"TELEMETRY_PROCESS_OPEN\",\"2\":\"TELEMETRY_PROCESS_CREATE\",\"3\":\"TELEMETRY_PROCESS_TERMINATE\",\"4\":\"TELEMETRY_PROCESS_LOAD_IMAGE\",\"5\":\"TELEMETRY_PROCESS_EXECUTE\",\"6\":\"TELEMETRY_PROCESS_CONNECT\",\"7\":\"TELEMETRY_PROCESS_TRACME\",\"8\":\"TELEMETRY_PROCESS_LOAD_KERNEL_IMAGE\",\"101\":\"TELEMETRY_FILE_CREATE\",\"102\":\"TELEMETRY_FILE_OPEN\",\"103\":\"TELEMETRY_FILE_DELETE\",\"104\":\"TELEMETRY_FILE_SET_SECURITY\",\"105\":\"TELEMETRY_FILE_COPY\",\"106\":\"TELEMETRY_FILE_MOVE\",\"107\":\"TELEMETRY_FILE_CLOSE\",\"108\":\"TELEMETRY_FILE_MODIFY_TIMESTAMP\",\"109\":\"TELEMETRY_FILE_MODIFY\",\"110\":\"TELEMETRY_FILE_SET_ATTRIBUTES\",\"111\":\"TELEMETRY_FILE_ENUMERATE\",\"112\":\"TELEMETRY_FILE_SET_EXTENDED_ATTRIBUTE\",\"113\":\"TELEMETRY_FILE_DELETE_EXTENDED_ATTRIBUTE\",\"201\":\"TELEMETRY_CONNECTION_CONNECT\",\"202\":\"TELEMETRY_CONNECTION_LISTEN\",\"203\":\"TELEMETRY_CONNECTION_CONNECT_INBOUND\",\"204\":\"TELEMETRY_CONNECTION_CONNECT_OUTBOUND\",\"301\":\"TELEMETRY_DNS_QUERY\",\"401\":\"TELEMETRY_REGISTRY_CREATE\",\"402\":\"TELEMETRY_REGISTRY_SET\",\"403\":\"TELEMETRY_REGISTRY_DELETE\",\"404\":\"TELEMETRY_REGISTRY_RENAME\",\"405\":\"TELEMETRY_REGISTRY_ENUMERATE\",\"406\":\"TELEMETRY_REGISTRY_ENUMERATEVALUE\",\"407\":\"TELEMETRY_REGISTRY_QUERYVALUE\",\"408\":\"TELEMETRY_REGISTRY_SAVE\",\"501\":\"TELEMETRY_ACCOUNT_ADD\",\"502\":\"TELEMETRY_ACCOUNT_DELETE\",\"503\":\"TELEMETRY_ACCOUNT_IMPERSONATE\",\"504\":\"TELEMETRY_ACCOUNT_MODIFY\",\"505\":\"TELEMETRY_ACCOUNT_LOGIN\",\"506\":\"TELEMETRY_ACCOUNT_LOGOUT\",\"601\":\"TELEMETRY_INTERNET_OPEN\",\"602\":\"TELEMETRY_INTERNET_CONNECT\",\"603\":\"TELEMETRY_INTERNET_DOWNLOAD\",\"701\":\"TELEMETRY_MODIFIED_PROCESS_CREATE_REMOTETHREAD\",\"702\":\"TELEMETRY_MODIFIED_PROCESS_WRITE_MEMORY\",\"703\":\"TELEMETRY_MODIFIED_PROCESS_WRITE_PROCESS\",\"704\":\"TELEMETRY_MODIFIED_PROCESS_READ_PROCESS\",\"705\":\"TELEMETRY_MODIFIED_PROCESS_WRITE_PROCESS_NAME\",\"801\":\"TELEMETRY_WINDOWS_HOOK_SET\",\"901\":\"TELEMETRY_AMSI_EXECUTE\",\"1001\":\"TELEMETRY_MEMORY_MODIFY\",\"1002\":\"TELEMETRY_MEMORY_MODIFY_PERMISSION\",\"1003\":\"TELEMETRY_MEMORY_READ\",\"1101\":\"TELEMETRY_BM_INVOKE\",\"1102\":\"TELEMETRY_BM_INVOKE_API\",\"1201\":\"TELEMETRY_APP_START\",\"1202\":\"TELEMETRY_APP_STOP\",\"1203\":\"TELEMETRY_APP_INSTALL\",\"1204\":\"TELEMETRY_APP_UNINSTALL\",\"1205\":\"TELEMETRY_APP_BEHAVIOR\",\"1301\":\"TELEMETRY_SYSTEM_EVENT_ENABLE\",\"1302\":\"TELEMETRY_SYSTEM_EVENT_DISABLE\",\"1303\":\"TELEMETRY_SYSTEM_CERTIFICATION_INSTALL\",\"1304\":\"TELEMETRY_SYSTEM_DEVICE_ROOTED\",\"1401\":\"TELEMETRY_PIPE_CREATE\",\"1402\":\"TELEMETRY_PIPE_CONNECT\",\"1601\":\"TELEMETRY_MAC_SYS_LOG_COLLECT\",\"1701\":\"TELEMETRY_DDR_FILE_COPY\",\"1702\":\"TELEMETRY_DDR_FILE_MOVE\",\"1703\":\"TELEMETRY_DDR_FILE_RENAME\",\"1704\":\"TELEMETRY_DDR_FILE_MODIFY\",\"1705\":\"TELEMETRY_DDR_FILE_DELETE\",\"1706\":\"TELEMETRY_DDR_FILE_UNZIP\",\"1707\":\"TELEMETRY_DDR_FILE_ZIP\",\"1708\":\"TELEMETRY_DDR_FILE_UPLOAD\",\"1709\":\"TELEMETRY_DDR_FILE_DOWNLOAD\",\"1710\":\"TELEMETRY_DDR_FILE_PRINT\",\"10101\":\"TELEMETRY_ASSOCIATION_PROCESS_IMAGE_FILE\",\"10102\":\"TELEMETRY_ASSOCIATION_AUTO_RUN_KEY_FULL_PATH\",\"10103\":\"TELEMETRY_ASSOCIATION_HOST_PROC_CMD_FULL_PATH\",\"10104\":\"TELEMETRY_ASSOCIATION_SERVICE_DLL\",\"10105\":\"TELEMETRY_ASSOCIATION_ARCHIVE_FILE\",\"10106\":\"TELEMETRY_ASSOCIATION_BROWSER_PROCESS\"}"
                ),
            )?;

            // Painless script
            // Source: Matcher m = /([+-]\\d\\d:\\d\\d)$/.matcher(ctx.trend_micro_vision_one?.telemetry?.timezone ?: \"\");\nctx._tmp_timezone = m.find() ? m.group(1) : \"UTC\";\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Matcher m = /([+-]\\d\\d:\\d\\d)$/.matcher(ctx.trend_micro_vision_one?.telemetry?.timezone ?: \"\");\nctx._tmp_timezone = m.find() ? m.group(1) : \"UTC\";\n"#
                ),
            )?;

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.event_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.event_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.telemetry.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.event_time".into(),
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
                        "date_trend_micro_vision_one_telemetry_event_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.event_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.first_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.first_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.telemetry.first_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.first_seen".into(),
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
                        "date_trend_micro_vision_one_telemetry_first_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.first_seen");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.last_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.last_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.telemetry.last_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.last_seen".into(),
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
                        "date_trend_micro_vision_one_telemetry_last_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.last_seen");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.log_received_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.log_received_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.log_received_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.log_received_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_log_received_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.log_received_time");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_file_creation") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.object_file_creation")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.object_file_creation",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.object_file_creation"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_object_file_creation",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.object_file_creation");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_file_modified_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.telemetry.object_file_modified_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.object_file_modified_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "trend_micro_vision_one.telemetry.object_file_modified_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_object_file_modified_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.object_file_modified_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.object_first_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.object_first_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.object_first_seen",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.object_first_seen"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_object_first_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.object_first_seen");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.object_last_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.object_last_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.telemetry.object_last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.object_last_seen"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_object_last_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.object_last_seen");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.object_launch_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.object_launch_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.object_launch_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.object_launch_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_object_launch_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.object_launch_time");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.parent_file_creation") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.parent_file_creation")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.parent_file_creation",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.parent_file_creation"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_parent_file_creation",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.parent_file_creation");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.parent_file_modified_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.telemetry.parent_file_modified_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.parent_file_modified_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "trend_micro_vision_one.telemetry.parent_file_modified_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_parent_file_modified_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.parent_file_modified_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_launch_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.parent_launch_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.parent_launch_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.parent_launch_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_parent_launch_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.parent_launch_time");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_file_creation") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.telemetry.process_file_creation")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.process_file_creation",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.process_file_creation"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_process_file_creation",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.process_file_creation");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_file_modified_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "trend_micro_vision_one.telemetry.process_file_modified_time",
                    ) {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.process_file_modified_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.process_file_modified_time".into(),
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
                        "date_trend_micro_vision_one_telemetry_process_file_modified_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.process_file_modified_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.process_launch_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.process_launch_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.process_launch_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.process_launch_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_process_launch_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.process_launch_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.src_file_creation") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.src_file_creation")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.src_file_creation",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.src_file_creation"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_src_file_creation",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.process_launch_time");
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.src_file_modified_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.telemetry.src_file_modified_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.telemetry.src_file_modified_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.src_file_modified_time"
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_trend_micro_vision_one_telemetry_src_file_modified_time",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.src_file_modified_time");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.src_first_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.src_first_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.telemetry.src_first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.src_first_seen".into(),
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
                        "date_trend_micro_vision_one_telemetry_src_first_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.src_first_seen");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.src_last_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.telemetry.src_last_seen")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601"],
                            event.get_str("_tmp_timezone"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.telemetry.src_last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.telemetry.src_last_seen".into(),
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
                        "date_trend_micro_vision_one_telemetry_src_last_seen",
                    )?;
                    event.remove("trend_micro_vision_one.telemetry.src_last_seen");
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

            event.remove("_tmp_timezone");

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.telemetry.object_current_file_size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.telemetry.object_current_file_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.object_current_file_size"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.telemetry.object_current_file_size",
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
                    "convert_trend_micro_vision_one_telemetry_object_current_file_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.object_current_file_size");
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
                if event.has_value("trend_micro_vision_one.telemetry.object_file_size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.telemetry.object_file_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.object_file_size".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.telemetry.object_file_size",
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
                    "convert_trend_micro_vision_one_telemetry_object_file_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.object_file_size");
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
                if event.has_value("trend_micro_vision_one.telemetry.parent_file_size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.telemetry.parent_file_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.parent_file_size".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.telemetry.parent_file_size",
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
                    "convert_trend_micro_vision_one_telemetry_parent_file_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.parent_file_size");
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
                if event.has_value("trend_micro_vision_one.telemetry.process_file_size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.telemetry.process_file_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.process_file_size".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.telemetry.process_file_size",
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
                    "convert_trend_micro_vision_one_telemetry_process_file_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.process_file_size");
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
                if event.has_value("trend_micro_vision_one.telemetry.raw_data_size") {
                    if let Some(val) = event.get("trend_micro_vision_one.telemetry.raw_data_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.raw_data_size".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.telemetry.raw_data_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_trend_micro_vision_one_telemetry_raw_data_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.raw_data_size");
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
                if event.has_value("trend_micro_vision_one.telemetry.src_file_size") {
                    if let Some(val) = event.get("trend_micro_vision_one.telemetry.src_file_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.telemetry.src_file_size".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.telemetry.src_file_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_trend_micro_vision_one_telemetry_src_file_size",
                )?;
                event.remove("trend_micro_vision_one.telemetry.src_file_size");
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.event_time") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.event_time")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.uuid") };
            if _cond {
                if let Some(v) = event.get("trend_micro_vision_one.telemetry.uuid").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.event_id") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.event_id")
                    .cloned()
                {
                    event.set("event.code", v)?;
                }
            }

            if event.has_value("event.code") {
                if let Some(val) = event.get("event.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.code".into(),
                            message,
                        }
                    })?;
                    event.set("event.code", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) ctx.file = [:];\nctx.file.created = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_creation\n];\nctx.file.created.removeIf(v -> v == null);\nif (ctx.file.created.size() == 1) ctx.file.created = ctx.file.created[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.file == null) ctx.file = [:];\nctx.file.created = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_creation,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_creation\n];\nctx.file.created.removeIf(v -> v == null);\nif (ctx.file.created.size() == 1) ctx.file.created = ctx.file.created[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_file_created",
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

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_file_hash_md5") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_file_hash_md5")
                    .cloned()
                {
                    event.set("file.hash.md5", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_file_hash_sha1") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_file_hash_sha1")
                    .cloned()
                {
                    event.set("file.hash.sha1", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_file_hash_sha256") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_file_hash_sha256")
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) ctx.file = [:];\nctx.file.mtime = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_modified_time\n];\nctx.file.mtime.removeIf(v -> v == null);\nif (ctx.file.mtime.size() == 1) ctx.file.mtime = ctx.file.mtime[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.file == null) ctx.file = [:];\nctx.file.mtime = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_modified_time,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_modified_time\n];\nctx.file.mtime.removeIf(v -> v == null);\nif (ctx.file.mtime.size() == 1) ctx.file.mtime = ctx.file.mtime[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_file_mtime",
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
                // Painless script
                // Source: if (ctx.file == null) ctx.file = [:];\nctx.file.path = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_path,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_path\n];\nctx.file.path.removeIf(v -> v == null);\nif (ctx.file.path.size() == 1) ctx.file.path = ctx.file.path[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.file == null) ctx.file = [:];\nctx.file.path = [\n  ctx.trend_micro_vision_one?.telemetry?.object_file_path,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_path\n];\nctx.file.path.removeIf(v -> v == null);\nif (ctx.file.path.size() == 1) ctx.file.path = ctx.file.path[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_file_path",
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
                // Painless script
                // Source: if (ctx.file == null) ctx.file = [:];\nctx.file.size = [\n  ctx.trend_micro_vision_one?.telemetry?.object_current_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.object_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_size\n];\nctx.file.size.removeIf(v -> v == null);\nif (ctx.file.size.size() == 1) ctx.file.size = ctx.file.size[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.file == null) ctx.file = [:];\nctx.file.size = [\n  ctx.trend_micro_vision_one?.telemetry?.object_current_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.object_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.src_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.process_file_size,\n  ctx.trend_micro_vision_one?.telemetry?.parent_file_size\n];\nctx.file.size.removeIf(v -> v == null);\nif (ctx.file.size.size() == 1) ctx.file.size = ctx.file.size[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_file_size",
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.object_user_domain") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_user_domain")
                    .cloned()
                {
                    event.set("file.user.domain", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.object_user_group_sids") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_user_group_sids")
                    .cloned()
                {
                    event.set("file.user.group.id", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.object_user") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.object_user")
                    .cloned()
                {
                    event.set("file.user.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.endpoint_guid") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.endpoint_guid")
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.endpoint_host_name") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.endpoint_host_name")
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.endpoint_ip") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.endpoint_ip")
                    .cloned()
                {
                    event.set("host.ip", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.endpoint_mac_address") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.endpoint_mac_address")
                    .cloned()
                {
                    event.set("host.mac", v)?;
                }
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!(":"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.group_id") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.group_id")
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.os_description") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.os_description")
                    .cloned()
                {
                    event.set("os.full", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.os_name") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.os_name")
                    .cloned()
                {
                    event.set("os.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.os_ver") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.os_ver")
                    .cloned()
                {
                    event.set("os.version", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.process == null) ctx.process = [:];\nctx.process.command_line = [\n  ctx.trend_micro_vision_one?.telemetry?.process_cmd,\n  ctx.trend_micro_vision_one?.telemetry?.object_cmd\n];\nctx.process.command_line.removeIf(v -> v == null);\nif (ctx.process.command_line.size() == 1) ctx.process.command_line = ctx.process.command_line[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.process == null) ctx.process = [:];\nctx.process.command_line = [\n  ctx.trend_micro_vision_one?.telemetry?.process_cmd,\n  ctx.trend_micro_vision_one?.telemetry?.object_cmd\n];\nctx.process.command_line.removeIf(v -> v == null);\nif (ctx.process.command_line.size() == 1) ctx.process.command_line = ctx.process.command_line[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_process_command_line",
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.process_file_path") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_file_path")
                    .cloned()
                {
                    event.set("process.executable", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_file_hash_md5") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_file_hash_md5")
                    .cloned()
                {
                    event.set("process.hash.md5", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_file_hash_sha1") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_file_hash_sha1")
                    .cloned()
                {
                    event.set("process.hash.sha1", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_file_hash_sha256") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_file_hash_sha256")
                    .cloned()
                {
                    event.set("process.hash.sha256", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.process == null) ctx.process = [:];\nctx.process.name = [\n  ctx.trend_micro_vision_one?.telemetry?.process_name,\n  ctx.trend_micro_vision_one?.telemetry?.object_name\n];\nctx.process.name.removeIf(v -> v == null);\nif (ctx.process.name.size() == 1) ctx.process.name = ctx.process.name[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.process == null) ctx.process = [:];\nctx.process.name = [\n  ctx.trend_micro_vision_one?.telemetry?.process_name,\n  ctx.trend_micro_vision_one?.telemetry?.object_name\n];\nctx.process.name.removeIf(v -> v == null);\nif (ctx.process.name.size() == 1) ctx.process.name = ctx.process.name[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_process_name",
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
                // Painless script
                // Source: if (ctx.process == null) ctx.process = [:];\nctx.process.pid = [\n  ctx.trend_micro_vision_one?.telemetry?.process_pid,\n  ctx.trend_micro_vision_one?.telemetry?.object_pid\n];\nctx.process.pid.removeIf(v -> v == null);\nif (ctx.process.pid.size() == 1) ctx.process.pid = ctx.process.pid[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.process == null) ctx.process = [:];\nctx.process.pid = [\n  ctx.trend_micro_vision_one?.telemetry?.process_pid,\n  ctx.trend_micro_vision_one?.telemetry?.object_pid\n];\nctx.process.pid.removeIf(v -> v == null);\nif (ctx.process.pid.size() == 1) ctx.process.pid = ctx.process.pid[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_process_pid",
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
                // Painless script
                // Source: if (ctx.process == null) ctx.process = [:];\nctx.process.start = [\n  ctx.trend_micro_vision_one?.telemetry?.process_launch_time,\n  ctx.trend_micro_vision_one?.telemetry?.object_launch_time\n];\nctx.process.start.removeIf(v -> v == null);\nif (ctx.process.start.size() == 1) ctx.process.start = ctx.process.start[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.process == null) ctx.process = [:];\nctx.process.start = [\n  ctx.trend_micro_vision_one?.telemetry?.process_launch_time,\n  ctx.trend_micro_vision_one?.telemetry?.object_launch_time\n];\nctx.process.start.removeIf(v -> v == null);\nif (ctx.process.start.size() == 1) ctx.process.start = ctx.process.start[0];\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_populate_process_start",
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.process_user_domain") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_user_domain")
                    .cloned()
                {
                    event.set("process.user.domain", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.process_user_group_sids") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_user_group_sids")
                    .cloned()
                {
                    event.set("process.user.group.id", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.process_user") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.process_user")
                    .cloned()
                {
                    event.set("process.user.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_cmd") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_cmd")
                    .cloned()
                {
                    event.set("process.parent.command_line", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_file_path") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_file_path")
                    .cloned()
                {
                    event.set("process.parent.executable", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.parent_file_hash_md5") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_file_hash_md5")
                    .cloned()
                {
                    event.set("process.parent.hash.md5", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.parent_file_hash_sha1") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_file_hash_sha1")
                    .cloned()
                {
                    event.set("process.parent.hash.sha1", v)?;
                }
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.telemetry.parent_file_hash_sha256") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_file_hash_sha256")
                    .cloned()
                {
                    event.set("process.parent.hash.sha256", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_name") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_name")
                    .cloned()
                {
                    event.set("process.parent.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_pid") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_pid")
                    .cloned()
                {
                    event.set("process.parent.pid", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_launch_time") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_launch_time")
                    .cloned()
                {
                    event.set("process.parent.start", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_user_domain") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_user_domain")
                    .cloned()
                {
                    event.set("process.parent.user.domain", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.parent_user") };
            if _cond {
                if let Some(v) = event
                    .get("trend_micro_vision_one.telemetry.parent_user")
                    .cloned()
                {
                    event.set("process.parent.user.name", v)?;
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.src") };
            if _cond {
                if let Some(v) = event.get("trend_micro_vision_one.telemetry.src").cloned() {
                    event.set("source.ip", v)?;
                }
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

            let _cond = { event.has_value("trend_micro_vision_one.telemetry.dst") };
            if _cond {
                if let Some(v) = event.get("trend_micro_vision_one.telemetry.dst").cloned() {
                    event.set("destination.ip", v)?;
                }
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

            let _cond = {
                event
                    .get("trend_micro_vision_one.telemetry.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("trend_micro_vision_one.telemetry.tags") {
                    foreach_array(event, "trend_micro_vision_one.telemetry.tags", |event| {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
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

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.has_value("host.ip") };
            if _cond {
                if let Some(v) = event.get("host.ip").cloned() {
                    event.set("related.ip", v)?;
                }
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

            let _cond = { event.has_value("file.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("file.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("trend_micro_vision_one.telemetry") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                    serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"),
                    _ => false,
                }))
            };
            if _cond {
                event.remove("trend_micro_vision_one.telemetry.dst");
                event.remove("trend_micro_vision_one.telemetry.endpoint_guid");
                event.remove("trend_micro_vision_one.telemetry.endpoint_host_name");
                event.remove("trend_micro_vision_one.telemetry.endpoint_ip");
                event.remove("trend_micro_vision_one.telemetry.endpoint_mac_address");
                event.remove("trend_micro_vision_one.telemetry.event_time");
                event.remove("trend_micro_vision_one.telemetry.group_id");
                event.remove("trend_micro_vision_one.telemetry.object_cmd");
                event.remove("trend_micro_vision_one.telemetry.object_current_file_size");
                event.remove("trend_micro_vision_one.telemetry.object_file_creation");
                event.remove("trend_micro_vision_one.telemetry.object_file_hash_md5");
                event.remove("trend_micro_vision_one.telemetry.object_file_hash_sha1");
                event.remove("trend_micro_vision_one.telemetry.object_file_hash_sha256");
                event.remove("trend_micro_vision_one.telemetry.object_file_modified_time");
                event.remove("trend_micro_vision_one.telemetry.object_file_path");
                event.remove("trend_micro_vision_one.telemetry.object_file_size");
                event.remove("trend_micro_vision_one.telemetry.object_launch_time");
                event.remove("trend_micro_vision_one.telemetry.object_name");
                event.remove("trend_micro_vision_one.telemetry.object_pid");
                event.remove("trend_micro_vision_one.telemetry.object_user");
                event.remove("trend_micro_vision_one.telemetry.object_user_domain");
                event.remove("trend_micro_vision_one.telemetry.object_user_group_sids");
                event.remove("trend_micro_vision_one.telemetry.os_description");
                event.remove("trend_micro_vision_one.telemetry.os_name");
                event.remove("trend_micro_vision_one.telemetry.os_ver");
                event.remove("trend_micro_vision_one.telemetry.parent_cmd");
                event.remove("trend_micro_vision_one.telemetry.parent_file_creation");
                event.remove("trend_micro_vision_one.telemetry.parent_file_hash_md5");
                event.remove("trend_micro_vision_one.telemetry.parent_file_hash_sha1");
                event.remove("trend_micro_vision_one.telemetry.parent_file_hash_sha256");
                event.remove("trend_micro_vision_one.telemetry.parent_file_modified_time");
                event.remove("trend_micro_vision_one.telemetry.parent_file_path");
                event.remove("trend_micro_vision_one.telemetry.parent_file_size");
                event.remove("trend_micro_vision_one.telemetry.parent_launch_time");
                event.remove("trend_micro_vision_one.telemetry.parent_name");
                event.remove("trend_micro_vision_one.telemetry.parent_pid");
                event.remove("trend_micro_vision_one.telemetry.parent_user");
                event.remove("trend_micro_vision_one.telemetry.parent_user_domain");
                event.remove("trend_micro_vision_one.telemetry.process_cmd");
                event.remove("trend_micro_vision_one.telemetry.process_file_creation");
                event.remove("trend_micro_vision_one.telemetry.process_file_hash_md5");
                event.remove("trend_micro_vision_one.telemetry.process_file_hash_sha1");
                event.remove("trend_micro_vision_one.telemetry.process_file_hash_sha256");
                event.remove("trend_micro_vision_one.telemetry.process_file_modified_time");
                event.remove("trend_micro_vision_one.telemetry.process_file_path");
                event.remove("trend_micro_vision_one.telemetry.process_file_size");
                event.remove("trend_micro_vision_one.telemetry.process_launch_time");
                event.remove("trend_micro_vision_one.telemetry.process_name");
                event.remove("trend_micro_vision_one.telemetry.process_pid");
                event.remove("trend_micro_vision_one.telemetry.process_user");
                event.remove("trend_micro_vision_one.telemetry.process_user_domain");
                event.remove("trend_micro_vision_one.telemetry.process_user_group_sids");
                event.remove("trend_micro_vision_one.telemetry.src");
                event.remove("trend_micro_vision_one.telemetry.src_file_creation");
                event.remove("trend_micro_vision_one.telemetry.src_file_modified_time");
                event.remove("trend_micro_vision_one.telemetry.src_file_path");
                event.remove("trend_micro_vision_one.telemetry.src_file_size");
                event.remove("trend_micro_vision_one.telemetry.tags");
                event.remove("trend_micro_vision_one.telemetry.uuid");
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
