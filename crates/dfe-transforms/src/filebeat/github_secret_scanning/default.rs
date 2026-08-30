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

            parse_json_field(event, "event.original", "github.secret_scanning")?;

            let _cond = {
                !(event
                    .get("github.secret_scanning")
                    .is_some_and(|v| v.is_object()))
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            event.set("event.kind", json!("alert"))?;

            let _cond = { !event.has_value("github.secret_scanning.resolved_at") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.has_value("github.secret_scanning.resolved_at") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("github.secret_scanning.number") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.secret_scanning.resolved_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.secret_scanning.updated_at") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("github.secret_scanning.created_at") };
            if _cond {
                if let Some(v) = event.get("github.secret_scanning.created_at").cloned() {
                    event.set("event.created", v)?;
                }
            }

            let _cond = {
                event.has_value("github.secret_scanning.created_at")
                    && !event.has_value("github.secret_scanning.updated_at")
                    && !event.has_value("github.secret_scanning.resolved_at")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.secret_scanning.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.secret_scanning.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("github.secret_scanning.updated_at")
                    && !event.has_value("github.secret_scanning.resolved_at")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.secret_scanning.updated_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.secret_scanning.updated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.secret_scanning.resolved_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.secret_scanning.resolved_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.secret_scanning.resolved_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("github.secret_scanning.repository") {
                event.rename("github.secret_scanning.repository", "_temp.repository")?;
            }

            if event.has_value("_temp.repository.id") {
                event.rename("_temp.repository.id", "github.repository.id")?;
            }

            if event.has_value("_temp.repository.name") {
                event.rename("_temp.repository.name", "github.repository.name")?;
            }

            if event.has_value("_temp.repository.full_name") {
                event.rename("_temp.repository.full_name", "github.repository.full_name")?;
            }

            if event.has_value("_temp.repository.private") {
                event.rename("_temp.repository.private", "github.repository.private")?;
            }

            if event.has_value("_temp.repository.html_url") {
                event.rename("_temp.repository.html_url", "github.repository.html_url")?;
            }

            if event.has_value("_temp.repository.url") {
                event.rename("_temp.repository.url", "github.repository.url")?;
            }

            if event.has_value("_temp.repository.description") {
                event.rename(
                    "_temp.repository.description",
                    "github.repository.description",
                )?;
            }

            if event.has_value("_temp.repository.fork") {
                event.rename("_temp.repository.fork", "github.repository.fork")?;
            }

            if event.has_value("_temp.repository.owner.login") {
                event.rename(
                    "_temp.repository.owner.login",
                    "github.repository.owner.login",
                )?;
            }

            if event.has_value("_temp.repository.owner.id") {
                event.rename("_temp.repository.owner.id", "github.repository.owner.id")?;
            }

            if event.has_value("_temp.repository.owner.url") {
                event.rename("_temp.repository.owner.url", "github.repository.owner.url")?;
            }

            if event.has_value("_temp.repository.owner.html_url") {
                event.rename(
                    "_temp.repository.owner.html_url",
                    "github.repository.owner.html_url",
                )?;
            }

            if event.has_value("_temp.repository.owner.type") {
                event.rename(
                    "_temp.repository.owner.type",
                    "github.repository.owner.type",
                )?;
            }

            if event.has_value("_temp.repository.owner.site_admin") {
                event.rename(
                    "_temp.repository.owner.site_admin",
                    "github.repository.owner.site_admin",
                )?;
            }

            if event.has_value("github.secret_scanning.resolved_by") {
                event.rename("github.secret_scanning.resolved_by", "_temp.resolved_by")?;
            }

            if event.has_value("_temp.resolved_by.name") {
                event.rename(
                    "_temp.resolved_by.name",
                    "github.secret_scanning.resolved_by.name",
                )?;
            }

            if event.has_value("_temp.resolved_by.email") {
                event.rename(
                    "_temp.resolved_by.email",
                    "github.secret_scanning.resolved_by.email",
                )?;
            }

            if event.has_value("_temp.resolved_by.login") {
                event.rename(
                    "_temp.resolved_by.login",
                    "github.secret_scanning.resolved_by.login",
                )?;
            }

            if event.has_value("_temp.resolved_by.id") {
                event.rename(
                    "_temp.resolved_by.id",
                    "github.secret_scanning.resolved_by.id",
                )?;
            }

            if event.has_value("_temp.resolved_by.node_id") {
                event.rename(
                    "_temp.resolved_by.node_id",
                    "github.secret_scanning.resolved_by.node_id",
                )?;
            }

            if event.has_value("_temp.resolved_by.url") {
                event.rename(
                    "_temp.resolved_by.url",
                    "github.secret_scanning.resolved_by.url",
                )?;
            }

            if event.has_value("_temp.resolved_by.html_url") {
                event.rename(
                    "_temp.resolved_by.html_url",
                    "github.secret_scanning.resolved_by.html_url",
                )?;
            }

            if event.has_value("_temp.resolved_by.type") {
                event.rename(
                    "_temp.resolved_by.type",
                    "github.secret_scanning.resolved_by.type",
                )?;
            }

            if event.has_value("_temp.resolved_by.site_admin") {
                event.rename(
                    "_temp.resolved_by.site_admin",
                    "github.secret_scanning.resolved_by.site_admin",
                )?;
            }

            if event.has_value("github.secret_scanning.url") {
                if let Some(input) = event.get_string("github.secret_scanning.url") {
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
                        let Some(pos) = remaining.find("/secret-scanning/alerts/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.repository", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/secret-scanning/alerts/") else {
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
                            path: "github.secret_scanning.url".into(),
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

            let _cond = { !event.has_value("github.secret_scanning.number") };
            if _cond {
                if event.has_value("_temp.number") {
                    event.rename("_temp.number", "github.secret_scanning.number")?;
                }
            }

            if event.has_value("github.secret_scanning.state") {
                map_strings(
                    event,
                    "github.secret_scanning.state",
                    "github.secret_scanning.state",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("github.secret_scanning.push_protection_bypassed_by") {
                event.rename(
                    "github.secret_scanning.push_protection_bypassed_by",
                    "_temp.push_protection_bypassed_by",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.name") {
                event.rename(
                    "_temp.push_protection_bypassed_by.name",
                    "github.secret_scanning.push_protection_bypassed_by.name",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.email") {
                event.rename(
                    "_temp.push_protection_bypassed_by.email",
                    "github.secret_scanning.push_protection_bypassed_by.email",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.login") {
                event.rename(
                    "_temp.push_protection_bypassed_by.login",
                    "github.secret_scanning.push_protection_bypassed_by.login",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.id") {
                event.rename(
                    "_temp.push_protection_bypassed_by.id",
                    "github.secret_scanning.push_protection_bypassed_by.id",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.node_id") {
                event.rename(
                    "_temp.push_protection_bypassed_by.node_id",
                    "github.secret_scanning.push_protection_bypassed_by.node_id",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.url") {
                event.rename(
                    "_temp.push_protection_bypassed_by.url",
                    "github.secret_scanning.push_protection_bypassed_by.url",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.html_url") {
                event.rename(
                    "_temp.push_protection_bypassed_by.html_url",
                    "github.secret_scanning.push_protection_bypassed_by.html_url",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.type") {
                event.rename(
                    "_temp.push_protection_bypassed_by.type",
                    "github.secret_scanning.push_protection_bypassed_by.type",
                )?;
            }

            if event.has_value("_temp.push_protection_bypassed_by.site_admin") {
                event.rename(
                    "_temp.push_protection_bypassed_by.site_admin",
                    "github.secret_scanning.push_protection_bypassed_by.site_admin",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_secret"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_secret"),
                        _ => false,
                    })
                    && event.has_value("github.secret_scanning.secret")
            };
            if _cond {
                // Painless script
                // Source: def secret = ctx.github.secret_scanning.secret.toString();\ndef masked_secret = secret;\nint num_start = 2;\nint num_end = 2;\ndef masked_value = String.join(\"\", Collections.nCopies(secret.length()-(num_start+num_end), \"X\"));\nmasked_secret = secret.substring(0,num_start) + masked_value + secret.substring(secret.length()-num_end);\nctx.github.secret_scanning.secret = masked_secret;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def secret = ctx.github.secret_scanning.secret.toString();\ndef masked_secret = secret;\nint num_start = 2;\nint num_end = 2;\ndef masked_value = String.join(\"\", Collections.nCopies(secret.length()-(num_start+num_end), \"X\"));\nmasked_secret = secret.substring(0,num_start) + masked_value + secret.substring(secret.length()-num_end);\nctx.github.secret_scanning.secret = masked_secret;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("github.secret_scanning.resolved_at") };
            if _cond {
                // Painless script
                // Source: def time_to_resolution = new HashMap();\ndef resolvedAtDt = ctx.github.secret_scanning.resolved_at;\ndef createdAtDt = ctx.github.secret_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(resolvedAtDt);\nlong resolvedAtEpoch = zdt.toEpochSecond();\ntime_to_resolution.put(\"sec\", resolvedAtEpoch - createdAtEpoch);\nctx.github.secret_scanning.time_to_resolution = time_to_resolution;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def time_to_resolution = new HashMap();\ndef resolvedAtDt = ctx.github.secret_scanning.resolved_at;\ndef createdAtDt = ctx.github.secret_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(resolvedAtDt);\nlong resolvedAtEpoch = zdt.toEpochSecond();\ntime_to_resolution.put(\"sec\", resolvedAtEpoch - createdAtEpoch);\nctx.github.secret_scanning.time_to_resolution = time_to_resolution;\n"#
                    ),
                )?;
            }

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
