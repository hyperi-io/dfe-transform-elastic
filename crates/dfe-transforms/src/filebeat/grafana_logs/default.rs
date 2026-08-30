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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("message", "event.original")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "grafana.log._json")?;
                Ok(())
            })();

            let _cond = { event.has_value("grafana.log._json.t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("grafana.log._json.t") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "grafana.log._json.t".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.level") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.level", "log.level")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.msg") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.msg", "message")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.logger") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.logger", "log.logger")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.caller") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.caller", "_temp.caller")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.method") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.method", "http.request.method")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.path") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.path", "url.path")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.status") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("grafana.log._json.status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "grafana.log._json.status".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.status_code", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.remote_addr") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.remote_addr", "client.ip")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.duration") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.duration", "_temp.duration_str")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.uname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.uname", "user.name")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.size") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.size", "http.response.body.bytes")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.referer") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.referer", "http.request.referrer")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.handler") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.handler", "grafana.log.handler")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.subUrl") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("grafana.log._json.subUrl", "grafana.log.subUrl")?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("grafana.log._json.orgId") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("grafana.log._json.orgId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "grafana.log._json.orgId".into(),
                                message,
                            }
                        })?;
                        event.set("grafana.log.orgId", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("grafana.log._json") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)t=%{TIMESTAMP_ISO8601:_temp.logfmt_t}(?:\\s|$)
                        let _ =
                            cached_grok!("(?:^|\\s)t=%{TIMESTAMP_ISO8601:_temp.logfmt_t}(?:\\s|$)")
                                .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)(?:level|lvl)=%{WORD:_temp.logfmt_level}(?:\\s|$)
                        let _ = cached_grok!(
                            "(?:^|\\s)(?:level|lvl)=%{WORD:_temp.logfmt_level}(?:\\s|$)"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)msg=\"%{DATA:_temp.logfmt_msg}\"
                        // Grok pattern: (?:^|\\s)msg=%{NOTSPACE:_temp.logfmt_msg}
                        let _ = extract_first_match(
                            &[
                                cached_grok!("(?:^|\\s)msg=\"%{DATA:_temp.logfmt_msg}\""),
                                cached_grok!("(?:^|\\s)msg=%{NOTSPACE:_temp.logfmt_msg}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)logger=%{NOTSPACE:_temp.logfmt_logger}(?:\\s|$)
                        let _ = cached_grok!(
                            "(?:^|\\s)logger=%{NOTSPACE:_temp.logfmt_logger}(?:\\s|$)"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)caller=%{NOTSPACE:_temp.logfmt_caller}(?:\\s|$)
                        let _ = cached_grok!(
                            "(?:^|\\s)caller=%{NOTSPACE:_temp.logfmt_caller}(?:\\s|$)"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)method=%{WORD:_temp.logfmt_method}(?:\\s|$)
                        let _ =
                            cached_grok!("(?:^|\\s)method=%{WORD:_temp.logfmt_method}(?:\\s|$)")
                                .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)path=%{NOTSPACE:_temp.logfmt_path}(?:\\s|$)
                        let _ =
                            cached_grok!("(?:^|\\s)path=%{NOTSPACE:_temp.logfmt_path}(?:\\s|$)")
                                .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)status=%{INT:_temp.logfmt_status}(?:\\s|$)
                        let _ = cached_grok!("(?:^|\\s)status=%{INT:_temp.logfmt_status}(?:\\s|$)")
                            .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)remote_addr=%{IP:_temp.logfmt_remote_addr}(?:\\s|$)
                        let _ = cached_grok!(
                            "(?:^|\\s)remote_addr=%{IP:_temp.logfmt_remote_addr}(?:\\s|$)"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)duration=%{NOTSPACE:_temp.logfmt_duration}(?:\\s|$)
                        let _ = cached_grok!(
                            "(?:^|\\s)duration=%{NOTSPACE:_temp.logfmt_duration}(?:\\s|$)"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)size=%{INT:_temp.logfmt_size}(?:\\s|$)
                        let _ = cached_grok!("(?:^|\\s)size=%{INT:_temp.logfmt_size}(?:\\s|$)")
                            .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^|\\s)uname=%{NOTSPACE:_temp.logfmt_uname}(?:\\s|$)
                        let _ =
                            cached_grok!("(?:^|\\s)uname=%{NOTSPACE:_temp.logfmt_uname}(?:\\s|$)")
                                .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_t") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp.logfmt_t") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp.logfmt_t".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_level") };
            if _cond {
                event.set(
                    "log.level",
                    json!(
                        event
                            .get("_temp.logfmt_level")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_msg") && !event.has_value("message") };
            if _cond {
                event.set(
                    "message",
                    json!(
                        event
                            .get("_temp.logfmt_msg")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_logger") };
            if _cond {
                event.set(
                    "log.logger",
                    json!(
                        event
                            .get("_temp.logfmt_logger")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_caller") };
            if _cond {
                event.set(
                    "_temp.caller",
                    json!(
                        event
                            .get("_temp.logfmt_caller")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_method") };
            if _cond {
                event.set(
                    "http.request.method",
                    json!(
                        event
                            .get("_temp.logfmt_method")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_path") };
            if _cond {
                event.set(
                    "url.path",
                    json!(
                        event
                            .get("_temp.logfmt_path")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_status") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("_temp.logfmt_status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp.logfmt_status".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.status_code", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_remote_addr") };
            if _cond {
                event.set(
                    "client.ip",
                    json!(
                        event
                            .get("_temp.logfmt_remote_addr")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_duration") };
            if _cond {
                event.set(
                    "_temp.duration_str",
                    json!(
                        event
                            .get("_temp.logfmt_duration")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.logfmt_size") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("_temp.logfmt_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp.logfmt_size".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.body.bytes", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.logfmt_uname") };
            if _cond {
                event.set(
                    "user.name",
                    json!(
                        event
                            .get("_temp.logfmt_uname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("http.request.method") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "http.request.method",
                        "http.request.method",
                        str::to_uppercase,
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.caller") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.caller") {
                        // Grok pattern: %{DATA:log.origin.file.name}:%{INT:log.origin.file.line}
                        let _ = cached_grok!(
                            "%{DATA:log.origin.file.name}:%{INT:log.origin.file.line}"
                        )
                        .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("log.origin.file.line") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("log.origin.file.line") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "log.origin.file.line".into(),
                                message,
                            }
                        })?;
                        event.set("log.origin.file.line", converted)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def dur = ctx._temp?.duration_str;\nif (dur == null) return;\nlong nanos;\nif (dur.endsWith('ms')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 2)) * 1000000.0);\n} else if (dur.endsWith(\"µs\") || dur.endsWith('us')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 2)) * 1000.0);\n} else if (dur.endsWith('ns')) {\n  nanos = (long)Double.parseDouble(dur.substring(0, dur.length() - 2));\n} else if (dur.endsWith('s')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 1)) * 1000000000.0);\n} else {\n  return;\n}\nctx.event.duration = nanos;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def dur = ctx._temp?.duration_str;\nif (dur == null) return;\nlong nanos;\nif (dur.endsWith('ms')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 2)) * 1000000.0);\n} else if (dur.endsWith(\"µs\") || dur.endsWith('us')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 2)) * 1000.0);\n} else if (dur.endsWith('ns')) {\n  nanos = (long)Double.parseDouble(dur.substring(0, dur.length() - 2));\n} else if (dur.endsWith('s')) {\n  nanos = (long)(Double.parseDouble(dur.substring(0, dur.length() - 1)) * 1000000000.0);\n} else {\n  return;\n}\nctx.event.duration = nanos;\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { !event.has_value("message") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("event.original").cloned() {
                        event.set("message", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def level = ctx.log?.level;\nif (level == null) return;\ndef lower = level.toLowerCase();\nif (lower == 'eror') ctx.log.level = 'error';\nelse if (lower == 'dbug') ctx.log.level = 'debug';\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def level = ctx.log?.level;\nif (level == null) return;\ndef lower = level.toLowerCase();\nif (lower == 'eror') ctx.log.level = 'error';\nelse if (lower == 'dbug') ctx.log.level = 'debug';\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def level = ctx.log?.level;\nif (level == null) return;\nlevel = level.toLowerCase();\nif (level == 'critical' || level == 'crit') ctx.event.severity = 2;\nelse if (level == 'error') ctx.event.severity = 3;\nelse if (level == 'warn' || level == 'warning') ctx.event.severity = 4;\nelse if (level == 'info') ctx.event.severity = 6;\nelse if (level == 'debug') ctx.event.severity = 7;\nelse if (level == 'trace') ctx.event.severity = 8;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def level = ctx.log?.level;\nif (level == null) return;\nlevel = level.toLowerCase();\nif (level == 'critical' || level == 'crit') ctx.event.severity = 2;\nelse if (level == 'error') ctx.event.severity = 3;\nelse if (level == 'warn' || level == 'warning') ctx.event.severity = 4;\nelse if (level == 'info') ctx.event.severity = 6;\nelse if (level == 'debug') ctx.event.severity = 7;\nelse if (level == 'trace') ctx.event.severity = 8;\n"#
                    ),
                )?;
                Ok(())
            })();

            event.remove("grafana.log._json");
            event.remove("_temp");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
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
                        "Processor {} with tag {} failed with message {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
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
