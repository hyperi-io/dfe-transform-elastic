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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            if let Some(v) = event.get("_ingest.timestamp").cloned() {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("state"))?;

            event.set("event.action", json!("person_deprovisioned"))?;

            event.append_unique("event.category", json!("iam"))?;

            event.append_unique("event.type", json!("user"))?;

            event.append_unique("event.type", json!("info"))?;

            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            let _cond = { event.get("json.email").is_some_and(|v| v.is_string()) };
            if _cond {
                if let Some(v) = event.get("json.email").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.get("json.name").is_some_and(|v| v.is_string()) };
            if _cond {
                if let Some(v) = event.get("json.name").cloned() {
                    event.set("user.full_name", v)?;
                }
            }

            let _cond = {
                !event.has_value("user.name")
                    && event.get("user.email").is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event.get("user.email").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                if let Some(v) = event.get("user.id").cloned() {
                    event.set("kolide.deprovisioned_person.id", v)?;
                }
            }

            if event.has_value("json.created_at") {
                event.rename("json.created_at", "kolide.deprovisioned_person.created_at")?;
            }

            let _cond = { event.has_value("kolide.deprovisioned_person.created_at") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("kolide.deprovisioned_person.created_at")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("kolide.deprovisioned_person.created_at", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "kolide.deprovisioned_person.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.last_authenticated_at") {
                event.rename(
                    "json.last_authenticated_at",
                    "kolide.deprovisioned_person.last_authenticated_at",
                )?;
            }

            let _cond = { event.has_value("kolide.deprovisioned_person.last_authenticated_at") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("kolide.deprovisioned_person.last_authenticated_at")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("kolide.deprovisioned_person.last_authenticated_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "kolide.deprovisioned_person.last_authenticated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.has_registered_device") {
                event.rename(
                    "json.has_registered_device",
                    "kolide.deprovisioned_person.has_registered_device",
                )?;
            }

            if event.has_value("json.api_url") {
                event.rename("json.api_url", "kolide.deprovisioned_person.api_url")?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = {
                event.has_value("event.original") && event.get_str("event.original") != Some("")
            };
            if _cond {
                parse_json_field(event, "event.original", "_tmp.fingerprint_source")?;
            }

            let _cond = { event.has_value("_tmp.fingerprint_source") };
            if _cond {
                event.remove("_tmp.fingerprint_source.last_authenticated_at");
            }

            let _cond = { event.has_value("_tmp.fingerprint_source") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("_tmp.fingerprint_source") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp.fingerprint_source".into(),
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
            event.remove("_tmp");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
