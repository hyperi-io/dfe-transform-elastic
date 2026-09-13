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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "digital_guardian.arc")?;
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
            }

            dot_expand(event, "digital_guardian.arc", "*")?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || ['', '{}', 'NA', 'None', 'null', '-'].contains(object)) {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec![
                        "{}".into(),
                        "NA".into(),
                        "None".into(),
                        "null".into(),
                        "-".into(),
                    ],
                    ..DropPolicy::none()
                },
                None,
            );

            // Painless script
            // Source: long bytesFromStr(String str) {\n  def factors = [\n    \"KB\": 1000L,\n    \"MB\": 1000000L,\n    \"GB\": 1000000000L,\n    \"TB\": 1000000000000L\n  ];\n  def parts = str.splitOnToken(' ');\n  double num = Double.parseDouble(parts[0]);\n  String unit = parts.length > 1 ? parts[1] : null;\n  if (factors.containsKey(unit)) {\n    return (long) (num * factors[unit]);\n  } else {\n    return (long) num;\n  }\n}\nif (ctx.digital_guardian?.arc?.dg_attachments?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_attachments.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_attachments.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.pi_fs instanceof String) {\n  ctx.digital_guardian.arc.pi_fs_bytes = bytesFromStr(ctx.digital_guardian.arc.pi_fs);\n}\nif (ctx.digital_guardian?.arc?.uad_br instanceof String) {\n  ctx.digital_guardian.arc.uad_br_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_br);\n}\nif (ctx.digital_guardian?.arc?.uad_bw instanceof String) {\n  ctx.digital_guardian.arc.uad_bw_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_bw);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"long bytesFromStr(String str) {\n  def factors = [\n    \"KB\": 1000L,\n    \"MB\": 1000000L,\n    \"GB\": 1000000000L,\n    \"TB\": 1000000000000L\n  ];\n  def parts = str.splitOnToken(' ');\n  double num = Double.parseDouble(parts[0]);\n  String unit = parts.length > 1 ? parts[1] : null;\n  if (factors.containsKey(unit)) {\n    return (long) (num * factors[unit]);\n  } else {\n    return (long) num;\n  }\n}\nif (ctx.digital_guardian?.arc?.dg_attachments?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_attachments.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_attachments.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.dg_file_size instanceof String) {\n  ctx.digital_guardian.arc.dg_file_size_bytes = bytesFromStr(ctx.digital_guardian.arc.dg_file_size);\n}\nif (ctx.digital_guardian?.arc?.pi_fs instanceof String) {\n  ctx.digital_guardian.arc.pi_fs_bytes = bytesFromStr(ctx.digital_guardian.arc.pi_fs);\n}\nif (ctx.digital_guardian?.arc?.uad_br instanceof String) {\n  ctx.digital_guardian.arc.uad_br_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_br);\n}\nif (ctx.digital_guardian?.arc?.uad_bw instanceof String) {\n  ctx.digital_guardian.arc.uad_bw_bytes = bytesFromStr(ctx.digital_guardian.arc.uad_bw);\n}"#
                ),
            )?;

            event.set("event.kind", json!("alert"))?;

            if let Some(v) = event
                .get("digital_guardian.arc.dg_guid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("digital_guardian.arc.dg_utype")
                    && event.has_value("digital_guardian.arc.inc_state")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.event == null) {\n  ctx.event = new HashMap();\n}\nctx.event.action = (ctx.digital_guardian.arc.dg_utype + \"-\" + ctx.digital_guardian.arc.inc_state).toLowerCase();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.event == null) {\n  ctx.event = new HashMap();\n}\nctx.event.action = (ctx.digital_guardian.arc.dg_utype + \"-\" + ctx.digital_guardian.arc.inc_state).toLowerCase();"#
                    ),
                )?;
            }

            let _cond = { event.has_value("digital_guardian.arc.inc_sev") };
            if _cond {
                // Painless script
                // Source: if (ctx.event == null) {\n  ctx.event = new HashMap();\n}\ndef sev = ctx.digital_guardian.arc.inc_sev;\nctx.event.severity = params.getOrDefault(sev, params['Unknown']);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.event == null) {\n  ctx.event = new HashMap();\n}\ndef sev = ctx.digital_guardian.arc.inc_sev;\nctx.event.severity = params.getOrDefault(sev, params['Unknown']);"#
                    ),
                    cached_params!(
                        "{\"Unknown\":9,\"Informational\":6,\"Low\":5,\"Minor\":5,\"Medium\":4,\"High\":2,\"Critical\":1}"
                    ),
                )?;
            }

            let _cond = { event.has_value("digital_guardian.arc.dg_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("digital_guardian.arc.dg_time") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("digital_guardian.arc.dg_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.dg_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.dg_processed_time") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("digital_guardian.arc.dg_processed_time")
                {
                    match parse_date_out(&date_str, &["epoch_millis"], None, None) {
                        Some(parsed) => {
                            event.set("digital_guardian.arc.dg_processed_time", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.dg_processed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.dg_local_timestamp") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("digital_guardian.arc.dg_local_timestamp")
                {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("digital_guardian.arc.dg_local_timestamp", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.dg_local_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.pi_fal") };
            if _cond {
                if let Some(date_str) = event.get_as_string("digital_guardian.arc.pi_fal") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("digital_guardian.arc.pi_fal", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.pi_fal".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.pi_fcl") };
            if _cond {
                if let Some(date_str) = event.get_as_string("digital_guardian.arc.pi_fcl") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("digital_guardian.arc.pi_fcl", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.pi_fcl".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.pi_fml") };
            if _cond {
                if let Some(date_str) = event.get_as_string("digital_guardian.arc.pi_fml") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("digital_guardian.arc.pi_fml", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.pi_fml".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.inc_mtime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("digital_guardian.arc.inc_mtime") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("digital_guardian.arc.inc_mtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.inc_mtime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.dg_alert.alert_etl") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("digital_guardian.arc.dg_alert.alert_etl")
                {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("digital_guardian.arc.dg_alert.alert_etl", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.dg_alert.alert_etl".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("digital_guardian.arc.dg_alert.alert_etu") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("digital_guardian.arc.dg_alert.alert_etu")
                {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd hh:mm:ss a", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("digital_guardian.arc.dg_alert.alert_etu", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "digital_guardian.arc.dg_alert.alert_etu".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("digital_guardian.arc.dg_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("digital_guardian.arc.dg_processed_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("@timestamp") {
                    event.set("@timestamp", v)?;
                }
            }

            if let Some(v) = event
                .get("digital_guardian.arc.inc_mtime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("@timestamp") {
                    event.set("@timestamp", v)?;
                }
            }

            if let Some(v) = event
                .get("digital_guardian.arc.dg_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("digital_guardian.arc.inc_creator")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("digital_guardian.arc.inc_creator") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("digital_guardian.arc.inc_creator")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("digital_guardian.arc.inc_assign") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("digital_guardian.arc.inc_assign")
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
                event.remove("digital_guardian.arc.dg_name");
                event.remove("digital_guardian.arc.dg_guid");
                event.remove("digital_guardian.arc.inc_mtime");
                event.remove("digital_guardian.arc.inc_sev");
                event.remove("digital_guardian.arc.inc_creator");
            }

            event.remove("json");

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
