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
                if let Some(v) = event.get("json.activity") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.category") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.details") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.loggedDateTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.loggedRole") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.loggedUser") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.audit = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.audit = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.audit.logged_date_time")
                    && event.get_str("trend_micro_vision_one.audit.logged_date_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.audit.logged_date_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.audit.logged_date_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.audit.logged_date_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_logged_date_time")?;
                    event.remove("trend_micro_vision_one.audit.logged_date_time");
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
                event.has_value("trend_micro_vision_one.audit.ingested_date_time")
                    && event.get_str("trend_micro_vision_one.audit.ingested_date_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.audit.ingested_date_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.audit.ingested_date_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.audit.ingested_date_time".into(),
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
                        "date_ingested_date_time",
                    )?;
                    event.remove("trend_micro_vision_one.audit.ingested_date_time");
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
                .get("trend_micro_vision_one.audit.logged_date_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("trend_micro_vision_one.audit.category")
                    && event.get_str("trend_micro_vision_one.audit.category") != Some("")
                    && event.has_value("trend_micro_vision_one.audit.activity")
                    && event.get_str("trend_micro_vision_one.audit.activity") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef category = ctx.trend_micro_vision_one.audit.category.toLowerCase();\ndef activity = ctx.trend_micro_vision_one.audit.activity.toLowerCase();\nif (['logon and logoff', 'saml single sign-on'].contains(category)) {\n  eventCategory.add('authentication');\n  if (['log on', 'enable single sign-on'].contains(activity)) {\n    eventType.add('start');\n  }\n  if (['log off', 'disable single sign-on'].contains(activity)) {\n    eventType.add('end');\n  } else {\n    eventType.add('info');\n  }\n}\nif (['account management', 'product connector', 'Notifications', 'detection model management', 'workbench', 'response management', 'search', 'managed xdr', 'third-party integration', 'service gateway inventory', 'endpoint inventory', 'endpoint security policies', 'zero trust secure access', 'sandbox analysis', 'oat', 'security playbooks'].contains(category)) {\n  eventCategory.add('authentication');\n  eventType.add('info');\n}\nif (category == 'network inventory') {\n  eventCategory.add('network');\n  eventType.add('info');\n}\nif (category == 'threat intelligence') {\n  eventCategory.add('threat');\n  eventType.add('indicator');\n}\nif (activity == 'email') {\n  eventCategory.add('email');\n}\nif (activity == 'file') {\n  eventCategory.add('file');\n}\nif (activity == 'threat') {\n  eventCategory.add('threat');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventCategory = new HashSet();\ndef eventType = new HashSet();\ndef category = ctx.trend_micro_vision_one.audit.category.toLowerCase();\ndef activity = ctx.trend_micro_vision_one.audit.activity.toLowerCase();\nif (['logon and logoff', 'saml single sign-on'].contains(category)) {\n  eventCategory.add('authentication');\n  if (['log on', 'enable single sign-on'].contains(activity)) {\n    eventType.add('start');\n  }\n  if (['log off', 'disable single sign-on'].contains(activity)) {\n    eventType.add('end');\n  } else {\n    eventType.add('info');\n  }\n}\nif (['account management', 'product connector', 'Notifications', 'detection model management', 'workbench', 'response management', 'search', 'managed xdr', 'third-party integration', 'service gateway inventory', 'endpoint inventory', 'endpoint security policies', 'zero trust secure access', 'sandbox analysis', 'oat', 'security playbooks'].contains(category)) {\n  eventCategory.add('authentication');\n  eventType.add('info');\n}\nif (category == 'network inventory') {\n  eventCategory.add('network');\n  eventType.add('info');\n}\nif (category == 'threat intelligence') {\n  eventCategory.add('threat');\n  eventType.add('indicator');\n}\nif (activity == 'email') {\n  eventCategory.add('email');\n}\nif (activity == 'file') {\n  eventCategory.add('file');\n}\nif (activity == 'threat') {\n  eventCategory.add('threat');\n}\nif (!eventCategory.isEmpty()) {\n  ctx.event.category = eventCategory;\n}\nif (!eventType.isEmpty()) {\n  ctx.event.type = eventType;\n}\n"#
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

            let _cond =
                { event.get_str("trend_micro_vision_one.audit.result") == Some("Successful") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond =
                { event.get_str("trend_micro_vision_one.audit.result") == Some("Unsuccessful") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.audit.logged_user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.audit.logged_role") };
            if _cond {
                event.append_unique(
                    "source.user.roles",
                    json!(
                        event
                            .get("trend_micro_vision_one.audit.logged_role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.audit.logged_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("trend_micro_vision_one.audit.logged_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

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
                event.remove("trend_micro_vision_one.audit.logged_date_time");
                event.remove("trend_micro_vision_one.audit.logged_user");
                event.remove("trend_micro_vision_one.audit.logged_role");
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
