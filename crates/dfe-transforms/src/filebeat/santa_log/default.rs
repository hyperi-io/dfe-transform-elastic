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

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: \\[%{TIMESTAMP_ISO8601:_tmp.timestamp}\\] (?P<log_level>(?:[^\\|]+)) santad: %{GREEDYDATA:_tmp.message}
                let _ = cached_grok_mapped!("\\[%{TIMESTAMP_ISO8601:_tmp.timestamp}\\] (?P<log_level>(?:[^\\|]+)) santad: %{GREEDYDATA:_tmp.message}", [("log_level", "log.level")]).extract_into(&input, event)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("_tmp.message") {
                    if let Some(kv_str) = event.get_string("_tmp.message") {
                        for pair in cached_regex!("\\|").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.message".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("santa.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "kv_santa_fields")?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            event.remove("_tmp.message");

            if event.has_value("santa.gid") {
                event.rename("santa.gid", "group.id")?;
            }

            if event.has_value("santa.group") {
                event.rename("santa.group", "group.name")?;
            }

            if event.has_value("santa.newpath") {
                event.rename("santa.newpath", "file.target_path")?;
            }

            if event.has_value("santa.path") {
                event.rename("santa.path", "file.path")?;
            }

            if event.has_value("santa.pid") {
                event.rename("santa.pid", "process.pid")?;
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

            if event.has_value("santa.ppid") {
                event.rename("santa.ppid", "process.parent.pid")?;
            }

            if event.has_value("process.parent.pid") {
                if let Some(val) = event.get("process.parent.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.parent.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.parent.pid", converted)?;
                }
            }

            if event.has_value("santa.process") {
                event.rename("santa.process", "process.name")?;
            }

            if event.has_value("santa.processpath") {
                event.rename("santa.processpath", "process.executable")?;
            }

            if event.has_value("santa.sha256") {
                event.rename("santa.sha256", "process.hash.sha256")?;
            }

            if event.has_value("santa.uid") {
                event.rename("santa.uid", "user.id")?;
            }

            if event.has_value("santa.user") {
                event.rename("santa.user", "user.name")?;
            }

            if event.has_value("santa.cert_sha256") {
                event.rename("santa.cert_sha256", "santa.certificate.sha256")?;
            }

            if event.has_value("santa.cert_cn") {
                event.rename("santa.cert_cn", "santa.certificate.common_name")?;
            }

            if event.has_value("santa.event_uid") {
                event.rename("santa.event_uid", "santa.event.uid")?;
            }

            if event.has_value("santa.event_user") {
                event.rename("santa.event_user", "santa.event.user")?;
            }

            if event.has_value("santa.appearance") {
                event.rename("santa.appearance", "santa.disk.appearance")?;
            }

            if event.has_value("santa.bsdname") {
                event.rename("santa.bsdname", "santa.disk.bsdname")?;
            }

            if event.has_value("santa.bus") {
                event.rename("santa.bus", "santa.disk.bus")?;
            }

            if event.has_value("santa.dmgpath") {
                event.rename("santa.dmgpath", "santa.disk.dmgpath")?;
            }

            if event.has_value("santa.fs") {
                event.rename("santa.fs", "santa.disk.fs")?;
            }

            if event.has_value("santa.graphical_session_id") {
                if let Some(val) = event.get("santa.graphical_session_id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "santa.graphical_session_id".into(),
                            message,
                        }
                    })?;
                    event.set("santa.graphical_session_id", converted)?;
                }
            }

            if event.has_value("santa.model") {
                event.rename("santa.model", "santa.disk.model")?;
            }

            if event.has_value("santa.mount") {
                event.rename("santa.mount", "santa.disk.mount")?;
            }

            if event.has_value("santa.pidversion") {
                if let Some(val) = event.get("santa.pidversion") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "santa.pidversion".into(),
                            message,
                        }
                    })?;
                    event.set("santa.pidversion", converted)?;
                }
            }

            if event.has_value("santa.serial") {
                event.rename("santa.serial", "santa.disk.serial")?;
            }

            if event.has_value("santa.volume") {
                event.rename("santa.volume", "santa.disk.volume")?;
            }

            if event.has_value("santa.teamid") {
                event.rename("santa.teamid", "santa.team_id")?;
            }

            let _cond = { event.has_value("process.pid") && event.has_value("santa.pidversion") };
            if _cond {
                event.set(
                    "process.entity_id",
                    json!(format!(
                        "{}-{}",
                        event
                            .get("process.pid")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("santa.pidversion")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("agent.id") && event.has_value("process.entity_id") };
            if _cond {
                event.set(
                    "process.entity_id",
                    json!(format!(
                        "{}-{}",
                        event
                            .get("agent.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("process.entity_id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("_tmp.timestamp");

            let _cond = { event.has_value("process.pid") };
            if _cond {
                if let Some(v) = event
                    .get("@timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.start", v)?;
                }
            }

            let _cond = { event.has_value("santa.disk.appearance") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("santa.disk.appearance") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("santa.disk.appearance", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "santa.disk.appearance".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("santa.args") {
                    let parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                    event.set("santa.args", Value::Array(parts))?;
                }
                Ok(())
            })();

            let _cond =
                { event.has_value("process.pid") && !event.has_value("process.executable") };
            if _cond {
                if event.has_value("file.path") {
                    event.rename("file.path", "process.executable")?;
                }
            }

            let _cond = { event.has_value("process.executable") };
            if _cond {
                event.append(
                    "process.args",
                    json!(
                        event
                            .get("process.executable")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("santa.args") {
                foreach_array(event, "santa.args", |event| {
                    event.append(
                        "process.args",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            event.remove("santa.args");

            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("santa.action") == Some("EXEC") };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = { event.get_str("santa.action") == Some("EXEC") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("santa.decision") == Some("ALLOW") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("santa.decision") == Some("DENY") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let v = json!(
                event
                    .get("santa.action")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
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

            let _cond = { event.has_value("santa.certificate.sha256") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("santa.certificate.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.hash") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("santa.certificate.common_name") };
            if _cond {
                event.append(
                    "file.x509.issuer.common_name",
                    json!(
                        event
                            .get("santa.certificate.common_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

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
