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
                // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.network_activity = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.network_activity = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.client_ip") {
                event.rename(
                    "trend_micro_vision_one.network_activity.client_ip",
                    "trend_micro_vision_one.network_activity.client.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.customer_id") {
                event.rename(
                    "trend_micro_vision_one.network_activity.customer_id",
                    "trend_micro_vision_one.network_activity.customer.id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.dst") {
                event.rename(
                    "trend_micro_vision_one.network_activity.dst",
                    "trend_micro_vision_one.network_activity.destination.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.dpt") {
                event.rename(
                    "trend_micro_vision_one.network_activity.dpt",
                    "trend_micro_vision_one.network_activity.destination.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.detection_type") {
                event.rename(
                    "trend_micro_vision_one.network_activity.detection_type",
                    "trend_micro_vision_one.network_activity.detection.type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.device_guid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.device_guid",
                    "trend_micro_vision_one.network_activity.device.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.endpoint_guid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.endpoint_guid",
                    "trend_micro_vision_one.network_activity.endpoint.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.endpoint_host_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.endpoint_host_name",
                    "trend_micro_vision_one.network_activity.endpoint.host_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.event_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.event_name",
                    "trend_micro_vision_one.network_activity.event.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.event_sub_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.event_sub_name",
                    "trend_micro_vision_one.network_activity.event.sub_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.event_time") {
                event.rename(
                    "trend_micro_vision_one.network_activity.event_time",
                    "trend_micro_vision_one.network_activity.event.time",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.file_hash") {
                event.rename(
                    "trend_micro_vision_one.network_activity.file_hash",
                    "trend_micro_vision_one.network_activity.file.hash_sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.file_hash_sha256") {
                event.rename(
                    "trend_micro_vision_one.network_activity.file_hash_sha256",
                    "trend_micro_vision_one.network_activity.file.hash_sha256",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.file_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.file_name",
                    "trend_micro_vision_one.network_activity.file.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.file_size") {
                event.rename(
                    "trend_micro_vision_one.network_activity.file_size",
                    "trend_micro_vision_one.network_activity.file.size",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.file_type") {
                event.rename(
                    "trend_micro_vision_one.network_activity.file_type",
                    "trend_micro_vision_one.network_activity.file.type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.mime_type") {
                event.rename(
                    "trend_micro_vision_one.network_activity.mime_type",
                    "trend_micro_vision_one.network_activity.file.mime_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.mal_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.mal_name",
                    "trend_micro_vision_one.network_activity.malware.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.object_id") {
                event.rename(
                    "trend_micro_vision_one.network_activity.object_id",
                    "trend_micro_vision_one.network_activity.object.id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.os_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.os_name",
                    "trend_micro_vision_one.network_activity.os.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.policy_uuid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.policy_uuid",
                    "trend_micro_vision_one.network_activity.policy.uuid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.request") {
                event.rename(
                    "trend_micro_vision_one.network_activity.request",
                    "trend_micro_vision_one.network_activity.request.url",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.request_base") {
                event.rename(
                    "trend_micro_vision_one.network_activity.request_base",
                    "trend_micro_vision_one.network_activity.request.base",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.request_method") {
                event.rename(
                    "trend_micro_vision_one.network_activity.request_method",
                    "trend_micro_vision_one.network_activity.request.method",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.request_mime_type") {
                event.rename(
                    "trend_micro_vision_one.network_activity.request_mime_type",
                    "trend_micro_vision_one.network_activity.request.mime_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.rule_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.rule_name",
                    "trend_micro_vision_one.network_activity.rule.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.rule_type") {
                event.rename(
                    "trend_micro_vision_one.network_activity.rule_type",
                    "trend_micro_vision_one.network_activity.rule.type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.rule_uuid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.rule_uuid",
                    "trend_micro_vision_one.network_activity.rule.uuid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.server_protocol") {
                event.rename(
                    "trend_micro_vision_one.network_activity.server_protocol",
                    "trend_micro_vision_one.network_activity.server.protocol",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.server_tls") {
                event.rename(
                    "trend_micro_vision_one.network_activity.server_tls",
                    "trend_micro_vision_one.network_activity.server.tls",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.src") {
                event.rename(
                    "trend_micro_vision_one.network_activity.src",
                    "trend_micro_vision_one.network_activity.source.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.spt") {
                event.rename(
                    "trend_micro_vision_one.network_activity.spt",
                    "trend_micro_vision_one.network_activity.source.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.tenant_guid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.tenant_guid",
                    "trend_micro_vision_one.network_activity.tenant.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.suid") {
                event.rename(
                    "trend_micro_vision_one.network_activity.suid",
                    "trend_micro_vision_one.network_activity.user.id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.principal_name") {
                event.rename(
                    "trend_micro_vision_one.network_activity.principal_name",
                    "trend_micro_vision_one.network_activity.user.principal_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.user_department") {
                event.rename(
                    "trend_micro_vision_one.network_activity.user_department",
                    "trend_micro_vision_one.network_activity.user.department",
                )?;
            }

            if event.has_value("trend_micro_vision_one.network_activity.user_domain") {
                event.rename(
                    "trend_micro_vision_one.network_activity.user_domain",
                    "trend_micro_vision_one.network_activity.user.domain",
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.network_activity.event.time")
                    && event.get_str("trend_micro_vision_one.network_activity.event.time")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventTime = Long.parseLong(ctx.trend_micro_vision_one.network_activity.event.time.toString()); if (eventTime < 10000000000L) {\n  ctx.trend_micro_vision_one.network_activity.event.time = eventTime * 1000L;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventTime = Long.parseLong(ctx.trend_micro_vision_one.network_activity.event.time.toString()); if (eventTime < 10000000000L) {\n  ctx.trend_micro_vision_one.network_activity.event.time = eventTime * 1000L;\n}"#
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
                event.has_value("trend_micro_vision_one.network_activity.event.time")
                    && event.get_str("trend_micro_vision_one.network_activity.event.time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.network_activity.event.time")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.network_activity.event.time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.network_activity.event.time"
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
                    event.remove("trend_micro_vision_one.network_activity.event.time");
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
                if event.has_value("trend_micro_vision_one.network_activity.act") {
                    if let Some(val) = event.get("trend_micro_vision_one.network_activity.act") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.act".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.network_activity.act", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_act_to_long")?;
                event.remove("trend_micro_vision_one.network_activity.act");
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

            let _cond = { event.has_value("trend_micro_vision_one.network_activity.act") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def act = ctx.trend_micro_vision_one.network_activity.act;\nif (params.act.containsKey(act.toString())) {\n  ctx.trend_micro_vision_one.network_activity.act_value = params.act[act.toString()];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def act = ctx.trend_micro_vision_one.network_activity.act;\nif (params.act.containsKey(act.toString())) {\n  ctx.trend_micro_vision_one.network_activity.act_value = params.act[act.toString()];\n}"#
                        ),
                        cached_params!(
                            "{\"act\":{\"0\":\"allow\",\"1\":\"monitor\",\"2\":\"block\",\"3\":\"warn\",\"4\":\"override\",\"5\":\"analyze\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_resolve_action_id_to_text_value",
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
                if event.has_value("trend_micro_vision_one.network_activity.detection.type") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.detection.type")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.detection.type"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.detection.type",
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
                    "convert_detection_type_to_long",
                )?;
                event.remove("trend_micro_vision_one.network_activity.detection.type");
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
                if event.has_value("trend_micro_vision_one.network_activity.destination.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.destination.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.destination.port"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.destination.port",
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
                event.remove("trend_micro_vision_one.network_activity.destination.port");
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
                if event.has_value("trend_micro_vision_one.network_activity.duration") {
                    if let Some(val) = event.get("trend_micro_vision_one.network_activity.duration")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.duration".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.duration",
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
                    "convert_duration_to_long",
                )?;
                event.remove("trend_micro_vision_one.network_activity.duration");
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
                if event.has_value("trend_micro_vision_one.network_activity.file.size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.file.size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.file.size".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.file.size",
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
                    "convert_file_size_to_long",
                )?;
                event.remove("trend_micro_vision_one.network_activity.file.size");
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
                if event.has_value("trend_micro_vision_one.network_activity.rt") {
                    if let Some(val) = event.get("trend_micro_vision_one.network_activity.rt") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.rt".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.network_activity.rt", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_rt_to_long")?;
                event.remove("trend_micro_vision_one.network_activity.rt");
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
                if event.has_value("trend_micro_vision_one.network_activity.score") {
                    if let Some(val) = event.get("trend_micro_vision_one.network_activity.score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.score".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.network_activity.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                event.remove("trend_micro_vision_one.network_activity.score");
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
                if event.has_value("trend_micro_vision_one.network_activity.source.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.source.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.source.port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.source.port",
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
                event.remove("trend_micro_vision_one.network_activity.source.port");
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
                if event.has_value("trend_micro_vision_one.network_activity.start") {
                    if let Some(val) = event.get("trend_micro_vision_one.network_activity.start") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.start".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.network_activity.start", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_start_to_long")?;
                event.remove("trend_micro_vision_one.network_activity.start");
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
                if event.has_value("trend_micro_vision_one.network_activity.client.ip") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.client.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.client.ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.client.ip",
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
                    "convert_client_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.network_activity.client.ip");
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
                if event.has_value("trend_micro_vision_one.network_activity.destination.ip") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.destination.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.destination.ip"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.destination.ip",
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
                    "convert_destination_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.network_activity.destination.ip");
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
                if event.has_value("trend_micro_vision_one.network_activity.source.ip") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.network_activity.source.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.network_activity.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.network_activity.source.ip",
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
                event.remove("trend_micro_vision_one.network_activity.source.ip");
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
                .get("trend_micro_vision_one.network_activity.event.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.duration")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.network_activity.duration")
                    .is_some_and(|v| v.is_number())
            };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = ctx.trend_micro_vision_one.network_activity.duration * 1000000;
                scale_field(
                    event,
                    &ScaleField::new(
                        "trend_micro_vision_one.network_activity.duration",
                        "event.duration",
                        Factor::Long(1000000),
                    ),
                );
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.ip", v)?;
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.source.ip")
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
                .get("trend_micro_vision_one.network_activity.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.destination.ip")
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
                .get("trend_micro_vision_one.network_activity.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.endpoint.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.os.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.user.principal_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.hash_sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.hash_sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.mime_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mime_type", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.file.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.request.mime_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.mime_type", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.server.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.rule.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.rule.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.uuid", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.server.tls")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.version", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.request.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.company_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.network_activity.user_agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.network_activity.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.network_activity.destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.network_activity.client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.network_activity.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.network_activity.user.principal_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.user.principal_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.network_activity.file.hash_sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.file.hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.network_activity.file.hash_sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.file.hash_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.network_activity.endpoint.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.endpoint.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.network_activity.request.base") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.network_activity.request.base")
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
                event.remove("trend_micro_vision_one.network_activity.client.ip");
                event.remove("trend_micro_vision_one.network_activity.company_name");
                event.remove("trend_micro_vision_one.network_activity.destination.ip");
                event.remove("trend_micro_vision_one.network_activity.destination.port");
                event.remove("trend_micro_vision_one.network_activity.endpoint.host_name");
                event.remove("trend_micro_vision_one.network_activity.event.time");
                event.remove("trend_micro_vision_one.network_activity.file.hash_sha1");
                event.remove("trend_micro_vision_one.network_activity.file.hash_sha256");
                event.remove("trend_micro_vision_one.network_activity.file.mime_type");
                event.remove("trend_micro_vision_one.network_activity.file.name");
                event.remove("trend_micro_vision_one.network_activity.file.size");
                event.remove("trend_micro_vision_one.network_activity.file.type");
                event.remove("trend_micro_vision_one.network_activity.os.name");
                event.remove("trend_micro_vision_one.network_activity.request.method");
                event.remove("trend_micro_vision_one.network_activity.request.mime_type");
                event.remove("trend_micro_vision_one.network_activity.request.url");
                event.remove("trend_micro_vision_one.network_activity.rule.name");
                event.remove("trend_micro_vision_one.network_activity.rule.type");
                event.remove("trend_micro_vision_one.network_activity.rule.uuid");
                event.remove("trend_micro_vision_one.network_activity.server.protocol");
                event.remove("trend_micro_vision_one.network_activity.server.tls");
                event.remove("trend_micro_vision_one.network_activity.source.ip");
                event.remove("trend_micro_vision_one.network_activity.source.port");
                event.remove("trend_micro_vision_one.network_activity.start");
                event.remove("trend_micro_vision_one.network_activity.user.domain");
                event.remove("trend_micro_vision_one.network_activity.user.id");
                event.remove("trend_micro_vision_one.network_activity.user.principal_name");
                event.remove("trend_micro_vision_one.network_activity.user_agent");
            }

            event.remove("json");

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
