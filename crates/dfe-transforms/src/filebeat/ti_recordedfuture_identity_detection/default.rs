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

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "recordedfuture.identity_detection")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event
                    .get("recordedfuture.identity_detection.password.hashes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def flat = [:];\nctx.related = ctx.related ?: [:];\nctx.related.hash = ctx.related.hash ?: [];\nfor (h in ctx.recordedfuture.identity_detection.password.hashes) {\n  if (h.algorithm != null) {\n    def algorithm = h.algorithm.toLowerCase();\n    if (h.hash != null) {\n      flat[algorithm] = h.hash;\n      ctx.related.hash.add(h.hash);\n    } else if (h.hash_prefix != null) {\n      flat[algorithm + '_prefix'] = h.hash_prefix;\n    }\n  }\n}\nif (flat.size() > 0) {\n  ctx.recordedfuture.identity_detection.password.hashes = flat;\n} else {\n  ctx.recordedfuture.identity_detection.password.remove('hashes');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def flat = [:];\nctx.related = ctx.related ?: [:];\nctx.related.hash = ctx.related.hash ?: [];\nfor (h in ctx.recordedfuture.identity_detection.password.hashes) {\n  if (h.algorithm != null) {\n    def algorithm = h.algorithm.toLowerCase();\n    if (h.hash != null) {\n      flat[algorithm] = h.hash;\n      ctx.related.hash.add(h.hash);\n    } else if (h.hash_prefix != null) {\n      flat[algorithm + '_prefix'] = h.hash_prefix;\n    }\n  }\n}\nif (flat.size() > 0) {\n  ctx.recordedfuture.identity_detection.password.hashes = flat;\n} else {\n  ctx.recordedfuture.identity_detection.password.remove('hashes');\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_flatten_password_hashes",
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

            if event.has_value("recordedfuture.identity_detection.type") {
                event.rename(
                    "recordedfuture.identity_detection.type",
                    "recordedfuture.identity_detection.detection_type",
                )?;
            }

            let _cond = {
                event.has_value("recordedfuture.identity_detection.created")
                    && event.get_str("recordedfuture.identity_detection.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("recordedfuture.identity_detection.created")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("recordedfuture.identity_detection.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "recordedfuture.identity_detection.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created")?;
                    if event
                        .remove("recordedfuture.identity_detection.created")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "recordedfuture.identity_detection.created".into(),
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
                event.has_value(
                    "recordedfuture.identity_detection.dump.compromise.exfiltration_date",
                ) && event
                    .get_str("recordedfuture.identity_detection.dump.compromise.exfiltration_date")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "recordedfuture.identity_detection.dump.compromise.exfiltration_date",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "recordedfuture.identity_detection.exfiltration_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "recordedfuture.identity_detection.dump.compromise.exfiltration_date".into(),
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
                        "date_dump_compromise_exfiltration_date",
                    )?;
                    if event
                        .remove(
                            "recordedfuture.identity_detection.dump.compromise.exfiltration_date",
                        )
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound { path: "recordedfuture.identity_detection.dump.compromise.exfiltration_date".into() });
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
                    .get("recordedfuture.identity_detection.cookies")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("recordedfuture.identity_detection.cookies")
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
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.expiration")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.expiration", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.expiration".into(),
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
                                    "date_cookie_expiration",
                                )?;
                                event.remove("_ingest._value.expiration");
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
                            "recordedfuture.identity_detection.cookies",
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
                event.has_value("recordedfuture.identity_detection.dump.downloaded")
                    && event.get_str("recordedfuture.identity_detection.dump.downloaded")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("recordedfuture.identity_detection.dump.downloaded")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("recordedfuture.identity_detection.dump.downloaded", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "recordedfuture.identity_detection.dump.downloaded"
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
                    event.set("_ingest.on_failure_processor_tag", "date_dump_downloaded")?;
                    if event
                        .remove("recordedfuture.identity_detection.dump.downloaded")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "recordedfuture.identity_detection.dump.downloaded".into(),
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
                    .get("recordedfuture.identity_detection.dump.breaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("recordedfuture.identity_detection.dump.breaches")
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
                                if let Some(date_str) = event.get_as_string("_ingest._value.start")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.start", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.start".into(),
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
                                    "date_dump_breach_start",
                                )?;
                                event.remove("_ingest._value.start");
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
                            "recordedfuture.identity_detection.dump.breaches",
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
                    .get("recordedfuture.identity_detection.dump.breaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("recordedfuture.identity_detection.dump.breaches")
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
                                if let Some(date_str) = event.get_as_string("_ingest._value.stop") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("_ingest._value.stop", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.stop".into(),
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
                                    "date_dump_breach_stop",
                                )?;
                                event.remove("_ingest._value.stop");
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
                            "recordedfuture.identity_detection.dump.breaches",
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
                    .get("recordedfuture.identity_detection.dump.breaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("recordedfuture.identity_detection.dump.breaches")
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
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.breached")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.breached", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.breached".into(),
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
                                    "date_dump_breach_breached",
                                )?;
                                event.remove("_ingest._value.breached");
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
                            "recordedfuture.identity_detection.dump.breaches",
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
                    .get("recordedfuture.identity_detection.cookies")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "recordedfuture.identity_detection.cookies",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value.http") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.http".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.http", converted)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_cookie_http_to_boolean",
                            )?;
                            event.remove("_ingest._value.http");
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
                    .get("recordedfuture.identity_detection.cookies")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "recordedfuture.identity_detection.cookies",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value.secure") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.secure".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.secure", converted)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_cookie_secure_to_boolean",
                            )?;
                            event.remove("_ingest._value.secure");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("recordedfuture.identity_detection.novel") {
                    if let Some(val) = event.get("recordedfuture.identity_detection.novel") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "recordedfuture.identity_detection.novel".into(),
                                message,
                            }
                        })?;
                        event.set("recordedfuture.identity_detection.novel", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_novel_to_boolean",
                )?;
                if event
                    .remove("recordedfuture.identity_detection.novel")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "recordedfuture.identity_detection.novel".into(),
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
                event.has_value("recordedfuture.identity_detection.dump.infrastructure.ip")
                    && event.get_str("recordedfuture.identity_detection.dump.infrastructure.ip")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("recordedfuture.identity_detection.dump.infrastructure.ip") {
                        if let Some(val) =
                            event.get("recordedfuture.identity_detection.dump.infrastructure.ip")
                        {
                            let converted =
                                convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                            path: "recordedfuture.identity_detection.dump.infrastructure.ip".into(),
                            message,
                        }
                                })?;
                            event.set(
                                "recordedfuture.identity_detection.dump.infrastructure.ip",
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
                        "convert_dump_infrastructure_ip_to_ip",
                    )?;
                    if event
                        .remove("recordedfuture.identity_detection.dump.infrastructure.ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "recordedfuture.identity_detection.dump.infrastructure.ip".into(),
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

            if let Some(v) = event
                .get("recordedfuture.identity_detection.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("threat"))?;

            let _cond = {
                event.has_value("recordedfuture.identity_detection.source_type")
                    && event
                        .get_str("recordedfuture.identity_detection.source_type")
                        .is_some_and(|s| s.to_lowercase().contains("malware"))
            };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
            }

            event.append_unique("event.type", json!("indicator"))?;

            let _cond = {
                event.has_value("event.category")
                    && event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("malware"))
                        }
                        serde_json::Value::String(s) => s.contains("malware"),
                        _ => false,
                    })
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.dump.compromise.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = { event.has_value("recordedfuture.identity_detection.dump.compromise.os") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString os = ctx.recordedfuture.identity_detection.dump.compromise.os.toLowerCase();\nfor (String os_type: params.os_type) {\n  if (os.contains(os_type)) {\n    ctx.host.os.put('type', os_type);\n    return;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString os = ctx.recordedfuture.identity_detection.dump.compromise.os.toLowerCase();\nfor (String os_type: params.os_type) {\n  if (os.contains(os_type)) {\n    ctx.host.os.put('type', os_type);\n    return;\n  }\n}\n"#
                        ),
                        cached_params!(
                            "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_host_os_type",
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
                .get("recordedfuture.identity_detection.dump.compromise.computer_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.dump.location.country.alpha2Code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.dump.location.country.displayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_name", v)?;
            }

            let _cond = {
                !event.has_value("source.geo.country_name")
                    || event.get_str("source.geo.country_name") == Some("")
            };
            if _cond {
                if let Some(v) = event
                    .get("recordedfuture.identity_detection.dump.location.country.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.geo.country_name", v)?;
                }
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.dump.location.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.dump.location.postal_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.postal_code", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.malware_family.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.software.name", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.malware_family.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.software.id", v)?;
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.authorization_service.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            let _cond = { event.has_value("url.full") && event.get_str("url.full") != Some("") };
            if _cond {
                if event.has_value("url.full") {
                    uri_parts(event, "url.full", "url", true, false)?;
                }
            }

            let _cond =
                { !event.has_value("url.domain") || event.get_str("url.domain") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("recordedfuture.identity_detection.authorization_service.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.domain", v)?;
                }
            }

            if let Some(v) = event
                .get("recordedfuture.identity_detection.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("recordedfuture.identity_detection.subject")
                    && event
                        .get("recordedfuture.identity_detection.subject")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(v) = event
                    .get("recordedfuture.identity_detection.subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("recordedfuture.identity_detection.subject") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.subject")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("recordedfuture.identity_detection.authorization_service.domain")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.authorization_service.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("recordedfuture.identity_detection.authorization_service.fqdn") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.authorization_service.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("recordedfuture.identity_detection.dump.compromise.computer_name")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.dump.compromise.computer_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("recordedfuture.identity_detection.dump.compromise.os_username")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.dump.compromise.os_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("recordedfuture.identity_detection.dump.infrastructure.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("recordedfuture.identity_detection.dump.infrastructure.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("recordedfuture.identity_detection.id");
            event.remove("recordedfuture.identity_detection.created");
            event.remove("recordedfuture.identity_detection.subject");
            event.remove("recordedfuture.identity_detection.authorization_service.url");
            event.remove("recordedfuture.identity_detection.malware_family.id");
            event.remove("recordedfuture.identity_detection.malware_family.name");
            event.remove("recordedfuture.identity_detection.dump.compromise.os");
            event.remove("recordedfuture.identity_detection.dump.compromise.computer_name");
            event.remove("recordedfuture.identity_detection.dump.compromise.exfiltration_date");
            event.remove("recordedfuture.identity_detection.dump.location.country.alpha2Code");
            event.remove("recordedfuture.identity_detection.dump.location.country.displayName");
            event.remove("recordedfuture.identity_detection.dump.location.city");
            event.remove("recordedfuture.identity_detection.dump.location.postal_code");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
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
