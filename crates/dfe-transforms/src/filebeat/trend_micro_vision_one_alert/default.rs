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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.items")
                    && event.get("json.items").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.createdDateTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.updatedDateTime") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.alert = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.alert = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        if event.has_value("_ingest._value.entity_type") {
                            event.rename("_ingest._value.entity_type", "_ingest._value.type")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        if event.has_value("_ingest._value.entity_id") {
                            event.rename("_ingest._value.entity_id", "_ingest._value.id")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: for (def entity : ctx.trend_micro_vision_one.alert.impact_scope.entities) {\n  if (entity.containsKey('entity_value')) {\n    def ev = entity.remove('entity_value');\n    if (ev instanceof Map) {\n      entity.put('value', ev);\n    } else {\n      entity.put('value', ['account_value': ev]);\n    }\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (def entity : ctx.trend_micro_vision_one.alert.impact_scope.entities) {\n  if (entity.containsKey('entity_value')) {\n    def ev = entity.remove('entity_value');\n    if (ev instanceof Map) {\n      entity.put('value', ev);\n    } else {\n      entity.put('value', ['account_value': ev]);\n    }\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_handle_entity_value",
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
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        if event.has_value("_ingest._value.related_indicator_ids") {
                            event.rename(
                                "_ingest._value.related_indicator_ids",
                                "_ingest._value.related_indicator_id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: for (def indicator : ctx.trend_micro_vision_one.alert.indicators) {\n  if (indicator.containsKey('value') && indicator.get('value') instanceof Map) {\n    indicator.put('value_object', indicator.remove('value'));\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (def indicator : ctx.trend_micro_vision_one.alert.indicators) {\n  if (indicator.containsKey('value') && indicator.get('value') instanceof Map) {\n    indicator.put('value_object', indicator.remove('value'));\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_handle_indicator_value",
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
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.filter_ids") {
                        event.rename("_ingest._value.filter_ids", "_ingest._value.filter_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.matched_indicator_pattern_ids") {
                        event.rename(
                            "_ingest._value.matched_indicator_pattern_ids",
                            "_ingest._value.matched_indicator.pattern_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.first_seen_date_times") {
                        event.rename(
                            "_ingest._value.first_seen_date_times",
                            "_ingest._value.first_seen_date",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.last_seen_date_times") {
                        event.rename(
                            "_ingest._value.last_seen_date_times",
                            "_ingest._value.last_seen_date",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: for (def indicator : ctx.trend_micro_vision_one.alert.indicators) {\n  if (indicator.fields == null) {\n    continue;\n  }\n  def fields = indicator.get('fields');\n  if (fields instanceof String) {\n    continue;\n  }\n  if (!(fields instanceof List)) {\n    indicator.fields = null;\n    continue;\n  }\n  if (fields.size() > 0 && fields[0] instanceof List) {\n    if (fields.size() == 1) {\n      Collections.sort(fields[0]);\n      indicator.put('fields', fields[0]);\n    } else {\n      def set = new HashSet();\n      for (def a : fields) {\n        if (a instanceof List) {\n          for (def e : a) {\n            set.add(e);\n          }\n        }\n      }\n      def list = new ArrayList(set);\n      Collections.sort(list);\n      indicator.put('fields', list);\n    }\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (def indicator : ctx.trend_micro_vision_one.alert.indicators) {\n  if (indicator.fields == null) {\n    continue;\n  }\n  def fields = indicator.get('fields');\n  if (fields instanceof String) {\n    continue;\n  }\n  if (!(fields instanceof List)) {\n    indicator.fields = null;\n    continue;\n  }\n  if (fields.size() > 0 && fields[0] instanceof List) {\n    if (fields.size() == 1) {\n      Collections.sort(fields[0]);\n      indicator.put('fields', fields[0]);\n    } else {\n      def set = new HashSet();\n      for (def a : fields) {\n        if (a instanceof List) {\n          for (def e : a) {\n            set.add(e);\n          }\n        }\n      }\n      def list = new ArrayList(set);\n      Collections.sort(list);\n      indicator.put('fields', list);\n    }\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_flatten_indicator_fields",
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

            if event.has_value("trend_micro_vision_one.alert.matched_rules") {
                event.rename(
                    "trend_micro_vision_one.alert.matched_rules",
                    "trend_micro_vision_one.alert.matched_rule",
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.matched_filters") {
                            event.rename(
                                "_ingest._value.matched_filters",
                                "_ingest._value.filter",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.filter") {
                            foreach_array(event, "_ingest._value.filter", |event| {
                                if event.has_value("_ingest._value.mitre_technique_ids") {
                                    event.rename(
                                        "_ingest._value.mitre_technique_ids",
                                        "_ingest._value.mitre_technique_id",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.filter") {
                            foreach_array(event, "_ingest._value.filter", |event| {
                                if event.has_value("_ingest._value.matched_date_time") {
                                    event.rename(
                                        "_ingest._value.matched_date_time",
                                        "_ingest._value.date",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.filter") {
                            foreach_array(event, "_ingest._value.filter", |event| {
                                if event.has_value("_ingest._value.matched_events") {
                                    event.rename(
                                        "_ingest._value.matched_events",
                                        "_ingest._value.events",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.filter") {
                            foreach_array(event, "_ingest._value.filter", |event| {
                                if event.has_value("_ingest._value.events") {
                                    foreach_array(event, "_ingest._value.events", |event| {
                                        if event.has_value("_ingest._value.matched_date_time") {
                                            event.rename(
                                                "_ingest._value.matched_date_time",
                                                "_ingest._value.date",
                                            )?;
                                        }
                                        Ok(())
                                    })?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("trend_micro_vision_one.alert.matched_indicator_patterns") {
                event.rename(
                    "trend_micro_vision_one.alert.matched_indicator_patterns",
                    "trend_micro_vision_one.alert.matched_indicators_pattern",
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_indicators_pattern")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_indicators_pattern",
                    |event| {
                        if event.has_value("_ingest._value.matched_logs") {
                            event.rename(
                                "_ingest._value.matched_logs",
                                "_ingest._value.matched_log",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.alert.score") {
                    if let Some(val) = event.get("trend_micro_vision_one.alert.score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.score".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.alert.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                if event.remove("trend_micro_vision_one.alert.score").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.score".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.desktop_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.desktop_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.impact_scope.desktop_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.desktop_count",
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
                    "convert_impact_scope_desktop_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.desktop_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.desktop_count".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.server_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.server_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.impact_scope.server_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.server_count",
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
                    "convert_impact_scope_server_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.server_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.server_count".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.account_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.account_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.impact_scope.account_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.account_count",
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
                    "convert_impact_scope_account_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.account_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.account_count".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.cloud_identity_count")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.cloud_identity_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.alert.impact_scope.cloud_identity_count"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.cloud_identity_count",
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
                    "convert_impact_scope_cloud_identity_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.cloud_identity_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.cloud_identity_count"
                            .into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.cloud_workload_count")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.cloud_workload_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.alert.impact_scope.cloud_workload_count"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.cloud_workload_count",
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
                    "convert_impact_scope_cloud_workload_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.cloud_workload_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.cloud_workload_count"
                            .into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.container_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.container_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.impact_scope.container_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.container_count",
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
                    "convert_impact_scope_container_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.container_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.container_count".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.impact_scope.email_address_count")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.impact_scope.email_address_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.alert.impact_scope.email_address_count"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.impact_scope.email_address_count",
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
                    "convert_impact_scope_email_address_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.impact_scope.email_address_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.impact_scope.email_address_count"
                            .into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.total_indicator_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.total_indicator_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.total_indicator_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.total_indicator_count",
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
                    "convert_total_indicator_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.total_indicator_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.total_indicator_count".into(),
                    });
                }
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
                if event.has_value("trend_micro_vision_one.alert.matched_indicator_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.alert.matched_indicator_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.alert.matched_indicator_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.alert.matched_indicator_count",
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
                    "convert_matched_indicator_count_to_long",
                )?;
                if event
                    .remove("trend_micro_vision_one.alert.matched_indicator_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "trend_micro_vision_one.alert.matched_indicator_count".into(),
                    });
                }
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
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.related_indicator_id") {
                                if let Some(val) = event.get("_ingest._value.related_indicator_id")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.related_indicator_id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.related_indicator_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_related_indicator_id_to_long",
                            )?;
                            event.remove("_ingest._value.related_indicator_id");
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

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        if event.has_value("_ingest._value.value.ips") {
                            foreach_array(event, "_ingest._value.value.ips", |event| {
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
                                        "convert_entity_value_ips_to_ip",
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
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.value_object.ips") {
                        foreach_array(event, "_ingest._value.value_object.ips", |event| {
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
                                    "convert_value_object_ips_to_ip",
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
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.alert.first_investigated_date_time")
                    && event.get_str("trend_micro_vision_one.alert.first_investigated_date_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("trend_micro_vision_one.alert.first_investigated_date_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.alert.first_investigated_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "trend_micro_vision_one.alert.first_investigated_date_time"
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
                        "date_first_investigated_date_time",
                    )?;
                    event.remove("trend_micro_vision_one.alert.first_investigated_date_time");
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
                event.has_value("trend_micro_vision_one.alert.updated_date_time")
                    && event.get_str("trend_micro_vision_one.alert.updated_date_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.alert.updated_date_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.alert.updated_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.alert.updated_date_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updated_date_time")?;
                    event.remove("trend_micro_vision_one.alert.updated_date_time");
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
                event.has_value("trend_micro_vision_one.alert.created_date_time")
                    && event.get_str("trend_micro_vision_one.alert.created_date_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.alert.created_date_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.alert.created_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.alert.created_date_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_date_time")?;
                    event.remove("trend_micro_vision_one.alert.created_date_time");
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
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.alert.indicators")
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
                            if event.has_value("_ingest._value.first_seen_date") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.first_seen_date").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if let Some(date_str) =
                                                    event.get_as_string("_ingest._value")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["ISO8601"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => {
                                                            event.set("_ingest._value", parsed)?
                                                        }
                                                        None => {
                                                            return Err(
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value".into(),
                                                                    message: format!(
                                                                        "unable to parse date [{date_str}]"
                                                                    ),
                                                                },
                                                            );
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "date",
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_tag",
                                                    "date_first_seen_date",
                                                )?;
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.first_seen_date",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "trend_micro_vision_one.alert.indicators",
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
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.alert.indicators")
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
                            if event.has_value("_ingest._value.last_seen_date") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.last_seen_date").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if let Some(date_str) =
                                                    event.get_as_string("_ingest._value")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["ISO8601"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => {
                                                            event.set("_ingest._value", parsed)?
                                                        }
                                                        None => {
                                                            return Err(
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value".into(),
                                                                    message: format!(
                                                                        "unable to parse date [{date_str}]"
                                                                    ),
                                                                },
                                                            );
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "date",
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_tag",
                                                    "date_last_seen_date",
                                                )?;
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.last_seen_date",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "trend_micro_vision_one.alert.indicators",
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
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.alert.matched_rule")
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
                            if event.has_value("_ingest._value.filter") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.filter").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if let Some(date_str) =
                                                    event.get_as_string("_ingest._value.date")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["ISO8601"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => event
                                                            .set("_ingest._value.date", parsed)?,
                                                        None => {
                                                            return Err(
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.date"
                                                                        .into(),
                                                                    message: format!(
                                                                        "unable to parse date [{date_str}]"
                                                                    ),
                                                                },
                                                            );
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "date",
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_tag",
                                                    "date_filter_date",
                                                )?;
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.filter",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "trend_micro_vision_one.alert.matched_rule",
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
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("trend_micro_vision_one.alert.matched_rule")
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
                            if event.has_value("_ingest._value.filter") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.filter").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            if event.has_value("_ingest._value.events") {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.events").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 1 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if let Some(date_str) = event
                                                                    .get_as_string(
                                                                        "_ingest._value.date",
                                                                    )
                                                                {
                                                                    match parse_date_out(
                                                                        &date_str,
                                                                        &["ISO8601"],
                                                                        None,
                                                                        None,
                                                                    ) {
                                                                        Some(parsed) => event.set(
                                                                            "_ingest._value.date",
                                                                            parsed,
                                                                        )?,
                                                                        None => {
                                                                            return Err(TransformError::ParseError {
                            path: "_ingest._value.date".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                                                        }
                                                                    }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "date")?;
                                                                event.set("_ingest.on_failure_processor_tag", "date_event_date")?;
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.events",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
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
                                            "_ingest._value.filter",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "trend_micro_vision_one.alert.matched_rule",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.alert.workbench_link") {
                    if !uri_parts(
                        event,
                        "trend_micro_vision_one.alert.workbench_link",
                        "url",
                        true,
                        false,
                    )? && event
                        .get_str("trend_micro_vision_one.alert.workbench_link")
                        .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "trend_micro_vision_one.alert.workbench_link".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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
                .get("trend_micro_vision_one.alert.updated_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.alert.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.set("event.kind", json!("alert"))?;

            let _cond = {
                event.has_value("trend_micro_vision_one.alert.description")
                    && event.get_str("trend_micro_vision_one.alert.description") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef description = ctx.trend_micro_vision_one.alert.description.toLowerCase();\nif (description.contains('logon')) {\n  eventCategory.add('authentication');\n  eventCategory.add('host');\n  eventType.add('info');\n} else if (description.contains('email')) {\n  eventCategory.add('email');\n  eventType.add('info');\n} else if (description.contains('network')) {\n  eventCategory.add('network');\n  eventType.add('info');\n} else {\n  eventCategory.add('malware');\n  eventType.add('info');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef description = ctx.trend_micro_vision_one.alert.description.toLowerCase();\nif (description.contains('logon')) {\n  eventCategory.add('authentication');\n  eventCategory.add('host');\n  eventType.add('info');\n} else if (description.contains('email')) {\n  eventCategory.add('email');\n  eventType.add('info');\n} else if (description.contains('network')) {\n  eventCategory.add('network');\n  eventType.add('info');\n} else {\n  eventCategory.add('malware');\n  eventType.add('info');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_set_event_category_and_type",
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
                .get("trend_micro_vision_one.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.alert.created_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.alert.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString severity_value = ctx.trend_micro_vision_one.alert.severity;\nif (severity_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString severity_value = ctx.trend_micro_vision_one.alert.severity;\nif (severity_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
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
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.alert.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            if event.has_value("log.level") {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.matched_rule")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.matched_rule",
                    |event| {
                        if event.has_value("_ingest._value.filter") {
                            foreach_array(event, "_ingest._value.filter", |event| {
                                if event.has_value("_ingest._value.mitre_technique_id") {
                                    foreach_array(
                                        event,
                                        "_ingest._value.mitre_technique_id",
                                        |event| {
                                            event.append_unique(
                                                "threat.technique.id",
                                                json!(
                                                    event.get("_ingest._value").map_or_else(
                                                        String::new,
                                                        template_to_string
                                                    )
                                                ),
                                            )?;
                                            Ok(())
                                        },
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("threat.technique.id") };
            if _cond {
                event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.impact_scope.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.alert.impact_scope.entities",
                    |event| {
                        if event.has_value("_ingest._value.value.ips") {
                            foreach_array(event, "_ingest._value.value.ips", |event| {
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
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.alert.indicators")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.alert.indicators", |event| {
                    if event.has_value("_ingest._value.value_object.ips") {
                        foreach_array(event, "_ingest._value.value_object.ips", |event| {
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
                    Ok(())
                })?;
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
                event.remove("trend_micro_vision_one.alert.id");
                event.remove("trend_micro_vision_one.alert.severity");
                event.remove("trend_micro_vision_one.alert.created_date");
                event.remove("trend_micro_vision_one.alert.updated_date");
                event.remove("trend_micro_vision_one.alert.first_investigated_date_time");
            }

            event.remove("json");
            event.remove("trend_micro_vision_one.alert.created_date_time");
            event.remove("trend_micro_vision_one.alert.updated_date_time");

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
