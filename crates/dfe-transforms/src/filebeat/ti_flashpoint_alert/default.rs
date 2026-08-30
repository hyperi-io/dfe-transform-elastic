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
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("alert"))?;

            // Painless script
            // Source: // Replace '-' with '_' in field names\nString normalize(String str) {\n  return str.replace('-', '_');\n}\n\n// Recursive function to process objects\ndef normalizeFields(def obj) {\n  if (obj instanceof Map) {\n    def newObj = new HashMap();\n    for (entry in obj.entrySet()) {\n      String newKey = normalize(entry.getKey());\n      newObj.put(newKey, normalizeFields(entry.getValue()));\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = new ArrayList();\n    for (item in obj) {\n      newList.add(normalizeFields(item));\n    }\n    return newList;\n  }\n  return obj;\n}\n\n// NOTE:\n// - Normalizes all field names (hyphen -> underscore)\n// - Populates ctx.ti_flashpoint.alert with normalized data\n// - Removes ctx.json after processing\nif (ctx.json != null) {\n  ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];\n  ctx.ti_flashpoint.alert = normalizeFields(ctx.json);\n  ctx.remove('json');\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Replace '-' with '_' in field names\nString normalize(String str) {\n  return str.replace('-', '_');\n}\n\n// Recursive function to process objects\ndef normalizeFields(def obj) {\n  if (obj instanceof Map) {\n    def newObj = new HashMap();\n    for (entry in obj.entrySet()) {\n      String newKey = normalize(entry.getKey());\n      newObj.put(newKey, normalizeFields(entry.getValue()));\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = new ArrayList();\n    for (item in obj) {\n      newList.add(normalizeFields(item));\n    }\n    return newList;\n  }\n  return obj;\n}\n\n// NOTE:\n// - Normalizes all field names (hyphen -> underscore)\n// - Populates ctx.ti_flashpoint.alert with normalized data\n// - Removes ctx.json after processing\nif (ctx.json != null) {\n  ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];\n  ctx.ti_flashpoint.alert = normalizeFields(ctx.json);\n  ctx.remove('json');\n}"#
                ),
            )?;

            let _cond = {
                event.has_value("ti_flashpoint.alert.created_at")
                    && event.get_str("ti_flashpoint.alert.created_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ti_flashpoint.alert.created_at") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd'T'HH:mm:ssXXXXX",
                                "yyyy-MM-dd' 'HH:mm:ss",
                                "yyyy-MM-dd' 'HH:mm:ssXXXXX",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("ti_flashpoint.alert.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.alert.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_alert_created_at_into_ti_flashpoint_alert_created_at_cce6fce9")?;
                    if event.remove("ti_flashpoint.alert.created_at").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.alert.created_at".into(),
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
                event.has_value("ti_flashpoint.alert.generated_at")
                    && event.get_str("ti_flashpoint.alert.generated_at") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ti_flashpoint.alert.generated_at")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd'T'HH:mm:ssXXXXX",
                                "yyyy-MM-dd' 'HH:mm:ss",
                                "yyyy-MM-dd' 'HH:mm:ssXXXXX",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.alert.generated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.alert.generated_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_alert_generated_at_into_ti_flashpoint_alert_generated_at_6be92a6f")?;
                    if event.remove("ti_flashpoint.alert.generated_at").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.alert.generated_at".into(),
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
                event.has_value("ti_flashpoint.alert.resource.sort_date")
                    && event.get_str("ti_flashpoint.alert.resource.sort_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.alert.resource.sort_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd'T'HH:mm:ssXXXXX",
                                "yyyy-MM-dd' 'HH:mm:ss",
                                "yyyy-MM-dd' 'HH:mm:ssXXXXX",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.alert.resource.sort_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.alert.resource.sort_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_alert_resource_sort_date_into_ti_flashpoint_alert_resource_sort_date_33d3d4c8")?;
                    if event
                        .remove("ti_flashpoint.alert.resource.sort_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.alert.resource.sort_date".into(),
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
                event.has_value("ti_flashpoint.alert.resource.created_at.date_time")
                    && event.get_str("ti_flashpoint.alert.resource.created_at.date_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("ti_flashpoint.alert.resource.created_at.date_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd' 'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd'T'HH:mm:ssXXXXX",
                                "yyyy-MM-dd' 'HH:mm:ssXXXXX",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ti_flashpoint.alert.resource.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ti_flashpoint.alert.resource.created_at.date_time"
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
                    event.set("_ingest.on_failure_processor_tag", "date_ti_flashpoint_alert_resource_created_at_date_time_into_ti_flashpoint_alert_resource_created_at_384a797f")?;
                    if event
                        .remove("ti_flashpoint.alert.resource.created_at.date_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "ti_flashpoint.alert.resource.created_at.date_time".into(),
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
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.alert.resource.media_v2").cloned();
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
                                if event.has_value("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult") {
                            if let Some(val) = event.get("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult".into(),
                            message,
                            })?;
                            event.set("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult", converted)?;
                            }
                            }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_image_enrichment_enrichments_v1_image_analysis_safe_search_adult_to_long_edf0408c")?;
                                if event.remove("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.adult".into() });
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
                            "ti_flashpoint.alert.resource.media_v2",
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
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.alert.resource.media_v2").cloned();
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
                                if event.has_value("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical") {
                            if let Some(val) = event.get("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical".into(),
                            message,
                            })?;
                            event.set("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical", converted)?;
                            }
                            }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_image_enrichment_enrichments_v1_image_analysis_safe_search_medical_to_long_400543e1")?;
                                if event.remove("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.medical".into() });
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
                            "ti_flashpoint.alert.resource.media_v2",
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
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.alert.resource.media_v2").cloned();
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
                                if event.has_value("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy") {
                            if let Some(val) = event.get("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy".into(),
                            message,
                            })?;
                            event.set("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy", converted)?;
                            }
                            }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_image_enrichment_enrichments_v1_image_analysis_safe_search_racy_to_long_f34e64b1")?;
                                if event.remove("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.racy".into() });
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
                            "ti_flashpoint.alert.resource.media_v2",
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
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.alert.resource.media_v2").cloned();
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
                                if event.has_value("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof") {
                            if let Some(val) = event.get("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof".into(),
                            message,
                            })?;
                            event.set("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof", converted)?;
                            }
                            }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_image_enrichment_enrichments_v1_image_analysis_safe_search_spoof_to_long_a6af8703")?;
                                if event.remove("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.spoof".into() });
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
                            "ti_flashpoint.alert.resource.media_v2",
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
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("ti_flashpoint.alert.resource.media_v2").cloned();
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
                                if event.has_value("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence") {
                            if let Some(val) = event.get("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence".into(),
                            message,
                            })?;
                            event.set("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence", converted)?;
                            }
                            }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert__ingest__value_image_enrichment_enrichments_v1_image_analysis_safe_search_violence_to_long_4517615d")?;
                                if event.remove("_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.image_enrichment.enrichments.v1.image_analysis.safe_search.violence".into() });
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
                            "ti_flashpoint.alert.resource.media_v2",
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
                if event.has_value("ti_flashpoint.alert.is_read") {
                    if let Some(val) = event.get("ti_flashpoint.alert.is_read") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "ti_flashpoint.alert.is_read".into(),
                                message,
                            }
                        })?;
                        event.set("ti_flashpoint.alert.is_read", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ti_flashpoint_alert_is_read_to_boolean_7b4e36be",
                )?;
                if event.remove("ti_flashpoint.alert.is_read").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "ti_flashpoint.alert.is_read".into(),
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

            if event.has_value("ti_flashpoint.alert.reason.id") {
                if let Some(val) = event.get("ti_flashpoint.alert.reason.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ti_flashpoint.alert.reason.id".into(),
                            message,
                        }
                    })?;
                    event.set("ti_flashpoint.alert.reason.id", converted)?;
                }
            }

            if event.has_value("ti_flashpoint.alert.resource.container.container.native_id") {
                if let Some(val) =
                    event.get("ti_flashpoint.alert.resource.container.container.native_id")
                {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ti_flashpoint.alert.resource.container.container.native_id"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "ti_flashpoint.alert.resource.container.container.native_id",
                        converted,
                    )?;
                }
            }

            if event.has_value("ti_flashpoint.alert.resource.container.native_id") {
                if let Some(val) = event.get("ti_flashpoint.alert.resource.container.native_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ti_flashpoint.alert.resource.container.native_id".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "ti_flashpoint.alert.resource.container.native_id",
                        converted,
                    )?;
                }
            }

            if event.has_value("ti_flashpoint.alert.resource.site_actor.native_id") {
                if let Some(val) = event.get("ti_flashpoint.alert.resource.site_actor.native_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "ti_flashpoint.alert.resource.site_actor.native_id".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "ti_flashpoint.alert.resource.site_actor.native_id",
                        converted,
                    )?;
                }
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.reason.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.resource.author")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.resource.container.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.name", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.resource.container.native_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if let Some(v) = event
                .get("ti_flashpoint.alert.resource.link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.append_unique(
                        "file.mime_type",
                        json!(
                            event
                                .get("_ingest._value.mime_type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.append_unique(
                        "file.hash.sha1",
                        json!(
                            event
                                .get("_ingest._value.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("ti_flashpoint.alert.resource.author") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ti_flashpoint.alert.resource.author")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.authors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.authors", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.phash")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.phash256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("ti_flashpoint.alert.resource.media_v2")
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
                foreach_array(event, "ti_flashpoint.alert.resource.media_v2", |event| {
                    event.remove("_ingest._value.mime_type");
                    event.remove("_ingest._value.sha1");
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
                event.remove("ti_flashpoint.alert.created_at");
                event.remove("ti_flashpoint.alert.id");
                event.remove("ti_flashpoint.alert.reason.name");
                event.remove("ti_flashpoint.alert.resource.author");
                event.remove("ti_flashpoint.alert.resource.container.name");
                event.remove("ti_flashpoint.alert.resource.container.native_id");
                event.remove("ti_flashpoint.alert.resource.link");
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
