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

            parse_json_field(event, "event.original", "github.issues")?;

            let _cond = { !(event.get("github.issues").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                condition_eq(
                    event.get("github.issues.created_at"),
                    event.get("github.issues.updated_at"),
                )
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("github.issues.closed_at")
                    || event.has_value("github.issues.pull_request.merged_at")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                !condition_eq(
                    event.get("github.issues.created_at"),
                    event.get("github.issues.updated_at"),
                ) && !event.has_value("github.issues.closed_at")
                    && !event.has_value("github.issues.pull_request.merged_at")
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("github.issues.state") == Some("open")
                    && event.get_str("github.issues.state_reason") == Some("reopened")
            };
            if _cond {
                event.set("event.action", json!("reopened"))?;
            }

            let _cond = {
                event.get_str("github.issues.state") == Some("open")
                    && event.get_str("github.issues.state_reason") != Some("reopened")
            };
            if _cond {
                event.set("event.action", json!("opened"))?;
            }

            let _cond = { event.get_str("github.issues.state") == Some("closed") };
            if _cond {
                event.set("event.action", json!("closed"))?;
            }

            if let Some(v) = event
                .get("github.issues.state_reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            let _cond = { event.has_value("github.issues.updated_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.issues.updated_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("github.issues.updated_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.issues.updated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.issues.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.issues.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("github.issues.created_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.issues.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("github.issues.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            event.set("github.issues.is_pr", json!(false))?;

            let _cond = { event.has_value("github.issues.pull_request") };
            if _cond {
                event.set("github.issues.is_pr", json!(true))?;
            }

            if event.has_value("github.issues.url") {
                if let Some(input) = event.get_string("github.issues.url") {
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
                        captured.push(("_temp_.owner", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/issues/") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.repository", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/issues/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.number", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "github.issues.url".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("_temp_.repository") };
            if _cond {
                event.set(
                    "github.repository.name",
                    json!(
                        event
                            .get("_temp_.repository")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.owner") && event.has_value("_temp_.repository") };
            if _cond {
                event.set(
                    "github.repository.html_url",
                    json!(format!(
                        "https://github.com/{}/{}",
                        event
                            .get("_temp_.owner")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp_.repository")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("_temp_.owner") && event.has_value("_temp_.repository") };
            if _cond {
                event.set(
                    "github.repository.url",
                    json!(format!(
                        "https://api.github.com/repos/{}/{}",
                        event
                            .get("_temp_.owner")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp_.repository")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("_temp_.owner") };
            if _cond {
                event.set(
                    "github.repository.owner.login",
                    json!(
                        event
                            .get("_temp_.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("github.issues.assignees") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("github.issues.assignees").cloned();
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
                            if event.remove("_ingest._value.node_id").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.node_id".into(),
                                });
                            }
                            if event.remove("_ingest._value.avatar_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.avatar_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.gravatar_id").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.gravatar_id".into(),
                                });
                            }
                            if event.remove("_ingest._value.followers_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.followers_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.following_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.following_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.gists_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.gists_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.starred_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.starred_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.subscriptions_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.subscriptions_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.organizations_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.organizations_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.repos_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.repos_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.events_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.events_url".into(),
                                });
                            }
                            if event.remove("_ingest._value.received_events_url").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.received_events_url".into(),
                                });
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
                            "github.issues.assignees",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("github.issues.closed_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.issues.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.issues.updated_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.issues.url") {
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

            event.remove("github.issues.user.node_id");
            event.remove("github.issues.user.avatar_url");
            event.remove("github.issues.user.gravatar_id");
            event.remove("github.issues.user.followers_url");
            event.remove("github.issues.user.following_url");
            event.remove("github.issues.user.gists_url");
            event.remove("github.issues.user.starred_url");
            event.remove("github.issues.user.subscriptions_url");
            event.remove("github.issues.user.organizations_url");
            event.remove("github.issues.user.repos_url");
            event.remove("github.issues.user.events_url");
            event.remove("github.issues.user.received_events_url");
            event.remove("github.issues.assignee.node_id");
            event.remove("github.issues.assignee.avatar_url");
            event.remove("github.issues.assignee.gravatar_id");
            event.remove("github.issues.assignee.followers_url");
            event.remove("github.issues.assignee.following_url");
            event.remove("github.issues.assignee.gists_url");
            event.remove("github.issues.assignee.starred_url");
            event.remove("github.issues.assignee.subscriptions_url");
            event.remove("github.issues.assignee.organizations_url");
            event.remove("github.issues.assignee.repos_url");
            event.remove("github.issues.assignee.events_url");
            event.remove("github.issues.assignee.received_events_url");
            event.remove("github.issues.closed_by.node_id");
            event.remove("github.issues.closed_by.avatar_url");
            event.remove("github.issues.closed_by.gravatar_id");
            event.remove("github.issues.closed_by.followers_url");
            event.remove("github.issues.closed_by.following_url");
            event.remove("github.issues.closed_by.gists_url");
            event.remove("github.issues.closed_by.starred_url");
            event.remove("github.issues.closed_by.subscriptions_url");
            event.remove("github.issues.closed_by.organizations_url");
            event.remove("github.issues.closed_by.repos_url");
            event.remove("github.issues.closed_by.events_url");
            event.remove("github.issues.closed_by.received_events_url");
            event.remove("github.issues.milestone");
            event.remove("github.issues.reactions");

            if event.has_value("github.issues.labels") {
                event.rename("github.issues.labels", "_temp_.labels")?;
            }

            let _cond = { event.has_value("_temp_.labels") };
            if _cond {
                // Painless script
                // Source: Map label;\nList labels = new ArrayList();\nList labels_raw = ctx._temp_.labels;\nString label_key, label_value;\nfor (Map label_raw: labels_raw) {\n    label = new HashMap();\n    label.put(\"name\", label_raw.name);\n    label.put(\"description\", label_raw.description);\n    labels.add(label);\n}\nctx.github.issues.labels = labels;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map label;\nList labels = new ArrayList();\nList labels_raw = ctx._temp_.labels;\nString label_key, label_value;\nfor (Map label_raw: labels_raw) {\n    label = new HashMap();\n    label.put(\"name\", label_raw.name);\n    label.put(\"description\", label_raw.description);\n    labels.add(label);\n}\nctx.github.issues.labels = labels;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("github.issues.closed_at") };
            if _cond {
                // Painless script
                // Source: def time_to_close = new HashMap();\ndef closedAtDt = ctx.github.issues.closed_at;\ndef createdAtDt = ctx.github.issues.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(closedAtDt);\nlong closedAtEpoch = zdt.toEpochSecond();\ntime_to_close.put(\"sec\", closedAtEpoch - createdAtEpoch);\nctx.github.issues.time_to_close = time_to_close;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def time_to_close = new HashMap();\ndef closedAtDt = ctx.github.issues.closed_at;\ndef createdAtDt = ctx.github.issues.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(closedAtDt);\nlong closedAtEpoch = zdt.toEpochSecond();\ntime_to_close.put(\"sec\", closedAtEpoch - createdAtEpoch);\nctx.github.issues.time_to_close = time_to_close;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("github.issues.user.login") };
            if _cond {
                if let Some(v) = event.get("github.issues.user.login").cloned() {
                    event.set("user.name", v)?;
                }
            }

            if event.has_value("github.issues.user.id") {
                if let Some(val) = event.get("github.issues.user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "github.issues.user.id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("github.issues.user.site_admin")
                    && event.get_bool("github.issues.user.site_admin") == Some(true)
            };
            if _cond {
                event.append_unique("user.roles", json!("site_admin"))?;
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

            let _cond = { event.has_value("github.issues.assignees") };
            if _cond {
                foreach_array(event, "github.issues.assignees", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.login")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            event.remove("_temp_");
            event.remove("github.issues.repository");

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
