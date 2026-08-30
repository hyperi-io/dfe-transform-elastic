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
            event.set("ecs.version", json!("9.1.0"))?;

            let _cond = { event.has_value("error.message") && !event.has_value("event.original") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            parse_json_field(
                event,
                "event.original",
                "beyondinsight_password_safe.useraudit",
            )?;

            // Painless script
            // Source: ctx.beyondinsight_password_safe.useraudit.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.beyondinsight_password_safe.useraudit.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // Painless script
            // Source: for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.useraudit[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.useraudit[field] =\n      Integer.toString(value.intValue());\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.useraudit[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.useraudit[field] =\n      Integer.toString(value.intValue());\n  }\n}\n"#
                ),
                cached_params!("{\"numeric_ids\":[\"AuditID\",\"UserID\"]}"),
            )?;

            // Painless script
            // Source: Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.useraudit.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.useraudit = renamedFields;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.useraudit.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.useraudit = renamedFields;\n"#
                ),
                cached_params!(
                    "{\"field_mappings\":{\"ActionType\":\"action_type\",\"AuditID\":\"audit_id\",\"CreateDate\":\"create_date\",\"IPAddress\":\"ip_address\",\"Section\":\"section\",\"UserID\":\"user_id\",\"UserName\":\"user_name\"}}"
                ),
            )?;

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.create_date") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("beyondinsight_password_safe.useraudit.create_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondinsight_password_safe.useraudit.create_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.useraudit.create_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.create_date") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.useraudit.create_date")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("ctx.beyondinsight_password_safe.useraudit.audit_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond =
                { event.get_str("beyondinsight_password_safe.useraudit.user_id") != Some("-1") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.useraudit.user_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.useraudit.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.ip_address") };
            if _cond {
                gsub_field(
                    event,
                    "beyondinsight_password_safe.useraudit.ip_address",
                    "beyondinsight_password_safe.useraudit.ip_address",
                    cached_regex!("\\b0+(\\d)"),
                    "$1",
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.ip_address") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("beyondinsight_password_safe.useraudit.ip_address")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondinsight_password_safe.useraudit.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondinsight_password_safe.useraudit.ip_address",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    if event
                        .remove("beyondinsight_password_safe.useraudit.ip_address")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "beyondinsight_password_safe.useraudit.ip_address".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.useraudit.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.useraudit.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.get("user.name").is_some_and(|v| v.is_string())
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
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
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("beyondinsight_password_safe.useraudit.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.useraudit.ip_address")
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

            let _cond =
                { event.get_str("beyondinsight_password_safe.useraudit.user_id") != Some("-1") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.useraudit.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
