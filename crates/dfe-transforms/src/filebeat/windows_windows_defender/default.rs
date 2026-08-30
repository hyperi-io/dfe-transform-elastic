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

            let _cond = { event.get_str("winlog.level") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.level")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("log.level", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.time_created") };
            if _cond {
                // on_failure: 3 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("winlog.time_created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "winlog.time_created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.remove("winlog.time_created").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "winlog.time_created".into(),
                            });
                        }
                        Ok(())
                    })();
                    event.append(
                        "error.message",
                        json!(format!(
                            "fail-{}",
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
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
            }

            if event.has_value("error.code") {
                if let Some(val) = event.get("error.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "error.code".into(),
                            message,
                        }
                    })?;
                    event.set("error.code", converted)?;
                }
            }

            event.set(
                "event.code",
                json!(
                    event
                        .get("winlog.event_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("event.code") == Some("5007") };
            if _cond {
                event.set("event.action", json!("antivirus-configuration-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("2000") };
            if _cond {
                event.set("event.action", json!("antivirus-updated"))?;
            }

            let _cond = { event.get_str("event.code") == Some("2050") };
            if _cond {
                event.set("event.action", json!("file-uploaded"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("1116")
                    || event.get_str("event.code") == Some("1117")
            };
            if _cond {
                event.set("event.action", json!("malware-detected"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("1117")
                    && event.get_str("winlog.event_data.Action_Name") == Some("Quarantine")
            };
            if _cond {
                event.set("event.action", json!("malware-quarantined"))?;
            }

            let _cond = { event.get_str("event.code") == Some("5007") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = { event.get_str("event.code") == Some("2050") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("file")]))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("1116")
                    || event.get_str("event.code") == Some("1117")
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("malware")]))?;
            }

            let _cond = { !event.has_value("event.category") };
            if _cond {
                if !event.has("event.category") {
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_str("winlog.event_data.Error_Description")
                    == Some("The operation completed successfully. ")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if let Some(v) = event
                .get("winlog.event_data.FWLink")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            // Painless script
            // Source: if (ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'configuration' && ctx?.event?.code == '5007') {\n      ctx.event.type = ['change'];\n      break;\n    }\n  }\n}\nif (ctx?.event?.type == null && ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'malware' || category == 'file' || category == 'process') {\n      ctx.event.type = ['info'];\n      break;\n    }\n  }\n}\nif (ctx?.event?.type == null && ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'process') {\n      ctx.event.type = ['start'];\n      break;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'configuration' && ctx?.event?.code == '5007') {\n      ctx.event.type = ['change'];\n      break;\n    }\n  }\n}\nif (ctx?.event?.type == null && ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'malware' || category == 'file' || category == 'process') {\n      ctx.event.type = ['info'];\n      break;\n    }\n  }\n}\nif (ctx?.event?.type == null && ctx?.event?.category instanceof List) {\n  for (category in ctx.event.category) {\n    if (category == 'process') {\n      ctx.event.type = ['start'];\n      break;\n    }\n  }\n}\n"#
                ),
            )?;

            if let Some(v) = event
                .get("winlog.event_data.Destination")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("destination.domain") {
                    event.set("destination.domain", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.record_id") {
                    if let Some(val) = event.get("winlog.record_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.record_id".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.record_id", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("winlog.event_data.Detection_User") {
                if let Some(s) = event.get_string("winlog.event_data.Detection_User") {
                    let mut parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_temp.user_parts", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("winlog.event_data.User") };
            if _cond {
                if let Some(s) = event.get_string("winlog.event_data.User") {
                    let mut parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_temp.user_parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.user_parts.0")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.user_parts.1")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp") };
            if _cond {
                event.remove("_temp");
            }

            let _cond = { event.has_value("winlog.message") };
            if _cond {
                event.remove("winlog.message");
            }

            if event.has_value("winlog.event_data.Path") {
                if let Some(input) = event.get_string("winlog.event_data.Path") {
                    // Grok pattern: file:_(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?
                    // Grok pattern: (?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "file:_(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?",
                                [("file_path", "file.path")]
                            ),
                            cached_grok_mapped!(
                                "(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?",
                                [("file_path", "file.path")]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.event_data.Path") {
                    if let Some(s) = event.get_string("winlog.event_data.Path") {
                        let mut parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("windows_defender.evidence_paths", Value::Array(parts))?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("windows_defender.evidence_paths") {
                    map_strings(
                        event,
                        "windows_defender.evidence_paths",
                        "windows_defender.evidence_paths",
                        |s| s.trim().to_string(),
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("windows_defender.evidence_paths") {
                    gsub_field(
                        event,
                        "windows_defender.evidence_paths",
                        "windows_defender.evidence_paths",
                        cached_regex!("file:_"),
                        "",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("windows_defender.evidence_paths") {
                    gsub_field(
                        event,
                        "windows_defender.evidence_paths",
                        "windows_defender.evidence_paths",
                        cached_regex!("process:_"),
                        "",
                    )?;
                }
                Ok(())
            })();

            if event.has_value("winlog.event_data.FileName") {
                if let Some(input) = event.get_string("winlog.event_data.FileName") {
                    // Grok pattern: file:_(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?
                    // Grok pattern: (?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "file:_(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?",
                                [("file_path", "file.path")]
                            ),
                            cached_grok_mapped!(
                                "(?P<file_path>[^;]+)(; process:_pid:%{NUMBER:process.pid})?",
                                [("file_path", "file.path")]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if let Some(v) = event
                .get("winlog.event_data.Path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("file.path") {
                    event.set("file.path", v)?;
                }
            }

            if let Some(v) = event
                .get("winlog.event_data.Filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("file.path") {
                    event.set("file.path", v)?;
                }
            }

            let _cond = { !event.has_value("file.name") };
            if _cond {
                if event.has_value("file.path") {
                    if let Some(input) = event.get_string("file.path") {
                        // Grok pattern: (?P<file_name>([^\\\\\\\\]*$))
                        let _ = cached_grok_mapped!(
                            "(?P<file_name>([^\\\\\\\\]*$))",
                            [("file_name", "file.name")]
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event.has_value("file.name")
                    && event.get("file.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("file.name") {
                    if let Some(input) = event.get_string("file.name") {
                        // Grok pattern: \\.%{GREEDYDATA:file.extension}$
                        let _ = cached_grok!("\\.%{GREEDYDATA:file.extension}$")
                            .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = { event.get_str("winlog.event_data.Sha256") != Some("-") };
            if _cond {
                if let Some(v) = event
                    .get("winlog.event_data.Sha256")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("file.hash.sha256") {
                        event.set("file.hash.sha256", v)?;
                    }
                }
            }

            if event.has_value("winlog.event_data.Target_Commandline") {
                event.rename(
                    "winlog.event_data.Target_Commandline",
                    "process.command_line",
                )?;
            }

            if event.has_value("winlog.event_data.Parent_Commandline") {
                event.rename(
                    "winlog.event_data.Parent_Commandline",
                    "process.parent.command_line",
                )?;
            }

            let _cond = {
                !event.has_value("process.executable")
                    && event.get_str("event.code") != Some("1126")
            };
            if _cond {
                if event.has_value("winlog.event_data.Process_Name") {
                    event.rename("winlog.event_data.Process_Name", "process.executable")?;
                }
            }

            let _cond =
                { !event.has_value("process.name") && event.get_str("event.code") == Some("1126") };
            if _cond {
                if event.has_value("winlog.event_data.Process_Name") {
                    event.rename("winlog.event_data.Process_Name", "process.name")?;
                }
            }

            let _cond = { !event.has_value("process.name") };
            if _cond {
                if event.has_value("process.executable") {
                    if let Some(input) = event.get_string("process.executable") {
                        // Grok pattern: (?P<process_name>([^\\\\]*$))
                        let _ = cached_grok_mapped!(
                            "(?P<process_name>([^\\\\]*$))",
                            [("process_name", "process.name")]
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            if let Some(v) = event
                .get("winlog.event_data.Destination")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("destination.domain") {
                    event.set("destination.domain", v)?;
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
