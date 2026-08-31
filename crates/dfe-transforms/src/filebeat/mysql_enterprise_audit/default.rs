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
            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            let _cond = {
                event.get("message").is_some_and(|v| v.is_string())
                    && event.get_str("message").is_some_and(|s| s.ends_with(","))
            };
            if _cond {
                // Painless script
                // Source: ctx.message = ctx.message.substring(0, ctx.message.length() - 1);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.message = ctx.message.substring(0, ctx.message.length() - 1);"#
                    ),
                )?;
            }

            parse_json_field(event, "message", "mysqlenterprise.audit")?;

            event.remove("message");

            let _cond = { event.has_value("mysqlenterprise.audit.timestamp") };
            if _cond {
                event.remove("@timestamp");
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            let _cond = { event.get_str("mysqlenterprise.audit.class") == Some("connection") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = {
                [
                    "create_user",
                    "delete_user",
                    "drop_user",
                    "grant",
                    "flush_privileges",
                ]
                .contains(
                    &event
                        .get_str("mysqlenterprise.audit.general_data.sql_command")
                        .unwrap_or(""),
                )
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { event.get_str("mysqlenterprise.audit.class") != Some("audit") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("mysqlenterprise.audit.class") == Some("connection") };
            if _cond {
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("mysqlenterprise.audit.event") == Some("connect") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("mysqlenterprise.audit.event") == Some("disconnect") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("mysqlenterprise.audit.connection_data.status")
                    && event.get_i64("mysqlenterprise.audit.connection_data.status") == Some(0)
                    || event.has_value("mysqlenterprise.audit.general_data.status")
                        && event.get_i64("mysqlenterprise.audit.general_data.status") == Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("mysqlenterprise.audit.connection_data.status")
                    && event
                        .get_i64("mysqlenterprise.audit.connection_data.status")
                        .is_some_and(|n| n > 0)
                    || event.has_value("mysqlenterprise.audit.general_data.status")
                        && event
                            .get_i64("mysqlenterprise.audit.general_data.status")
                            .is_some_and(|n| n > 0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.has_value("mysqlenterprise.audit.event") };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "mysql-{}",
                        event
                            .get("mysqlenterprise.audit.event")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("mysqlenterprise.audit") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v instanceof String && v.isEmpty() == true);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v instanceof String && v.isEmpty() == true);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            if event.has_value("mysqlenterprise.audit.account.user") {
                event.rename("mysqlenterprise.audit.account.user", "server.user.name")?;
            }

            if event.has_value("mysqlenterprise.audit.account.host") {
                event.rename("mysqlenterprise.audit.account.host", "client.domain")?;
            }

            if event.has_value("mysqlenterprise.audit.login.os") {
                event.rename("mysqlenterprise.audit.login.os", "client.user.name")?;
            }

            if event.has_value("mysqlenterprise.audit.login.ip") {
                event.rename("mysqlenterprise.audit.login.ip", "client.ip")?;
            }

            if event.has_value("mysqlenterprise.audit.startup_data.os_version") {
                event.rename(
                    "mysqlenterprise.audit.startup_data.os_version",
                    "host.os.full",
                )?;
            }

            if event.has_value("mysqlenterprise.audit.startup_data.mysql_version") {
                event.rename(
                    "mysqlenterprise.audit.startup_data.mysql_version",
                    "service.version",
                )?;
            }

            if event.has_value("mysqlenterprise.audit.startup_data.server_id") {
                event.rename("mysqlenterprise.audit.startup_data.server_id", "service.id")?;
            }

            if event.has_value("mysqlenterprise.audit.startup_data.args") {
                event.rename("mysqlenterprise.audit.startup_data.args", "process.args")?;
            }

            event.set("process.name", json!("mysqld"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let joined = event.get("process.args").and_then(|v| join_values(v, " "));
                if let Some(joined) = joined {
                    event.set("process.command_line", json!(joined))?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("process.args") };
            if _cond {
                // Painless script
                // Source: ctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n  ctx.process.executable = ctx.process.args[0];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n  ctx.process.executable = ctx.process.args[0];\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                ["create_user", "delete_user", "drop_user"].contains(
                    &event
                        .get_str("mysqlenterprise.audit.general_data.sql_command")
                        .unwrap_or(""),
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("mysqlenterprise.audit.general_data.query")
                    {
                        // Grok pattern: (?i)(?:CREATE|DROP)\\s+USER(?:\\s+IF\\s+(?:NOT\\s+)?EXISTS)?\\s+(?:(?:(?P<__quote>['\"`]))(?P<user_target_name>(?:(?~\\k<__quote>)))(?:(?:\\k<__quote>))|(?P<user_target_name>(?:(?:[^\\s@;]*+))))(?:@(?:(?:(?P<__quote>['\"`]))(?P<user_target_domain>(?:(?~\\k<__quote>)))(?:(?:\\k<__quote>))|(?P<user_target_domain>(?:(?:[^\\s@;]*+)))))?
                        if !cached_grok_mapped!("(?i)(?:CREATE|DROP)\\s+USER(?:\\s+IF\\s+(?:NOT\\s+)?EXISTS)?\\s+(?:(?:(?P<__quote>['\"`]))(?P<user_target_name>(?:(?~\\k<__quote>)))(?:(?:\\k<__quote>))|(?P<user_target_name>(?:(?:[^\\s@;]*+))))(?:@(?:(?:(?P<__quote>['\"`]))(?P<user_target_domain>(?:(?~\\k<__quote>)))(?:(?:\\k<__quote>))|(?P<user_target_domain>(?:(?:[^\\s@;]*+)))))?", [("user_target_name", "user.target.name"), ("user_target_name", "user.target.name"), ("user_target_domain", "user.target.domain"), ("user_target_domain", "user.target.domain")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            event.remove("__quote");

            let _cond = { event.has_value("user.target") };
            if _cond {
                let v = json!(
                    event
                        .get("server.user.name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.get_str("mysqlenterprise.audit.general_data.sql_command")
                    == Some("create_user")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("mysqlenterprise.audit.general_data.sql_command") == Some("drop_user")
                    || event.get_str("mysqlenterprise.audit.general_data.sql_command")
                        == Some("delete_user")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
                event.append("event.type", json!("deletion"))?;
            }

            if event.has_value("mysqlenterprise.audit.connection_data.connection_attributes._pid") {
                if let Some(val) =
                    event.get("mysqlenterprise.audit.connection_data.connection_attributes._pid")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "mysqlenterprise.audit.connection_data.connection_attributes._pid"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            if event.has_value("mysqlenterprise.audit.connection_id") {
                if let Some(val) = event.get("mysqlenterprise.audit.connection_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "mysqlenterprise.audit.connection_id".into(),
                            message,
                        }
                    })?;
                    event.set("mysqlenterprise.audit.connection_id", converted)?;
                }
            }

            if event.has_value("mysqlenterprise.audit.id") {
                if let Some(val) = event.get("mysqlenterprise.audit.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "mysqlenterprise.audit.id".into(),
                            message,
                        }
                    })?;
                    event.set("mysqlenterprise.audit.id", converted)?;
                }
            }

            if event.has_value("mysqlenterprise.audit.shutdown_data.server_id") {
                if let Some(val) = event.get("mysqlenterprise.audit.shutdown_data.server_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "mysqlenterprise.audit.shutdown_data.server_id".into(),
                            message,
                        }
                    })?;
                    event.set("mysqlenterprise.audit.shutdown_data.server_id", converted)?;
                }
            }

            if event.has_value("service.id") {
                if let Some(val) = event.get("service.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "service.id".into(),
                            message,
                        }
                    })?;
                    event.set("service.id", converted)?;
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

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
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

            let _cond = { event.has_value("mysqlenterprise.audit.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mysqlenterprise.audit.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mysqlenterprise.audit.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("mysqlenterprise.audit.event");
            event.remove("mysqlenterprise.audit.timestamp");
            event.remove("mysqlenterprise.audit.connection_data.connection_attributes._pid");

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
