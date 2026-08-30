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
            let _cond = { event.get_str("message") == Some("empty_events_placeholder") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("9.3.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.modified_at") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.type", json!("indicator"))?;

            event.append("event.category", json!("threat"))?;

            // Painless script
            // Source: // Replace '-' with '_' in field names\nString normalize(String str) {\n  return str.replace('-', '_');\n}\n\n// Recursive function to process objects\ndef normalizeFields(def obj) {\n  if (obj instanceof Map) {\n    def newObj = new HashMap();\n    for (entry in obj.entrySet()) {\n      String newKey = normalize(entry.getKey());\n      newObj.put(newKey, normalizeFields(entry.getValue()));\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = new ArrayList();\n    for (item in obj) {\n      newList.add(normalizeFields(item));\n    }\n    return newList;\n  }\n  return obj;\n}\n\n// NOTE:\n// - Normalizes all field names (hyphen -> underscore)\n// - Populates ctx.ti_flashpoint.indicator with normalized data\n// - Removes ctx.json after processing\nif (ctx.json != null) {\n  ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];\n  ctx.ti_flashpoint.indicator = normalizeFields(ctx.json);\n  ctx.remove('json');\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Replace '-' with '_' in field names\nString normalize(String str) {\n  return str.replace('-', '_');\n}\n\n// Recursive function to process objects\ndef normalizeFields(def obj) {\n  if (obj instanceof Map) {\n    def newObj = new HashMap();\n    for (entry in obj.entrySet()) {\n      String newKey = normalize(entry.getKey());\n      newObj.put(newKey, normalizeFields(entry.getValue()));\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = new ArrayList();\n    for (item in obj) {\n      newList.add(normalizeFields(item));\n    }\n    return newList;\n  }\n  return obj;\n}\n\n// NOTE:\n// - Normalizes all field names (hyphen -> underscore)\n// - Populates ctx.ti_flashpoint.indicator with normalized data\n// - Removes ctx.json after processing\nif (ctx.json != null) {\n  ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];\n  ctx.ti_flashpoint.indicator = normalizeFields(ctx.json);\n  ctx.remove('json');\n}"#
                ),
            )?;

            let _cond = {
                event.has_value("ti_flashpoint.indicator.created_at")
                    && event.get_str("ti_flashpoint.indicator.created_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.indicator.created_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.indicator.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_created_at_into_ti_flashpoint_indicator_created_at_425aeb53")?;
                    if event.remove("ti_flashpoint.indicator.created_at").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.created_at".into(),
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
            }

            let _cond = {
                event.has_value("ti_flashpoint.indicator.last_seen_at")
                    && event.get_str("ti_flashpoint.indicator.last_seen_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.indicator.last_seen_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.indicator.last_seen_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.last_seen_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_last_seen_at_into_ti_flashpoint_indicator_last_seen_at_89fe6397")?;
                    if event
                        .remove("ti_flashpoint.indicator.last_seen_at")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.last_seen_at".into(),
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
            }

            let _cond = {
                event.has_value("ti_flashpoint.indicator.modified_at")
                    && event.get_str("ti_flashpoint.indicator.modified_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.indicator.modified_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.indicator.modified_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.modified_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_modified_at_into_ti_flashpoint_indicator_modified_at_ab37f0f7")?;
                    if event
                        .remove("ti_flashpoint.indicator.modified_at")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.modified_at".into(),
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
            }

            let _cond = {
                event.has_value("ti_flashpoint.indicator.score.last_scored_at")
                    && event.get_str("ti_flashpoint.indicator.score.last_scored_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.indicator.score.last_scored_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.indicator.score.last_scored_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.score.last_scored_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_score_last_scored_at_into_ti_flashpoint_indicator_score_last_scored_at_3a1841be")?;
                    if event
                        .remove("ti_flashpoint.indicator.score.last_scored_at")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.score.last_scored_at".into(),
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
            }

            let _cond = {
                event.has_value("ti_flashpoint.indicator.sort_date")
                    && event.get_str("ti_flashpoint.indicator.sort_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ti_flashpoint.indicator.sort_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.indicator.sort_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.sort_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_sort_date_into_ti_flashpoint_indicator_sort_date_5dd8c107")?;
                    if event.remove("ti_flashpoint.indicator.sort_date").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.sort_date".into(),
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
            }

            let _cond = {
                event.has_value("ti_flashpoint.indicator.latest_sighting.sighted_at")
                    && event.get_str("ti_flashpoint.indicator.latest_sighting.sighted_at")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.indicator.latest_sighting.sighted_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "ti_flashpoint.indicator.latest_sighting.sighted_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.indicator.latest_sighting.sighted_at"
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
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_indicator_latest_sighting_sighted_at_into_ti_flashpoint_indicator_latest_sighting_sighted_at_e2c676a4")?;
                    if event
                        .remove("ti_flashpoint.indicator.latest_sighting.sighted_at")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.indicator.latest_sighting.sighted_at".into(),
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
            }

            let _cond = {
                event
                    .get("ti_flashpoint.indicator.sightings")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.indicator.sightings").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.sighted_at")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.sighted_at", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.sighted_at".into(),
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
                                event.set("_ingest.on_failure_processor_tag", "date__ingest__value_sighted_at_into__ingest__value_sighted_at_2c9dd7b3")?;
                                if event.remove("_ingest._value.sighted_at").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.sighted_at".into(),
                                    });
                                }
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
                            "ti_flashpoint.indicator.sightings",
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
                    .get("ti_flashpoint.indicator.latest_sighting.related_iocs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("ti_flashpoint.indicator.latest_sighting.related_iocs")
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.score.last_scored_at")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.score.last_scored_at", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.score.last_scored_at".into(),
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
                                event.set("_ingest.on_failure_processor_tag", "date__ingest__value_score_last_scored_at_into__ingest__value_score_last_scored_at_5db754c4")?;
                                if event
                                    .remove("_ingest._value.score.last_scored_at")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.score.last_scored_at".into(),
                                    });
                                }
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
                            "ti_flashpoint.indicator.latest_sighting.related_iocs",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("ti_flashpoint.indicator.total_sightings") {
                    if let Some(val) = event.get("ti_flashpoint.indicator.total_sightings") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "ti_flashpoint.indicator.total_sightings".into(),
                                message,
                            }
                        })?;
                        event.set("ti_flashpoint.indicator.total_sightings", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ti_flashpoint_indicator_total_sightings_to_long_398da6b1",
                )?;
                if event
                    .remove("ti_flashpoint.indicator.total_sightings")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "ti_flashpoint.indicator.total_sightings".into(),
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
                    .get("ti_flashpoint.indicator.latest_sighting.related_iocs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("ti_flashpoint.indicator.latest_sighting.related_iocs")
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
                                if event.has_value("_ingest._value.score.raw_score") {
                                    if let Some(val) = event.get("_ingest._value.score.raw_score") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.score.raw_score".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.score.raw_score", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert__ingest__value_score_raw_score_to_long_bbda34b7",
                                )?;
                                if event.remove("_ingest._value.score.raw_score").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.score.raw_score".into(),
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
                            "ti_flashpoint.indicator.latest_sighting.related_iocs",
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
                    .get("ti_flashpoint.indicator.sightings")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.indicator.sightings").cloned();
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
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.related_iocs").cloned();
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
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.score.raw_score")
                                                {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.score.raw_score")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                            path: "_ingest._value.score.raw_score".into(),
                            message,
                            }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.score.raw_score",
                                                            converted,
                                                        )?;
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
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_score_raw_score_to_long_56228ea4")?;
                                                if event
                                                    .remove("_ingest._value.score.raw_score")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.score.raw_score"
                                                            .into(),
                                                    });
                                                }
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
                                            "_ingest._value.related_iocs",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
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
                            "ti_flashpoint.indicator.sightings",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.hashes.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.hashes.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.hashes.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.last_seen_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.last_seen", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.malware_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.malware_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.description", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.modified_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.indicator.modified_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.modified_at", v)?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.id") };
            if _cond {
                event.append_unique(
                    "threat.indicator.id",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.platform_urls.ignite") };
            if _cond {
                event.append_unique(
                    "threat.indicator.reference",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.platform_urls.ignite")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.href") };
            if _cond {
                event.append_unique(
                    "threat.indicator.reference",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.href")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.indicator.mitre_attack_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.indicator.mitre_attack_ids", |event| {
                    event.append_unique(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("_ingest._value.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.indicator.mitre_attack_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.indicator.mitre_attack_ids", |event| {
                    event.append_unique(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.type") };
            if _cond {
                // Painless script
                // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.type = params[ctx.ti_flashpoint.indicator.type];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.type = params[ctx.ti_flashpoint.indicator.type];"#
                    ),
                    cached_params!(
                        "{\"domain\":\"domain-name\",\"file\":\"file\",\"ipv4\":\"ipv4-addr\",\"url\":\"url\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.hashes.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.hashes.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.hashes.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.hashes.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_flashpoint.indicator.hashes.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("ti_flashpoint.indicator.hashes.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.indicator.mitre_attack_ids")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.indicator.mitre_attack_ids", |event| {
                    event.remove("_ingest._value.id");
                    event.remove("_ingest._value.name");
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
                event.remove("ti_flashpoint.indicator.created_at");
                event.remove("ti_flashpoint.indicator.hashes.md5");
                event.remove("ti_flashpoint.indicator.hashes.sha1");
                event.remove("ti_flashpoint.indicator.hashes.sha256");
                event.remove("ti_flashpoint.indicator.href");
                event.remove("ti_flashpoint.indicator.id");
                event.remove("ti_flashpoint.indicator.last_seen_at");
                event.remove("ti_flashpoint.indicator.malware_description");
                event.remove("ti_flashpoint.indicator.modified_at");
                event.remove("ti_flashpoint.indicator.platform_urls.ignite");
            }

            // Painless script
            // Source: void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);"#
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
