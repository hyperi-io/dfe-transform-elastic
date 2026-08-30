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

            event.append_unique("event.kind", json!("state"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "strict_date_optional_time_nanos",
                                "yyyy-MM-dd'T'HH:mm:ss.SZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSZ",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("tanium.endpoint_config.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_timestamp")?;
                    event.append_unique(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.action") {
                event.rename("json.action", "tanium.endpoint_config.action")?;
            }

            if let Some(v) = event
                .get("tanium.endpoint_config.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.user.user_id") {
                    if let Some(val) = event.get("json.user.user_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.user.user_id".into(),
                                message,
                            }
                        })?;
                        event.set("tanium.endpoint_config.user.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_user_user_id",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("tanium.endpoint_config.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.user.persona_id") {
                    if let Some(val) = event.get("json.user.persona_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.user.persona_id".into(),
                                message,
                            }
                        })?;
                        event.set("tanium.endpoint_config.user.persona_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_user_persona_id",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.config_item.id") {
                    if let Some(val) = event.get("json.config_item.id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.config_item.id".into(),
                                message,
                            }
                        })?;
                        event.set("tanium.endpoint_config.item.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_config_item_id",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.config_item.domain") {
                event.rename(
                    "json.config_item.domain",
                    "tanium.endpoint_config.item.domain",
                )?;
            }

            if event.has_value("json.config_item.data_category") {
                event.rename(
                    "json.config_item.data_category",
                    "tanium.endpoint_config.item.data_category",
                )?;
            }

            if event.has_value("json.module.solution_id") {
                event.rename(
                    "json.module.solution_id",
                    "tanium.endpoint_config.module.solution_id",
                )?;
            }

            if event.has_value("json.module.solution_context_id") {
                event.rename(
                    "json.module.solution_context_id",
                    "tanium.endpoint_config.module.solution_context_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.manifest.windows_saved_action_id") {
                    if let Some(val) = event.get("json.manifest.windows_saved_action_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.manifest.windows_saved_action_id".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "tanium.endpoint_config.manifest.windows_saved_action_id",
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
                    "convert_json_manifest_windows_saved_action_id",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.manifest.non_windows_saved_action_id") {
                    if let Some(val) = event.get("json.manifest.non_windows_saved_action_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.manifest.non_windows_saved_action_id".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "tanium.endpoint_config.manifest.non_windows_saved_action_id",
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
                    "convert_json_manifest_non_windows_saved_action_id",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.manifest.manifest_revision") {
                    if let Some(val) = event.get("json.manifest.manifest_revision") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.manifest.manifest_revision".into(),
                                message,
                            }
                        })?;
                        event.set("tanium.endpoint_config.manifest.revision", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_manifest_manifest_revision",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.manifest.service_uuid") {
                event.rename(
                    "json.manifest.service_uuid",
                    "tanium.endpoint_config.manifest.service_uuid",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.manifest.item_count") {
                    if let Some(val) = event.get("json.manifest.item_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.manifest.item_count".into(),
                                message,
                            }
                        })?;
                        event.set("tanium.endpoint_config.manifest.item_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_manifest_item_count",
                )?;
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                    .get("json.manifest.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.manifest.items").cloned();
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
                            {
                                // A foreach walks a LIST or an OBJECT: over an object Elastic
                                // binds `_ingest._key` per entry, which is what a target of
                                // `<field>.{{{_ingest._key}}}` reads.
                                let subject = event.get("_ingest._value.ids").cloned();
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
                                            if event.has_value("_ingest._value") {
                                                if let Some(val) = event.get("_ingest._value") {
                                                    let converted = convert_value(val, "long")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value".into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event.set("_ingest._value", converted)?;
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
                                            event.set(
                                                "_ingest.on_failure_processor_tag",
                                                "convert_ingest_value",
                                            )?;
                                            if event.remove("_ingest._value").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value".into(),
                                                });
                                            }
                                            event.append_unique(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
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
                                        "_ingest._value.ids",
                                        if keyed {
                                            Value::Object(fields)
                                        } else {
                                            Value::Array(list)
                                        },
                                    )?;
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
                            "json.manifest.items",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.manifest.items") {
                event.rename(
                    "json.manifest.items",
                    "tanium.endpoint_config.manifest.items",
                )?;
            }

            let _cond =
                { event.has_value("error.message") && event.get_str("error.message") != Some("") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
                event.remove("tanium.endpoint_config.action");
                event.remove("tanium.endpoint_config.user.id");
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
