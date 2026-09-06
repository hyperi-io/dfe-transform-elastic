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
                if let Some(v) = event.get("json.firstObserved") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastModifiedTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.vendorCaseId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Abnormal"))?;

            event.set("observer.product", json!("Inbound Email Security"))?;

            let _cond = {
                event.has_value("json.lastModifiedTime")
                    && event.get_str("json.lastModifiedTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastModifiedTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("abnormal_security.vendor_case.last_modified_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastModifiedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastModifiedTime")?;
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
                event.has_value("json.firstObservedTime")
                    && event.get_str("json.firstObservedTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstObservedTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("abnormal_security.vendor_case.first_observed_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.firstObservedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstObservedTime")?;
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
                .get("abnormal_security.vendor_case.first_observed_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("json.vendorCaseId") {
                if let Some(val) = event.get("json.vendorCaseId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.vendorCaseId".into(),
                            message,
                        }
                    })?;
                    event.set("abnormal_security.vendor_case.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("abnormal_security.vendor_case.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
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
                event.remove("abnormal_security.vendor_case.first_observed");
                event.remove("abnormal_security.vendor_case.id");
            }

            if event.has_value("json.vendorDomain") {
                event.rename("json.vendorDomain", "abnormal_security.vendor_case.domain")?;
            }

            let _cond = {
                event.has_value("json") && event.get("json.insights").is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: List keysToCopy = params.keysToCopy;\nList insights = new ArrayList();\n\nfor (insight in ctx.json.insights) {\n  Map new_insight = new HashMap();\n  for (key in keysToCopy) {\n    if (insight.containsKey(key)) {\n      new_insight[key] = insight[key];\n    }\n  }\n  insights.add(new_insight);\n}\nctx.abnormal_security.vendor_case.insights = insights;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"List keysToCopy = params.keysToCopy;\nList insights = new ArrayList();\n\nfor (insight in ctx.json.insights) {\n  Map new_insight = new HashMap();\n  for (key in keysToCopy) {\n    if (insight.containsKey(key)) {\n      new_insight[key] = insight[key];\n    }\n  }\n  insights.add(new_insight);\n}\nctx.abnormal_security.vendor_case.insights = insights;"#
                    ),
                    cached_params!("{\"keysToCopy\":[\"highlight\",\"description\"]}"),
                )?;
            }

            let _cond = {
                event.has_value("json") && event.get("json.timeline").is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: List keysToCopy = params.keysToCopy; List timeline = new ArrayList(); for (event in ctx.json.timeline) {\n  Map new_event = new HashMap();\n  for (key in keysToCopy) {\n    if (event.containsKey(key)) {\n     // Inline snake_case conversion logic\n      StringBuilder sb = new StringBuilder();\n      for (int i = 0; i < key.length(); i++) {\n        char c = key.charAt(i);\n        if (Character.isUpperCase(c)) {\n          if (i > 0) {\n            sb.append('_');\n          }\n          sb.append(Character.toLowerCase(c));\n        } else {\n          sb.append(c);\n        }\n      }\n      String snake_key = sb.toString();\n      new_event[snake_key] = event[key];\n    }\n  }\n  timeline.add(new_event);\n} ctx.abnormal_security.vendor_case.timeline = timeline;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"List keysToCopy = params.keysToCopy; List timeline = new ArrayList(); for (event in ctx.json.timeline) {\n  Map new_event = new HashMap();\n  for (key in keysToCopy) {\n    if (event.containsKey(key)) {\n     // Inline snake_case conversion logic\n      StringBuilder sb = new StringBuilder();\n      for (int i = 0; i < key.length(); i++) {\n        char c = key.charAt(i);\n        if (Character.isUpperCase(c)) {\n          if (i > 0) {\n            sb.append('_');\n          }\n          sb.append(Character.toLowerCase(c));\n        } else {\n          sb.append(c);\n        }\n      }\n      String snake_key = sb.toString();\n      new_event[snake_key] = event[key];\n    }\n  }\n  timeline.add(new_event);\n} ctx.abnormal_security.vendor_case.timeline = timeline;"#
                    ),
                    cached_params!(
                        "{\"keysToCopy\":[\"eventTimestamp\",\"senderAddress\",\"recipientAddress\",\"subject\",\"markedAs\",\"threatId\"]}"
                    ),
                )?;
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
