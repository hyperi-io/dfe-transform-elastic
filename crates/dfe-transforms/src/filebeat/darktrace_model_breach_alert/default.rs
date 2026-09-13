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

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^(?P<log_syslog_appname>(?:[a-zA-Z]*))\\s*%{GREEDYDATA:message}$
                if !cached_grok_mapped!(
                    "^(?P<log_syslog_appname>(?:[a-zA-Z]*))\\s*%{GREEDYDATA:message}$",
                    [("log_syslog_appname", "log.syslog.appname")]
                )
                .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
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
                if let Some(v) = event.get("json.creationTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.model.phid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.pbid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.time") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.get_str("json.model.category").is_some_and(|s| {
                    ["critical", "suspicious"].contains(&s.to_lowercase().as_str())
                })
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.get_str("json.model.category").is_some_and(|s| {
                    ["compliance", "informational"].contains(&s.to_lowercase().as_str())
                })
            };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.get_str("event.kind") == Some("alert") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("threat")]))?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: for (component in ctx.json.triggeredComponents) { if (component?.metric?.label?.toLowerCase().contains('connection')) { ctx.event?.type?.add('connection'); if (ctx.event.category == null) { ctx.event.category = new ArrayList(); } ctx.event.category.add('network'); } }
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (component in ctx.json.triggeredComponents) { if (component?.metric?.label?.toLowerCase().contains('connection')) { ctx.event?.type?.add('connection'); if (ctx.event.category == null) { ctx.event.category = new ArrayList(); } ctx.event.category.add('network'); } }"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.aianalystData")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.aianalystData").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject = event.get("_ingest._value.related").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                    if event.remove("_ingest._value").is_none() {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path: "_ingest._value".into(),
                                                            },
                                                        );
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.related",
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
                                "json.aianalystData",
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
            }

            if event.has_value("json.aianalystData") {
                event.rename(
                    "json.aianalystData",
                    "darktrace.model_breach_alert.aianalyst_data",
                )?;
            }

            let _cond = { event.has_value("json.breachUrl") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(
                        event,
                        "json.breachUrl",
                        "darktrace.model_breach_alert.breach_url",
                        true,
                        false,
                    )? && event
                        .get_str("json.breachUrl")
                        .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "json.breachUrl".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    if event.remove("json.breachUrl").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.breachUrl".into(),
                        });
                    }
                    event.append(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.breach_url.original")
                    .cloned()
                {
                    event.set("event.url", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.commentCount") {
                    if let Some(val) = event.get("json.commentCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.commentCount".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.comment.count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            let _cond = { event.has_value("json.creationTime") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.creationTime") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("darktrace.model_breach_alert.creation_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.creationTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.creationTime");
                    event.append(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.creation_time")
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.devicescore") {
                    if let Some(val) = event.get("json.devicescore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.devicescore".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.device_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            if event.has_value("json.device.credentials") {
                event.rename(
                    "json.device.credentials",
                    "darktrace.model_breach_alert.device.credentials",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.device.did") {
                    if let Some(val) = event.get("json.device.did") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device.did".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.device.did", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                event.has_value("darktrace.model_breach_alert.device.did")
                    && event
                        .get_i64("darktrace.model_breach_alert.device.did")
                        .is_some_and(|n| n < 0)
            };
            if _cond {
                event.remove("darktrace.model_breach_alert.device.did");
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("darktrace.model_breach_alert.device.did") {
                    if let Some(val) = event.get("darktrace.model_breach_alert.device.did") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "darktrace.model_breach_alert.device.did".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("darktrace.model_breach_alert.device.did")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "darktrace.model_breach_alert.device.did".into(),
                    });
                }
                event.append(
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

            let _cond = { event.has_value("json.device.firstSeen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.device.firstSeen") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("darktrace.model_breach_alert.device.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.device.firstSeen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.device.firstseen");
                    event.append(
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.device.hostname") {
                    if let Some(val) = event.get("json.device.hostname") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device.hostname".into(),
                                message,
                            }
                        })?;
                        event.set("json.device._temp_.hostname_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.device.hostname").cloned() {
                        event.set("host.hostname", v)?;
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.device._temp_.hostname_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.device.hostname") {
                event.rename(
                    "json.device.hostname",
                    "darktrace.model_breach_alert.device.hostname",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.device.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.device.ip".into(),
                            message,
                        })?;
                    event.set("darktrace.model_breach_alert.device._temp_.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.device._temp_.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.device.ip6") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.device.ip6".into(),
                            message,
                        })?;
                    event.set("darktrace.model_breach_alert.device._temp_.ip6", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.device._temp_.ip6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.device.ip") {
                event.rename("json.device.ip", "darktrace.model_breach_alert.device.ip")?;
            }

            if event.has_value("json.device.ip6") {
                event.rename("json.device.ip6", "darktrace.model_breach_alert.device.ip6")?;
            }

            event.remove("darktrace.model_breach_alert.device._temp_");

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "host.ip", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.device.ips", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value._temp_.ip", converted)?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.device.ips", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value._temp_.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.ips").cloned();
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
                                        event.get_as_string("_ingest._value.timems")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.timems", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.timems".into(),
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
                                    event.remove("_ingest._value.timems");
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
                                "json.device.ips",
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
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.ips").cloned();
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
                                        event.get_as_string("_ingest._value.time")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &[
                                                "ISO8601",
                                                "UNIX_MS",
                                                "MMM dd HH:mm:ss",
                                                "yyyy-MM-dd HH:mm:ss",
                                            ],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.time".into(),
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
                                    event.remove("_ingest._value.time");
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
                                "json.device.ips",
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
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.ips").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.sid") {
                                        if let Some(val) = event.get("_ingest._value.sid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.sid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.sid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.sid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.sid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.ips",
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
            }

            let _cond = { event.get("json.device.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.device.ips", |event| {
                        event.remove("_ingest._value._temp_");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.device.ips") {
                event.rename("json.device.ips", "darktrace.model_breach_alert.device.ips")?;
            }

            let _cond = { event.has_value("json.device.lastSeen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.device.lastSeen") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("darktrace.model_breach_alert.device.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.device.lastSeen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.device.lastseen");
                    event.append(
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

            if event.has_value("json.device.macaddress") {
                gsub_field(
                    event,
                    "json.device.macaddress",
                    "darktrace.model_breach_alert.device.mac_address",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("darktrace.model_breach_alert.device.mac_address") {
                map_strings(
                    event,
                    "darktrace.model_breach_alert.device.mac_address",
                    "darktrace.model_breach_alert.device.mac_address",
                    str::to_uppercase,
                )?;
            }

            let _cond = {
                event.has_value("darktrace.model_breach_alert.device.mac_address")
                    && event.get_str("darktrace.model_breach_alert.device.mac_address") != Some("")
            };
            if _cond {
                event.append(
                    "host.mac",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.device.mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.device.sid") {
                    if let Some(val) = event.get("json.device.sid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device.sid".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.device.sid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.tid") {
                                        if let Some(val) = event.get("_ingest._value.tid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.tid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.tid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.tid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.tid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.thid") {
                                        if let Some(val) = event.get("_ingest._value.thid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.thid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.thid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.thid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.thid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.expiry") {
                                        if let Some(val) = event.get("_ingest._value.expiry") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.expiry".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.expiry", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.expiry").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.expiry".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.restricted") {
                                        if let Some(val) = event.get("_ingest._value.restricted") {
                                            let converted = convert_value(val, "boolean").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.restricted".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.restricted", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.restricted").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.restricted".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.isReferenced") {
                                        if let Some(val) = event.get("_ingest._value.isReferenced")
                                        {
                                            let converted = convert_value(val, "boolean").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.isReferenced".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.is_referenced", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.isReferenced").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.isReferenced".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.data.auto") {
                                        if let Some(val) = event.get("_ingest._value.data.auto") {
                                            let converted = convert_value(val, "boolean").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.data.auto".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.data.auto", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.data.auto").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.data.auto".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.device.tags").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.data.color") {
                                        if let Some(val) = event.get("_ingest._value.data.color") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.data.color".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.data.color", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.color").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.color".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.device.tags",
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
            }

            let _cond = { event.get("json.device.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.device.tags", |event| {
                        event.remove("_ingest._value.isReferenced");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.device.tags") {
                event.rename(
                    "json.device.tags",
                    "darktrace.model_breach_alert.device.tags",
                )?;
            }

            if event.has_value("json.device.typelabel") {
                event.rename(
                    "json.device.typelabel",
                    "darktrace.model_breach_alert.device.type_label",
                )?;
            }

            if event.has_value("json.device.typename") {
                event.rename(
                    "json.device.typename",
                    "darktrace.model_breach_alert.device.type_name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.device.type_name")
                    .cloned()
                {
                    event.set("host.type", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.device.vendor") {
                event.rename(
                    "json.device.vendor",
                    "darktrace.model_breach_alert.device.vendor",
                )?;
            }

            let _cond = {
                event
                    .get("json.mitreTechniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitreTechniques", |event| {
                        if event.has_value("_ingest._value.techniqueID") {
                            event.rename("_ingest._value.techniqueID", "_ingest._value.id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.mitreTechniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.mitreTechniques", |event| {
                        if event.has_value("_ingest._value.technique") {
                            event.rename("_ingest._value.technique", "_ingest._value.name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.mitreTechniques") {
                event.rename(
                    "json.mitreTechniques",
                    "darktrace.model_breach_alert.mitre_techniques",
                )?;
            }

            let _cond = {
                event
                    .get("darktrace.model_breach_alert.mitre_techniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "darktrace.model_breach_alert.mitre_techniques",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "threat.technique.id",
                                    json!(
                                        event
                                            .get("_ingest._value.id")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("darktrace.model_breach_alert.mitre_techniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "darktrace.model_breach_alert.mitre_techniques",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "threat.technique.name",
                                    json!(
                                        event
                                            .get("_ingest._value.name")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.model.actions.antigena.action") {
                event.rename(
                    "json.model.actions.antigena.action",
                    "darktrace.model_breach_alert.model.actions.antigena.action",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.actions.antigena.action")
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.actions.antigena.duration") {
                    if let Some(val) = event.get("json.model.actions.antigena.duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.antigena.duration".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.antigena.duration",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.antigena.confirm") {
                    if let Some(val) = event.get("json.model.actions.antigena.confirm") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.antigena.confirm".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.actions.antigena.is_confirm_by_human_operator", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.antigena.threshold") {
                    if let Some(val) = event.get("json.model.actions.antigena.threshold") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.antigena.threshold".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.antigena.threshold",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.alert") {
                    if let Some(val) = event.get("json.model.actions.alert") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.alert".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.is_alerting",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.breach") {
                    if let Some(val) = event.get("json.model.actions.breach") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.breach".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.is_breach",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.setPriority") {
                    if let Some(val) = event.get("json.model.actions.setPriority") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.setPriority".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.is_priority_set",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.setTag") {
                    if let Some(val) = event.get("json.model.actions.setTag") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.setTag".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.is_tag_set",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.setType") {
                    if let Some(val) = event.get("json.model.actions.setType") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.setType".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.is_type_set",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.actions.model") {
                    if let Some(val) = event.get("json.model.actions.model") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.actions.model".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.actions.model",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.activeTimes.version") {
                    if let Some(val) = event.get("json.model.activeTimes.version") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.activeTimes.version".into(),
                                message,
                            }
                        })?;
                        event.set("json.model.activeTimes.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.model.activeTimes.version").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.activeTimes.version".into(),
                    });
                }
                event.append(
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

            if event.has_value("json.model.activeTimes") {
                event.rename(
                    "json.model.activeTimes",
                    "darktrace.model_breach_alert.model.active_times",
                )?;
            }

            if event.has_value("json.model.behaviour") {
                event.rename(
                    "json.model.behaviour",
                    "darktrace.model_breach_alert.model.behaviour",
                )?;
            }

            if event.has_value("json.model.category") {
                event.rename(
                    "json.model.category",
                    "darktrace.model_breach_alert.model.category",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.category")
                    .cloned()
                {
                    event.set("rule.category", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.model.created.by") {
                event.rename(
                    "json.model.created.by",
                    "darktrace.model_breach_alert.model.created.by",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.model.created.by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("darktrace.model_breach_alert.model.created.by") };
            if _cond {
                event.append_unique(
                    "rule.author",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.model.created.by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("json.model.defeats")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.model.defeats", |event| {
                        if event.has_value("_ingest._value.defeatID") {
                            if let Some(val) = event.get("_ingest._value.defeatID") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.defeatID".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.model.defeats")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.model.defeats", |event| {
                        event.remove("_ingest._value.defeatID");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.model.defeats")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.model.defeats").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject = event.get("_ingest._value").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                    if event.has_value("_ingest._value.defeatID") {
                                                        if let Some(val) =
                                                            event.get("_ingest._value.defeatID")
                                                        {
                                                            let converted =
                                                                convert_value(val, "string")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.defeatID".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.id",
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
                                                    if event
                                                        .remove("_ingest._value.defeatID")
                                                        .is_none()
                                                    {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path: "_ingest._value.defeatID"
                                                                    .into(),
                                                            },
                                                        );
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value",
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
                                "json.model.defeats",
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
            }

            let _cond = {
                event
                    .get("json.model.defeats")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.model.defeats", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                event.remove("_ingest._value.defeatID");
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.model.defeats") {
                event.rename(
                    "json.model.defeats",
                    "darktrace.model_breach_alert.model.defeats",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.delay") {
                    if let Some(val) = event.get("json.model.delay") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.delay".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.delay", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            if event.has_value("json.model.description") {
                event.rename(
                    "json.model.description",
                    "darktrace.model_breach_alert.model.description",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.description")
                    .cloned()
                {
                    event.set("rule.description", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.model.edited.by") {
                event.rename(
                    "json.model.edited.by",
                    "darktrace.model_breach_alert.model.edited.by",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("darktrace.model_breach_alert.model.edited.by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.compliance") {
                    if let Some(val) = event.get("json.model.compliance") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.compliance".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.in_compliance_behavior_category",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.interval") {
                    if let Some(val) = event.get("json.model.interval") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.interval".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.interval", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.active") {
                    if let Some(val) = event.get("json.model.active") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.active".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.is_active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.autoSuppress") {
                    if let Some(val) = event.get("json.model.autoSuppress") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.autoSuppress".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.is_auto_suppress",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.autoUpdatable") {
                    if let Some(val) = event.get("json.model.autoUpdatable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.autoUpdatable".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.is_auto_updatable",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.autoUpdate") {
                    if let Some(val) = event.get("json.model.autoUpdate") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.autoUpdate".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.is_auto_update",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.sequenced") {
                    if let Some(val) = event.get("json.model.sequenced") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.sequenced".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.is_sequenced", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.sharedEndpoints") {
                    if let Some(val) = event.get("json.model.sharedEndpoints") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.sharedEndpoints".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.model_breach_alert.model.is_shared_endpoints",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                    .get("json.model.logic.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def data = ctx.json.model.logic.data; if (ctx.json.model.logic?.type != null) { if (['componentList', 'weightedComponentList'].contains(ctx.json.model.logic?.type)) { ctx[\"json\"][\"model\"][\"logic\"][params.get(ctx.json.model.logic?.type)] = data; } else { ctx[\"json\"][\"model\"][\"logic\"][\"data_\" + ctx.json.model.logic?.type] = data; } } ctx.json.model.logic.remove(\"data\");
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def data = ctx.json.model.logic.data; if (ctx.json.model.logic?.type != null) { if (['componentList', 'weightedComponentList'].contains(ctx.json.model.logic?.type)) { ctx[\"json\"][\"model\"][\"logic\"][params.get(ctx.json.model.logic?.type)] = data; } else { ctx[\"json\"][\"model\"][\"logic\"][\"data_\" + ctx.json.model.logic?.type] = data; } } ctx.json.model.logic.remove(\"data\");"#
                        ),
                        cached_params!(
                            "{\"componentList\":\"data_component_list\",\"weightedComponentList\":\"data_weighted_component_list\"}"
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.logic.data_component_list") {
                    if let Some(val) = event.get("json.model.logic.data_component_list") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.logic.data_component_list".into(),
                                message,
                            }
                        })?;
                        event.set("json.model.logic.data_component_list", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("json.model.logic.data_component_list")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.logic.data_component_list".into(),
                    });
                }
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.logic.data_weighted_component_list.cid") {
                    if let Some(val) =
                        event.get("json.model.logic.data_weighted_component_list.cid")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.logic.data_weighted_component_list.cid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "json.model.logic.data_weighted_component_list.cid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("json.model.logic.data_weighted_component_list.cid")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.logic.data_weighted_component_list.cid".into(),
                    });
                }
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.logic.data_weighted_component_list.weight") {
                    if let Some(val) =
                        event.get("json.model.logic.data_weighted_component_list.weight")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.logic.data_weighted_component_list.weight".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "json.model.logic.data_weighted_component_list.weight",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("json.model.logic.data_weighted_component_list.weight")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.logic.data_weighted_component_list.weight".into(),
                    });
                }
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.logic.version") {
                    if let Some(val) = event.get("json.model.logic.version") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.logic.version".into(),
                                message,
                            }
                        })?;
                        event.set("json.model.logic.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.model.logic.version").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.logic.version".into(),
                    });
                }
                event.append(
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
                if event.has_value("json.model.logic.targetScore") {
                    if let Some(val) = event.get("json.model.logic.targetScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.logic.targetScore".into(),
                                message,
                            }
                        })?;
                        event.set("json.model.logic.target_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            event.remove("json.model.logic.targetScore");

            if event.has_value("json.model.logic") {
                event.rename(
                    "json.model.logic",
                    "darktrace.model_breach_alert.model.logic",
                )?;
            }

            let _cond = { event.has_value("json.model.modified") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.model.modified") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "UNIX_MS",
                                "MMM dd HH:mm:ss",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("darktrace.model_breach_alert.model.modified", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.model.modified".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.model.modified");
                    event.append(
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

            if event.has_value("json.model.name") {
                event.rename("json.model.name", "darktrace.model_breach_alert.model.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.name")
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.phid") {
                    if let Some(val) = event.get("json.model.phid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.phid".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.phid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.pid") {
                    if let Some(val) = event.get("json.model.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.pid".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.priority") {
                    if let Some(val) = event.get("json.model.priority") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.priority".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.priority", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.model.priority").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.priority".into(),
                    });
                }
                event.append(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.priority")
                    .cloned()
                {
                    event.set("event.severity", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.model.tags") {
                event.rename("json.model.tags", "darktrace.model_breach_alert.model.tags")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.tags")
                    .cloned()
                {
                    event.set("rule.ruleset", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.throttle") {
                    if let Some(val) = event.get("json.model.throttle") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.throttle".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.throttle", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.model.userID") {
                    if let Some(val) = event.get("json.model.userID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.userID".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.userid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            if event.has_value("json.model.uuid") {
                event.rename("json.model.uuid", "darktrace.model_breach_alert.model.uuid")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.model_breach_alert.model.uuid")
                    .cloned()
                {
                    event.set("rule.uuid", v)?;
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.model.version") {
                    if let Some(val) = event.get("json.model.version") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.model.version".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.model.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.model.version").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.model.version".into(),
                    });
                }
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("darktrace.model_breach_alert.model.version") {
                    if let Some(val) = event.get("darktrace.model_breach_alert.model.version") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "darktrace.model_breach_alert.model.version".into(),
                                message,
                            }
                        })?;
                        event.set("rule.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("darktrace.model_breach_alert.model.version")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "darktrace.model_breach_alert.model.version".into(),
                    });
                }
                event.append(
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
                if event.has_value("json.pbscore") {
                    if let Some(val) = event.get("json.pbscore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pbscore".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.pb_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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
                if event.has_value("json.pbid") {
                    if let Some(val) = event.get("json.pbid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pbid".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.pbid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.score") {
                    if let Some(val) = event.get("json.score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.score".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.model_breach_alert.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.score").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.score".into(),
                    });
                }
                event.append(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.model_breach_alert.score").cloned() {
                    event.set("event.risk_score", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("event.risk_score") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: def normalizedRiskScore = ctx.event.risk_score * 100.0; ctx.event.risk_score_norm = normalizedRiskScore;
                scale_field(
                    event,
                    &ScaleField::new(
                        "event.risk_score",
                        "event.risk_score_norm",
                        Factor::Double(100.0),
                    ),
                );
            }

            let _cond = { event.has_value("json.time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("darktrace.model_breach_alert.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.time");
                    event.append(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.model_breach_alert.time").cloned() {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.acknowledged") {
                event.rename(
                    "json.acknowledged",
                    "darktrace.model_breach_alert.acknowledged",
                )?;
            }

            let _cond = {
                event
                    .get("darktrace.model_breach_alert.acknowledged")
                    .is_some_and(|v| v.is_object())
                    && event.has_value("darktrace.model_breach_alert.acknowledged.time")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("darktrace.model_breach_alert.acknowledged.time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("darktrace.model_breach_alert.acknowledged.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "darktrace.model_breach_alert.acknowledged.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_acknowledged_time")?;
                    event.remove("darktrace.model_breach_alert.acknowledged.time");
                    event.append(
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.darktrace = ctx.darktrace ?: [:];\nctx.darktrace.model_breach_alert = ctx.darktrace?.model_breach_alert ?: [:];\nif (ctx.darktrace?.model_breach_alert?.acknowledged == null) {\n  ctx.darktrace.model_breach_alert.is_acknowledged = false;\n  return;\n}\nif (!(ctx.darktrace.model_breach_alert.acknowledged instanceof Map)) {\n  // It appears that some versions of the data from Darktrace\n  // have the value at acknowledged as a boolean. Don't handle\n  // that here. Rename it to is_acknowledged below.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.acknowledged.time == null) {\n  // No time, so be non-commital.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.time == null) {\n  // Assume any time noted in json.acknowledged.time\n  // is in the past.\n  ctx.darktrace.model_breach_alert.is_acknowledged = true;\n  return;\n}\ndef time = ctx.darktrace.model_breach_alert.time;\ndef acknowledged = ctx.darktrace.model_breach_alert.acknowledged.time;\nctx.darktrace.model_breach_alert.is_acknowledged = ZonedDateTime.parse(acknowledged).isBefore(ZonedDateTime.parse(time));\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.darktrace = ctx.darktrace ?: [:];\nctx.darktrace.model_breach_alert = ctx.darktrace?.model_breach_alert ?: [:];\nif (ctx.darktrace?.model_breach_alert?.acknowledged == null) {\n  ctx.darktrace.model_breach_alert.is_acknowledged = false;\n  return;\n}\nif (!(ctx.darktrace.model_breach_alert.acknowledged instanceof Map)) {\n  // It appears that some versions of the data from Darktrace\n  // have the value at acknowledged as a boolean. Don't handle\n  // that here. Rename it to is_acknowledged below.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.acknowledged.time == null) {\n  // No time, so be non-commital.\n  return;\n}\nif (ctx.darktrace.model_breach_alert.time == null) {\n  // Assume any time noted in json.acknowledged.time\n  // is in the past.\n  ctx.darktrace.model_breach_alert.is_acknowledged = true;\n  return;\n}\ndef time = ctx.darktrace.model_breach_alert.time;\ndef acknowledged = ctx.darktrace.model_breach_alert.acknowledged.time;\nctx.darktrace.model_breach_alert.is_acknowledged = ZonedDateTime.parse(acknowledged).isBefore(ZonedDateTime.parse(time));\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "handle_acknowledged_status",
                )?;
                event.append(
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
                    .get("darktrace.model_breach_alert.acknowledged")
                    .is_some_and(|v| v.is_boolean())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "darktrace.model_breach_alert.acknowledged",
                        "darktrace.model_breach_alert.is_acknowledged",
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("darktrace.model_breach_alert.acknowledged")
                    .is_some_and(|v| v.is_object())
                    && event.has_value("darktrace.model_breach_alert.acknowledged.username")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("darktrace.model_breach_alert.acknowledged.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.cbid") {
                                        if let Some(val) = event.get("_ingest._value.cbid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.cbid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.cbid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.cbid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.cbid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.chid") {
                                        if let Some(val) = event.get("_ingest._value.chid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.chid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.chid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.chid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.chid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.cid") {
                                        if let Some(val) = event.get("_ingest._value.cid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.cid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.cid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.cid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.cid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.interval") {
                                        if let Some(val) = event.get("_ingest._value.interval") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.interval".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.interval", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.interval").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.interval".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: for (component in ctx.json.triggeredComponents) { component.logic.data = component?.logic?.data.toString(); }
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (component in ctx.json.triggeredComponents) { component.logic.data = component?.logic?.data.toString(); }"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.metric.mlid") {
                                        if let Some(val) = event.get("_ingest._value.metric.mlid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.metric.mlid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.metric.mlid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.metric.mlid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.metric.mlid".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                    if event.remove("_ingest._value.size").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.size".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.threshold") {
                                        if let Some(val) = event.get("_ingest._value.threshold") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.threshold".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.threshold", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.threshold").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.threshold".into(),
                                        });
                                    }
                                    event.append(
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                        event.get_as_string("_ingest._value.time")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.time".into(),
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
                                    event.remove("_ingest._value.time");
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.triggeredComponents", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "event.start",
                                json!(
                                    event
                                        .get("_ingest._value.time")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                    if event.has_value("_ingest._value.cfid") {
                                                        if let Some(val) =
                                                            event.get("_ingest._value.cfid")
                                                        {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.cfid".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.cfid",
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
                                                    if event.remove("_ingest._value.cfid").is_none()
                                                    {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path: "_ingest._value.cfid".into(),
                                                            },
                                                        );
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.triggeredComponents", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.triggeredFilters", |event| {
                                if event.has_value("_ingest._value.comparatorType") {
                                    event.rename(
                                        "_ingest._value.comparatorType",
                                        "_ingest._value.comparator_type",
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.triggeredComponents", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.triggeredFilters", |event| {
                                if event.has_value("_ingest._value.filterType") {
                                    event.rename(
                                        "_ingest._value.filterType",
                                        "_ingest._value.filter_type",
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.data.auto",
                                                    ) {
                                                        if let Some(val) = event.get(
                                                            "_ingest._value.trigger.tag.data.auto",
                                                        ) {
                                                            let converted =
                                                                convert_value(val, "boolean")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.data.auto".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set("_ingest._value.trigger.tag.data.auto", converted)?;
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
                                                    if event
                                                        .remove(
                                                            "_ingest._value.trigger.tag.data.auto",
                                                        )
                                                        .is_none()
                                                    {
                                                        return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.data.auto".into() });
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.data.color",
                                                    ) {
                                                        if let Some(val) = event.get(
                                                            "_ingest._value.trigger.tag.data.color",
                                                        ) {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.data.color".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set("_ingest._value.trigger.tag.data.color", converted)?;
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
                                                    if event
                                                        .remove(
                                                            "_ingest._value.trigger.tag.data.color",
                                                        )
                                                        .is_none()
                                                    {
                                                        return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.data.color".into() });
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.expiry",
                                                    ) {
                                                        if let Some(val) = event.get(
                                                            "_ingest._value.trigger.tag.expiry",
                                                        ) {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.expiry".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.trigger.tag.expiry",
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
                                                    if event
                                                        .remove("_ingest._value.trigger.tag.expiry")
                                                        .is_none()
                                                    {
                                                        return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.expiry".into() });
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.isReferenced",
                                                    ) {
                                                        if let Some(val) = event.get("_ingest._value.trigger.tag.isReferenced") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.isReferenced".into(),
                            message,
                            })?;
                            event.set("_ingest._value.trigger.tag.is_referenced", converted)?;
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
                                                    if event.remove("_ingest._value.trigger.tag.isReferenced").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.isReferenced".into() });
                            }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.restricted",
                                                    ) {
                                                        if let Some(val) = event.get(
                                                            "_ingest._value.trigger.tag.restricted",
                                                        ) {
                                                            let converted =
                                                                convert_value(val, "boolean")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.restricted".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set("_ingest._value.trigger.tag.restricted", converted)?;
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
                                                    if event
                                                        .remove(
                                                            "_ingest._value.trigger.tag.restricted",
                                                        )
                                                        .is_none()
                                                    {
                                                        return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.restricted".into() });
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                        "_ingest._value.trigger.tag.thid",
                                                    ) {
                                                        if let Some(val) = event
                                                            .get("_ingest._value.trigger.tag.thid")
                                                        {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.thid".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.trigger.tag.thid",
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
                                                    if event
                                                        .remove("_ingest._value.trigger.tag.thid")
                                                        .is_none()
                                                    {
                                                        return Err(TransformError::FieldNotFound { path: "_ingest._value.trigger.tag.thid".into() });
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.triggeredComponents").cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.triggeredFilters").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
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
                                                    if event
                                                        .has_value("_ingest._value.trigger.tag.tid")
                                                    {
                                                        if let Some(val) = event
                                                            .get("_ingest._value.trigger.tag.tid")
                                                        {
                                                            let converted =
                                                                convert_value(val, "long")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                            path: "_ingest._value.trigger.tag.tid".into(),
                            message,
                            }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.trigger.tag.tid",
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
                                                    if event
                                                        .remove("_ingest._value.trigger.tag.tid")
                                                        .is_none()
                                                    {
                                                        return Err(
                                                            TransformError::FieldNotFound {
                                                                path:
                                                                    "_ingest._value.trigger.tag.tid"
                                                                        .into(),
                                                            },
                                                        );
                                                    }
                                                    event.append(
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
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
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
                                                "_ingest._value.triggeredFilters",
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
                                "json.triggeredComponents",
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
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.triggeredComponents", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.triggeredFilters", |event| {
                                event.remove("_ingest._value.trigger.tag.isReferenced");
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.triggeredComponents")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.triggeredComponents", |event| {
                        if event.has_value("_ingest._value.triggeredFilters") {
                            event.rename(
                                "_ingest._value.triggeredFilters",
                                "_ingest._value.triggered_filters",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.triggeredComponents") {
                event.rename(
                    "json.triggeredComponents",
                    "darktrace.model_breach_alert.triggered_components",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("darktrace.model_breach_alert.time");
                    event.remove("darktrace.model_breach_alert.model.actions.antigena.action");
                    event.remove("darktrace.model_breach_alert.creation_time");
                    event.remove("darktrace.model_breach_alert.score");
                    event.remove("darktrace.model_breach_alert.model.priority");
                    event.remove("darktrace.model_breach_alert.device.did");
                    event.remove("darktrace.model_breach_alert.device.mac_address");
                    event.remove("darktrace.model_breach_alert.device.type_name");
                    event.remove("darktrace.model_breach_alert.model.created.by");
                    event.remove("darktrace.model_breach_alert.model.category");
                    event.remove("darktrace.model_breach_alert.model.description");
                    event.remove("darktrace.model_breach_alert.model.name");
                    event.remove("darktrace.model_breach_alert.model.tags");
                    event.remove("darktrace.model_breach_alert.model.uuid");
                    event.remove("darktrace.model_breach_alert.model.version");
                    event.remove("darktrace.model_breach_alert.mitre_techniques");
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("darktrace.model_breach_alert.triggered_components")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "darktrace.model_breach_alert.triggered_components",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.time");
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
