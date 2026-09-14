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

            event.set("event.action", json!("database_audit"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            event.set("event.type", Value::Array(vec![json!("access")]))?;

            event.set("event.outcome", json!("success"))?;

            let _cond = {
                event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("LENGTH :")),
                    serde_json::Value::String(s) => s.contains("LENGTH :"),
                    _ => false,
                })
            };
            if _cond {
                event.set("oracle_event_type", json!("database"))?;
            }

            let _cond = {
                event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("LENGTH:")),
                    serde_json::Value::String(s) => s.contains("LENGTH:"),
                    _ => false,
                })
            };
            if _cond {
                event.set("oracle_event_type", json!("object"))?;
            }

            event.set("event.outcome", json!("success"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: %{GREEDYDATA:tmp_timestamp}\\nLENGTH : '%{GREEDYDATA:LENGTH}'\\nACTION :\\[\\d+\\] (?m)%{GREEDYDATA:action}(DATABASE USER):\\S+ '(?P<db_user>(?:[^']+))'\\n(?m)%{GREEDYDATA:audit}
                // Grok pattern: %{GREEDYDATA:tmp_timestamp}\\nLENGTH : '%{GREEDYDATA:LENGTH}'\\nACTION :\\[\\d+\\] (?m)%{GREEDYDATA:action}
                // Grok pattern: %{GREEDYDATA:tmp_timestamp}\\nLENGTH: \"%{GREEDYDATA:LENGTH}\"\\n%{GREEDYDATA:audit}
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "%{GREEDYDATA:tmp_timestamp}\\nLENGTH : '%{GREEDYDATA:LENGTH}'\\nACTION :\\[\\d+\\] (?m)%{GREEDYDATA:action}(DATABASE USER):\\S+ '(?P<db_user>(?:[^']+))'\\n(?m)%{GREEDYDATA:audit}"
                        ),
                        cached_grok!(
                            "%{GREEDYDATA:tmp_timestamp}\\nLENGTH : '%{GREEDYDATA:LENGTH}'\\nACTION :\\[\\d+\\] (?m)%{GREEDYDATA:action}"
                        ),
                        cached_grok!(
                            "%{GREEDYDATA:tmp_timestamp}\\nLENGTH: \"%{GREEDYDATA:LENGTH}\"\\n%{GREEDYDATA:audit}"
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = {
                event.get_str("oracle_event_type") == Some("database")
                    && event.has_value("audit")
                    && event.get_as_string("audit").is_some_and(|s| !s.is_empty())
            };
            if _cond {
                if let Some(kv_str) = event.get_string("audit") {
                    let mut kv_gap = false;
                    for pair in cached_regex!("\\\n(?=[a-zA-Z])").split(&kv_str).into_iter() {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = ({
                            let parts = cached_regex!(":\\S\\d+\\S(?= ')").splitn(&pair, 2);
                            match (parts.first(), parts.get(1)) {
                                (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                _ => None,
                            }
                        })
                        .filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "audit".into(),
                                split: ":\\S\\d+\\S(?= ')".into(),
                            });
                        };
                        {
                            let key = &key[..];
                            let key = key.trim_matches(|c: char| matches!(c, ' '));
                            let value = value.trim_matches(|c: char| matches!(c, ' ' | '\''));
                            if !key.is_empty() {
                                kv_put(event, &format!("oracle.database_audit.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.get_str("oracle_event_type") == Some("object")
                    && event.has_value("audit")
                    && event.get_as_string("audit").is_some_and(|s| !s.is_empty())
            };
            if _cond {
                if let Some(kv_str) = event.get_string("audit") {
                    let mut kv_gap = false;
                    for pair in kv_str.split("\" ") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = ({
                            let parts = cached_regex!(":\\S\\d+\\S (?=\")").splitn(pair, 2);
                            match (parts.first(), parts.get(1)) {
                                (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                _ => None,
                            }
                        })
                        .filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "audit".into(),
                                split: ":\\S\\d+\\S (?=\")".into(),
                            });
                        };
                        {
                            let key = &key[..];
                            let value = value
                                .as_str()
                                .strip_prefix(['(', '[', '<', '"', '\''])
                                .unwrap_or(value.as_str());
                            let value = value
                                .strip_suffix([']', ')', '>', '"', '\''])
                                .unwrap_or(value);
                            if !key.is_empty() {
                                kv_put(event, &format!("oracle.database_audit.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("log.file.path") };
            if _cond {
                if let Some(input) = event.get_string("log.file.path") {
                    // Grok pattern: %{BASE10NUM:process.pid}\\_%{BASE10NUM}\\.aud(\\.log)?$
                    if !cached_grok!("%{BASE10NUM:process.pid}\\_%{BASE10NUM}\\.aud(\\.log)?$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("action") {
                event.rename("action", "oracle.database_audit.action")?;
            }

            if event.has_value("db_user") {
                event.rename("db_user", "oracle.database_audit.database_user")?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: ctx.oracle.database_audit = ctx.oracle.database_audit.entrySet().stream().collect(Collectors.toMap(entry -> entry.getKey().toLowerCase(), Map.Entry::getValue));
            rewrite_keys(
                event,
                &RewriteKeys::new(
                    "oracle.database_audit".into(),
                    "oracle.database_audit".into(),
                    vec![KeyRewriteStep::Lowercase],
                ),
            );

            // Painless script, resolved to its runners at generation time
            // Source: ctx.oracle.database_audit = ctx?.oracle?.database_audit.entrySet().stream().collect(Collectors.toMap(e -> e.getKey().replace(' ', '_'), e -> e.getValue()));
            rename_map_keys(
                event,
                &RenameMapKeys::new("oracle.database_audit".into(), ' ', '_'),
            );

            gsub_field(
                event,
                "oracle.database_audit.action",
                "oracle.database_audit.action",
                cached_regex!("\\n"),
                "",
            )?;

            gsub_field(
                event,
                "oracle.database_audit.action",
                "oracle.database_audit.action",
                cached_regex!("\\s{2,}"),
                " ",
            )?;

            gsub_field(
                event,
                "oracle.database_audit.action",
                "oracle.database_audit.action",
                cached_regex!("^\\'"),
                "",
            )?;

            gsub_field(
                event,
                "oracle.database_audit.action",
                "oracle.database_audit.action",
                cached_regex!("\\'$"),
                "",
            )?;

            if event.has_value("oracle.database_audit.action_number") {
                map_strings(
                    event,
                    "oracle.database_audit.action_number",
                    "oracle.database_audit.action_number",
                    |s| s.trim().to_string(),
                )?;
            }

            let _cond = { event.has_value("oracle.database_audit") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v instanceof String && v.isEmpty() == true);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        empty_strings: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            event.remove("@timestamp");

            if let Some(date_str) = event.get_as_string("tmp_timestamp") {
                match parse_date_out(
                    &date_str,
                    &["EEE MMM [ d][dd] HH:mm:ss uuuu XXX"],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "tmp_timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(input) = event.get_string("tmp_timestamp") {
                // Grok pattern: %{ISO8601_TIMEZONE:event.timezone}$
                if !cached_grok!("%{ISO8601_TIMEZONE:event.timezone}$")
                    .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!(
                    event
                        .get("oracle.database_audit.privilege")
                        .map_or_else(String::new, template_to_string)
                )]);
                if !painless_is_empty_value(&v) {
                    event.set("user.roles", v)?;
                }
                Ok(())
            })();

            event.remove("oracle.database_audit.privilege");

            if event.has_value("LENGTH") {
                event.rename("LENGTH", "oracle.database_audit.length")?;
            }

            if event.has_value("oracle.database_audit.client_user") {
                event.rename("oracle.database_audit.client_user", "client.user.name")?;
            }

            if event.has_value("oracle.database_audit.userid") {
                event.rename("oracle.database_audit.userid", "client.user.id")?;
            }

            if event.has_value("oracle.database_audit.current_user") {
                event.rename("oracle.database_audit.current_user", "client.user.name")?;
            }

            if event.has_value("oracle.database_audit.comment$text") {
                event.rename(
                    "oracle.database_audit.comment$text",
                    "oracle.database_audit.comment_text",
                )?;
            }

            if event.has_value("oracle.database_audit.obj$creator") {
                event.rename(
                    "oracle.database_audit.obj$creator",
                    "oracle.database_audit.obj_creator",
                )?;
            }

            if event.has_value("oracle.database_audit.obj$name") {
                event.rename(
                    "oracle.database_audit.obj$name",
                    "oracle.database_audit.obj_name",
                )?;
            }

            if event.has_value("oracle.database_audit.os$userid") {
                event.rename(
                    "oracle.database_audit.os$userid",
                    "oracle.database_audit.os_userid",
                )?;
            }

            if event.has_value("oracle.database_audit.ses$actions") {
                event.rename(
                    "oracle.database_audit.ses$actions",
                    "oracle.database_audit.ses_actions",
                )?;
            }

            if event.has_value("oracle.database_audit.ses$tid") {
                event.rename(
                    "oracle.database_audit.ses$tid",
                    "oracle.database_audit.ses_tid",
                )?;
            }

            if event.has_value("oracle.database_audit.client_address") {
                event.rename("oracle.database_audit.client_address", "client.address")?;
            }

            if event.has_value("oracle.database_audit.userhost") {
                event.rename("oracle.database_audit.userhost", "server.address")?;
            }

            if event.has_value("oracle.database_audit.database_user") {
                event.rename("oracle.database_audit.database_user", "server.user.name")?;
            }

            if event.has_value("oracle.database_audit.length") {
                if let Some(val) = event.get("oracle.database_audit.length") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "oracle.database_audit.length".into(),
                            message,
                        }
                    })?;
                    event.set("oracle.database_audit.length", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("client.address") {
                    if let Some(input) = event.get_string("client.address") {
                        // Grok pattern: (?:%{IP:client.ip}|%{GREEDYDATA:client.domain})
                        if !cached_grok!("(?:%{IP:client.ip}|%{GREEDYDATA:client.domain})")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("server.address") {
                    if let Some(input) = event.get_string("server.address") {
                        // Grok pattern: (?:%{IP:server.ip}|%{GREEDYDATA:server.domain})
                        if !cached_grok!("(?:%{IP:server.ip}|%{GREEDYDATA:server.domain})")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("oracle.database_audit.sessionid") {
                event.rename(
                    "oracle.database_audit.sessionid",
                    "oracle.database_audit.session_id",
                )?;
            }

            if event.has_value("oracle.database_audit.client_terminal") {
                event.rename(
                    "oracle.database_audit.client_terminal",
                    "oracle.database_audit.client.terminal",
                )?;
            }

            if event.has_value("oracle.database_audit.client_address") {
                event.rename(
                    "oracle.database_audit.client_address",
                    "oracle.database_audit.client.address",
                )?;
            }

            if event.has_value("oracle.database_audit.database_user") {
                event.rename(
                    "oracle.database_audit.database_user",
                    "oracle.database_audit.database.user",
                )?;
            }

            if event.has_value("oracle.database_audit.userhost") {
                event.rename(
                    "oracle.database_audit.userhost",
                    "oracle.database_audit.database.host",
                )?;
            }

            if event.has_value("oracle.database_audit.dbid") {
                event.rename(
                    "oracle.database_audit.dbid",
                    "oracle.database_audit.database.id",
                )?;
            }

            if event.has_value("oracle.database_audit.entry_id") {
                event.rename(
                    "oracle.database_audit.entry_id",
                    "oracle.database_audit.entry.id",
                )?;
            }

            if event.has_value("oracle.database_audit.entryid") {
                event.rename(
                    "oracle.database_audit.entryid",
                    "oracle.database_audit.entry.id",
                )?;
            }

            if event.has_value("oracle.database_audit.entry.id") {
                if let Some(val) = event.get("oracle.database_audit.entry.id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "oracle.database_audit.entry.id".into(),
                            message,
                        }
                    })?;
                    event.set("oracle.database_audit.entry.id", converted)?;
                }
            }

            if event.has_value("process.pid") {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            let _cond = { event.has_value("server.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("server.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("client.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("client.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("server.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("tmp_timestamp");
            event.remove("audit");
            event.remove("oracle_event_type");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
