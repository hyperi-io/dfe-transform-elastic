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

            parse_json_field(event, "event.original", "github.code_scanning")?;

            let _cond = {
                !(event
                    .get("github.code_scanning")
                    .is_some_and(|v| v.is_object()))
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            event.set("event.kind", json!("alert"))?;

            let _cond = {
                !event.has_value("github.code_scanning.fixed_at")
                    && !event.has_value("github.code_scanning.dismissed_at")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("github.code_scanning.fixed_at")
                    || event.has_value("github.code_scanning.dismissed_at")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.has_value("github.code_scanning.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.code_scanning.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.code_scanning.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.code_scanning.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.code_scanning.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.code_scanning.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.code_scanning.updated_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.code_scanning.updated_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.code_scanning.updated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.code_scanning.dismissed_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.code_scanning.dismissed_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("github.code_scanning.dismissed_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.code_scanning.dismissed_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("github.code_scanning.repository") {
                event.rename("github.code_scanning.repository", "_temp")?;
            }

            if event.has_value("_temp.id") {
                event.rename("_temp.id", "github.repository.id")?;
            }

            if event.has_value("_temp.name") {
                event.rename("_temp.name", "github.repository.name")?;
            }

            if event.has_value("_temp.full_name") {
                event.rename("_temp.full_name", "github.repository.full_name")?;
            }

            if event.has_value("_temp.private") {
                event.rename("_temp.private", "github.repository.private")?;
            }

            if event.has_value("_temp.html_url") {
                event.rename("_temp.html_url", "github.repository.html_url")?;
            }

            if event.has_value("_temp.url") {
                event.rename("_temp.url", "github.repository.url")?;
            }

            if event.has_value("_temp.description") {
                event.rename("_temp.description", "github.repository.description")?;
            }

            if event.has_value("_temp.fork") {
                event.rename("_temp.fork", "github.repository.fork")?;
            }

            if event.has_value("_temp.owner.login") {
                event.rename("_temp.owner.login", "github.repository.owner.login")?;
            }

            if event.has_value("_temp.owner.id") {
                event.rename("_temp.owner.id", "github.repository.owner.id")?;
            }

            if event.has_value("_temp.owner.url") {
                event.rename("_temp.owner.url", "github.repository.owner.url")?;
            }

            if event.has_value("_temp.owner.html_url") {
                event.rename("_temp.owner.html_url", "github.repository.owner.html_url")?;
            }

            if event.has_value("_temp.owner.type") {
                event.rename("_temp.owner.type", "github.repository.owner.type")?;
            }

            if event.has_value("_temp.owner.site_admin") {
                event.rename(
                    "_temp.owner.site_admin",
                    "github.repository.owner.site_admin",
                )?;
            }

            if event.has_value("github.code_scanning.url") {
                if let Some(input) = event.get_string("github.code_scanning.url") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("https://api.github.com/repos/")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.owner", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/code-scanning/alerts/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.repository", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/code-scanning/alerts/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp.number", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "github.code_scanning.url".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                !event.has_value("github.repository.html_url")
                    && event.has_value("_temp.owner")
                    && event.has_value("_temp.repository")
            };
            if _cond {
                event.set(
                    "github.repository.html_url",
                    json!(format!(
                        "https://github.com/{}/{}",
                        event
                            .get("_temp.owner")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp.repository")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                !event.has_value("github.repository.url")
                    && event.has_value("_temp.owner")
                    && event.has_value("_temp.repository")
            };
            if _cond {
                event.set(
                    "github.repository.url",
                    json!(format!(
                        "https://api.github.com/repos/{}/{}",
                        event
                            .get("_temp.owner")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp.repository")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { !event.has_value("github.repository.name") };
            if _cond {
                if event.has_value("_temp.repository") {
                    event.rename("_temp.repository", "github.repository.name")?;
                }
            }

            let _cond = { !event.has_value("github.repository.owner.login") };
            if _cond {
                if event.has_value("_temp.owner") {
                    event.rename("_temp.owner", "github.repository.owner.login")?;
                }
            }

            let _cond = { !event.has_value("github.code_scanning.number") };
            if _cond {
                if event.has_value("_temp.number") {
                    event.rename("_temp.number", "github.code_scanning.number")?;
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("github.code_scanning.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.code_scanning.dismissed_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.code_scanning.number") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.code_scanning.updated_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.repository.name") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.repository.owner.login") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("github.code_scanning.state") {
                map_strings(
                    event,
                    "github.code_scanning.state",
                    "github.code_scanning.state",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("github.code_scanning.rule.security_severity_level") {
                map_strings(
                    event,
                    "github.code_scanning.rule.security_severity_level",
                    "github.code_scanning.rule.security_severity_level",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("github.code_scanning.dismissed_by") {
                event.rename("github.code_scanning.dismissed_by", "_temp.dismissed_by")?;
            }

            event.remove("github.code_scanning.dismissed_by");

            if event.has_value("_temp.dismissed_by.login") {
                event.rename(
                    "_temp.dismissed_by.login",
                    "github.code_scanning.dismissed_by.login",
                )?;
            }

            if event.has_value("_temp.dismissed_by.id") {
                event.rename(
                    "_temp.dismissed_by.id",
                    "github.code_scanning.dismissed_by.id",
                )?;
            }

            if event.has_value("_temp.dismissed_by.url") {
                event.rename(
                    "_temp.dismissed_by.url",
                    "github.code_scanning.dismissed_by.url",
                )?;
            }

            if event.has_value("_temp.dismissed_by.html_url") {
                event.rename(
                    "_temp.dismissed_by.html_url",
                    "github.code_scanning.dismissed_by.html_url",
                )?;
            }

            if event.has_value("_temp.dismissed_by.type") {
                event.rename(
                    "_temp.dismissed_by.type",
                    "github.code_scanning.dismissed_by.type",
                )?;
            }

            if event.has_value("_temp.dismissed_by.site_admin") {
                event.rename(
                    "_temp.dismissed_by.site_admin",
                    "github.code_scanning.dismissed_by.site_admin",
                )?;
            }

            if event.has_value("github.code_scanning.most_recent_instance.message.text") {
                event.rename(
                    "github.code_scanning.most_recent_instance.message.text",
                    "message",
                )?;
            }

            let _cond = {
                event.has_value("github.code_scanning.fixed_at")
                    || event.has_value("github.code_scanning.dismissed_at")
            };
            if _cond {
                // Painless script
                // Source: def time_to_resolution = new HashMap();\ndef fixedAtDt = ctx.github.code_scanning.fixed_at;\ndef dismissedAtDt = ctx.github.code_scanning.dismissed_at;\ndef createdAtDt = ctx.github.code_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nif (fixedAtDt != null) {\n    zdt = ZonedDateTime.parse(fixedAtDt);\n    long fixedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", fixedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\nelse {\n    zdt = ZonedDateTime.parse(dismissedAtDt);\n    long dismissedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", dismissedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def time_to_resolution = new HashMap();\ndef fixedAtDt = ctx.github.code_scanning.fixed_at;\ndef dismissedAtDt = ctx.github.code_scanning.dismissed_at;\ndef createdAtDt = ctx.github.code_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nif (fixedAtDt != null) {\n    zdt = ZonedDateTime.parse(fixedAtDt);\n    long fixedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", fixedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\nelse {\n    zdt = ZonedDateTime.parse(dismissedAtDt);\n    long dismissedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", dismissedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("github.code_scanning.rule.id") };
            if _cond {
                event.rename("github.code_scanning.rule.id", "rule.id")?;
            }

            let _cond = { event.has_value("github.code_scanning.rule.name") };
            if _cond {
                event.rename("github.code_scanning.rule.name", "rule.name")?;
            }

            let _cond = { event.has_value("github.code_scanning.rule.description") };
            if _cond {
                event.rename("github.code_scanning.rule.description", "rule.description")?;
            }

            let _cond = { event.has_value("github.code_scanning.rule.tags") };
            if _cond {
                if event.has_value("github.code_scanning.rule.tags") {
                    foreach_array(event, "github.code_scanning.rule.tags", |event| {
                        event.append(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            event.remove("github.code_scanning.rule.tags");

            event.remove("_temp");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
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
