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

            event.set("ecs.version", json!("8.11.0"))?;

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
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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
                            .get("_ingest.pipeline")
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
                event.has_value("json.listFindingsResults")
                    && event
                        .get("json.listFindingsResults")
                        .is_some_and(|v| match v {
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
                if let Some(v) = event.get("json.finding.createTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.finding.eventTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.finding.muteUpdateTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.finding.name") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.finding.resourceName") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.resource") {
                event.rename("json.resource", "json.finding.resource")?;
            }

            if event.has_value("json.stateChange") {
                event.rename("json.stateChange", "json.finding.stateChange")?;
            }

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.google_scc = ctx.google_scc ?: [:];\nif (ctx.json != null) {\n  ctx.google_scc.finding = convertToSnakeCase(ctx.json.finding);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n// Apply the conversion\nctx.google_scc = ctx.google_scc ?: [:];\nif (ctx.json != null) {\n  ctx.google_scc.finding = convertToSnakeCase(ctx.json.finding);\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("google_scc.finding.event_time")
                    && event.get_str("google_scc.finding.event_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("google_scc.finding.event_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("google_scc.finding.event_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.event_time".into(),
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
                        "date_finding_event_time",
                    )?;
                    if event.remove("google_scc.finding.event_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.event_time".into(),
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
                                .get("_ingest.pipeline")
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
                event.has_value("google_scc.finding.create_time")
                    && event.get_str("google_scc.finding.create_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("google_scc.finding.create_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("google_scc.finding.create_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.create_time".into(),
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
                        "date_finding_createTime",
                    )?;
                    if event.remove("google_scc.finding.create_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.create_time".into(),
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
                                .get("_ingest.pipeline")
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.connections").cloned();
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
                                if event.has_value("_ingest._value.destination_ip") {
                                    if let Some(val) = event.get("_ingest._value.destination_ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.destination_ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.destination.ip", converted)?;
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
                                if event.remove("_ingest._value.destination_ip").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.destination_ip".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "google_scc.finding.connections",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.connections").cloned();
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
                                if event.has_value("_ingest._value.destination_port") {
                                    if let Some(val) = event.get("_ingest._value.destination_port")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.destination_port".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.destination.port", converted)?;
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
                                if event.remove("_ingest._value.destination_port").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.destination_port".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "google_scc.finding.connections",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.connections").cloned();
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
                                if event.has_value("_ingest._value.source_ip") {
                                    if let Some(val) = event.get("_ingest._value.source_ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.source_ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.source.ip", converted)?;
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
                                if event.remove("_ingest._value.source_ip").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.source_ip".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "google_scc.finding.connections",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.connections").cloned();
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
                                if event.has_value("_ingest._value.source_port") {
                                    if let Some(val) = event.get("_ingest._value.source_port") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.source_port".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.source.port", converted)?;
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
                                if event.remove("_ingest._value.source_port").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.source_port".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "google_scc.finding.connections",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.remove("_ingest._value.source_port");
                    event.remove("_ingest._value.source_ip");
                    event.remove("_ingest._value.destination_port");
                    event.remove("_ingest._value.destination_ip");
                    Ok(())
                })?;
            }

            if event.has_value("google_scc.finding.mitre_attack.additional_tactics") {
                event.rename(
                    "google_scc.finding.mitre_attack.additional_tactics",
                    "google_scc.finding.mitre_attack.additional.tactics",
                )?;
            }

            if event.has_value("google_scc.finding.mitre_attack.primary_tactic") {
                event.rename(
                    "google_scc.finding.mitre_attack.primary_tactic",
                    "google_scc.finding.mitre_attack.primary.tactic",
                )?;
            }

            if event.has_value("google_scc.finding.mitre_attack.additional_techniques") {
                event.rename(
                    "google_scc.finding.mitre_attack.additional_techniques",
                    "google_scc.finding.mitre_attack.additional.techniques",
                )?;
            }

            if event.has_value("google_scc.finding.mitre_attack.primary_techniques") {
                event.rename(
                    "google_scc.finding.mitre_attack.primary_techniques",
                    "google_scc.finding.mitre_attack.primary.techniques",
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.log_entries")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.log_entries").cloned();
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
                            if let Err(err) =
                                (|| -> Result<()> {
                                    if let Some(date_str) = event.get_as_string(
                                        "_ingest._value.cloud_logging_entry.timestamp",
                                    ) {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.cloud_logging_entry.timestamp",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.cloud_logging_entry.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                            }
                                        }
                                    }
                                    Ok(())
                                })()
                            {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_finding_log_entries_cloud_logging_entry_timestamp",
                                )?;
                                event.remove("_ingest._value.cloud_logging_entry.timestamp");
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
                            "google_scc.finding.log_entries",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get_str("google_scc.finding.access.caller_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.access.caller_ip") {
                        if let Some(val) = event.get("google_scc.finding.access.caller_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.access.caller_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.finding.access.caller_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_finding_access_caller_ip_to_ip",
                    )?;
                    if event
                        .remove("google_scc.finding.access.caller_ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.access.caller_ip".into(),
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
                                .get("_ingest.pipeline")
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

            if event.has_value("google_scc.finding.access.principal_email") {
                event.rename(
                    "google_scc.finding.access.principal_email",
                    "google_scc.finding.access.principal.email",
                )?;
            }

            if event.has_value("google_scc.finding.access.principal_subject") {
                event.rename(
                    "google_scc.finding.access.principal_subject",
                    "google_scc.finding.access.principal.subject",
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.access.service_account_delegation_info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.access.service_account_delegation_info",
                    |event| {
                        if event.has_value("_ingest._value.principal_email") {
                            event.rename(
                                "_ingest._value.principal_email",
                                "_ingest._value.principal.email",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.access.service_account_delegation_info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.access.service_account_delegation_info",
                    |event| {
                        if event.has_value("_ingest._value.principal_subject") {
                            event.rename(
                                "_ingest._value.principal_subject",
                                "_ingest._value.principal.subject",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("google_scc.finding.access.service_account_delegation_info") {
                event.rename(
                    "google_scc.finding.access.service_account_delegation_info",
                    "google_scc.finding.access.service_account.delegation_info",
                )?;
            }

            if event.has_value("google_scc.finding.access.service_account_key_name") {
                event.rename(
                    "google_scc.finding.access.service_account_key_name",
                    "google_scc.finding.access.service_account.key_name",
                )?;
            }

            if event.has_value("google_scc.finding.finding_class") {
                event.rename(
                    "google_scc.finding.finding_class",
                    "google_scc.finding.class",
                )?;
            }

            if event.has_value("google_scc.finding.cloud_dlp_data_profile.data_profile") {
                event.rename(
                    "google_scc.finding.cloud_dlp_data_profile.data_profile",
                    "google_scc.finding.cloud_dlp.data_profile.value",
                )?;
            }

            let _cond =
                { event.get_str("google_scc.finding.cloud_dlp_inspection.full_scan") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.cloud_dlp_inspection.full_scan") {
                        if let Some(val) =
                            event.get("google_scc.finding.cloud_dlp_inspection.full_scan")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.cloud_dlp_inspection.full_scan"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.finding.cloud_dlp.inspection.full_scan",
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
                        "convert_finding_cloud_dlp_inspection_full_scan_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.cloud_dlp_inspection.info_type_count") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.cloud_dlp_inspection.info_type_count") {
                        if let Some(val) =
                            event.get("google_scc.finding.cloud_dlp_inspection.info_type_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.cloud_dlp_inspection.info_type_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.finding.cloud_dlp.inspection.info_type.count",
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
                        "convert_finding_cloud_dlp_inspection_info_type_count_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("google_scc.finding.cloud_dlp_inspection.info_type") {
                event.rename(
                    "google_scc.finding.cloud_dlp_inspection.info_type",
                    "google_scc.finding.cloud_dlp.inspection.info_type.value",
                )?;
            }

            if event.has_value("google_scc.finding.cloud_dlp_inspection.inspect_job") {
                event.rename(
                    "google_scc.finding.cloud_dlp_inspection.inspect_job",
                    "google_scc.finding.cloud_dlp.inspection.inspect_job",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.billing.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.billing.contacts",
                    "google_scc.finding.contacts.billing",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.legal.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.legal.contacts",
                    "google_scc.finding.contacts.legal",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.security.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.security.contacts",
                    "google_scc.finding.contacts.security",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.all.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.all.contacts",
                    "google_scc.finding.contacts.all",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.product_updates.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.product_updates.contacts",
                    "google_scc.finding.contacts.product_updates",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.suspension.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.suspension.contacts",
                    "google_scc.finding.contacts.suspension",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.technical.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.technical.contacts",
                    "google_scc.finding.contacts.technical",
                )?;
            }

            if event.has_value("google_scc.finding.contacts.technical_incidents.contacts") {
                event.rename_over(
                    "google_scc.finding.contacts.technical_incidents.contacts",
                    "google_scc.finding.contacts.technical_incidents",
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.files").cloned();
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
                                if event.has_value("_ingest._value.size") {
                                    if let Some(val) = event.get("_ingest._value.size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_files_size_to_long",
                                )?;
                                if event.remove("_ingest._value.size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.size".into(),
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
                            "google_scc.finding.files",
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
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.files").cloned();
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
                                if event.has_value("_ingest._value.hashed_size") {
                                    if let Some(val) = event.get("_ingest._value.hashed_size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.hashed_size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.hashed_size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_files_hashed_size_to_long",
                                )?;
                                if event.remove("_ingest._value.hashed_size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.hashed_size".into(),
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
                            "google_scc.finding.files",
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
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.files").cloned();
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
                                if event.has_value("_ingest._value.partially_hashed") {
                                    if let Some(val) = event.get("_ingest._value.partially_hashed")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.partially_hashed".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.partially_hashed", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_files_partially_hashed_to_boolean",
                                )?;
                                if event.remove("_ingest._value.partially_hashed").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.partially_hashed".into(),
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
                            "google_scc.finding.files",
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
                    .get("google_scc.finding.indicator.ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("google_scc.finding.indicator.ip_addresses")
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
                                    "convert_finding_indicator_ip_addresses_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "google_scc.finding.indicator.ip_addresses",
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
                    .get("google_scc.finding.indicator.signatures")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("google_scc.finding.indicator.signatures")
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
                            if event.has_value("_ingest._value.memory_hash_signature.detections") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event
                                        .get("_ingest._value.memory_hash_signature.detections")
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value(
                                                    "_ingest._value.percent_pages_matched",
                                                ) {
                                                    if let Some(val) = event
                                                        .get("_ingest._value.percent_pages_matched")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                            path: "_ingest._value.percent_pages_matched".into(),
                            message,
                            }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.percent_pages_matched",
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
                                                event.set("_ingest.on_failure_processor_tag", "convert_finding_indicator_signatures_percent_pages_matched_to_long")?;
                                                if event
                                                    .remove("_ingest._value.percent_pages_matched")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path:
                                                            "_ingest._value.percent_pages_matched"
                                                                .into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}'Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                            "_ingest._value.memory_hash_signature.detections",
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
                            "google_scc.finding.indicator.signatures",
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
                    .get("google_scc.finding.indicator.signatures")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.indicator.signatures", |event| {
                    if event.has_value("_ingest._value.yara_rule_signature.yara_rule") {
                        event.rename(
                            "_ingest._value.yara_rule_signature.yara_rule",
                            "_ingest._value.yara.rule",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_code_modification")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_code_modification",
                    ) {
                        if let Some(val) = event
                            .get("google_scc.finding.kernel_root_kit.unexpected_code_modification")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_code_modification".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.code_modification",
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
                        "convert_finding_kernel_root_kit_unexpected_code_modification_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_ftrace_handler")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("google_scc.finding.kernel_root_kit.unexpected_ftrace_handler")
                    {
                        if let Some(val) = event
                            .get("google_scc.finding.kernel_root_kit.unexpected_ftrace_handler")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_ftrace_handler".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.ftrace_handler",
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
                        "convert_finding_kernel_root_kit_unexpected_ftrace_handler_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_interrupt_handler")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_interrupt_handler",
                    ) {
                        if let Some(val) = event
                            .get("google_scc.finding.kernel_root_kit.unexpected_interrupt_handler")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_interrupt_handler".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.interrupt_handler",
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
                        "convert_finding_kernel_root_kit_unexpected_interrupt_handler_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_kernel_code_pages")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_kernel_code_pages",
                    ) {
                        if let Some(val) = event
                            .get("google_scc.finding.kernel_root_kit.unexpected_kernel_code_pages")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_kernel_code_pages".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.kernel_code_pages",
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
                        "convert_finding_kernel_root_kit_unexpected_kernel_code_pages_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_kprobe_handler")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("google_scc.finding.kernel_root_kit.unexpected_kprobe_handler")
                    {
                        if let Some(val) = event
                            .get("google_scc.finding.kernel_root_kit.unexpected_kprobe_handler")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_kprobe_handler".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.kprobe_handler",
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
                        "convert_finding_kernel_root_kit_unexpected_kprobe_handler_to_boolean",
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_processes_in_runqueue")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_processes_in_runqueue",
                    ) {
                        if let Some(val) = event.get(
                            "google_scc.finding.kernel_root_kit.unexpected_processes_in_runqueue",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_processes_in_runqueue".into(),
                            message,
                        })?;
                            event.set("google_scc.finding.kernel_root_kit.unexpected.processes_in_runqueue", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_finding_kernel_root_kit_unexpected_processes_in_runqueue_to_boolean")?;
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
                                .get("_ingest.pipeline")
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
                event.get_str(
                    "google_scc.finding.kernel_root_kit.unexpected_read_only_data_modification",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_read_only_data_modification",
                    ) {
                        if let Some(val) = event.get("google_scc.finding.kernel_root_kit.unexpected_read_only_data_modification") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_read_only_data_modification".into(),
                            message,
                        })?;
                    event.set("google_scc.finding.kernel_root_kit.unexpected.read_only_data_modification", converted)?;
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_finding_kernel_root_kit_unexpected_read_only_data_modification_to_boolean")?;
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.kernel_root_kit.unexpected_system_call_handler")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "google_scc.finding.kernel_root_kit.unexpected_system_call_handler",
                    ) {
                        if let Some(val) = event.get(
                            "google_scc.finding.kernel_root_kit.unexpected_system_call_handler",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.kernel_root_kit.unexpected_system_call_handler".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.kernel_root_kit.unexpected.system_call_handler",
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
                        "convert_finding_kernel_root_kit_unexpected_system_call_handler_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("google_scc.finding.mute") {
                event.rename("google_scc.finding.mute", "google_scc.finding.mute.state")?;
            }

            if event.has_value("google_scc.finding.mute_initiator") {
                event.rename(
                    "google_scc.finding.mute_initiator",
                    "google_scc.finding.mute.initiator",
                )?;
            }

            let _cond = {
                event.has_value("google_scc.finding.mute_update_time")
                    && event.get_str("google_scc.finding.mute_update_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_scc.finding.mute_update_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_scc.finding.mute.update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.mute_update_time".into(),
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
                        "date_finding_mute_update_time",
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
                                .get("_ingest.pipeline")
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
                event.has_value("google_scc.finding.mute_info.static_mute.apply_time")
                    && event.get_str("google_scc.finding.mute_info.static_mute.apply_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_scc.finding.mute_info.static_mute.apply_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_scc.finding.mute_info.static_mute.apply_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.mute_info.static_mute.apply_time"
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
                        "date_finding_mute_info_static_mute_apply_time",
                    )?;
                    if event
                        .remove("google_scc.finding.mute_info.static_mute.apply_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.mute_info.static_mute.apply_time".into(),
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
                                .get("_ingest.pipeline")
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
                    .get("google_scc.finding.mute_info.dynamic_mute_records")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("google_scc.finding.mute_info.dynamic_mute_records")
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
                                    event.get_as_string("_ingest._value.match_time")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.match_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.match_time".into(),
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
                                    "date_finding_mute_info_dynamic_mute_records_match_time",
                                )?;
                                event.remove("_ingest._value.match_time");
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
                            "google_scc.finding.mute_info.dynamic_mute_records",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.binary.size") {
                                    if let Some(val) = event.get("_ingest._value.binary.size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.binary.size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.binary.size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_finding_processes_binary_size_to_long",
                                )?;
                                if event.remove("_ingest._value.binary.size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.binary.size".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.binary.hashed_size") {
                                    if let Some(val) =
                                        event.get("_ingest._value.binary.hashed_size")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.binary.hashed_size"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.binary.hashed_size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_finding_processes_binary_hashed_size_to_long",
                                )?;
                                if event.remove("_ingest._value.binary.hashed_size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.binary.hashed_size".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.binary.partially_hashed") {
                                    if let Some(val) =
                                        event.get("_ingest._value.binary.partially_hashed")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.binary.partially_hashed"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.binary.partially_hashed",
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
                                    "convert_finding_processes_binary_partially_hashed_to_boolean",
                                )?;
                                if event
                                    .remove("_ingest._value.binary.partially_hashed")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.binary.partially_hashed".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                            if event.has_value("_ingest._value.libraries") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.libraries").cloned();
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
                                                if event.has_value("_ingest._value.hashed_size") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.hashed_size")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.hashed_size"
                                                                            .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.hashed_size",
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
                                                event.set("_ingest.on_failure_processor_tag", "convert_finding_processes_libraries_hashed_size_to_long")?;
                                                if event
                                                    .remove("_ingest._value.hashed_size")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.hashed_size".into(),
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
                                            "_ingest._value.libraries",
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                            if event.has_value("_ingest._value.libraries") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.libraries").cloned();
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
                                                if event.has_value("_ingest._value.size") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.size")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.size"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.size",
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
                                                event.set("_ingest.on_failure_processor_tag", "convert_finding_processes_libraries_size_to_long")?;
                                                if event.remove("_ingest._value.size").is_none() {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.size".into(),
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
                                            "_ingest._value.libraries",
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                            if event.has_value("_ingest._value.libraries") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.libraries").cloned();
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
                                            if let Err(err) =
                                                (|| -> Result<()> {
                                                    if event.has_value(
                                                        "_ingest._value.partially_hashed",
                                                    ) {
                                                        if let Some(val) = event
                                                            .get("_ingest._value.partially_hashed")
                                                        {
                                                            let converted =
                                                                convert_value(val, "boolean")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.partially_hashed".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.partially_hashed",
                                                                converted,
                                                            )?;
                                                        }
                                                    }
                                                    Ok(())
                                                })()
                                            {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_finding_processes_libraries_partially_hashed_to_boolean")?;
                                                if event
                                                    .remove("_ingest._value.partially_hashed")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.partially_hashed"
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
                                            "_ingest._value.libraries",
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.script.hashed_size") {
                                    if let Some(val) =
                                        event.get("_ingest._value.script.hashed_size")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.script.hashed_size"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.script.hashed_size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_finding_processes_script_hashed_size_to_long",
                                )?;
                                if event.remove("_ingest._value.script.hashed_size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.script.hashed_size".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.script.size") {
                                    if let Some(val) = event.get("_ingest._value.script.size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.script.size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.script.size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_finding_processes_script_size_to_long",
                                )?;
                                if event.remove("_ingest._value.script.size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.script.size".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("google_scc.finding.processes").cloned();
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
                                if event.has_value("_ingest._value.script.partially_hashed") {
                                    if let Some(val) =
                                        event.get("_ingest._value.script.partially_hashed")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.script.partially_hashed"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.script.partially_hashed",
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
                                    "convert_finding_processes_script_partially_hashed_to_boolean",
                                )?;
                                if event
                                    .remove("_ingest._value.script.partially_hashed")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.script.partially_hashed".into(),
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
                            "google_scc.finding.processes",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    if event.has_value("_ingest._value.parent_pid") {
                        event.rename("_ingest._value.parent_pid", "_ingest._value.parent.pid")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("google_scc.finding.resource.folders") {
                event.rename(
                    "google_scc.finding.resource.folders",
                    "google_scc.finding.resource.gcp_metadata.folders",
                )?;
            }

            if event.has_value("google_scc.finding.resource.parent_display_name") {
                event.rename(
                    "google_scc.finding.resource.parent_display_name",
                    "google_scc.finding.resource.gcp_metadata.parent_display_name",
                )?;
            }

            if event.has_value("google_scc.finding.resource.parent_name") {
                event.rename(
                    "google_scc.finding.resource.parent_name",
                    "google_scc.finding.resource.gcp_metadata.parent",
                )?;
            }

            if event.has_value("google_scc.finding.resource.project_display_name") {
                event.rename(
                    "google_scc.finding.resource.project_display_name",
                    "google_scc.finding.resource.gcp_metadata.project_display_name",
                )?;
            }

            if event.has_value("google_scc.finding.resource.project_name") {
                event.rename(
                    "google_scc.finding.resource.project_name",
                    "google_scc.finding.resource.gcp_metadata.project",
                )?;
            }

            if event.has_value("google_scc.finding.resource.organization") {
                event.rename(
                    "google_scc.finding.resource.organization",
                    "google_scc.finding.resource.gcp_metadata.organization",
                )?;
            }

            if event.has_value("google_scc.finding.security_marks.marks") {
                event.rename(
                    "google_scc.finding.security_marks.marks",
                    "google_scc.finding.security_marks.value",
                )?;
            }

            if event.has_value("google_scc.finding.source_properties.supporting_data") {
                event.rename(
                    "google_scc.finding.source_properties.supporting_data",
                    "google_scc.finding.source_properties_supporting_data",
                )?;
            }

            let _cond = {
                event.get_str("google_scc.finding.vulnerability.cve.cvssv3.base_score") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.vulnerability.cve.cvssv3.base_score") {
                        if let Some(val) =
                            event.get("google_scc.finding.vulnerability.cve.cvssv3.base_score")
                        {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.vulnerability.cve.cvssv3.base_score"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.finding.vulnerability.cve.cvssv3.base_score",
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
                        "convert_finding_vulnerability_cve_cvssv3_base_score_to_float",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.cvssv3.base_score")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.cvssv3.base_score".into(),
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
                                .get("_ingest.pipeline")
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
                event.has_value("google_scc.finding.vulnerability.cve.exploit_release_date")
                    && event.get_str("google_scc.finding.vulnerability.cve.exploit_release_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("google_scc.finding.vulnerability.cve.exploit_release_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_scc.finding.vulnerability.cve.exploit_release_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "google_scc.finding.vulnerability.cve.exploit_release_date"
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
                        "date_finding_vulnerability_cve_exploit_release_date",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.exploit_release_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.exploit_release_date"
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
                                .get("_ingest.pipeline")
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
                event.has_value("google_scc.finding.vulnerability.cve.first_exploitation_date")
                    && event.get_str("google_scc.finding.vulnerability.cve.first_exploitation_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_scc.finding.vulnerability.cve.first_exploitation_date",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_scc.finding.vulnerability.cve.first_exploitation_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "google_scc.finding.vulnerability.cve.first_exploitation_date".into(),
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
                        "date_finding_vulnerability_cve_first_exploitation_date",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.first_exploitation_date")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.first_exploitation_date"
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.vulnerability.cve.observed_in_the_wild")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.vulnerability.cve.observed_in_the_wild")
                    {
                        if let Some(val) =
                            event.get("google_scc.finding.vulnerability.cve.observed_in_the_wild")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "google_scc.finding.vulnerability.cve.observed_in_the_wild"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.finding.vulnerability.cve.observed_in_the_wild",
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
                        "convert_finding_vulnerability_cve_observed_in_the_wild_to_boolean",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.observed_in_the_wild")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.observed_in_the_wild"
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
                                .get("_ingest.pipeline")
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
                event.get_str("google_scc.finding.vulnerability.cve.upstream_fix_available")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("google_scc.finding.vulnerability.cve.upstream_fix_available")
                    {
                        if let Some(val) =
                            event.get("google_scc.finding.vulnerability.cve.upstream_fix_available")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_scc.finding.vulnerability.cve.upstream_fix_available".into(),
                            message,
                        })?;
                            event.set(
                                "google_scc.finding.vulnerability.cve.upstream_fix_available",
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
                        "convert_finding_vulnerability_cve_upstream_fix_available_to_boolean",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.upstream_fix_available")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.upstream_fix_available"
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
                                .get("_ingest.pipeline")
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
                { event.get_str("google_scc.finding.vulnerability.cve.zero_day") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.vulnerability.cve.zero_day") {
                        if let Some(val) =
                            event.get("google_scc.finding.vulnerability.cve.zero_day")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.vulnerability.cve.zero_day".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("google_scc.finding.vulnerability.cve.zero_day", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_finding_vulnerability_cve_zero_day_to_boolean",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.cve.zero_day")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.cve.zero_day".into(),
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
                                .get("_ingest.pipeline")
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

            if event.has_value("google_scc.finding.vulnerability.cve.cvssv3.attack_complexity") {
                event.rename(
                    "google_scc.finding.vulnerability.cve.cvssv3.attack_complexity",
                    "google_scc.finding.vulnerability.cve.cvssv3.attack.complexity",
                )?;
            }

            if event.has_value("google_scc.finding.vulnerability.cve.cvssv3.attack_vector") {
                event.rename(
                    "google_scc.finding.vulnerability.cve.cvssv3.attack_vector",
                    "google_scc.finding.vulnerability.cve.cvssv3.attack.vector",
                )?;
            }

            let _cond = {
                event.get_str("google_scc.finding.vulnerability.provider_risk_score") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.vulnerability.provider_risk_score") {
                        if let Some(val) =
                            event.get("google_scc.finding.vulnerability.provider_risk_score")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.vulnerability.provider_risk_score"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.finding.vulnerability.provider_risk_score",
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
                        "convert_finding_vulnerability_provider_risk_score_to_long",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.provider_risk_score")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.provider_risk_score".into(),
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("google_scc.finding.vulnerability.reachable") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.vulnerability.reachable") {
                        if let Some(val) = event.get("google_scc.finding.vulnerability.reachable") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_scc.finding.vulnerability.reachable".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.finding.vulnerability.reachable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_finding_vulnerability_reachable_to_boolean",
                    )?;
                    if event
                        .remove("google_scc.finding.vulnerability.reachable")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_scc.finding.vulnerability.reachable".into(),
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
                                .get("_ingest.pipeline")
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
                    .has_value("google_scc.finding.vulnerability.security_bulletin.submission_time")
                    && event.get_str(
                        "google_scc.finding.vulnerability.security_bulletin.submission_time",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_scc.finding.vulnerability.security_bulletin.submission_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.finding.vulnerability.security_bulletin.submission_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_scc.finding.vulnerability.security_bulletin.submission_time".into(),
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
                        "date_finding_vulnerability_security_bulletin_submission_time",
                    )?;
                    if event
                        .remove(
                            "google_scc.finding.vulnerability.security_bulletin.submission_time",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path:
                                "google_scc.finding.vulnerability.security_bulletin.submission_time"
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
                                .get("_ingest.pipeline")
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
                .get("google_scc.finding.event_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("THREAT") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                event.set("event.kind", json!("state"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("VULNERABILITY") };
            if _cond {
                event.append("event.category", json!("vulnerability"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("THREAT") };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("THREAT") };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("INACTIVE")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("ACTIVE")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("STATE_UNSPECIFIED")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            if let Some(v) = event
                .get("google_scc.finding.create_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.has_value("google_scc.finding.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def severity = ctx.google_scc.finding.severity;\n  if (severity == 'LOW' || severity == 'SEVERITY_UNSPECIFIED') {\n    ctx.event.severity = 21;\n  } else if (severity == 'MEDIUM') {\n    ctx.event.severity = 47;\n  } else if (severity == 'HIGH') {\n    ctx.event.severity = 73;\n  } else if (severity == 'CRITICAL') {\n    ctx.event.severity = 99;\n  }
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def severity = ctx.google_scc.finding.severity;\n  if (severity == 'LOW' || severity == 'SEVERITY_UNSPECIFIED') {\n    ctx.event.severity = 21;\n  } else if (severity == 'MEDIUM') {\n    ctx.event.severity = 47;\n  } else if (severity == 'HIGH') {\n    ctx.event.severity = 73;\n  } else if (severity == 'CRITICAL') {\n    ctx.event.severity = 99;\n  }"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_severity",
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
                                .get("_ingest.pipeline")
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

            event.set("observer.vendor", json!("Google Security Command Center"))?;

            let _cond = {
                event
                    .get("google_scc.finding.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.containers", |event| {
                    event.append_unique(
                        "container.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.gcp_metadata.organization") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.resource.gcp_metadata.organization") {
                        if let Some(input) = event
                            .get_string("google_scc.finding.resource.gcp_metadata.organization")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("organizations/") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("cloud.account.id", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.resource.gcp_metadata.organization"
                                        .into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_to_extract_cloud_account_id",
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
                                .get("_ingest.pipeline")
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

            let _cond = { !event.has_value("cloud.account.id") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.resource.aws_metadata.account.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.type") == Some("google.compute.Instance")
            };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.resource.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.type") == Some("google.compute.Instance")
            };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.resource.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.cloud_provider")
                    == Some("GOOGLE_CLOUD_PLATFORM")
            };
            if _cond {
                event.set("cloud.provider", json!("gcp"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.cloud_provider")
                    == Some("AMAZON_WEB_SERVICES")
            };
            if _cond {
                event.set("cloud.provider", json!("aws"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.cloud_provider")
                    == Some("MICROSOFT_AZURE")
            };
            if _cond {
                event.set("cloud.provider", json!("azure"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_scc.finding.resource.location") {
                    if let Some(input) = event.get_string("google_scc.finding.resource.location") {
                        // Grok pattern: ^%{DATA:cloud.region}-(?P<_temp>.)$
                        // Grok pattern: ^%{DATA:cloud.region}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!("^%{DATA:cloud.region}-(?P<_temp>.)$"),
                                cached_grok!("^%{DATA:cloud.region}$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "grok_to_extract_cloud_region",
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
                            .get("_ingest.pipeline")
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
                .get("google_scc.finding.resource.location")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.gcp_metadata.project")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.id", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.gcp_metadata.project_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.name", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.service")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            let _cond = {
                event.get_str("google_scc.finding.resource.type") == Some("google.compute.Instance")
            };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.resource.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "destination.ip",
                        json!(
                            event
                                .get("_ingest._value.destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "destination.port",
                        json!(
                            event
                                .get("_ingest._value.destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("destination.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.port").cloned();
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
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_destination_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
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
                            "destination.port",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("_ingest._value.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "source.port",
                        json!(
                            event
                                .get("_ingest._value.source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("source.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.port").cloned();
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
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_source_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
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
                            "source.port",
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
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "network.transport",
                        json!(
                            event
                                .get("_ingest._value.protocol")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("google_scc.finding.parent") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_scc.finding.parent") {
                        if let Some(input) = event.get_string("google_scc.finding.parent") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("organizations/") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("/sources/") else {
                                    break 'dissect false;
                                };
                                captured.push(("organization.id", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("/sources/") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("google_scc.finding.source_id", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "google_scc.finding.parent".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_to_extract_organization_id",
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
                                .get("_ingest.pipeline")
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
                    .get("google_scc.finding.kubernetes.access_reviews")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.kubernetes.access_reviews",
                    |event| {
                        event.append_unique(
                            "orchestrator.namespace",
                            json!(
                                event
                                    .get("_ingest._value.ns")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.kubernetes.access_reviews")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.kubernetes.access_reviews",
                    |event| {
                        event.append_unique(
                            "orchestrator.resource.name",
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

            let _cond = {
                event
                    .get("google_scc.finding.kubernetes.access_reviews")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.kubernetes.access_reviews",
                    |event| {
                        event.append_unique(
                            "orchestrator.api_version",
                            json!(
                                event
                                    .get("_ingest._value.version")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.kubernetes.access_reviews")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.kubernetes.access_reviews",
                    |event| {
                        event.append_unique(
                            "orchestrator.resource.type",
                            json!(
                                event
                                    .get("_ingest._value.resource")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("google_scc.finding.kubernetes") };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.files", |event| {
                    event.append_unique(
                        "file.path",
                        json!(
                            event
                                .get("_ingest._value.path")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.files", |event| {
                    event.append_unique(
                        "file.size",
                        json!(
                            event
                                .get("_ingest._value.size")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("file.size").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("file.size").cloned();
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
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_file_size_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
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
                            "file.size",
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
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.files", |event| {
                    event.append_unique(
                        "file.hash.sha256",
                        json!(
                            event
                                .get("_ingest._value.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.offending_package.package_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.name", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.offending_package.package_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.version", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.fixed_package.package_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.fixed_version", v)?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.append_unique(
                        "process.parent.pid",
                        json!(
                            event
                                .get("_ingest._value.parent.pid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("process.parent.pid")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("process.parent.pid").cloned();
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
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_process_parent_pid_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
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
                            "process.parent.pid",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.append_unique(
                        "process.pid",
                        json!(
                            event
                                .get("_ingest._value.pid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("process.pid").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("process.pid").cloned();
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
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_process_pid_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
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
                            "process.pid",
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
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.append_unique(
                        "process.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.id", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.name", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.resource.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.type", v)?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("INACTIVE")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("result.evaluation", json!("passed"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("ACTIVE")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("result.evaluation", json!("failed"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.state") == Some("STATE_UNSPECIFIED")
                    && event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION")
            };
            if _cond {
                event.set("result.evaluation", json!("unknown"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.category")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("google_scc.finding.category") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("rule.uuid", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.description", v)?;
                }
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("MISCONFIGURATION") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.source_properties.recommendation")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.remediation", v)?;
                }
            }

            let _cond = {
                event
                    .get("google_scc.finding.mitre_attack.additional.tactics")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.mitre_attack.additional.tactics",
                    |event| {
                        event.append_unique(
                            "threat.tactic.name",
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

            let _cond = { event.has_value("google_scc.finding.mitre_attack.primary.tactic") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("google_scc.finding.mitre_attack.primary.tactic")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.mitre_attack.additional.techniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.mitre_attack.additional.techniques",
                    |event| {
                        event.append_unique(
                            "threat.technique.name",
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

            let _cond = {
                event
                    .get("google_scc.finding.mitre_attack.primary.techniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.mitre_attack.primary.techniques",
                    |event| {
                        event.append_unique(
                            "threat.technique.name",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.threat?.tactic?.name instanceof List) {\n  def list = new ArrayList();\n  for (def value: ctx.threat.tactic.name) {\n    list.add(params.tactic.get(value));\n  }\n  ctx.threat.tactic.put('id', list);\n}\nif (ctx.threat?.technique?.name instanceof List) {\n  def list = new ArrayList();\n  for (def value: ctx.threat.technique.name) {\n    list.add(params.technique.get(value));\n  }\n  ctx.threat.technique.put('id', list);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.threat?.tactic?.name instanceof List) {\n  def list = new ArrayList();\n  for (def value: ctx.threat.tactic.name) {\n    list.add(params.tactic.get(value));\n  }\n  ctx.threat.tactic.put('id', list);\n}\nif (ctx.threat?.technique?.name instanceof List) {\n  def list = new ArrayList();\n  for (def value: ctx.threat.technique.name) {\n    list.add(params.technique.get(value));\n  }\n  ctx.threat.technique.put('id', list);\n}"#
                    ),
                    cached_params!(
                        "{\"tactic\":{\"RECONNAISSANCE\":\"TA0043\",\"RESOURCE_DEVELOPMENT\":\"TA0042\",\"INITIAL_ACCESS\":\"TA0001\",\"EXECUTION\":\"TA0002\",\"PERSISTENCE\":\"TA0003\",\"PRIVILEGE_ESCALATION\":\"TA0004\",\"DEFENSE_EVASION\":\"TA0005\",\"CREDENTIAL_ACCESS\":\"TA0006\",\"DISCOVERY\":\"TA0007\",\"LATERAL_MOVEMENT\":\"TA0008\",\"COLLECTION\":\"TA0009\",\"COMMAND_AND_CONTROL\":\"TA0011\",\"EXFILTRATION\":\"TA0010\",\"IMPACT\":\"TA0040\"},\"technique\":{\"DATA_OBFUSCATION\":\"T1001\",\"DATA_OBFUSCATION_STEGANOGRAPHY\":\"T1001.002\",\"OS_CREDENTIAL_DUMPING\":\"T1003\",\"OS_CREDENTIAL_DUMPING_PROC_FILESYSTEM\":\"T1003.007\",\"OS_CREDENTIAL_DUMPING_ETC_PASSWORD_AND_ETC_SHADOW\":\"T1003.008\",\"DATA_FROM_LOCAL_SYSTEM\":\"T1005\",\"AUTOMATED_EXFILTRATION\":\"T1020\",\"OBFUSCATED_FILES_OR_INFO\":\"T1027\",\"STEGANOGRAPHY\":\"T1027.003\",\"COMPILE_AFTER_DELIVERY\":\"T1027.004\",\"COMMAND_OBFUSCATION\":\"T1027.010\",\"SCHEDULED_TRANSFER\":\"T1029\",\"SYSTEM_OWNER_USER_DISCOVERY\":\"T1033\",\"MASQUERADING\":\"T1036\",\"MATCH_LEGITIMATE_NAME_OR_LOCATION\":\"T1036.005\",\"BOOT_OR_LOGON_INITIALIZATION_SCRIPTS\":\"T1037\",\"STARTUP_ITEMS\":\"T1037.005\",\"NETWORK_SERVICE_DISCOVERY\":\"T1046\",\"SCHEDULED_TASK_JOB\":\"T1053\",\"SCHEDULED_TASK_JOB_CRON\":\"T1053.003\",\"CONTAINER_ORCHESTRATION_JOB\":\"T1053.007\",\"PROCESS_INJECTION\":\"T1055\",\"INPUT_CAPTURE\":\"T1056\",\"INPUT_CAPTURE_KEYLOGGING\":\"T1056.001\",\"PROCESS_DISCOVERY\":\"T1057\",\"COMMAND_AND_SCRIPTING_INTERPRETER\":\"T1059\",\"UNIX_SHELL\":\"T1059.004\",\"PYTHON\":\"T1059.006\",\"EXPLOITATION_FOR_PRIVILEGE_ESCALATION\":\"T1068\",\"PERMISSION_GROUPS_DISCOVERY\":\"T1069\",\"CLOUD_GROUPS\":\"T1069.003\",\"INDICATOR_REMOVAL\":\"T1070\",\"INDICATOR_REMOVAL_CLEAR_LINUX_OR_MAC_SYSTEM_LOGS\":\"T1070.002\",\"INDICATOR_REMOVAL_CLEAR_COMMAND_HISTORY\":\"T1070.003\",\"INDICATOR_REMOVAL_FILE_DELETION\":\"T1070.004\",\"INDICATOR_REMOVAL_TIMESTOMP\":\"T1070.006\",\"INDICATOR_REMOVAL_CLEAR_MAILBOX_DATA\":\"T1070.008\",\"APPLICATION_LAYER_PROTOCOL\":\"T1071\",\"DNS\":\"T1071.004\",\"SOFTWARE_DEPLOYMENT_TOOLS\":\"T1072\",\"VALID_ACCOUNTS\":\"T1078\",\"DEFAULT_ACCOUNTS\":\"T1078.001\",\"LOCAL_ACCOUNTS\":\"T1078.003\",\"CLOUD_ACCOUNTS\":\"T1078.004\",\"FILE_AND_DIRECTORY_DISCOVERY\":\"T1083\",\"ACCOUNT_DISCOVERY_LOCAL_ACCOUNT\":\"T1087.001\",\"PROXY\":\"T1090\",\"EXTERNAL_PROXY\":\"T1090.002\",\"MULTI_HOP_PROXY\":\"T1090.003\",\"ACCOUNT_MANIPULATION\":\"T1098\",\"ADDITIONAL_CLOUD_CREDENTIALS\":\"T1098.001\",\"ADDITIONAL_CLOUD_ROLES\":\"T1098.003\",\"SSH_AUTHORIZED_KEYS\":\"T1098.004\",\"ADDITIONAL_CONTAINER_CLUSTER_ROLES\":\"T1098.006\",\"MULTI_STAGE_CHANNELS\":\"T1104\",\"INGRESS_TOOL_TRANSFER\":\"T1105\",\"NATIVE_API\":\"T1106\",\"BRUTE_FORCE\":\"T1110\",\"AUTOMATED_COLLECTION\":\"T1119\",\"SHARED_MODULES\":\"T1129\",\"DATA_ENCODING\":\"T1132\",\"STANDARD_ENCODING\":\"T1132.001\",\"ACCESS_TOKEN_MANIPULATION\":\"T1134\",\"TOKEN_IMPERSONATION_OR_THEFT\":\"T1134.001\",\"CREATE_ACCOUNT\":\"T1136\",\"LOCAL_ACCOUNT\":\"T1136.001\",\"DEOBFUSCATE_DECODE_FILES_OR_INFO\":\"T1140\",\"EXPLOIT_PUBLIC_FACING_APPLICATION\":\"T1190\",\"SUPPLY_CHAIN_COMPROMISE\":\"T1195\",\"COMPROMISE_SOFTWARE_DEPENDENCIES_AND_DEVELOPMENT_TOOLS\":\"T1195.001\",\"EXPLOITATION_FOR_CLIENT_EXECUTION\":\"T1203\",\"USER_EXECUTION\":\"T1204\",\"LINUX_AND_MAC_FILE_AND_DIRECTORY_PERMISSIONS_MODIFICATION\":\"T1222.002\",\"DOMAIN_POLICY_MODIFICATION\":\"T1484\",\"DATA_DESTRUCTION\":\"T1485\",\"DATA_ENCRYPTED_FOR_IMPACT\":\"T1486\",\"SERVICE_STOP\":\"T1489\",\"INHIBIT_SYSTEM_RECOVERY\":\"T1490\",\"FIRMWARE_CORRUPTION\":\"T1495\",\"RESOURCE_HIJACKING\":\"T1496\",\"NETWORK_DENIAL_OF_SERVICE\":\"T1498\",\"CLOUD_SERVICE_DISCOVERY\":\"T1526\",\"STEAL_APPLICATION_ACCESS_TOKEN\":\"T1528\",\"ACCOUNT_ACCESS_REMOVAL\":\"T1531\",\"TRANSFER_DATA_TO_CLOUD_ACCOUNT\":\"T1537\",\"STEAL_WEB_SESSION_COOKIE\":\"T1539\",\"CREATE_OR_MODIFY_SYSTEM_PROCESS\":\"T1543\",\"EVENT_TRIGGERED_EXECUTION\":\"T1546\",\"BOOT_OR_LOGON_AUTOSTART_EXECUTION\":\"T1547\",\"KERNEL_MODULES_AND_EXTENSIONS\":\"T1547.006\",\"SHORTCUT_MODIFICATION\":\"T1547.009\",\"ABUSE_ELEVATION_CONTROL_MECHANISM\":\"T1548\",\"ABUSE_ELEVATION_CONTROL_MECHANISM_SETUID_AND_SETGID\":\"T1548.001\",\"ABUSE_ELEVATION_CONTROL_MECHANISM_SUDO_AND_SUDO_CACHING\":\"T1548.003\",\"UNSECURED_CREDENTIALS\":\"T1552\",\"CREDENTIALS_IN_FILES\":\"T1552.001\",\"BASH_HISTORY\":\"T1552.003\",\"PRIVATE_KEYS\":\"T1552.004\",\"SUBVERT_TRUST_CONTROL\":\"T1553\",\"INSTALL_ROOT_CERTIFICATE\":\"T1553.004\",\"COMPROMISE_HOST_SOFTWARE_BINARY\":\"T1554\",\"CREDENTIALS_FROM_PASSWORD_STORES\":\"T1555\",\"MODIFY_AUTHENTICATION_PROCESS\":\"T1556\",\"PLUGGABLE_AUTHENTICATION_MODULES\":\"T1556.003\",\"MULTI_FACTOR_AUTHENTICATION\":\"T1556.006\",\"IMPAIR_DEFENSES\":\"T1562\",\"DISABLE_OR_MODIFY_TOOLS\":\"T1562.001\",\"INDICATOR_BLOCKING\":\"T1562.006\",\"DISABLE_OR_MODIFY_LINUX_AUDIT_SYSTEM\":\"T1562.012\",\"HIDE_ARTIFACTS\":\"T1564\",\"HIDDEN_FILES_AND_DIRECTORIES\":\"T1564.001\",\"HIDDEN_USERS\":\"T1564.002\",\"EXFILTRATION_OVER_WEB_SERVICE\":\"T1567\",\"EXFILTRATION_TO_CLOUD_STORAGE\":\"T1567.002\",\"DYNAMIC_RESOLUTION\":\"T1568\",\"LATERAL_TOOL_TRANSFER\":\"T1570\",\"HIJACK_EXECUTION_FLOW\":\"T1574\",\"HIJACK_EXECUTION_FLOW_DYNAMIC_LINKER_HIJACKING\":\"T1574.006\",\"MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE\":\"T1578\",\"CREATE_SNAPSHOT\":\"T1578.001\",\"CLOUD_INFRASTRUCTURE_DISCOVERY\":\"T1580\",\"DEVELOP_CAPABILITIES\":\"T1587\",\"DEVELOP_CAPABILITIES_MALWARE\":\"T1587.001\",\"OBTAIN_CAPABILITIES\":\"T1588\",\"OBTAIN_CAPABILITIES_MALWARE\":\"T1588.001\",\"OBTAIN_CAPABILITIES_VULNERABILITIES\":\"T1588.006\",\"ACTIVE_SCANNING\":\"T1595\",\"SCANNING_IP_BLOCKS\":\"T1595.001\",\"STAGE_CAPABILITIES\":\"T1608\",\"UPLOAD_MALWARE\":\"T1608.001\",\"CONTAINER_ADMINISTRATION_COMMAND\":\"T1609\",\"DEPLOY_CONTAINER\":\"T1610\",\"ESCAPE_TO_HOST\":\"T1611\",\"CONTAINER_AND_RESOURCE_DISCOVERY\":\"T1613\",\"REFLECTIVE_CODE_LOADING\":\"T1620\",\"STEAL_OR_FORGE_AUTHENTICATION_CERTIFICATES\":\"T1649\",\"FINANCIAL_THEFT\":\"T1657\"}}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "painless_set_threat_tactic_id_and_threat_technique_id",
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
                            .get("_ingest.pipeline")
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
                .get("google_scc.finding.external_uri")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            let _cond = {
                event.has_value("google_scc.finding.external_uri")
                    && event.get_str("google_scc.finding.external_uri") != Some("")
            };
            if _cond {
                if event.has_value("url.original") {
                    uri_parts(event, "url.original", "url", true, false)?;
                }
            }

            if let Some(v) = event
                .get("google_scc.finding.access.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.database.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            if let Some(v) = event
                .get("google_scc.finding.access.principal.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.access.service_account_delegation_info")
                    .is_some_and(|v| v.is_array())
                    && !event.has_value("user.id")
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.access.service_account_delegation_info",
                    |event| {
                        event.append_unique(
                            "user.id",
                            json!(
                                event
                                    .get("_ingest._value.principal.email")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.data_access_events")
                    .is_some_and(|v| v.is_array())
                    && !event.has_value("user.id")
            };
            if _cond {
                foreach_array(event, "google_scc.finding.data_access_events", |event| {
                    event.append_unique(
                        "user.id",
                        json!(
                            event
                                .get("_ingest._value.principal_email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("user.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event
                    .get("google_scc.finding.iam_bindings")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.iam_bindings", |event| {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("_ingest._value.role")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("VULNERABILITY") };
            if _cond {
                event.set("vulnerability.classification", json!("CVSS"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.class") == Some("VULNERABILITY")
                    && event
                        .get_str("google_scc.finding.vulnerability.cve.id")
                        .is_some_and(|s| s.starts_with("CVE"))
            };
            if _cond {
                event.set("vulnerability.enumeration", json!("CVE"))?;
            }

            let _cond = {
                event.get_str("google_scc.finding.class") == Some("VULNERABILITY")
                    && event
                        .get_str("google_scc.finding.vulnerability.cve.id")
                        .is_some_and(|s| s.starts_with("GHSA"))
            };
            if _cond {
                event.set("vulnerability.enumeration", json!("GHSA"))?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("VULNERABILITY") };
            if _cond {
                event.set("vulnerability.score.version", json!("3.1"))?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.cve.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            let _cond = {
                event.has_value("google_scc.finding.category")
                    && event.get_str("google_scc.finding.class") == Some("VULNERABILITY")
            };
            if _cond {
                event.append_unique(
                    "vulnerability.category",
                    json!(
                        event
                            .get("google_scc.finding.category")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("VULNERABILITY") };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.description", v)?;
                }
            }

            let _cond = {
                event
                    .get("google_scc.finding.vulnerability.cve.references")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.vulnerability.cve.references",
                    |event| {
                        event.append_unique(
                            "vulnerability.reference",
                            json!(
                                event
                                    .get("_ingest._value.uri")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.cve.cvssv3.base_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            if let Some(v) = event
                .get("google_scc.finding.vulnerability.cve.exploit_release_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.published_date", v)?;
            }

            let _cond = {
                event.has_value("google_scc.finding.severity")
                    && event.get_str("google_scc.finding.class") == Some("VULNERABILITY")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String severity = ctx.google_scc.finding.severity; if (severity == 'SEVERITY_UNSPECIFIED') {\n  ctx.vulnerability.put('severity', 'None');\n} else if (severity == 'LOW') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'MEDIUM') {\n  ctx.vulnerability.put('severity', 'Medium');\n} else if (severity == 'HIGH') {\n  ctx.vulnerability.put('severity', 'High');\n} else if (severity == 'CRITICAL') {\n  ctx.vulnerability.put('severity', 'Critical');\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String severity = ctx.google_scc.finding.severity; if (severity == 'SEVERITY_UNSPECIFIED') {\n  ctx.vulnerability.put('severity', 'None');\n} else if (severity == 'LOW') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'MEDIUM') {\n  ctx.vulnerability.put('severity', 'Medium');\n} else if (severity == 'HIGH') {\n  ctx.vulnerability.put('severity', 'High');\n} else if (severity == 'CRITICAL') {\n  ctx.vulnerability.put('severity', 'Critical');\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_map_severity_to_CVSS",
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
                event.has_value("package.name")
                    && event.has_value("package.version")
                    && event.has_value("vulnerability.id")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {} {} - {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("package.name")
                    && event.has_value("package.version")
                    && !event.has_value("vulnerability.title")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {} {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond =
                { event.has_value("vulnerability.id") && !event.has_value("vulnerability.title") };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found - {}",
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.get_str("google_scc.finding.class") == Some("VULNERABILITY") };
            if _cond {
                event.set(
                    "vulnerability.scanner.vendor",
                    json!("Google Security Command Center"),
                )?;
            }

            let _cond = {
                event.get_str("google_scc.finding.class") == Some("VULNERABILITY")
                    && event
                        .get_str("google_scc.finding.vulnerability.cve.id")
                        .is_some_and(|s| s.starts_with("CVE"))
            };
            if _cond {
                if let Some(v) = event
                    .get("google_scc.finding.vulnerability.cve.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.cve", v)?;
                }
            }

            let _cond = {
                event.has_value("google_scc.finding.resource.name")
                    && event.get_str("google_scc.finding.resource.type")
                        == Some("google.compute.Instance")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_scc.finding.resource.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.indicator.ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.indicator.ip_addresses",
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

            let _cond = { event.has_value("google_scc.finding.access.caller_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_scc.finding.access.caller_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_scc.finding.access.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_scc.finding.access.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_scc.finding.database.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_scc.finding.database.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_scc.finding.access.principal.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_scc.finding.access.principal.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.access.service_account_delegation_info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.access.service_account_delegation_info",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.principal.email")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.data_access_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.data_access_events", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.principal_email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.billing")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.billing", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.legal")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.legal", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.security")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.security", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.all")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.all", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.product_updates")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.contacts.product_updates",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.email")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.suspension")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.suspension", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.technical")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.contacts.technical", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.contacts.technical_incidents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "google_scc.finding.contacts.technical_incidents",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.email")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.script.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.binary.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.processes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.processes", |event| {
                    if event.has_value("_ingest._value.libraries") {
                        foreach_array(event, "_ingest._value.libraries", |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.sha256")
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
                event
                    .get("google_scc.finding.files")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "google_scc.finding.files", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            event.remove("json");
            event.remove("_temp");
            event.remove("google_scc.finding.cloud_dlp_inspection.full_scan");
            event.remove("google_scc.finding.cloud_dlp_inspection.info_type_count");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_code_modification");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_ftrace_handler");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_interrupt_handler");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_kernel_code_pages");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_kprobe_handler");
            event.remove("google_scc.finding.kernel_root_kit.unexpected_processes_in_runqueue");
            event.remove(
                "google_scc.finding.kernel_root_kit.unexpected_read_only_data_modification",
            );
            event.remove("google_scc.finding.kernel_root_kit.unexpected_system_call_handler");
            event.remove("google_scc.finding.mute_update_time");

            let _cond = {
                event
                    .get("google_scc.finding.vulnerability.cve.references")
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
                foreach_array(
                    event,
                    "google_scc.finding.vulnerability.cve.references",
                    |event| {
                        event.remove("_ingest._value.uri");
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.connections")
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
                foreach_array(event, "google_scc.finding.connections", |event| {
                    event.remove("_ingest._value.destination.ip");
                    event.remove("_ingest._value.destination.port");
                    event.remove("_ingest._value.source.ip");
                    event.remove("_ingest._value.source.port");
                    event.remove("_ingest._value.protocol");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.containers")
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
                foreach_array(event, "google_scc.finding.containers", |event| {
                    event.remove("_ingest._value.name");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.files")
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
                foreach_array(event, "google_scc.finding.files", |event| {
                    event.remove("_ingest._value.path");
                    event.remove("_ingest._value.size");
                    event.remove("_ingest._value.sha256");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.processes")
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
                foreach_array(event, "google_scc.finding.processes", |event| {
                    event.remove("_ingest._value.name");
                    event.remove("_ingest._value.pid");
                    event.remove("_ingest._value.parent.pid");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.iam_bindings")
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
                foreach_array(event, "google_scc.finding.iam_bindings", |event| {
                    event.remove("_ingest._value.role");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("google_scc.finding.kubernetes.access_reviews")
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
                foreach_array(
                    event,
                    "google_scc.finding.kubernetes.access_reviews",
                    |event| {
                        event.remove("_ingest._value.ns");
                        event.remove("_ingest._value.name");
                        event.remove("_ingest._value.version");
                        event.remove("_ingest._value.resource");
                        Ok(())
                    },
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
                event.remove("google_scc.finding.event_time");
                event.remove("google_scc.finding.create_time");
                event.remove("google_scc.finding.name");
                event.remove("google_scc.finding.resource.name");
                event.remove("google_scc.finding.resource.type");
                event.remove("google_scc.finding.resource.display_name");
                event.remove("google_scc.finding.resource.service");
                event.remove("google_scc.finding.description");
                event.remove("google_scc.finding.mitre_attack.additional.tactics");
                event.remove("google_scc.finding.mitre_attack.primary.tactic");
                event.remove("google_scc.finding.mitre_attack.additional.techniques");
                event.remove("google_scc.finding.mitre_attack.primary.techniques");
                event.remove("google_scc.finding.resource.location");
                event.remove("google_scc.finding.resource.gcp_metadata.project");
                event.remove("google_scc.finding.resource.gcp_metadata.project_display_name");
                event.remove("google_scc.finding.vulnerability.offending_package.package_name");
                event.remove("google_scc.finding.vulnerability.offending_package.package_version");
                event.remove("google_scc.finding.vulnerability.fixed_package.package_version");
                event.remove("google_scc.finding.external_uri");
                event.remove("google_scc.finding.vulnerability.cve.id");
                event.remove("google_scc.finding.vulnerability.cve.cvssv3.base_score");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
