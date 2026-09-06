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

            parse_json_field(event, "event.original", "axonius.identity")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "axonius.identity.transform_unique_id",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("info"))?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.identity.adapter_list_length") {
                    if let Some(val) = event.get("axonius.identity.adapter_list_length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.adapter_list_length".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.adapter_list_length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_adapter_list_length_to_long",
                )?;
                event.remove("axonius.identity.adapter_list_length");
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

            // Painless script
            // Source: if (ctx.axonius?.identity?.event?.data instanceof Map) {\n  Map eventData = ctx.axonius.identity.event.data;\n  for (String key : new ArrayList(eventData.keySet())) {\n    if (ctx.axonius.identity.containsKey(key)) {\n      ctx.axonius.identity['data_' + key] = eventData[key];\n    } else {\n      ctx.axonius.identity[key] = eventData[key];\n    }\n  }\n  ctx.axonius.identity.event.remove('data');\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.axonius?.identity?.event?.data instanceof Map) {\n  Map eventData = ctx.axonius.identity.event.data;\n  for (String key : new ArrayList(eventData.keySet())) {\n    if (ctx.axonius.identity.containsKey(key)) {\n      ctx.axonius.identity['data_' + key] = eventData[key];\n    } else {\n      ctx.axonius.identity[key] = eventData[key];\n    }\n  }\n  ctx.axonius.identity.event.remove('data');\n}"#
                ),
            )?;

            let _cond = {
                event.has_value("axonius.identity.event.accurate_for_datetime")
                    && event.get_str("axonius.identity.event.accurate_for_datetime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("axonius.identity.event.accurate_for_datetime")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                "yyyy-MM-dd",
                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("axonius.identity.event.accurate_for_datetime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "axonius.identity.event.accurate_for_datetime".into(),
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
                        "date_event_accurate_for_datetime",
                    )?;
                    event.remove("axonius.identity.event.accurate_for_datetime");
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
                .get("axonius.identity.event.accurate_for_datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("axonius.identity.event.action_if_exists")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.action") {
                    if let Some(s) = event.get_string("event.action") {
                        let mut parts: Vec<Value> = cached_regex!("\\s+")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("event.action", Value::Array(parts))?;
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

            let _cond =
                { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
                event.has_value("axonius.identity.accurate_for_datetime")
                    && event.get_str("axonius.identity.accurate_for_datetime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("axonius.identity.accurate_for_datetime")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                "yyyy-MM-dd",
                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("axonius.identity.accurate_for_datetime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "axonius.identity.accurate_for_datetime".into(),
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
                        "date_accurate_for_datetime",
                    )?;
                    event.remove("axonius.identity.accurate_for_datetime");
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
                .get("axonius.identity.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("axonius.identity.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("axonius.identity.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("axonius.identity.tenant_number")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.tenant_number", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "long").map_err(|message| {
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
                            "convert_identity_tenant_number_to_long",
                        )?;
                        event.remove("_ingest._value");
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

            let _cond = {
                event.has_value("axonius.identity.fetch_time")
                    && event.get_str("axonius.identity.fetch_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("axonius.identity.fetch_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                "yyyy-MM-dd",
                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("axonius.identity.fetch_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "axonius.identity.fetch_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_fetch_time")?;
                    event.remove("axonius.identity.fetch_time");
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
                event.has_value("axonius.identity.first_fetch_time")
                    && event.get_str("axonius.identity.first_fetch_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("axonius.identity.first_fetch_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                "yyyy-MM-dd",
                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("axonius.identity.first_fetch_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "axonius.identity.first_fetch_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_first_fetch_time")?;
                    event.remove("axonius.identity.first_fetch_time");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.identity.from_last_fetch") {
                    if let Some(val) = event.get("axonius.identity.from_last_fetch") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.from_last_fetch".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.from_last_fetch", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_from_last_fetch_to_boolean",
                )?;
                event.remove("axonius.identity.from_last_fetch");
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
                if event.has_value("axonius.identity.has_administrative_permissions") {
                    if let Some(val) = event.get("axonius.identity.has_administrative_permissions")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.has_administrative_permissions".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.has_administrative_permissions", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_has_administrative_permissions_to_boolean",
                )?;
                event.remove("axonius.identity.has_administrative_permissions");
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
                if event.has_value("axonius.identity.is_admin") {
                    if let Some(val) = event.get("axonius.identity.is_admin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.is_admin".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.is_admin", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_admin_to_boolean",
                )?;
                event.remove("axonius.identity.is_admin");
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
                if event.has_value("axonius.identity.is_built_in") {
                    if let Some(val) = event.get("axonius.identity.is_built_in") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.is_built_in".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.is_built_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_built_in_to_boolean",
                )?;
                event.remove("axonius.identity.is_built_in");
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
                if event.has_value("axonius.identity.is_fetched_from_adapter") {
                    if let Some(val) = event.get("axonius.identity.is_fetched_from_adapter") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.is_fetched_from_adapter".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.is_fetched_from_adapter", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_fetched_from_adapter_to_boolean",
                )?;
                event.remove("axonius.identity.is_fetched_from_adapter");
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
                if event.has_value("axonius.identity.is_managed_by_sso") {
                    if let Some(val) = event.get("axonius.identity.is_managed_by_sso") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.is_managed_by_sso".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.is_managed_by_sso", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_managed_by_sso_to_boolean",
                )?;
                event.remove("axonius.identity.is_managed_by_sso");
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
                if event.has_value("axonius.identity.is_privileged") {
                    if let Some(val) = event.get("axonius.identity.is_privileged") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.is_privileged".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.is_privileged", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_privileged_to_boolean",
                )?;
                event.remove("axonius.identity.is_privileged");
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
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.active_from_direct_adapter") {
                            if let Some(val) =
                                event.get("_ingest._value.active_from_direct_adapter")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.active_from_direct_adapter"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("_ingest._value.active_from_direct_adapter", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_active_from_direct_adapter_to_boolean",
                        )?;
                        event.remove("_ingest._value.active_from_direct_adapter");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.has_administrative_permissions") {
                            if let Some(val) =
                                event.get("_ingest._value.has_administrative_permissions")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.has_administrative_permissions"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "_ingest._value.has_administrative_permissions",
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
                            "convert_nested_applications_has_administrative_permissions_to_boolean",
                        )?;
                        event.remove("_ingest._value.has_administrative_permissions");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_deleted") {
                            if let Some(val) = event.get("_ingest._value.is_deleted") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_deleted".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_deleted", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_deleted_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_deleted");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_from_direct_adapter") {
                            if let Some(val) = event.get("_ingest._value.is_from_direct_adapter") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_from_direct_adapter".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_from_direct_adapter", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_from_direct_adapter_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_from_direct_adapter");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_managed") {
                            if let Some(val) = event.get("_ingest._value.is_managed") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_managed".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_managed", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_managed_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_managed");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_suspended") {
                            if let Some(val) = event.get("_ingest._value.is_suspended") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_suspended".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_suspended", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_suspended_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_suspended");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_unmanaged_extension") {
                            if let Some(val) = event.get("_ingest._value.is_unmanaged_extension") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_unmanaged_extension".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_unmanaged_extension", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_unmanaged_extension_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_unmanaged_extension");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_user_external") {
                            if let Some(val) = event.get("_ingest._value.is_user_external") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_user_external".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_user_external", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_user_external_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_user_external");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_user_paid") {
                            if let Some(val) = event.get("_ingest._value.is_user_paid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_user_paid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_user_paid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_is_user_paid_to_boolean",
                        )?;
                        event.remove("_ingest._value.is_user_paid");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("axonius.identity.nested_applications").cloned();
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
                                    event.get_as_string("_ingest._value.last_access")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                            "yyyy-MM-dd",
                                            "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_access", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_access".into(),
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
                                    "date_nested_applications_last_access",
                                )?;
                                event.remove("_ingest._value.last_access");
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
                            "axonius.identity.nested_applications",
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
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.last_access_count") {
                            if let Some(val) = event.get("_ingest._value.last_access_count") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.last_access_count".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.last_access_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_last_access_count_to_long",
                        )?;
                        event.remove("_ingest._value.last_access_count");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.last_access_count_60_days") {
                            if let Some(val) = event.get("_ingest._value.last_access_count_60_days")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.last_access_count_60_days".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.last_access_count_60_days", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_last_access_count_60_days_to_long",
                        )?;
                        event.remove("_ingest._value.last_access_count_60_days");
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

            let _cond = {
                event
                    .get("axonius.identity.nested_applications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_applications", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.last_access_count_90_days") {
                            if let Some(val) = event.get("_ingest._value.last_access_count_90_days")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.last_access_count_90_days".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.last_access_count_90_days", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nested_applications_last_access_count_90_days_to_long",
                        )?;
                        event.remove("_ingest._value.last_access_count_90_days");
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

            let _cond = {
                event.has_value("axonius.identity.nested_grants_last_updated")
                    && event.get_str("axonius.identity.nested_grants_last_updated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("axonius.identity.nested_grants_last_updated")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                "yyyy-MM-dd",
                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("axonius.identity.nested_grants_last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "axonius.identity.nested_grants_last_updated".into(),
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
                        "date_nested_grants_last_updated",
                    )?;
                    event.remove("axonius.identity.nested_grants_last_updated");
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
                    .get("axonius.identity.nested_resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.identity.nested_resources", |event| {
                    if event.has_value("_ingest._value.value") {
                        if let Some(val) = event.get("_ingest._value.value") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.value", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.identity.not_fetched_count") {
                    if let Some(val) = event.get("axonius.identity.not_fetched_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.not_fetched_count".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.not_fetched_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_not_fetched_count_to_long",
                )?;
                event.remove("axonius.identity.not_fetched_count");
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
                if event.has_value("axonius.identity.operational_users_count") {
                    if let Some(val) = event.get("axonius.identity.operational_users_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.operational_users_count".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.operational_users_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_operational_users_count_to_long",
                )?;
                event.remove("axonius.identity.operational_users_count");
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
                if event.has_value("axonius.identity.total_users_count") {
                    if let Some(val) = event.get("axonius.identity.total_users_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.total_users_count".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.total_users_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_total_users_count_to_long",
                )?;
                event.remove("axonius.identity.total_users_count");
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
                if event.has_value("axonius.identity.event.hidden_for_gui") {
                    if let Some(val) = event.get("axonius.identity.event.hidden_for_gui") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.event.hidden_for_gui".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.event.hidden_for_gui", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_hidden_for_gui_to_boolean",
                )?;
                event.remove("axonius.identity.event.hidden_for_gui");
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
                if event.has_value("axonius.identity.permissions") {
                    if let Some(val) = event.get("axonius.identity.permissions") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.permissions".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.permissions", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_permissions_to_long",
                )?;
                if event.has_value("axonius.identity.permissions") {
                    event.rename(
                        "axonius.identity.permissions",
                        "axonius.identity.permissions_list",
                    )?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("axonius.identity.asset_type") == Some("accounts") };
            if _cond {
                // Begin nested pipeline: "pipeline-account"
                if event.has_value("axonius.identity.roles") {
                    event.rename("axonius.identity.roles", "axonius.identity.roles_accounts")?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.active_users") {
                        if let Some(val) = event.get("axonius.identity.active_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.active_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.active_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_active_users_to_long",
                    )?;
                    event.remove("axonius.identity.active_users");
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
                    if event.has_value("axonius.identity.admin_non_operational_users") {
                        if let Some(val) = event.get("axonius.identity.admin_non_operational_users")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.admin_non_operational_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.admin_non_operational_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_admin_non_operational_users_to_long",
                    )?;
                    event.remove("axonius.identity.admin_non_operational_users");
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
                    if event.has_value("axonius.identity.admin_operational_active_users") {
                        if let Some(val) =
                            event.get("axonius.identity.admin_operational_active_users")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.admin_operational_active_users".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "axonius.identity.admin_operational_active_users",
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
                        "convert_admin_operational_active_users_to_long",
                    )?;
                    event.remove("axonius.identity.admin_operational_active_users");
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
                    if event.has_value("axonius.identity.admin_operational_inactive_users") {
                        if let Some(val) =
                            event.get("axonius.identity.admin_operational_inactive_users")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.admin_operational_inactive_users"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "axonius.identity.admin_operational_inactive_users",
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
                        "convert_admin_operational_inactive_users_to_long",
                    )?;
                    event.remove("axonius.identity.admin_operational_inactive_users");
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
                    if event.has_value("axonius.identity.admin_operational_users") {
                        if let Some(val) = event.get("axonius.identity.admin_operational_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.admin_operational_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.admin_operational_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_admin_operational_users_to_long",
                    )?;
                    event.remove("axonius.identity.admin_operational_users");
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
                    if event.has_value("axonius.identity.admins") {
                        if let Some(val) = event.get("axonius.identity.admins") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.admins".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.admins", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_admins_to_long")?;
                    event.remove("axonius.identity.admins");
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
                    .get("axonius.identity.application_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("service.id", v)?;
                }
                if let Some(v) = event
                    .get("axonius.identity.application_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("service.name", v)?;
                }
                let _cond = {
                    event.has_value("axonius.identity.created_date")
                        && event.get_str("axonius.identity.created_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.created_date")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.created_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.created_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_created_date")?;
                        event.remove("axonius.identity.created_date");
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
                    .get("axonius.identity.created_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.deleted_users") {
                        if let Some(val) = event.get("axonius.identity.deleted_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.deleted_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.deleted_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_deleted_users_to_long",
                    )?;
                    event.remove("axonius.identity.deleted_users");
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
                    if event.has_value("axonius.identity.direct_not_sso_users") {
                        if let Some(val) = event.get("axonius.identity.direct_not_sso_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.direct_not_sso_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.direct_not_sso_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_direct_not_sso_users_to_long",
                    )?;
                    event.remove("axonius.identity.direct_not_sso_users");
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
                    .get("axonius.identity.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond = {
                    event.has_value("user.email")
                        && event.get("user.email").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.email".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("axonius.identity.email") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.external_users") {
                        if let Some(val) = event.get("axonius.identity.external_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.external_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.external_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_external_users_to_long",
                    )?;
                    event.remove("axonius.identity.external_users");
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
                    if event.has_value("axonius.identity.inactive_users") {
                        if let Some(val) = event.get("axonius.identity.inactive_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.inactive_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.inactive_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_inactive_users_to_long",
                    )?;
                    event.remove("axonius.identity.inactive_users");
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
                    if event.has_value("axonius.identity.is_managed_by_direct_app") {
                        if let Some(val) = event.get("axonius.identity.is_managed_by_direct_app") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.is_managed_by_direct_app".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.is_managed_by_direct_app", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_managed_by_direct_app_to_boolean",
                    )?;
                    event.remove("axonius.identity.is_managed_by_direct_app");
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
                    event.has_value("axonius.identity.last_enrichment_run")
                        && event.get_str("axonius.identity.last_enrichment_run") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.last_enrichment_run")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.last_enrichment_run", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.last_enrichment_run".into(),
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
                            "date_last_enrichment_run",
                        )?;
                        event.remove("axonius.identity.last_enrichment_run");
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.managed_non_operational_users") {
                        if let Some(val) =
                            event.get("axonius.identity.managed_non_operational_users")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.managed_non_operational_users".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("axonius.identity.managed_non_operational_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_managed_non_operational_users_to_long",
                    )?;
                    event.remove("axonius.identity.managed_non_operational_users");
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
                    if event.has_value("axonius.identity.managed_operational_users") {
                        if let Some(val) = event.get("axonius.identity.managed_operational_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.managed_operational_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.managed_operational_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_managed_operational_users_to_long",
                    )?;
                    event.remove("axonius.identity.managed_operational_users");
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
                    if event.has_value("axonius.identity.managed_users") {
                        if let Some(val) = event.get("axonius.identity.managed_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.managed_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.managed_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_managed_users_to_long",
                    )?;
                    event.remove("axonius.identity.managed_users");
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
                    if event.has_value("axonius.identity.managed_users_by_app") {
                        if let Some(val) = event.get("axonius.identity.managed_users_by_app") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.managed_users_by_app".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.managed_users_by_app", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_managed_users_by_app_to_long",
                    )?;
                    event.remove("axonius.identity.managed_users_by_app");
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
                    if event.has_value("axonius.identity.managed_users_by_sso") {
                        if let Some(val) = event.get("axonius.identity.managed_users_by_sso") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.managed_users_by_sso".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.managed_users_by_sso", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_managed_users_by_sso_to_long",
                    )?;
                    event.remove("axonius.identity.managed_users_by_sso");
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
                    if event.has_value("axonius.identity.orphaned_users") {
                        if let Some(val) = event.get("axonius.identity.orphaned_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.orphaned_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.orphaned_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_orphaned_users_to_long",
                    )?;
                    event.remove("axonius.identity.orphaned_users");
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
                    if event.has_value("axonius.identity.paid_users") {
                        if let Some(val) = event.get("axonius.identity.paid_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.paid_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.paid_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_paid_users_to_long",
                    )?;
                    event.remove("axonius.identity.paid_users");
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
                    if event.has_value("axonius.identity.suspended_users") {
                        if let Some(val) = event.get("axonius.identity.suspended_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.suspended_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.suspended_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_suspended_users_to_long",
                    )?;
                    event.remove("axonius.identity.suspended_users");
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
                    if event.has_value("axonius.identity.unlinked_users") {
                        if let Some(val) = event.get("axonius.identity.unlinked_users") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.unlinked_users".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.unlinked_users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_unlinked_users_to_long",
                    )?;
                    event.remove("axonius.identity.unlinked_users");
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
                // End nested pipeline: "pipeline-account"
            }

            let _cond = { event.get_str("axonius.identity.asset_type") == Some("groups") };
            if _cond {
                // Begin nested pipeline: "pipeline-group"
                let _cond = {
                    event.has_value("axonius.identity.first_seen")
                        && event.get_str("axonius.identity.first_seen") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.first_seen") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.first_seen", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.first_seen".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_first_seen")?;
                        event.remove("axonius.identity.first_seen");
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
                        .get("axonius.identity.user_count_link")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_count_link", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.bracketWeight") {
                                if let Some(val) = event.get("_ingest._value.bracketWeight") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.bracketWeight".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.bracketWeight", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_count_link_bracketWeight_to_double",
                            )?;
                            event.remove("_ingest._value.bracketWeight");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_count_link")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_count_link", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.leftBracket") {
                                if let Some(val) = event.get("_ingest._value.leftBracket") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.leftBracket".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.leftBracket", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_count_link_leftBracket_to_double",
                            )?;
                            event.remove("_ingest._value.leftBracket");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_count_link")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_count_link", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.not") {
                                if let Some(val) = event.get("_ingest._value.not") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.not".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.not", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_count_link_not_to_boolean",
                            )?;
                            event.remove("_ingest._value.not");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_count_link")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_count_link", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.rightBracket") {
                                if let Some(val) = event.get("_ingest._value.rightBracket") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.rightBracket".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.rightBracket", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_count_link_rightBracket_to_double",
                            )?;
                            event.remove("_ingest._value.rightBracket");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_count_link")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_count_link", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                // End nested pipeline: "pipeline-group"
            }

            let _cond = { event.get_str("axonius.identity.asset_type") == Some("certificates") };
            if _cond {
                // Begin nested pipeline: "pipeline-certificate"
                let _cond = {
                    event.has_value("axonius.identity.begins_on")
                        && event.get_str("axonius.identity.begins_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.begins_on") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.begins_on", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.begins_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_begins_on")?;
                        event.remove("axonius.identity.begins_on");
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
                    .get("axonius.identity.begins_on")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.bit_size") {
                        if let Some(val) = event.get("axonius.identity.bit_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.bit_size".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.bit_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_bit_size_to_long",
                    )?;
                    event.remove("axonius.identity.bit_size");
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
                    event.has_value("axonius.identity.expires_on")
                        && event.get_str("axonius.identity.expires_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.expires_on") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.expires_on", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.expires_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_expires_on")?;
                        event.remove("axonius.identity.expires_on");
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
                    .get("axonius.identity.expires_on")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.end", v)?;
                }
                let _cond = { event.has_value("axonius.identity.issuer.common_name") };
                if _cond {
                    event.append_unique(
                        "file.x509.issuer.common_name",
                        json!(
                            event
                                .get("axonius.identity.issuer.common_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.issuer.country_name") };
                if _cond {
                    event.append_unique(
                        "file.x509.issuer.country",
                        json!(
                            event
                                .get("axonius.identity.issuer.country_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.issuer.organization") };
                if _cond {
                    event.append_unique(
                        "file.x509.issuer.organization",
                        json!(
                            event
                                .get("axonius.identity.issuer.organization")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("axonius.identity.serial_number")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.x509.serial_number", v)?;
                }
                let _cond = { event.has_value("axonius.identity.subject.common_name") };
                if _cond {
                    event.append_unique(
                        "file.x509.subject.common_name",
                        json!(
                            event
                                .get("axonius.identity.subject.common_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.subject.country_name") };
                if _cond {
                    event.append_unique(
                        "file.x509.subject.country",
                        json!(
                            event
                                .get("axonius.identity.subject.country_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.subject.locality") };
                if _cond {
                    event.append_unique(
                        "file.x509.subject.locality",
                        json!(
                            event
                                .get("axonius.identity.subject.locality")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.subject.organization") };
                if _cond {
                    event.append_unique(
                        "file.x509.subject.organization",
                        json!(
                            event
                                .get("axonius.identity.subject.organization")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.subject.state") };
                if _cond {
                    event.append_unique(
                        "file.x509.subject.state_or_province",
                        json!(
                            event
                                .get("axonius.identity.subject.state")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline-certificate"
            }

            let _cond = { event.get_str("axonius.identity.asset_type") == Some("users") };
            if _cond {
                // Begin nested pipeline: "pipeline-user"
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.account_disabled") {
                        if let Some(val) = event.get("axonius.identity.account_disabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.account_disabled".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.account_disabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_account_disabled_to_boolean",
                    )?;
                    event.remove("axonius.identity.account_disabled");
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
                    // Painless script
                    // Source: void appendUnique(Map container, String field, def value) {\n  if (value == null) return;\n  def current = container[field];\n  if (current == null) {\n    container[field] = [value];\n  } else if (current instanceof List) {\n    if (!current.contains(value)) {\n      current.add(value);\n    }\n  } else if (current != value) {\n    container[field] = [current, value];\n  }\n}\nif (ctx.axonius?.identity?.associated_devices instanceof List) {\n  if (ctx.device == null) ctx.device = [:];\n  if (ctx.device.model == null) ctx.device.model = [:];\n  for (def device : ctx.axonius.identity.associated_devices) {\n    if (!(device instanceof Map)) continue;\n    if (device.device_id instanceof List) {\n      for (def v : device.device_id) {\n        if (v != null) appendUnique(ctx.device, 'id', v.toString());\n      }\n    }\n    if (device.device_model instanceof List) {\n      for (def v : device.device_model) {\n        if (v != null) appendUnique(ctx.device.model, 'name', v.toString());\n      }\n    }\n    if (device.device_serial instanceof List) {\n      for (def v : device.device_serial) {\n        if (v != null) appendUnique(ctx.device, 'serial_number', v.toString());\n      }\n    }\n    if (device.device_preferred_mac_address instanceof List) {\n      for (int i = 0; i < device.device_preferred_mac_address.size(); i++) {\n        def mac = device.device_preferred_mac_address[i];\n        if (mac != null) {\n          device.device_preferred_mac_address[i] = mac.toString().replace(':', '-').toUpperCase();\n        }\n      }\n    }\n  }\n}\nif (ctx.axonius?.identity?.associated_employees instanceof List) {\n  if (ctx.related == null) ctx.related = [:];\n  for (def emp : ctx.axonius.identity.associated_employees) {\n    if (!(emp instanceof Map)) continue;\n    if (emp.username instanceof List) {\n      for (def v : emp.username) {\n        if (v != null) appendUnique(ctx.related, 'user', v.toString());\n      }\n    }\n  }\n}\nif (ctx.axonius?.identity?.associated_groups instanceof List) {\n  if (ctx.related == null) ctx.related = [:];\n  for (def grp : ctx.axonius.identity.associated_groups) {\n    if (!(grp instanceof Map)) continue;\n    if (grp.display_name != null) {\n      appendUnique(ctx.related, 'user', grp.display_name.toString());\n    }\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"void appendUnique(Map container, String field, def value) {\n  if (value == null) return;\n  def current = container[field];\n  if (current == null) {\n    container[field] = [value];\n  } else if (current instanceof List) {\n    if (!current.contains(value)) {\n      current.add(value);\n    }\n  } else if (current != value) {\n    container[field] = [current, value];\n  }\n}\nif (ctx.axonius?.identity?.associated_devices instanceof List) {\n  if (ctx.device == null) ctx.device = [:];\n  if (ctx.device.model == null) ctx.device.model = [:];\n  for (def device : ctx.axonius.identity.associated_devices) {\n    if (!(device instanceof Map)) continue;\n    if (device.device_id instanceof List) {\n      for (def v : device.device_id) {\n        if (v != null) appendUnique(ctx.device, 'id', v.toString());\n      }\n    }\n    if (device.device_model instanceof List) {\n      for (def v : device.device_model) {\n        if (v != null) appendUnique(ctx.device.model, 'name', v.toString());\n      }\n    }\n    if (device.device_serial instanceof List) {\n      for (def v : device.device_serial) {\n        if (v != null) appendUnique(ctx.device, 'serial_number', v.toString());\n      }\n    }\n    if (device.device_preferred_mac_address instanceof List) {\n      for (int i = 0; i < device.device_preferred_mac_address.size(); i++) {\n        def mac = device.device_preferred_mac_address[i];\n        if (mac != null) {\n          device.device_preferred_mac_address[i] = mac.toString().replace(':', '-').toUpperCase();\n        }\n      }\n    }\n  }\n}\nif (ctx.axonius?.identity?.associated_employees instanceof List) {\n  if (ctx.related == null) ctx.related = [:];\n  for (def emp : ctx.axonius.identity.associated_employees) {\n    if (!(emp instanceof Map)) continue;\n    if (emp.username instanceof List) {\n      for (def v : emp.username) {\n        if (v != null) appendUnique(ctx.related, 'user', v.toString());\n      }\n    }\n  }\n}\nif (ctx.axonius?.identity?.associated_groups instanceof List) {\n  if (ctx.related == null) ctx.related = [:];\n  for (def grp : ctx.axonius.identity.associated_groups) {\n    if (!(grp instanceof Map)) continue;\n    if (grp.display_name != null) {\n      appendUnique(ctx.related, 'user', grp.display_name.toString());\n    }\n  }\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_process_associated_entities",
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
                if let Some(v) = event
                    .get("axonius.identity.aws_iam_identity_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.service.name", v)?;
                }
                if let Some(v) = event
                    .get("axonius.identity.azure_account_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.breaches_data")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: String parseDate(String input, def fmts, def outFmt) {\n  // Strip \"EEE,\" / \"EEE, \" day-of-week prefix if present so we don't\n  // depend on it being internally consistent (the source data is known\n  // to occasionally have a day-of-week that doesn't match the date).\n  String stripped = input;\n  if (input.length() > 4 && input.indexOf(',') == 3) {\n    stripped = input.substring(4).trim();\n  }\n  for (def fmt : fmts) {\n    try {\n      return LocalDateTime.parse(stripped, fmt).atZone(ZoneId.of('UTC')).format(outFmt);\n    } catch (Exception e1) {\n      try {\n        return LocalDate.parse(stripped, fmt).atStartOfDay(ZoneId.of('UTC')).format(outFmt);\n      } catch (Exception e2) { /* try next */ }\n    }\n  }\n  return null;\n}\nvoid appendUnique(Map container, String field, def value) {\n  if (value == null) return;\n  def current = container[field];\n  if (current == null) {\n    container[field] = [value];\n  } else if (current instanceof List) {\n    if (!current.contains(value)) {\n      current.add(value);\n    }\n  } else if (current != value) {\n    container[field] = [current, value];\n  }\n}\ndef fmts = [\n  DateTimeFormatter.ofPattern(\"dd MMM yyyy HH:mm:ss 'GMT'\", Locale.ENGLISH),\n  DateTimeFormatter.ofPattern(\"yyyy-MM-dd\", Locale.ENGLISH)\n];\ndef outFmt = DateTimeFormatter.ofPattern(\"yyyy-MM-dd'T'HH:mm:ss.SSSXXX\");\nfor (def item : ctx.axonius.identity.breaches_data) {\n  if (!(item instanceof Map)) continue;\n  for (def f : params.dateFields) {\n    if (item[f] == null || item[f] == '') continue;\n    String parsed = parseDate(item[f].toString(), fmts, outFmt);\n    if (parsed != null) {\n      item[f] = parsed;\n    } else {\n      item.remove(f);\n    }\n  }\n  if (item.added_date != null) {\n    if (ctx.threat == null) ctx.threat = [:];\n    if (ctx.threat.enrichments == null) ctx.threat.enrichments = [:];\n    if (ctx.threat.enrichments.indicator == null) ctx.threat.enrichments.indicator = [:];\n    appendUnique(ctx.threat.enrichments.indicator, 'first_seen', item.added_date.toString());\n  }\n  if (item.pwn_count != null && !(item.pwn_count instanceof Long || item.pwn_count instanceof Integer)) {\n    try {\n      item.pwn_count = Long.parseLong(item.pwn_count.toString());\n    } catch (NumberFormatException e) {\n      item.remove('pwn_count');\n    }\n  }\n  for (def f : params.boolFields) {\n    if (!item.containsKey(f) || item[f] == null) continue;\n    def v = item[f];\n    if (v instanceof Boolean) continue;\n    def s = v.toString().toLowerCase();\n    if (s == '1' || s == 'true') {\n      item[f] = true;\n    } else if (s == '' || s == '0' || s == 'false') {\n      item[f] = false;\n    } else {\n      item.remove(f);\n    }\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"String parseDate(String input, def fmts, def outFmt) {\n  // Strip \"EEE,\" / \"EEE, \" day-of-week prefix if present so we don't\n  // depend on it being internally consistent (the source data is known\n  // to occasionally have a day-of-week that doesn't match the date).\n  String stripped = input;\n  if (input.length() > 4 && input.indexOf(',') == 3) {\n    stripped = input.substring(4).trim();\n  }\n  for (def fmt : fmts) {\n    try {\n      return LocalDateTime.parse(stripped, fmt).atZone(ZoneId.of('UTC')).format(outFmt);\n    } catch (Exception e1) {\n      try {\n        return LocalDate.parse(stripped, fmt).atStartOfDay(ZoneId.of('UTC')).format(outFmt);\n      } catch (Exception e2) { /* try next */ }\n    }\n  }\n  return null;\n}\nvoid appendUnique(Map container, String field, def value) {\n  if (value == null) return;\n  def current = container[field];\n  if (current == null) {\n    container[field] = [value];\n  } else if (current instanceof List) {\n    if (!current.contains(value)) {\n      current.add(value);\n    }\n  } else if (current != value) {\n    container[field] = [current, value];\n  }\n}\ndef fmts = [\n  DateTimeFormatter.ofPattern(\"dd MMM yyyy HH:mm:ss 'GMT'\", Locale.ENGLISH),\n  DateTimeFormatter.ofPattern(\"yyyy-MM-dd\", Locale.ENGLISH)\n];\ndef outFmt = DateTimeFormatter.ofPattern(\"yyyy-MM-dd'T'HH:mm:ss.SSSXXX\");\nfor (def item : ctx.axonius.identity.breaches_data) {\n  if (!(item instanceof Map)) continue;\n  for (def f : params.dateFields) {\n    if (item[f] == null || item[f] == '') continue;\n    String parsed = parseDate(item[f].toString(), fmts, outFmt);\n    if (parsed != null) {\n      item[f] = parsed;\n    } else {\n      item.remove(f);\n    }\n  }\n  if (item.added_date != null) {\n    if (ctx.threat == null) ctx.threat = [:];\n    if (ctx.threat.enrichments == null) ctx.threat.enrichments = [:];\n    if (ctx.threat.enrichments.indicator == null) ctx.threat.enrichments.indicator = [:];\n    appendUnique(ctx.threat.enrichments.indicator, 'first_seen', item.added_date.toString());\n  }\n  if (item.pwn_count != null && !(item.pwn_count instanceof Long || item.pwn_count instanceof Integer)) {\n    try {\n      item.pwn_count = Long.parseLong(item.pwn_count.toString());\n    } catch (NumberFormatException e) {\n      item.remove('pwn_count');\n    }\n  }\n  for (def f : params.boolFields) {\n    if (!item.containsKey(f) || item[f] == null) continue;\n    def v = item[f];\n    if (v instanceof Boolean) continue;\n    def s = v.toString().toLowerCase();\n    if (s == '1' || s == 'true') {\n      item[f] = true;\n    } else if (s == '' || s == '0' || s == 'false') {\n      item[f] = false;\n    } else {\n      item.remove(f);\n    }\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"boolFields\":[\"is_fabricated\",\"is_retired\",\"is_sensitive\",\"is_spam_list\",\"is_verified\"],\"dateFields\":[\"added_date\",\"breach_date\",\"modified_date\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_process_breaches_data",
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
                    .get("axonius.identity.cloud_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.provider", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.distinct_associated_devices_count") {
                        if let Some(val) =
                            event.get("axonius.identity.distinct_associated_devices_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.distinct_associated_devices_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "axonius.identity.distinct_associated_devices_count",
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
                        "convert_distinct_associated_devices_count_to_long",
                    )?;
                    event.remove("axonius.identity.distinct_associated_devices_count");
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
                    if event.has_value("axonius.identity.email_activity.is_deleted") {
                        if let Some(val) = event.get("axonius.identity.email_activity.is_deleted") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_activity.is_deleted".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.email_activity.is_deleted", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_activity_is_deleted_to_boolean",
                    )?;
                    event.remove("axonius.identity.email_activity.is_deleted");
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
                    if event.has_value("axonius.identity.email_activity.read_count") {
                        if let Some(val) = event.get("axonius.identity.email_activity.read_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_activity.read_count".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.email_activity.read_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_activity_read_count_to_long",
                    )?;
                    event.remove("axonius.identity.email_activity.read_count");
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
                    if event.has_value("axonius.identity.email_activity.receive_count") {
                        if let Some(val) =
                            event.get("axonius.identity.email_activity.receive_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_activity.receive_count".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("axonius.identity.email_activity.receive_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_activity_receive_count_to_long",
                    )?;
                    event.remove("axonius.identity.email_activity.receive_count");
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
                    event.has_value("axonius.identity.email_activity.report_date")
                        && event.get_str("axonius.identity.email_activity.report_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.email_activity.report_date")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event
                                    .set("axonius.identity.email_activity.report_date", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.email_activity.report_date".into(),
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
                            "date_email_activity_report_date",
                        )?;
                        event.remove("axonius.identity.email_activity.report_date");
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
                    event.has_value("axonius.identity.user_pass_last_used")
                        && event.get_str("axonius.identity.user_pass_last_used") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.user_pass_last_used")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.user_pass_last_used", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.user_pass_last_used".into(),
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
                            "date_user_pass_last_used",
                        )?;
                        event.remove("axonius.identity.user_pass_last_used");
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.email_activity.report_period") {
                        if let Some(val) =
                            event.get("axonius.identity.email_activity.report_period")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_activity.report_period".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("axonius.identity.email_activity.report_period", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_activity_report_period_to_long",
                    )?;
                    event.remove("axonius.identity.email_activity.report_period");
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
                    if event.has_value("axonius.identity.email_activity.send_count") {
                        if let Some(val) = event.get("axonius.identity.email_activity.send_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_activity.send_count".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.email_activity.send_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_email_activity_send_count_to_long",
                    )?;
                    event.remove("axonius.identity.email_activity.send_count");
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
                    if event
                        .has_value("axonius.identity.email_notification.alternative_host_reminder")
                    {
                        if let Some(val) = event
                            .get("axonius.identity.email_notification.alternative_host_reminder")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                path: "axonius.identity.email_notification.alternative_host_reminder".into(),
                message,
                }
                            })?;
                            event.set(
                                "axonius.identity.email_notification.alternative_host_reminder",
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
                        "convert_email_notification_alternative_host_reminder_to_boolean",
                    )?;
                    event.remove("axonius.identity.email_notification.alternative_host_reminder");
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
                    if event
                        .has_value("axonius.identity.email_notification.cancel_meeting_reminder")
                    {
                        if let Some(val) =
                            event.get("axonius.identity.email_notification.cancel_meeting_reminder")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                path: "axonius.identity.email_notification.cancel_meeting_reminder".into(),
                message,
                }
                            })?;
                            event.set(
                                "axonius.identity.email_notification.cancel_meeting_reminder",
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
                        "convert_email_notification_cancel_meeting_reminder_to_boolean",
                    )?;
                    event.remove("axonius.identity.email_notification.cancel_meeting_reminder");
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
                    if event.has_value("axonius.identity.email_notification.jbh_reminder") {
                        if let Some(val) =
                            event.get("axonius.identity.email_notification.jbh_reminder")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.email_notification.jbh_reminder".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "axonius.identity.email_notification.jbh_reminder",
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
                        "convert_email_notification_jbh_reminder_to_boolean",
                    )?;
                    event.remove("axonius.identity.email_notification.jbh_reminder");
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
                let _cond = { event.has_value("axonius.identity.feature") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def obj = ctx.axonius.identity.feature;\nfor (def f : params.boolFields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}\nfor (def f : params.longFields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Long || v instanceof Integer) {\n    continue;\n  }\n  try {\n    obj[f] = Long.parseLong(v.toString());\n  } catch (NumberFormatException e) {\n    obj.remove(f);\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"def obj = ctx.axonius.identity.feature;\nfor (def f : params.boolFields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}\nfor (def f : params.longFields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Long || v instanceof Integer) {\n    continue;\n  }\n  try {\n    obj[f] = Long.parseLong(v.toString());\n  } catch (NumberFormatException e) {\n    obj.remove(f);\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"boolFields\":[\"cn_meeting\",\"in_meeting\",\"large_meeting\",\"webinar\",\"zoom_phone\"],\"longFields\":[\"meeting_capacity\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_feature_fields",
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
                let _cond = { event.has_value("axonius.identity.first_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.first_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("axonius.identity.hire_date")
                        && event.get_str("axonius.identity.hire_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.hire_date") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.hire_date", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.hire_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_hire_date")?;
                        event.remove("axonius.identity.hire_date");
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
                let _cond = { event.has_value("axonius.identity.in_meeting") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def obj = ctx.axonius.identity.in_meeting;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"def obj = ctx.axonius.identity.in_meeting;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"fields\":[\"allow_live_streaming\",\"annotation\",\"attendee_on_hold\",\"auto_saving_chat\",\"breakout_room\",\"chat\",\"closed_caption\",\"co_host\",\"e2e_encryption\",\"entry_exit_chime\",\"far_end_camera_control\",\"feedback\",\"group_hd\",\"non_verbal_feedback\",\"polling\",\"private_chat\",\"record_play_voice\",\"remote_control\",\"remote_support\",\"share_dual_camera\",\"show_meeting_control_toolbar\",\"virtual_background\",\"waiting_room\",\"workplace_by_facebook\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_in_meeting_booleans",
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
                let _cond = { event.has_value("axonius.identity") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def obj = ctx.axonius.identity;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"def obj = ctx.axonius.identity;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"fields\":[\"internal_is_admin\",\"is_active\",\"is_delegated_admin\",\"is_from_sso_provider\",\"is_latest_last_seen\",\"is_managed_by_application\",\"is_mfa_enforced\",\"is_mfa_enrolled\",\"is_non_editable\",\"is_paid\",\"is_permission_adapter\",\"is_saas_user\",\"is_user_active\",\"is_user_deleted\",\"is_user_external\",\"is_user_inactive\",\"is_user_suspended\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_identity_top_level_booleans",
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
                    event.has_value("axonius.identity.last_login_attempt")
                        && event.get_str("axonius.identity.last_login_attempt") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.last_login_attempt")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.last_login_attempt", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.last_login_attempt".into(),
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
                            "date_last_login_attempt",
                        )?;
                        event.remove("axonius.identity.last_login_attempt");
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
                    event.has_value("axonius.identity.last_logon")
                        && event.get_str("axonius.identity.last_logon") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.last_logon") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.last_logon", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.last_logon".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_last_logon")?;
                        event.remove("axonius.identity.last_logon");
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
                let _cond = { event.has_value("axonius.identity.last_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.last_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("axonius.identity.last_password_change")
                        && event.get_str("axonius.identity.last_password_change") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.last_password_change")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.last_password_change", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.last_password_change".into(),
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
                            "date_last_password_change",
                        )?;
                        event.remove("axonius.identity.last_password_change");
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
                    event.has_value("axonius.identity.last_seen")
                        && event.get_str("axonius.identity.last_seen") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.last_seen") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.last_seen", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.last_seen".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_last_seen")?;
                        event.remove("axonius.identity.last_seen");
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
                    .get("axonius.identity.mail")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond = {
                    event.has_value("user.email")
                        && event.get("user.email").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.email".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("axonius.identity.mail") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.mail")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.manager_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.manager_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("axonius.identity.max_added_date")
                        && event.get_str("axonius.identity.max_added_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.max_added_date")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.max_added_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.max_added_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_max_added_date")?;
                        event.remove("axonius.identity.max_added_date");
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
                    event.has_value("axonius.identity.max_breach_date")
                        && event.get_str("axonius.identity.max_breach_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.max_breach_date")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.max_breach_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.max_breach_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_max_breach_date")?;
                        event.remove("axonius.identity.max_breach_date");
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
                    event.has_value("axonius.identity.max_modified_date")
                        && event.get_str("axonius.identity.max_modified_date") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.max_modified_date")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.max_modified_date", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.max_modified_date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_max_modified_date")?;
                        event.remove("axonius.identity.max_modified_date");
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
                    event.has_value("axonius.identity.nested_grants_managers_last_updated")
                        && event.get_str("axonius.identity.nested_grants_managers_last_updated")
                            != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) =
                        (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "axonius.identity.nested_grants_managers_last_updated",
                            ) {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                        "yyyy-MM-dd",
                                        "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                    ],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "axonius.identity.nested_grants_managers_last_updated",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                path: "axonius.identity.nested_grants_managers_last_updated".into(),
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
                            "date_nested_grants_managers_last_updated",
                        )?;
                        event.remove("axonius.identity.nested_grants_managers_last_updated");
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
                        .get("axonius.identity.nested_managers")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.nested_managers", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.nested_permissions")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.nested_permissions", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.has_administrative_permissions") {
                                if let Some(val) =
                                    event.get("_ingest._value.has_administrative_permissions")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path:
                                                    "_ingest._value.has_administrative_permissions"
                                                        .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.has_administrative_permissions",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_nested_permissions_has_administrative_permissions_to_boolean")?;
                            event.remove("_ingest._value.has_administrative_permissions");
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
                let _cond = {
                    event
                        .get("axonius.identity.nested_permissions")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.nested_permissions", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_admin") {
                                if let Some(val) = event.get("_ingest._value.is_admin") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_admin".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_admin", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_nested_permissions_is_admin_to_boolean",
                            )?;
                            event.remove("_ingest._value.is_admin");
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
                let _cond = {
                    event
                        .get("axonius.identity.oracle_cloud_cis_incompliant")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "axonius.identity.oracle_cloud_cis_incompliant",
                        |event| {
                            if event.has_value("_ingest._value.rule_section") {
                                if let Some(val) = event.get("_ingest._value.rule_section") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.rule_section".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.rule_section", converted)?;
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.oracle_cloud_cis_incompliant")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "axonius.identity.oracle_cloud_cis_incompliant",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.rule_cis_version") {
                                    if let Some(val) = event.get("_ingest._value.rule_cis_version")
                                    {
                                        let converted =
                                            convert_value(val, "float").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.rule_cis_version".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.rule_cis_version", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_oracle_cloud_cis_incompliant_rule_cis_version_to_float")?;
                                event.remove("_ingest._value.rule_cis_version");
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
                    if event.has_value("axonius.identity.password_never_expires") {
                        if let Some(val) = event.get("axonius.identity.password_never_expires") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.password_never_expires".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.password_never_expires", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_password_never_expires_to_boolean",
                    )?;
                    event.remove("axonius.identity.password_never_expires");
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
                    if event.has_value("axonius.identity.password_not_required") {
                        if let Some(val) = event.get("axonius.identity.password_not_required") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.password_not_required".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.password_not_required", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_password_not_required_to_boolean",
                    )?;
                    event.remove("axonius.identity.password_not_required");
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
                if event.has_value("axonius.identity.pmi") {
                    if let Some(val) = event.get("axonius.identity.pmi") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.identity.pmi".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.identity.pmi", converted)?;
                    }
                }
                let _cond = { event.has_value("axonius.identity.recording") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def obj = ctx.axonius.identity.recording;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"def obj = ctx.axonius.identity.recording;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"fields\":[\"auto_delete_cmr\",\"auto_delete_cmr_days\",\"auto_recording\",\"cloud_recording\",\"host_pause_stop_recording\",\"local_recording\",\"record_audio_file\",\"record_gallery_view\",\"record_speaker_view\",\"recording_audio_transcript\",\"save_chat_text\",\"show_timestamp\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_recording_booleans",
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.recovery_question_set") {
                        if let Some(val) = event.get("axonius.identity.recovery_question_set") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.recovery_question_set".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.recovery_question_set", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_recovery_question_set_to_boolean",
                    )?;
                    event.remove("axonius.identity.recovery_question_set");
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
                let _cond = { event.has_value("axonius.identity.schedule_meeting") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def obj = ctx.axonius.identity.schedule_meeting;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"def obj = ctx.axonius.identity.schedule_meeting;\nfor (def f : params.fields) {\n  if (!obj.containsKey(f) || obj[f] == null) {\n    continue;\n  }\n  def v = obj[f];\n  if (v instanceof Boolean) {\n    continue;\n  }\n  def s = v.toString().toLowerCase();\n  if (s == '1' || s == 'true') {\n    obj[f] = true;\n  } else if (s == '' || s == '0' || s == 'false') {\n    obj[f] = false;\n  } else {\n    obj.remove(f);\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"fields\":[\"force_pmi_jbh_password\",\"host_video\",\"join_before_host\",\"participants_video\",\"pstn_password_protected\",\"require_password_for_instant_meetings\",\"require_password_for_pmi_meetings\",\"require_password_for_scheduled_meetings\",\"require_password_for_scheduling_new_meetings\",\"use_pmi_for_instant_meetings\",\"use_pmi_for_scheduled_meetings\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_schedule_meeting_booleans",
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
                let _cond = { event.has_value("axonius.identity.snow_full_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.snow_full_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("axonius.identity.status_changed")
                        && event.get_str("axonius.identity.status_changed") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("axonius.identity.status_changed")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.status_changed", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.status_changed".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_status_changed")?;
                        event.remove("axonius.identity.status_changed");
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.telephony.show_international_numbers_link")
                    {
                        if let Some(val) =
                            event.get("axonius.identity.telephony.show_international_numbers_link")
                        {
                            let converted =
                                convert_value(val, "boolean").map_err(|message| {
                                    TransformError::ParseError {
                path: "axonius.identity.telephony.show_international_numbers_link".into(),
                message,
                }
                                })?;
                            event.set(
                                "axonius.identity.telephony.show_international_numbers_link",
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
                        "convert_telephony_show_international_numbers_link_to_boolean",
                    )?;
                    event.remove("axonius.identity.telephony.show_international_numbers_link");
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
                    if event.has_value("axonius.identity.telephony.third_party_audio") {
                        if let Some(val) = event.get("axonius.identity.telephony.third_party_audio")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.telephony.third_party_audio".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.telephony.third_party_audio", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_telephony_third_party_audio_to_boolean",
                    )?;
                    event.remove("axonius.identity.telephony.third_party_audio");
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
                    if event.has_value("axonius.identity.tsp.call_out") {
                        if let Some(val) = event.get("axonius.identity.tsp.call_out") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.tsp.call_out".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.tsp.call_out", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tsp_call_out_to_boolean",
                    )?;
                    event.remove("axonius.identity.tsp.call_out");
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
                    if event.has_value("axonius.identity.tsp.show_international_numbers_link") {
                        if let Some(val) =
                            event.get("axonius.identity.tsp.show_international_numbers_link")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.tsp.show_international_numbers_link"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "axonius.identity.tsp.show_international_numbers_link",
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
                        "convert_tsp_show_international_numbers_link_to_boolean",
                    )?;
                    event.remove("axonius.identity.tsp.show_international_numbers_link");
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
                    if event.has_value("axonius.identity.u_vip") {
                        if let Some(val) = event.get("axonius.identity.u_vip") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.u_vip".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.u_vip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_u_vip_to_boolean",
                    )?;
                    event.remove("axonius.identity.u_vip");
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
                    event.has_value("axonius.identity.updated_on")
                        && event.get_str("axonius.identity.updated_on") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.updated_on") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("axonius.identity.updated_on", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.updated_on".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_updated_on")?;
                        event.remove("axonius.identity.updated_on");
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
                        .get("axonius.identity.user_apps")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: for (def item : ctx.axonius.identity.user_apps) {\n  for (def f : params.fields) {\n    if (!item.containsKey(f) || item[f] == null) {\n      continue;\n    }\n    def v = item[f];\n    if (v instanceof Boolean) {\n      continue;\n    }\n    def s = v.toString().toLowerCase();\n    if (s == '1' || s == 'true') {\n      item[f] = true;\n    } else if (s == '' || s == '0' || s == 'false') {\n      item[f] = false;\n    } else {\n      item.remove(f);\n    }\n  }\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"for (def item : ctx.axonius.identity.user_apps) {\n  for (def f : params.fields) {\n    if (!item.containsKey(f) || item[f] == null) {\n      continue;\n    }\n    def v = item[f];\n    if (v instanceof Boolean) {\n      continue;\n    }\n    def s = v.toString().toLowerCase();\n    if (s == '1' || s == 'true') {\n      item[f] = true;\n    } else if (s == '' || s == '0' || s == 'false') {\n      item[f] = false;\n    } else {\n      item.remove(f);\n    }\n  }\n}"#
                            ),
                            cached_params!(
                                "{\"fields\":[\"active_from_direct_adapter\",\"is_from_direct_adapter\",\"is_managed\",\"is_saas_application\",\"is_unmanaged_extension\",\"is_user_deleted\",\"is_user_external\",\"is_user_paid\",\"is_user_suspended\"]}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_convert_user_apps_booleans",
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
                        .get("axonius.identity.user_apps")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("axonius.identity.user_apps").cloned();
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
                                        event.get_as_string("_ingest._value.last_access")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &[
                                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                                "yyyy-MM-dd",
                                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                            ],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.last_access", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.last_access".into(),
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
                                        "date_user_apps_last_access",
                                    )?;
                                    event.remove("_ingest._value.last_access");
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
                                "axonius.identity.user_apps",
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
                    event.has_value("axonius.identity.user_created")
                        && event.get_str("axonius.identity.user_created") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("axonius.identity.user_created")
                        {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                    "yyyy-MM-dd",
                                    "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("axonius.identity.user_created", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "axonius.identity.user_created".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_user_created")?;
                        event.remove("axonius.identity.user_created");
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
                    .get("axonius.identity.user_created")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.user_factors")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("axonius.identity.user_factors").cloned();
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
                                        event.get_as_string("_ingest._value.created")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &[
                                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                                "yyyy-MM-dd",
                                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                            ],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.created", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.created".into(),
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
                                        "date_user_factors_created",
                                    )?;
                                    event.remove("_ingest._value.created");
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
                                "axonius.identity.user_factors",
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
                        .get("axonius.identity.user_factors")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_factors", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_enabled") {
                                if let Some(val) = event.get("_ingest._value.is_enabled") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_enabled".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_enabled", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_factors_is_enabled_to_boolean",
                            )?;
                            event.remove("_ingest._value.is_enabled");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_factors")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("axonius.identity.user_factors").cloned();
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
                                        event.get_as_string("_ingest._value.last_updated")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &[
                                                "EEE, dd MMM yyyy HH:mm:ss 'GMT'",
                                                "yyyy-MM-dd",
                                                "EEE,dd MMM yyyy HH:mm:ss 'GMT'",
                                            ],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.last_updated", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.last_updated".into(),
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
                                        "date_user_factors_last_updated",
                                    )?;
                                    event.remove("_ingest._value.last_updated");
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
                                "axonius.identity.user_factors",
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
                    .get("axonius.identity.user_full_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.full_name", v)?;
                }
                let _cond = { event.has_value("axonius.identity.user_full_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.user_full_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.user_is_password_enabled") {
                        if let Some(val) = event.get("axonius.identity.user_is_password_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.user_is_password_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.user_is_password_enabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_user_is_password_enabled_to_boolean",
                    )?;
                    event.remove("axonius.identity.user_is_password_enabled");
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
                let _cond = { event.has_value("axonius.identity.user_manager") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.user_manager")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("axonius.identity.user_manager_mail") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.user_manager_mail")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.user_permissions")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_permissions", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_admin") {
                                if let Some(val) = event.get("_ingest._value.is_admin") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_admin".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_admin", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_permissions_is_admin_to_boolean",
                            )?;
                            event.remove("_ingest._value.is_admin");
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
                let _cond = {
                    event
                        .get("axonius.identity.user_related_resources")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.user_related_resources", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.id") {
                                if let Some(val) = event.get("_ingest._value.id") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_user_related_resources_id_into_keyword",
                            )?;
                            event.remove("_ingest._value.id");
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
                if let Some(v) = event
                    .get("axonius.identity.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                let _cond = { event.has_value("axonius.identity.username") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("axonius.identity.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.identity.verified") {
                        if let Some(val) = event.get("axonius.identity.verified") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.identity.verified".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.identity.verified", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_verified_to_boolean",
                    )?;
                    event.remove("axonius.identity.verified");
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
                        .get("axonius.identity.associated_devices")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.associated_devices", |event| {
                        event.remove("_ingest._value.device_id");
                        event.remove("_ingest._value.device_model");
                        event.remove("_ingest._value.device_serial");
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("axonius.identity.breaches_data")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "axonius.identity.breaches_data", |event| {
                        event.remove("_ingest._value.added_date");
                        Ok(())
                    })?;
                }
                // End nested pipeline: "pipeline-user"
            }

            event.remove("axonius.identity.event.accurate_for_datetime");
            event.remove("axonius.identity.display_name");
            event.remove("axonius.identity.application_id");
            event.remove("axonius.identity.application_name");
            event.remove("axonius.identity.created_date");
            event.remove("axonius.identity.email");
            event.remove("axonius.identity.begins_on");
            event.remove("axonius.identity.expires_on");
            event.remove("axonius.identity.issuer.common_name");
            event.remove("axonius.identity.issuer.country_name");
            event.remove("axonius.identity.issuer.organization");
            event.remove("axonius.identity.serial_number");
            event.remove("axonius.identity.subject.common_name");
            event.remove("axonius.identity.subject.country_name");
            event.remove("axonius.identity.subject.locality");
            event.remove("axonius.identity.subject.organization");
            event.remove("axonius.identity.subject.state");
            event.remove("axonius.identity.aws_iam_identity_type");
            event.remove("axonius.identity.azure_account_id");
            event.remove("axonius.identity.cloud_provider");
            event.remove("axonius.identity.mail");
            event.remove("axonius.identity.user_created");
            event.remove("axonius.identity.user_full_name");
            event.remove("axonius.identity.username");

            event.remove("json");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
