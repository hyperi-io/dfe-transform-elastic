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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.set("event.action", json!("osquery_status"))?;

            let _cond = { event.has_value("json.u") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.u") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.u".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse json.u: {}",
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

            if event.has_value("json.s") {
                event.rename("json.s", "kolide.osquery_status.severity")?;
            }

            if event.has_value("kolide.osquery_status.severity") {
                if let Some(val) = event.get("kolide.osquery_status.severity") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "kolide.osquery_status.severity".into(),
                            message,
                        }
                    })?;
                    event.set("kolide.osquery_status.severity", converted)?;
                }
            }

            if event.has_value("json.f") {
                event.rename("json.f", "log.origin.file.name")?;
            }

            if event.has_value("json.i") {
                event.rename("json.i", "log.origin.file.line")?;
            }

            if event.has_value("json.m") {
                event.rename("json.m", "message")?;
            }

            if event.has_value("json.h") {
                event.rename("json.h", "kolide.osquery_status.host_identifier")?;
            }

            if event.has_value("json.request_id") {
                event.rename("json.request_id", "kolide.osquery_status.request_id")?;
            }

            if event.has_value("json.kolide_decorations.device_id") {
                if let Some(val) = event.get("json.kolide_decorations.device_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.kolide_decorations.device_id".into(),
                            message,
                        }
                    })?;
                    event.set("host.id", converted)?;
                }
            }

            if event.has_value("json.kolide_decorations.device_display_name") {
                event.rename("json.kolide_decorations.device_display_name", "host.name")?;
            }

            if let Some(v) = event
                .get("host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("json.kolide_decorations.serial_number") {
                event.rename(
                    "json.kolide_decorations.serial_number",
                    "kolide.osquery_status.device_serial",
                )?;
            }

            if event.has_value("json.kolide_decorations.hardware_uuid") {
                event.rename(
                    "json.kolide_decorations.hardware_uuid",
                    "kolide.osquery_status.hardware_uuid",
                )?;
            }

            if event.has_value("json.kolide_decorations.enrolled_at") {
                event.rename(
                    "json.kolide_decorations.enrolled_at",
                    "kolide.osquery_status.enrolled_at",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_at") {
                event.rename(
                    "json.kolide_decorations.device_registered_at",
                    "kolide.osquery_status.device_registered_at",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_id") {
                if let Some(val) = event.get("json.kolide_decorations.device_registered_owner_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.kolide_decorations.device_registered_owner_id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_name") {
                event.rename(
                    "json.kolide_decorations.device_registered_owner_name",
                    "user.full_name",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_email") {
                event.rename(
                    "json.kolide_decorations.device_registered_owner_email",
                    "user.email",
                )?;
            }

            let _cond = { event.has_value("json.kolide_decorations.remote_ip") };
            if _cond {
                // Painless script
                // Source: if (ctx.host == null) { ctx.host = new HashMap(); }\nctx.host.ip = [ ctx.json.kolide_decorations.remote_ip ];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.host == null) { ctx.host = new HashMap(); }\nctx.host.ip = [ ctx.json.kolide_decorations.remote_ip ];"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("host.ip") && event.get("host.ip").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("host.ip.0")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Begin nested pipeline: "categorize"
            event.set("event.kind", json!("event"))?;
            event.append_unique("event.category", json!("process"))?;
            event.append_unique("event.type", json!("info"))?;
            let _cond = { event.get_str("kolide.osquery_status.severity") == Some("0") };
            if _cond {
                event.set("log.level", json!("info"))?;
            }
            let _cond = { event.get_str("kolide.osquery_status.severity") == Some("1") };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }
            let _cond = { event.get_str("kolide.osquery_status.severity") == Some("2") };
            if _cond {
                event.set("log.level", json!("error"))?;
            }
            let _cond = { event.get_str("kolide.osquery_status.severity") == Some("3") };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }
            let _cond = {
                !event.has_value("log.level") && event.has_value("kolide.osquery_status.severity")
            };
            if _cond {
                event.set("log.level", json!("info"))?;
            }
            // End nested pipeline: "categorize"

            if let Some(v) = event
                .get("kolide.osquery_status.request_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.id".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            let _cond = { !event.has_value("event.id") && event.has_value("event.original") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.original") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.original".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
