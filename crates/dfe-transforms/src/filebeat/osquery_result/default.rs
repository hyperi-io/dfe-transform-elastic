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
            parse_json_field(event, "message", "json")?;

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.rename("@timestamp", "event.created")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.unixTime") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.unixTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: def dict = ['result': new HashMap()]; \nfor (entry in ctx['json'].entrySet()) { \n    dict['result'][entry.getKey()] = entry.getValue(); \n} \nctx['osquery'] = dict; \nctx.remove('json');\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def dict = ['result': new HashMap()]; \nfor (entry in ctx['json'].entrySet()) { \n    dict['result'][entry.getKey()] = entry.getValue(); \n} \nctx['osquery'] = dict; \nctx.remove('json');\n"#
                ),
            )?;

            let _cond = { event.has_value("osquery.result") };
            if _cond {
                // Painless script
                // Source: for (key in params.keys) {\n    if (ctx['osquery']['result'][key] == null) {\n        continue;\n    }\n\n    def dict = new HashMap();\n    for (entry in ctx['osquery']['result'][key].entrySet()) { \n        if (entry.getValue() != '') {\n            dict[entry.getKey()] = entry.getValue();\n        }\n    }\n    ctx['osquery']['result'][key] = dict; \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (key in params.keys) {\n    if (ctx['osquery']['result'][key] == null) {\n        continue;\n    }\n\n    def dict = new HashMap();\n    for (entry in ctx['osquery']['result'][key].entrySet()) { \n        if (entry.getValue() != '') {\n            dict[entry.getKey()] = entry.getValue();\n        }\n    }\n    ctx['osquery']['result'][key] = dict; \n}\n"#
                    ),
                    cached_params!("{\"keys\":[\"columns\",\"decorations\"]}"),
                )?;
            }

            if event.has_value("osquery.result.hostIdentifier") {
                event.rename(
                    "osquery.result.hostIdentifier",
                    "osquery.result.host_identifier",
                )?;
            }

            if event.has_value("osquery.result.unixTime") {
                event.rename("osquery.result.unixTime", "osquery.result.unix_time")?;
            }

            if event.has_value("osquery.result.calendarTime") {
                event.rename(
                    "osquery.result.calendarTime",
                    "osquery.result.calendar_time",
                )?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let v = json!(
                event
                    .get("osquery.result.action")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.action", v)?;
            }

            let _cond = { event.has_value("osquery.result.columns.atime") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("osquery.result.columns.atime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("file.accessed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "osquery.result.columns.atime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("osquery.result.columns.ctime") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("osquery.result.columns.ctime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("file.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "osquery.result.columns.ctime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("osquery.result.columns.mtime") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("osquery.result.columns.mtime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("file.mtime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "osquery.result.columns.mtime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let v = json!(
                event
                    .get("osquery.result.columns.directory")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.directory", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.filename")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.name", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.gid")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.gid", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.inode")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.inode", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.mode")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.mode", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.path")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.path", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.size")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.size", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.type")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.type", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.uid")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("file.uid", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.decorations.username")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("osquery.result.host_identifier")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.hostname", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.decorations.host_uuid")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.id", v)?;
            }

            let v = json!(
                event
                    .get("osquery.result.columns.process")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.name", v)?;
            }

            let _cond = { event.get_str("osquery.result.columns.source_url") != Some("null") };
            if _cond {
                let v = json!(
                    event
                        .get("osquery.result.columns.source_url")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("url.full", v)?;
                }
            }

            let v = json!(
                event
                    .get("osquery.result.name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("rule.name", v)?;
            }

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("file.size") {
                if let Some(val) = event.get("file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "file.size".into(),
                            message,
                        }
                    })?;
                    event.set("file.size", converted)?;
                }
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
