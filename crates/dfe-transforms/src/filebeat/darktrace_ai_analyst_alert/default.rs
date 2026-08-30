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
                let _ = cached_grok_mapped!(
                    "^(?P<log_syslog_appname>(?:[a-zA-Z]*))\\s*%{GREEDYDATA:message}$",
                    [("log_syslog_appname", "log.syslog.appname")]
                )
                .extract_into(&input, event)?;
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
                if let Some(v) = event.get("json.activityId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.createdAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.currentGroup") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.get_str("json.category").is_some_and(|s| {
                    ["critical", "suspicious"].contains(&s.to_lowercase().as_str())
                })
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.get_str("json.category").is_some_and(|s| {
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

            // SKIPPED: condition not transpiled: ctx.event?.category instanceof Collection && ctx.event.category.contains('threat')
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.type", Value::Array(vec![json!("indicator")]))?;
            }

            if event.has_value("json.activityId") {
                event.rename("json.activityId", "darktrace.ai_analyst_alert.activity_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.aiaScore") {
                    if let Some(val) = event.get("json.aiaScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.aiaScore".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.aia_score", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.aia_score").cloned() {
                    event.set("event.risk_score", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.aia_score").cloned() {
                    event.set("event.risk_score_norm", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get("json.attackPhases").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.attackPhases").cloned();
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
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value".into(),
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
                                "json.attackPhases",
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

            if event.has_value("json.attackPhases") {
                event.rename(
                    "json.attackPhases",
                    "darktrace.ai_analyst_alert.attack_phases",
                )?;
            }

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.breachDevices").cloned();
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
                                    if event.has_value("_ingest._value.did") {
                                        if let Some(val) = event.get("_ingest._value.did") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.did".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.did", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.did").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.did".into(),
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
                                "json.breachDevices",
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
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "host.id",
                                json!(
                                    event
                                        .get("_ingest._value.did")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.id") {
                    if let Some(val) = event.get("host.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "host.id".into(),
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
                if event.remove("host.id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "host.id".into(),
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

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.hostname") {
                                if let Some(val) = event.get("_ingest._value.hostname") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.hostname".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value._temp_.hostname_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "host.hostname",
                                    json!(
                                        event
                                            .get("_ingest._value.hostname")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value._temp_.hostname_ip")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("host.hostname").cloned() {
                    event.set("related.hosts", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.identifier") {
                                if let Some(val) = event.get("_ingest._value.identifier") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.identifier".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value._temp_.identifier_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "host.name",
                                    json!(
                                        event
                                            .get("_ingest._value.identifier")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value._temp_.identifier_ip")
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

            let _cond = { event.get("host.name").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "host.name", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hosts",
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

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
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

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "host.ip",
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

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        if event.has_value("_ingest._value.mac") {
                            gsub_field(
                                event,
                                "_ingest._value.mac",
                                "_ingest._value.mac_address",
                                cached_regex!("[:.]"),
                                "-",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        if event.has_value("_ingest._value.mac_address") {
                            map_strings(
                                event,
                                "_ingest._value.mac_address",
                                "_ingest._value.mac_address",
                                str::to_uppercase,
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "host.mac",
                                json!(
                                    event
                                        .get("_ingest._value.mac_address")
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
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.breachDevices").cloned();
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
                                "json.breachDevices",
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
                    .get("json.breachDevices")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.breachDevices", |event| {
                        event.remove("_ingest._value._temp_");
                        event.remove("_ingest._value.mac");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.breachDevices") {
                event.rename(
                    "json.breachDevices",
                    "darktrace.ai_analyst_alert.breach_devices",
                )?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "darktrace.ai_analyst_alert.category")?;
            }

            if event.has_value("json.children") {
                event.rename("json.children", "darktrace.ai_analyst_alert.children")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.children").cloned() {
                    event.set("threat.enrichments.matched.id", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.createdAt") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("darktrace.ai_analyst_alert.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.createdAt");
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
                if let Some(v) = event.get("darktrace.ai_analyst_alert.created_at").cloned() {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.currentGroup") {
                event.rename(
                    "json.currentGroup",
                    "darktrace.ai_analyst_alert.current_group",
                )?;
            }

            let _cond = { event.has_value("darktrace.ai_analyst_alert.current_group") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("darktrace.ai_analyst_alert.current_group")
                        .cloned()
                    {
                        event.set("threat.group.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // ignore_failure: true
                                                    let _ = (|| -> Result<()> {
                                                        if let Some(val) =
                                                            event.get("_ingest._value.ip")
                                                        {
                                                            let converted = convert_value(
                                                                val, "ip",
                                                            )
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.ip"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                            event.set(
                                                                "_ingest._value._temp_.ip",
                                                                converted,
                                                            )?;
                                                        }
                                                        Ok(())
                                                    })(
                                                    );
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // ignore_failure: true
                                                    let _ = (|| -> Result<()> {
                                                        event.append_unique(
                                                            "related.ip",
                                                            json!(
                                                                event
                                                                    .get("_ingest._value._temp_.ip")
                                                                    .map_or_else(
                                                                        String::new,
                                                                        template_to_string
                                                                    )
                                                            ),
                                                        )?;
                                                        Ok(())
                                                    })(
                                                    );
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // on_failure: 1 handler(s)
                                                    if let Err(err) = (|| -> Result<()> {
                                                        if event
                                                            .has_value("_ingest._value.hostname")
                                                        {
                                                            if let Some(val) =
                                                                event.get("_ingest._value.hostname")
                                                            {
                                                                let converted = convert_value(
                                                                    val, "ip",
                                                                )
                                                                .map_err(|message| {
                                                                    TransformError::ParseError {
                    path: "_ingest._value.hostname".into(),
                    message,
                    }
                                                                })?;
                                                                event.set("_ingest._value._temp_.hostname_ip", converted)?;
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
                                                        // ignore_failure: true
                                                        let _ = (|| -> Result<()> {
                                                            event.append_unique("related.hosts", json!(event.get("_ingest._value.hostname").map_or_else(String::new, template_to_string)))?;
                                                            Ok(())
                                                        })(
                                                        );
                                                        event.remove("_ingest.on_failure_message");
                                                        event.remove(
                                                            "_ingest.on_failure_processor_type",
                                                        );
                                                        event.remove(
                                                            "_ingest.on_failure_processor_tag",
                                                        );
                                                        if event
                                                            .get_object("_ingest")
                                                            .is_some_and(|m| m.is_empty())
                                                        {
                                                            event.remove("_ingest");
                                                        }
                                                    }
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // ignore_failure: true
                                                    let _ = (|| -> Result<()> {
                                                        event.append_unique("related.ip", json!(event.get("_ingest._value._temp_.hostname_ip").map_or_else(String::new, template_to_string)))?;
                                                        Ok(())
                                                    })(
                                                    );
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // on_failure: 1 handler(s)
                                                    if let Err(err) = (|| -> Result<()> {
                                                        if event
                                                            .has_value("_ingest._value.identifier")
                                                        {
                                                            if let Some(val) = event
                                                                .get("_ingest._value.identifier")
                                                            {
                                                                let converted = convert_value(
                                                                    val, "ip",
                                                                )
                                                                .map_err(|message| {
                                                                    TransformError::ParseError {
                    path: "_ingest._value.identifier".into(),
                    message,
                    }
                                                                })?;
                                                                event.set("_ingest._value._temp_.identifier_ip", converted)?;
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
                                                        // ignore_failure: true
                                                        let _ = (|| -> Result<()> {
                                                            event.append_unique("related.hosts", json!(event.get("_ingest._value.identifier").map_or_else(String::new, template_to_string)))?;
                                                            Ok(())
                                                        })(
                                                        );
                                                        event.remove("_ingest.on_failure_message");
                                                        event.remove(
                                                            "_ingest.on_failure_processor_type",
                                                        );
                                                        event.remove(
                                                            "_ingest.on_failure_processor_tag",
                                                        );
                                                        if event
                                                            .get_object("_ingest")
                                                            .is_some_and(|m| m.is_empty())
                                                        {
                                                            event.remove("_ingest");
                                                        }
                                                    }
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    // ignore_failure: true
                                                    let _ = (|| -> Result<()> {
                                                        event.append_unique("related.ip", json!(event.get("_ingest._value._temp_.identifier_ip").map_or_else(String::new, template_to_string)))?;
                                                        Ok(())
                                                    })(
                                                    );
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    if event.has_value("_ingest._value.mac") {
                                                        gsub_field(
                                                            event,
                                                            "_ingest._value.mac",
                                                            "_ingest._value.mac_address",
                                                            cached_regex!("[:.]"),
                                                            "-",
                                                        )?;
                                                    }
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    if event.has_value("_ingest._value.mac_address")
                                                    {
                                                        map_strings(
                                                            event,
                                                            "_ingest._value.mac_address",
                                                            "_ingest._value.mac_address",
                                                            str::to_uppercase,
                                                        )?;
                                                    }
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.details").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.details", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.contents", |event| {
                                        // ignore_failure: true
                                        let _ = (|| -> Result<()> {
                                            foreach_array(
                                                event,
                                                "_ingest._value.values",
                                                |event| {
                                                    event.remove("_ingest._value._temp_");
                                                    event.remove("_ingest._value.mac");
                                                    Ok(())
                                                },
                                            )?;
                                            Ok(())
                                        })();
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.details") {
                event.rename("json.details", "darktrace.ai_analyst_alert.details")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.groupByActivity") {
                    if let Some(val) = event.get("json.groupByActivity") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.groupByActivity".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.group_by_activity", converted)?;
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
                !event.has_value("threat.group.id")
                    && event.get_bool("darktrace.ai_analyst_alert.group_by_activity") == Some(true)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("darktrace.ai_analyst_alert.activity_id").cloned() {
                        event.set("threat.group.id", v)?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.groupCategory") {
                event.rename(
                    "json.groupCategory",
                    "darktrace.ai_analyst_alert.group_category",
                )?;
            }

            if event.has_value("json.groupPreviousGroups") {
                event.rename(
                    "json.groupPreviousGroups",
                    "darktrace.ai_analyst_alert.group_previous_groups",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.groupScore") {
                    if let Some(val) = event.get("json.groupScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.groupScore".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.group_score", converted)?;
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

            if event.has_value("json.groupingIds") {
                event.rename(
                    "json.groupingIds",
                    "darktrace.ai_analyst_alert.grouping_ids",
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "darktrace.ai_analyst_alert.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.incidentEventUrl") };
            if _cond {
                uri_parts(
                    event,
                    "json.incidentEventUrl",
                    "darktrace.ai_analyst_alert.incident_event_url",
                    true,
                    false,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.ai_analyst_alert.incident_event_url.original")
                    .cloned()
                {
                    event.set("event.url", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.acknowledged") {
                    if let Some(val) = event.get("json.acknowledged") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.acknowledged".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.is_acknowledged", converted)?;
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
                if event.has_value("json.externalTriggered") {
                    if let Some(val) = event.get("json.externalTriggered") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.externalTriggered".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.ai_analyst_alert.is_external_triggered",
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
                if event.has_value("json.pinned") {
                    if let Some(val) = event.get("json.pinned") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pinned".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.is_pinned", converted)?;
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
                if event.has_value("json.userTriggered") {
                    if let Some(val) = event.get("json.userTriggered") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.userTriggered".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.ai_analyst_alert.is_user_triggered", converted)?;
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

            let _cond = { event.get("json.periods").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def duration = new ArrayList(); for (event in ctx.json.periods) { duration.add((event?.end - event?.start) * params.NANOS_IN_A_MILLI_SECOND); } ctx.event.duration = duration;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def duration = new ArrayList(); for (event in ctx.json.periods) { duration.add((event?.end - event?.start) * params.NANOS_IN_A_MILLI_SECOND); } ctx.event.duration = duration;"#
                        ),
                        cached_params!("{\"NANOS_IN_A_MILLI_SECOND\":1000000}"),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.periods").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.periods").cloned();
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
                                        event.get_as_string("_ingest._value.end")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.end", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.end".into(),
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
                                    event.remove("_ingest._value.end");
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
                                "json.periods",
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

            let _cond = { event.get("json.periods").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.periods").cloned();
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
                                        event.get_as_string("_ingest._value.start")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
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
                                "json.periods",
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

            if event.has_value("json.periods") {
                event.rename("json.periods", "darktrace.ai_analyst_alert.periods")?;
            }

            let _cond = {
                event
                    .get("darktrace.ai_analyst_alert.periods")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "darktrace.ai_analyst_alert.periods", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "event.end",
                                json!(
                                    event
                                        .get("_ingest._value.end")
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
                    .get("darktrace.ai_analyst_alert.periods")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "darktrace.ai_analyst_alert.periods", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "event.start",
                                json!(
                                    event
                                        .get("_ingest._value.start")
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
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.relatedBreaches").cloned();
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
                                        event.get_as_string("_ingest._value.timestamp")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "UNIX_MS", "MMM dd HH:mm:ss"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.timestamp", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.timestamp".into(),
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
                                    event.remove("_ingest._value.timestamp");
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
                                "json.relatedBreaches",
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
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.relatedBreaches").cloned();
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
                                    if event.has_value("_ingest._value.pbid") {
                                        if let Some(val) = event.get("_ingest._value.pbid") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.pbid".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.pbid", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.pbid").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.pbid".into(),
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
                                "json.relatedBreaches",
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
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.relatedBreaches").cloned();
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
                                    if event.has_value("_ingest._value.threatScore") {
                                        if let Some(val) = event.get("_ingest._value.threatScore") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.threatScore".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.threat_score", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    if event.remove("_ingest._value.threatScore").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._value.threatScore".into(),
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
                                "json.relatedBreaches",
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
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.relatedBreaches", |event| {
                        if event.has_value("_ingest._value.modelName") {
                            event
                                .rename("_ingest._value.modelName", "_ingest._value.model_name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.relatedBreaches", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "rule.name",
                                json!(
                                    event
                                        .get("_ingest._value.model_name")
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
                    .get("json.relatedBreaches")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.relatedBreaches", |event| {
                        event.remove("_ingest._value.threatScore");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.relatedBreaches") {
                event.rename(
                    "json.relatedBreaches",
                    "darktrace.ai_analyst_alert.related_breaches",
                )?;
            }

            if event.has_value("json.summariser") {
                event.rename("json.summariser", "darktrace.ai_analyst_alert.summariser")?;
            }

            if event.has_value("json.summary") {
                event.rename("json.summary", "darktrace.ai_analyst_alert.summary")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.summary").cloned() {
                    event.set("message", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.title") {
                event.rename("json.title", "darktrace.ai_analyst_alert.title")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.ai_analyst_alert.title").cloned() {
                    event.set("event.reason", v)?;
                }
                Ok(())
            })();

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
                    event.remove("darktrace.ai_analyst_alert.created_at");
                    event.remove("darktrace.ai_analyst_alert.summary");
                    event.remove("darktrace.ai_analyst_alert.id");
                    event.remove("darktrace.ai_analyst_alert.title");
                    event.remove("darktrace.ai_analyst_alert.aia_score");
                    event.remove("darktrace.ai_analyst_alert.children");
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("darktrace.ai_analyst_alert.related_breaches")
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
                        "darktrace.ai_analyst_alert.related_breaches",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.model_name");
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
                    .get("darktrace.ai_analyst_alert.periods")
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
                    foreach_array(event, "darktrace.ai_analyst_alert.periods", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.remove("_ingest._value.start");
                            event.remove("_ingest._value.end");
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("darktrace.ai_analyst_alert.breach_devices")
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
                        "darktrace.ai_analyst_alert.breach_devices",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.did");
                                event.remove("_ingest._value.mac_address");
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
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
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
