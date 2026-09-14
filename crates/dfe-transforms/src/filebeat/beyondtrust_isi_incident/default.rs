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

            parse_json_field(event, "event.original", "beyondtrust_isi.incident")?;

            event.remove("json");

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("intrusion_detection"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("beyondtrust_isi.incident.definitionId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_isi.incident.incidentId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("beyondtrust_isi.incident.severity") {
                    if let Some(val) = event.get("beyondtrust_isi.incident.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_isi.incident.severity".into(),
                                message,
                            }
                        })?;
                        event.set("beyondtrust_isi.incident.severity", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("beyondtrust_isi.incident.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: // Severity can arrive either as a number (1-4) or as a textual label,\n// so first resolve it to a canonical label before deriving a score.\ndef severity = ctx.beyondtrust_isi.incident.severity;\nString label = null;\n\nif (severity instanceof Long) {\n  label = params.number_to_label.get(String.valueOf(severity));\n} else if (severity instanceof String) {\n  label = (String) severity;\n}\n\n// Drop unrecognized or out-of-range values so only valid labels remain.\nif (label == null) {\n  ctx.beyondtrust_isi.incident.remove('severity');\n  return;\n}\n\nctx.beyondtrust_isi.incident.severity = label;\n\ndef score = params.label_to_score.get(label.toLowerCase());\nif (score != null) {\n  ctx.event.severity = score;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"// Severity can arrive either as a number (1-4) or as a textual label,\n// so first resolve it to a canonical label before deriving a score.\ndef severity = ctx.beyondtrust_isi.incident.severity;\nString label = null;\n\nif (severity instanceof Long) {\n  label = params.number_to_label.get(String.valueOf(severity));\n} else if (severity instanceof String) {\n  label = (String) severity;\n}\n\n// Drop unrecognized or out-of-range values so only valid labels remain.\nif (label == null) {\n  ctx.beyondtrust_isi.incident.remove('severity');\n  return;\n}\n\nctx.beyondtrust_isi.incident.severity = label;\n\ndef score = params.label_to_score.get(label.toLowerCase());\nif (score != null) {\n  ctx.event.severity = score;\n}"#
                        ),
                        cached_params!(
                            "{\"number_to_label\":{\"1\":\"Low\",\"2\":\"Medium\",\"3\":\"High\",\"4\":\"Critical\"},\"label_to_score\":{\"low\":21,\"medium\":47,\"high\":73,\"critical\":99}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_incident_severity",
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
                .get("beyondtrust_isi.incident.tenantId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = {
                event.has_value("beyondtrust_isi.incident.timestamp")
                    && event.get_str("beyondtrust_isi.incident.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("beyondtrust_isi.incident.timestamp")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "MM/dd/yyyy HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("beyondtrust_isi.incident.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_isi.incident.timestamp".into(),
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
                    event.remove("beyondtrust_isi.incident.timestamp");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event
                    .get("beyondtrust_isi.incident.entityName")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("beyondtrust_isi.incident.entityName") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("beyondtrust_isi.incident.entityName") {
                        if let Some(s) = event.get_string("beyondtrust_isi.incident.entityName") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event
                                .set("beyondtrust_isi.incident.entityName", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                event
                    .get("beyondtrust_isi.incident.entityType")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("beyondtrust_isi.incident.entityType") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("beyondtrust_isi.incident.entityType") {
                        if let Some(s) = event.get_string("beyondtrust_isi.incident.entityType") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event
                                .set("beyondtrust_isi.incident.entityType", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                event
                    .get("beyondtrust_isi.incident.source")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("beyondtrust_isi.incident.source") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("beyondtrust_isi.incident.source") {
                        if let Some(s) = event.get_string("beyondtrust_isi.incident.source") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set("beyondtrust_isi.incident.source", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                event
                    .get("beyondtrust_isi.incident.location")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("beyondtrust_isi.incident.location") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("beyondtrust_isi.incident.location") {
                        if let Some(s) = event.get_string("beyondtrust_isi.incident.location") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set("beyondtrust_isi.incident.location", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                .get("beyondtrust_isi.incident.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_isi.incident.definitionSummary")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_isi.incident.definitionSummary")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_isi.incident.link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_isi.incident.link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.reference", v)?;
            }

            event.remove("beyondtrust_isi.incident.definitionId");
            event.remove("beyondtrust_isi.incident.incidentId");
            event.remove("beyondtrust_isi.incident.link");
            event.remove("beyondtrust_isi.incident.definitionSummary");
            event.remove("beyondtrust_isi.incident.tenantId");
            event.remove("beyondtrust_isi.incident.timestamp");

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

            // Painless script
            // Source: String camelToSnake(String str) {\n  StringBuilder result = new StringBuilder();\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0) {\n        char prev = str.charAt(i - 1);\n        boolean nextIsLower = (i + 1 < str.length()) && Character.isLowerCase(str.charAt(i + 1));\n        boolean prevIsDigit = Character.isDigit(prev);\n        if (Character.isLowerCase(prev) || prevIsDigit || (Character.isUpperCase(prev) && nextIsLower)) {\n          result.append('_');\n        }\n      }\n      result.append(Character.toLowerCase(c));\n    } else {\n      result.append(c);\n    }\n  }\n  return result.toString();\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nctx.beyondtrust_isi = ctx.beyondtrust_isi ?: [:];\nif (ctx.beyondtrust_isi?.incident != null) {\n  ctx.beyondtrust_isi.incident = convertToSnakeCase(ctx.beyondtrust_isi.incident);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String camelToSnake(String str) {\n  StringBuilder result = new StringBuilder();\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0) {\n        char prev = str.charAt(i - 1);\n        boolean nextIsLower = (i + 1 < str.length()) && Character.isLowerCase(str.charAt(i + 1));\n        boolean prevIsDigit = Character.isDigit(prev);\n        if (Character.isLowerCase(prev) || prevIsDigit || (Character.isUpperCase(prev) && nextIsLower)) {\n          result.append('_');\n        }\n      }\n      result.append(Character.toLowerCase(c));\n    } else {\n      result.append(c);\n    }\n  }\n  return result.toString();\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nctx.beyondtrust_isi = ctx.beyondtrust_isi ?: [:];\nif (ctx.beyondtrust_isi?.incident != null) {\n  ctx.beyondtrust_isi.incident = convertToSnakeCase(ctx.beyondtrust_isi.incident);\n}"#
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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
