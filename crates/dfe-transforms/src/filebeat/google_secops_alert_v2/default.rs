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
            event.set("ecs.version", json!("8.17.0"))?;

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

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.google_secops = ctx.google_secops ?: [:];\nctx.google_secops.alert_v2 = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.google_secops = ctx.google_secops ?: [:];\nctx.google_secops.alert_v2 = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            event.remove("json");

            let _cond = {
                event
                    .get("google_secops.alert_v2.detection")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: String[] kvFields = new String[] {\"detection_fields\", \"outcomes\", \"rule_labels\"};\nfor (def detection : ctx.google_secops.alert_v2.detection) {\n  for (def fieldName : kvFields) {\n    if (!(detection[fieldName] instanceof List)) {\n      continue;\n    }\n    def flat = new HashMap();\n    for (def entry : detection[fieldName]) {\n      if (entry?.key == null || entry.key == '') {\n        continue;\n      }\n      if (entry.value != null && entry.value != '') {\n        flat[entry.key] = entry.value;\n      } else if (entry.source != null && entry.source != '') {\n        flat[entry.key] = entry.source;\n      }\n    }\n    if (flat.isEmpty()) {\n      detection.remove(fieldName);\n    } else {\n      detection[fieldName] = flat;\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] kvFields = new String[] {\"detection_fields\", \"outcomes\", \"rule_labels\"};\nfor (def detection : ctx.google_secops.alert_v2.detection) {\n  for (def fieldName : kvFields) {\n    if (!(detection[fieldName] instanceof List)) {\n      continue;\n    }\n    def flat = new HashMap();\n    for (def entry : detection[fieldName]) {\n      if (entry?.key == null || entry.key == '') {\n        continue;\n      }\n      if (entry.value != null && entry.value != '') {\n        flat[entry.key] = entry.value;\n      } else if (entry.source != null && entry.source != '') {\n        flat[entry.key] = entry.source;\n      }\n    }\n    if (flat.isEmpty()) {\n      detection.remove(fieldName);\n    } else {\n      detection[fieldName] = flat;\n    }\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.created_time")
                    && event.get_str("google_secops.alert_v2.created_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert_v2.created_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert_v2.created_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert_v2.created_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_time")?;
                    event.remove("google_secops.alert_v2.created_time");
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

            event.set("event.kind", json!("alert"))?;

            event.append_unique("event.category", json!("intrusion_detection"))?;

            event.append_unique("event.type", json!("info"))?;

            if let Some(v) = event
                .get("google_secops.alert_v2.created_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.security_result",
                        |event| {
                            if event.has_value("_ingest._value.action") {
                                foreach_array(event, "_ingest._value.action", |event| {
                                    event.append_unique(
                                        "event.action",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.google_secops?.alert_v2?.event?.security_result instanceof List) {\n  for (list in ctx.google_secops?.alert_v2?.event.security_result) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].equalsIgnoreCase('critical')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('error')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('high')) {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].equalsIgnoreCase('informational')) {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].equalsIgnoreCase('low')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('medium')) {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].equalsIgnoreCase('none')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('unknown_severity')) {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.google_secops?.alert_v2?.event?.security_result instanceof List) {\n  for (list in ctx.google_secops?.alert_v2?.event.security_result) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].equalsIgnoreCase('critical')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('error')) {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].equalsIgnoreCase('high')) {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].equalsIgnoreCase('informational')) {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].equalsIgnoreCase('low')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('medium')) {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].equalsIgnoreCase('none')) {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].equalsIgnoreCase('unknown_severity')) {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.security_result",
                        |event| {
                            if event.has_value("_ingest._value.attack_details.tactics") {
                                foreach_array(
                                    event,
                                    "_ingest._value.attack_details.tactics",
                                    |event| {
                                        event.append_unique(
                                            "threat.tactic.id",
                                            json!(
                                                event
                                                    .get("_ingest._value.id")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.security_result",
                        |event| {
                            if event.has_value("_ingest._value.attack_details.tactics") {
                                foreach_array(
                                    event,
                                    "_ingest._value.attack_details.tactics",
                                    |event| {
                                        event.append_unique(
                                            "threat.tactic.name",
                                            json!(
                                                event
                                                    .get("_ingest._value.name")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.security_result",
                        |event| {
                            if event.has_value("_ingest._value.attack_details.techniques") {
                                foreach_array(
                                    event,
                                    "_ingest._value.attack_details.techniques",
                                    |event| {
                                        event.append_unique(
                                            "threat.technique.id",
                                            json!(
                                                event
                                                    .get("_ingest._value.id")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.security_result",
                        |event| {
                            if event.has_value("_ingest._value.attack_details.techniques") {
                                foreach_array(
                                    event,
                                    "_ingest._value.attack_details.techniques",
                                    |event| {
                                        event.append_unique(
                                            "threat.technique.name",
                                            json!(
                                                event
                                                    .get("_ingest._value.name")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.append_unique(
                        "rule.description",
                        json!(
                            event
                                .get("_ingest._value.description")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.risk_score") {
                            if let Some(val) = event.get("_ingest._value.risk_score") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.risk_score".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.risk_score", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_detection_risk_score_to_long",
                        )?;
                        event.remove("_ingest._value.risk_score");
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
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.variables.risk_score.int64_val") {
                            if let Some(val) =
                                event.get("_ingest._value.variables.risk_score.int64_val")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.variables.risk_score.int64_val"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "_ingest._value.variables.risk_score.int64_val",
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
                            "convert_detection_variables_risk_score_int64_val_to_long",
                        )?;
                        event.remove("_ingest._value.variables.risk_score.int64_val");
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
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.variables.risk_score.value") {
                            if let Some(val) =
                                event.get("_ingest._value.variables.risk_score.value")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.variables.risk_score.value".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("_ingest._value.variables.risk_score.value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_detection_variables_risk_score_value_to_long",
                        )?;
                        event.remove("_ingest._value.variables.risk_score.value");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.append_unique(
                        "rule.id",
                        json!(
                            event
                                .get("_ingest._value.rule_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.append_unique(
                        "rule.name",
                        json!(
                            event
                                .get("_ingest._value.rule_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.append_unique(
                        "google_secops.alert_v2.friendly_name",
                        json!(
                            event
                                .get("_ingest._value.rule_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.append_unique(
                        "rule.version",
                        json!(
                            event
                                .get("_ingest._value.rule_version")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.detection_time")
                    && event.get_str("google_secops.alert_v2.detection_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert_v2.detection_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert_v2.detection_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert_v2.detection_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_detection_time")?;
                    event.remove("google_secops.alert_v2.detection_time");
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
                .get("google_secops.alert_v2.detection_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "google_secops.alert_v2.event.metadata.base_labels.allow_scoped_access",
                ) {
                    if let Some(val) = event.get(
                        "google_secops.alert_v2.event.metadata.base_labels.allow_scoped_access",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert_v2.event.metadata.base_labels.allow_scoped_access".into(),
                            message,
                        })?;
                        event.set(
                            "google_secops.alert_v2.event.metadata.base_labels.allow_scoped_access",
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
                    "convert_event_metadata_base_labels_allow_scoped_access_to_boolean",
                )?;
                event.remove(
                    "google_secops.alert_v2.event.metadata.base_labels.allow_scoped_access",
                );
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
                .get("google_secops.alert_v2.event.metadata.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "google_secops.alert_v2.event.metadata.enrichment_labels.allow_scoped_access",
                ) {
                    if let Some(val) = event.get("google_secops.alert_v2.event.metadata.enrichment_labels.allow_scoped_access") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert_v2.event.metadata.enrichment_labels.allow_scoped_access".into(),
                            message,
                        })?;
                    event.set("google_secops.alert_v2.event.metadata.enrichment_labels.allow_scoped_access", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_metadata_enrichment_labels_allow_scoped_access_to_boolean",
                )?;
                event.remove(
                    "google_secops.alert_v2.event.metadata.enrichment_labels.allow_scoped_access",
                );
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
                event.has_value("google_secops.alert_v2.event.metadata.event_timestamp")
                    && event.get_str("google_secops.alert_v2.event.metadata.event_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert_v2.event.metadata.event_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert_v2.event.metadata.event_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert_v2.event.metadata.event_timestamp"
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
                        "date_event_metadata_event_timestamp",
                    )?;
                    event.remove("google_secops.alert_v2.event.metadata.event_timestamp");
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
                event.has_value("google_secops.alert_v2.event.metadata.ingested_timestamp")
                    && event.get_str("google_secops.alert_v2.event.metadata.ingested_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("google_secops.alert_v2.event.metadata.ingested_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert_v2.event.metadata.ingested_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "google_secops.alert_v2.event.metadata.ingested_timestamp"
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
                        "date_event_metadata_ingested_timestamp",
                    )?;
                    event.remove("google_secops.alert_v2.event.metadata.ingested_timestamp");
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
                .get("google_secops.alert_v2.event.metadata.product_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.product", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.metadata.vendor_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.data",
                                json!(
                                    event
                                        .get("_ingest._value.data")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.type",
                                json!(
                                    event
                                        .get("_ingest._value.type")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.questions",
                        |event| {
                            event.append_unique(
                                "dns.question.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.questions",
                        |event| {
                            event.append_unique(
                                "dns.question.type",
                                json!(
                                    event
                                        .get("_ingest._value.type")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.bcc") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.bcc",
                        |event| {
                            event.append_unique(
                                "email.bcc.address",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.cc") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.cc",
                        |event| {
                            event.append_unique(
                                "email.cc.address",
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
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.network.email.from") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.network.email.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.network.email.reply_to") };
            if _cond {
                event.append_unique(
                    "email.reply_to.address",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.network.email.reply_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.subject") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.subject",
                        |event| {
                            event.append_unique(
                                "email.subject",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.to") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.to",
                        |event| {
                            event.append_unique(
                                "email.to.address",
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
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.network.http.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert_v2.event.network.http.response_code") {
                    if let Some(val) =
                        event.get("google_secops.alert_v2.event.network.http.response_code")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert_v2.event.network.http.response_code"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_secops.alert_v2.event.network.http.response_code",
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
                    "convert_event_network_http_response_code_to_long",
                )?;
                event.remove("google_secops.alert_v2.event.network.http.response_code");
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
                .get("google_secops.alert_v2.event.network.http.response_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.network.http.user_agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.network.ip_protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.asset.ip",
                        |event| {
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
                                    "convert_event_principal_asset_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.asset.ip",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.about")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.about") {
                    foreach_array(event, "google_secops.alert_v2.event.about", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user.user_display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.about")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.about") {
                    foreach_array(event, "google_secops.alert_v2.event.about", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user.userid")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.intermediary")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.intermediary") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.intermediary",
                        |event| {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("_ingest._value.hostname")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond =
                { event.has_value("google_secops.alert_v2.event.principal.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.file.full_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.principal.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.principal.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.principal.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.group.group_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.principal.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.ip") {
                                    if let Some(val) = event.get("_ingest._value.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_event_principal_ip_geo_artifact_ip_to_ip",
                                )?;
                                event.remove("_ingest._value.ip");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            event.append_unique(
                                "host.ip",
                                json!(
                                    event
                                        .get("_ingest._value.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.location.region_coordinates.latitude",
                                ) {
                                    if let Some(val) = event
                                        .get("_ingest._value.location.region_coordinates.latitude")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.location.region_coordinates.latitude".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.location.region_coordinates.lat",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_event_principal_ip_geo_artifact_location_region_coordinates_latitude_to_double")?;
                                event.remove("_ingest._value.location.region_coordinates.lat");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.location.region_coordinates.longitude",
                                ) {
                                    if let Some(val) = event
                                        .get("_ingest._value.location.region_coordinates.longitude")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.location.region_coordinates.longitude".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.location.region_coordinates.lon",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_event_principal_ip_geo_artifact_location_region_coordinates_longitude_to_double")?;
                                event.remove("_ingest._value.location.region_coordinates.lon");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip_geo_artifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip_geo_artifact") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip_geo_artifact",
                        |event| {
                            event.remove("_ingest._value.location.region_coordinates.latitude");
                            event.remove("_ingest._value.location.region_coordinates.longitude");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip",
                        |event| {
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
                                    "convert_event_principal_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.mac") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.mac",
                        |event| {
                            event.append_unique(
                                "host.mac",
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
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_host_mac")?;
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
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_host_mac")?;
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
                if event.has_value("google_secops.alert_v2.event.principal.port") {
                    if let Some(val) = event.get("google_secops.alert_v2.event.principal.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert_v2.event.principal.port".into(),
                                message,
                            }
                        })?;
                        event.set("google_secops.alert_v2.event.principal.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_principal_port_to_long",
                )?;
                event.remove("google_secops.alert_v2.event.principal.port");
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
                .get("google_secops.alert_v2.event.principal.process.command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.file.full_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond =
                { event.has_value("google_secops.alert_v2.event.principal.process.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.process.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("google_secops.alert_v2.event.principal.process.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.process.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("google_secops.alert_v2.event.principal.process.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.process.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.parent_process.command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.parent_process.file.full_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.parent_process.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.parent_process.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.process.parent_process.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha256", v)?;
            }

            let _cond = {
                event.has_value(
                    "google_secops.alert_v2.event.principal.process.parent_process.file.md5",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "google_secops.alert_v2.event.principal.process.parent_process.file.sha1",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "google_secops.alert_v2.event.principal.process.parent_process.file.sha256",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.get_str("google_secops.alert_v2.event.principal.process.parent_process.pid")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_secops.alert_v2.event.principal.process.parent_process.pid",
                    ) {
                        if let Some(val) = event.get(
                            "google_secops.alert_v2.event.principal.process.parent_process.pid",
                        ) {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert_v2.event.principal.process.parent_process.pid".into(),
                            message,
                        })?;
                            event.set(
                                "google_secops.alert_v2.event.principal.process.parent_process.pid",
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
                        "convert_event_principal_process_parent_process_pid_to_long",
                    )?;
                    event.remove(
                        "google_secops.alert_v2.event.principal.process.parent_process.pid",
                    );
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
                .get("google_secops.alert_v2.event.principal.process.parent_process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert_v2.event.principal.process.parent_process.parent_process.parent_process.parent_process.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond =
                { event.get_str("google_secops.alert_v2.event.principal.process.pid") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_secops.alert_v2.event.principal.process.pid") {
                        if let Some(val) =
                            event.get("google_secops.alert_v2.event.principal.process.pid")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_secops.alert_v2.event.principal.process.pid"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_secops.alert_v2.event.principal.process.pid",
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
                        "convert_event_principal_process_pid_to_long",
                    )?;
                    event.remove("google_secops.alert_v2.event.principal.process.pid");
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
                .get("google_secops.alert_v2.event.principal.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.attribute.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.attribute.roles") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.attribute.roles",
                        |event| {
                            event.append_unique(
                                "user.roles",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.email_addresses",
                        |event| {
                            event.append_unique(
                                "user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.email_addresses",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.group_identifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.group_identifiers")
                {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.group_identifiers",
                        |event| {
                            event.append_unique(
                                "user.group.id",
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
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.user.user_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.principal.user.user_display_name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.user.user_display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.principal.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.principal.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.principal.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.security_result")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.security_result") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("google_secops.alert_v2.event.security_result")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
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
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.first_discovered_time")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.first_discovered_time",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.first_discovered_time"
                                                        .into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_event_security_result_first_discovered_time",
                                    )?;
                                    event.remove("_ingest._value.first_discovered_time");
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
                                "google_secops.alert_v2.event.security_result",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.src.asset.ip",
                        |event| {
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
                                    "convert_event_src_asset_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.src.asset.ip",
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
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.target.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.src.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.src.ip", |event| {
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
                                "convert_event_src_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.src.ip", |event| {
                        event.append_unique(
                            "source.ip",
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

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.src.ip", |event| {
                        event.append_unique(
                            "related.ip",
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

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.mac") {
                    foreach_array(event, "google_secops.alert_v2.event.src.mac", |event| {
                        event.append_unique(
                            "source.mac",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.mac") {
                    gsub_field(
                        event,
                        "source.mac",
                        "source.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_source_mac")?;
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
                if event.has_value("source.mac") {
                    map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_source_mac")?;
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
                    .get("google_secops.alert_v2.event.src.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.src.user.email_addresses",
                        |event| {
                            event.append_unique(
                                "source.user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.src.user.email_addresses",
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
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.src.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.src.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.src.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.asset.ip",
                        |event| {
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
                                    "convert_event_target_asset_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.asset.ip",
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
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.target.cloud.project.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.target.project.name", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.target.file.last_modification_time")
                    && event
                        .get_str("google_secops.alert_v2.event.target.file.last_modification_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_secops.alert_v2.event.target.file.last_modification_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert_v2.event.target.file.last_modification_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "google_secops.alert_v2.event.target.file.last_modification_time".into(),
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
                        "date_event_target_file_last_modification_time",
                    )?;
                    event.remove("google_secops.alert_v2.event.target.file.last_modification_time");
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

            let _cond = { event.has_value("google_secops.alert_v2.event.target.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.target.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.target.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.target.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.target.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.target.ip", |event| {
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
                                "convert_event_target_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.target.ip", |event| {
                        event.append_unique(
                            "destination.ip",
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

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.target.ip", |event| {
                        event.append_unique(
                            "related.ip",
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

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.mac") {
                    foreach_array(event, "google_secops.alert_v2.event.target.mac", |event| {
                        event.append_unique(
                            "destination.mac",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.mac") {
                    gsub_field(
                        event,
                        "destination.mac",
                        "destination.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_destination_mac")?;
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
                if event.has_value("destination.mac") {
                    map_strings(
                        event,
                        "destination.mac",
                        "destination.mac",
                        str::to_uppercase,
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "uppercase_destination_mac",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert_v2.event.target.port") {
                    if let Some(val) = event.get("google_secops.alert_v2.event.target.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert_v2.event.target.port".into(),
                                message,
                            }
                        })?;
                        event.set("google_secops.alert_v2.event.target.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_target_port_to_long",
                )?;
                event.remove("google_secops.alert_v2.event.target.port");
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
                .get("google_secops.alert_v2.event.target.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.event.target.process.file.first_seen_time")
                    && event
                        .get_str("google_secops.alert_v2.event.target.process.file.first_seen_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_secops.alert_v2.event.target.process.file.first_seen_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert_v2.event.target.process.file.first_seen_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "google_secops.alert_v2.event.target.process.file.first_seen_time".into(),
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
                        "date_event_target_process_file_first_seen_time",
                    )?;
                    event
                        .remove("google_secops.alert_v2.event.target.process.file.first_seen_time");
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
                event.has_value(
                    "google_secops.alert_v2.event.target.process.file.last_modification_time",
                ) && event.get_str(
                    "google_secops.alert_v2.event.target.process.file.last_modification_time",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_secops.alert_v2.event.target.process.file.last_modification_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_secops.alert_v2.event.target.process.file.last_modification_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_secops.alert_v2.event.target.process.file.last_modification_time".into(),
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
                        "date_event_target_process_file_last_modification_time",
                    )?;
                    event.remove(
                        "google_secops.alert_v2.event.target.process.file.last_modification_time",
                    );
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
                    .get("google_secops.alert_v2.event.target.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.user.email_addresses",
                        |event| {
                            event.append_unique(
                                "destination.user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.user.email_addresses",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.process_ancestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.process_ancestors") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.process_ancestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parent_process.file.md5")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.process_ancestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.process_ancestors") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.process_ancestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parent_process.file.sha1")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.process_ancestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.process_ancestors") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.process_ancestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parent_process.file.sha256")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.user.group_identifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.user.group_identifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.user.group_identifiers",
                        |event| {
                            event.append_unique(
                                "destination.user.group.id",
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
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.target.user.user_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.event.target.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.id", v)?;
            }

            let _cond =
                { event.has_value("google_secops.alert_v2.event.target.user.user_display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.user.user_display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert_v2.event.target.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert_v2.event.target.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert_v2.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.time_window.end_time")
                    && event.get_str("google_secops.alert_v2.time_window.end_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert_v2.time_window.end_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert_v2.time_window.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert_v2.time_window.end_time".into(),
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
                        "date_time_window_end_time",
                    )?;
                    event.remove("google_secops.alert_v2.time_window.end_time");
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
                .get("google_secops.alert_v2.time_window.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert_v2.time_window.start_time")
                    && event.get_str("google_secops.alert_v2.time_window.start_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert_v2.time_window.start_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("google_secops.alert_v2.time_window.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert_v2.time_window.start_time".into(),
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
                        "date_time_window_start_time",
                    )?;
                    event.remove("google_secops.alert_v2.time_window.start_time");
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
                .get("google_secops.alert_v2.time_window.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.target.ip", |event| {
                        event.remove("_ingest._value");
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.user.email_addresses",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.target.user.group_identifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.target.user.group_identifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.target.user.group_identifiers",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.bcc") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.bcc",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.cc") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.cc",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.subject") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.subject",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.email.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.email.to") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.email.to",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.group_identifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.group_identifiers")
                {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.group_identifiers",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.email_addresses",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.user.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.user.email_addresses") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.src.user.email_addresses",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.src.ip") {
                    foreach_array(event, "google_secops.alert_v2.event.src.ip", |event| {
                        event.remove("_ingest._value");
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.ip",
                        |event| {
                            event.remove("_ingest._value");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.answers",
                        |event| {
                            event.remove("_ingest._value.data");
                            event.remove("_ingest._value.name");
                            event.remove("_ingest._value.type");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.network.dns.questions",
                        |event| {
                            event.remove("_ingest._value.name");
                            event.remove("_ingest._value.type");
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert_v2.event.principal.user.attribute.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert_v2.event.principal.user.attribute.roles") {
                    foreach_array(
                        event,
                        "google_secops.alert_v2.event.principal.user.attribute.roles",
                        |event| {
                            event.remove("_ingest._value.name");
                            Ok(())
                        },
                    )?;
                }
            }

            if event.has_value("google_secops.alert_v2.detection") {
                foreach_array(event, "google_secops.alert_v2.detection", |event| {
                    event.remove("_ingest._value.description");
                    event.remove("_ingest._value.rule_id");
                    event.remove("_ingest._value.rule_name");
                    event.remove("_ingest._value.rule_version");
                    Ok(())
                })?;
            }

            event.remove("google_secops.alert_v2.created_time");
            event.remove("google_secops.alert_v2.detection_time");
            event.remove("google_secops.alert_v2.event.metadata.description");
            event.remove("google_secops.alert_v2.event.metadata.product_name");
            event.remove("google_secops.alert_v2.event.metadata.vendor_name");
            event.remove("google_secops.alert_v2.event.network.email.from");
            event.remove("google_secops.alert_v2.event.network.email.reply_to");
            event.remove("google_secops.alert_v2.event.network.http.method");
            event.remove("google_secops.alert_v2.event.network.http.response_code");
            event.remove("google_secops.alert_v2.event.network.http.user_agent");
            event.remove("google_secops.alert_v2.event.network.ip_protocol");
            event.remove("google_secops.alert_v2.event.principal.file.full_path");
            event.remove("google_secops.alert_v2.event.principal.file.md5");
            event.remove("google_secops.alert_v2.event.principal.file.sha1");
            event.remove("google_secops.alert_v2.event.principal.file.sha256");
            event.remove("google_secops.alert_v2.event.principal.group.group_display_name");
            event.remove("google_secops.alert_v2.event.principal.hostname");
            event.remove("google_secops.alert_v2.event.principal.process.command_line");
            event.remove("google_secops.alert_v2.event.principal.process.file.full_path");
            event.remove("google_secops.alert_v2.event.principal.process.file.md5");
            event.remove("google_secops.alert_v2.event.principal.process.file.sha1");
            event.remove("google_secops.alert_v2.event.principal.process.file.sha256");
            event.remove(
                "google_secops.alert_v2.event.principal.process.parent_process.command_line",
            );
            event.remove(
                "google_secops.alert_v2.event.principal.process.parent_process.file.full_path",
            );
            event.remove("google_secops.alert_v2.event.principal.process.parent_process.file.md5");
            event.remove("google_secops.alert_v2.event.principal.process.parent_process.file.sha1");
            event.remove(
                "google_secops.alert_v2.event.principal.process.parent_process.file.sha256",
            );
            event.remove("google_secops.alert_v2.event.principal.process.parent_process.pid");
            event.remove("google_secops.alert_v2.event.principal.process.pid");
            event.remove("google_secops.alert_v2.event.principal.user.user_display_name");
            event.remove("google_secops.alert_v2.event.principal.user.userid");
            event.remove("google_secops.alert_v2.event.src.hostname");
            event.remove("google_secops.alert_v2.event.src.user.userid");
            event.remove("google_secops.alert_v2.event.target.cloud.project.name");
            event.remove("google_secops.alert_v2.event.target.hostname");
            event.remove("google_secops.alert_v2.event.target.port");
            event.remove("google_secops.alert_v2.event.target.user.user_display_name");
            event.remove("google_secops.alert_v2.event.target.user.userid");
            event.remove("google_secops.alert_v2.id");
            event.remove("google_secops.alert_v2.time_window.end_time");
            event.remove("google_secops.alert_v2.time_window.start_time");

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
