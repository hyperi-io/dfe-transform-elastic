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
                if let Some(v) = event.get("json.last_updated") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.last_updated_status") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.message") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.uuid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event
                    .get_str("json.status")
                    .is_some_and(|s| ["active", "resolved"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.has_value("json.last_updated") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_updated") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("darktrace.system_status_alert.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_updated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.last_updated");
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
                    .get("darktrace.system_status_alert.last_updated")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.uuid") {
                event.rename("json.uuid", "darktrace.system_status_alert.uuid")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.system_status_alert.uuid").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.message") {
                event.rename("json.message", "darktrace.system_status_alert.message")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.system_status_alert.message").cloned() {
                    event.set("event.reason", v)?;
                }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.priority") {
                    if let Some(val) = event.get("json.priority") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.priority".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.system_status_alert.priority", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.priority").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.priority".into(),
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
                if let Some(v) = event.get("darktrace.system_status_alert.priority").cloned() {
                    event.set("event.risk_score", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("darktrace.system_status_alert.priority").cloned() {
                    event.set("event.risk_score_norm", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.url") };
            if _cond {
                uri_parts(
                    event,
                    "json.url",
                    "darktrace.system_status_alert.url",
                    true,
                    false,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("darktrace.system_status_alert.url.original")
                    .cloned()
                {
                    event.set("event.url", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "darktrace.system_status_alert.hostname")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("darktrace.system_status_alert.hostname") {
                    if let Some(val) = event.get("darktrace.system_status_alert.hostname") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "darktrace.system_status_alert.hostname".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "darktrace.system_status_alert._temp_.hostname_ip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("darktrace.system_status_alert.hostname").cloned() {
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
                            .get("darktrace.system_status_alert._temp_.hostname_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

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
                if let Some(val) = event.get("json.ip_address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.ip_address".into(),
                            message,
                        })?;
                    event.set("darktrace.system_status_alert._temp_.ip_address", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("darktrace.system_status_alert._temp_.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("darktrace.system_status_alert._temp_.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.ip_address") {
                event.rename(
                    "json.ip_address",
                    "darktrace.system_status_alert.ip_address",
                )?;
            }

            event.remove("darktrace.system_status_alert._temp_");

            if event.has_value("json.acknowledge_timeout") {
                event.rename(
                    "json.acknowledge_timeout",
                    "darktrace.system_status_alert.acknowledge_timeout",
                )?;
            }

            if event.has_value("json.alert_name") {
                event.rename(
                    "json.alert_name",
                    "darktrace.system_status_alert.alert_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.child_id") {
                    if let Some(val) = event.get("json.child_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.child_id".into(),
                                message,
                            }
                        })?;
                        event.set("darktrace.system_status_alert.child_id", converted)?;
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

            let _cond = { event.has_value("json.last_updated_status") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_updated_status") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "UNIX_MS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("darktrace.system_status_alert.last_updated_status", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_updated_status".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("json.last_updated_status");
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

            if event.has_value("json.name") {
                event.rename("json.name", "darktrace.system_status_alert.name")?;
            }

            if event.has_value("json.priority_level") {
                event.rename(
                    "json.priority_level",
                    "darktrace.system_status_alert.priority_level",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "json.status",
                    "darktrace.system_status_alert.status",
                    str::to_lowercase,
                )?;
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
                    event.remove("darktrace.system_status_alert.last_updated");
                    event.remove("darktrace.system_status_alert.uuid");
                    event.remove("darktrace.system_status_alert.message");
                    event.remove("darktrace.system_status_alert.priority");
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
