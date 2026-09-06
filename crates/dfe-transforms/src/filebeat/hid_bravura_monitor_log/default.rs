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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (^[[:cntrl:]])?%{TIMESTAMP_ISO8601:logdate}.%{NONNEGINT} - \\[(%{DATA:pslogid})?\\] %{DATA:log.logger} \\[%{NONNEGINT:process.pid},%{NONNEGINT:process.thread.id}\\] %{DATA:log.level}: (?P<msg>(?:(.|\n)*))
                if !cached_grok!("(^[[:cntrl:]])?%{TIMESTAMP_ISO8601:logdate}.%{NONNEGINT} - \\[(%{DATA:pslogid})?\\] %{DATA:log.logger} \\[%{NONNEGINT:process.pid},%{NONNEGINT:process.thread.id}\\] %{DATA:log.level}: (?P<msg>(?:(.|\n)*))").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            let _cond = {
                event.get("msg").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("last message repeated")),
                    serde_json::Value::String(s) => s.contains("last message repeated"),
                    _ => false,
                })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get("log.level").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Perf")),
                    serde_json::Value::String(s) => s.contains("Perf"),
                    _ => false,
                })
            };
            if _cond {
                if event.has_value("event.original") {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (^[[:cntrl:]])?%{TIMESTAMP_ISO8601}.%{NONNEGINT} - \\[%{DATA}\\] %{DATA} \\[%{NONNEGINT},%{NONNEGINT}\\] %{DATA}: %{NOTSPACE:hid_bravura_monitor.perf.kind}. %{GREEDYDATA:kvpairs}
                        if !cached_grok!("(^[[:cntrl:]])?%{TIMESTAMP_ISO8601}.%{NONNEGINT} - \\[%{DATA}\\] %{DATA} \\[%{NONNEGINT},%{NONNEGINT}\\] %{DATA}: %{NOTSPACE:hid_bravura_monitor.perf.kind}. %{GREEDYDATA:kvpairs}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
            }

            let _cond = {
                event.get("log.level").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Perf")),
                    serde_json::Value::String(s) => s.contains("Perf"),
                    _ => false,
                })
            };
            if _cond {
                event.set("log.level", json!("Perf"))?;
            }

            let _cond = {
                event.get("log.level").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Perf")),
                    serde_json::Value::String(s) => s.contains("Perf"),
                    _ => false,
                })
            };
            if _cond {
                if event.has_value("kvpairs") {
                    if let Some(kv_str) = event.get_string("kvpairs") {
                        for pair in cached_regex!(" \\| ").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(": ") else {
                                return Err(TransformError::ParseError {
                                    path: "kvpairs".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = key.trim_matches(|c| " \\r\\n".contains(c));
                                let value = value.trim_matches(|c| " {}\\r\\n".contains(c));
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("hid_bravura_monitor.perf.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("hid_bravura_monitor.perf.kind") == Some("PerfAjax") };
            if _cond {
                if event.has_value("hid_bravura_monitor.perf.User") {
                    event.rename("hid_bravura_monitor.perf.User", "user.id")?;
                }
            }

            let _cond = {
                event.get("log.level").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Perf")),
                    serde_json::Value::String(s) => s.contains("Perf"),
                    _ => false,
                })
            };
            if _cond {
                // Painless script
                // Source: Map m = new HashMap(); ctx['hid_bravura_monitor']['perf'].forEach((k,v) -> m.put(k.toLowerCase(), v)); ctx['hid_bravura_monitor'].remove('perf'); ctx['hid_bravura_monitor']['perf'] = new HashMap(); m.forEach((k,v) -> ctx['hid_bravura_monitor']['perf'][k] = v );
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map m = new HashMap(); ctx['hid_bravura_monitor']['perf'].forEach((k,v) -> m.put(k.toLowerCase(), v)); ctx['hid_bravura_monitor'].remove('perf'); ctx['hid_bravura_monitor']['perf'] = new HashMap(); m.forEach((k,v) -> ctx['hid_bravura_monitor']['perf'][k] = v );"#
                    ),
                )?;
            }

            let _cond = { event.get_str("hid_bravura_monitor.perf.kind") == Some("PerfExe") };
            if _cond {
                if let Some(v) = event
                    .get("log.logger")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("hid_bravura_monitor.perf.exe", v)?;
                }
            }

            event.remove("kvpairs");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("pslogid") {
                    if let Some(input) = event.get_string("pslogid") {
                        // Grok pattern: %{UUID:hid_bravura_monitor.request.id}
                        // Grok pattern: %{[A-Fa-f0-9]{32}:hid_bravura_monitor.request.id}
                        if !extract_first_match(
                            &[
                                cached_grok!("%{UUID:hid_bravura_monitor.request.id}"),
                                cached_grok!("%{[A-Fa-f0-9]{32}:hid_bravura_monitor.request.id}"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                !event.has_value("hid_bravura_monitor.request.id")
                    && event.get_str("hid_bravura_monitor.perf.kind") != Some("PerfAjax")
            };
            if _cond {
                if event.has_value("pslogid") {
                    event.rename("pslogid", "user.id")?;
                }
            }

            event.remove("pslogid");

            if let Some(date_str) = event.get_as_string("logdate") {
                match parse_date_out(
                    &date_str,
                    &["yyyy-MM-dd HH:mm:ss.SSS"],
                    event.get_str("event.timezone"),
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "logdate".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("msg", "message")?;

            if event.remove("logdate").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "logdate".into(),
                });
            }

            let _cond = { event.get_str("hid_bravura_monitor.node") == Some("0.0.0.0") };
            if _cond {
                if let Some(v) = event
                    .get("host.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("hid_bravura_monitor.node", v)?;
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

            if event.has_value("process.thread.id") {
                if let Some(val) = event.get("process.thread.id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.thread.id".into(),
                            message,
                        }
                    })?;
                    event.set("process.thread.id", converted)?;
                }
            }

            if event.has_value("hid_bravura_monitor.perf.duration") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.duration") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.duration".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.duration", converted)?;
                }
            }

            if event.has_value("hid_bravura_monitor.perf.kernel") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.kernel") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.kernel".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.kernel", converted)?;
                }
            }

            if event.has_value("hid_bravura_monitor.perf.user") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.user") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.user".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.user", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "", "hid_bravura_monitor.perf.kind")?;
                Ok(())
            })();

            if event.has_value("hid_bravura_monitor.perf.line") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.line") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.line".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.line", converted)?;
                }
            }

            if event.has_value("hid_bravura_monitor.perf.records") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.records") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.records".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.records", converted)?;
                }
            }

            if event.has_value("hid_bravura_monitor.perf.result") {
                if let Some(val) = event.get("hid_bravura_monitor.perf.result") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hid_bravura_monitor.perf.result".into(),
                            message,
                        }
                    })?;
                    event.set("hid_bravura_monitor.perf.result", converted)?;
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
