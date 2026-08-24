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
            let _cond = { event.get_str("winlog.event_id") == Some("4103") };
            if _cond {
                if let Some(kv_str) = event.get_string("winlog.event_data.ContextInfo") {
                    for pair in cached_regex!("\\n(?!\\n)\\s+").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = ({
                            let parts = cached_regex!("[:=]").splitn(&pair, 2);
                            match (parts.first(), parts.get(1)) {
                                (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                _ => None,
                            }
                        }) else {
                            return Err(TransformError::ParseError {
                                path: "winlog.event_data.ContextInfo".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " \n\t".contains(c));
                            let value = value.trim_matches(|c| " \n\t".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("winlog.event_data.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("winlog.event_data") };
            if _cond {
                // Painless script
                // Source: def newEventData = new HashMap();\nfor (entry in ctx.winlog.event_data.entrySet()) {\n  def newKey = /\\s/.matcher(entry.getKey().toString()).replaceAll(\"\");\n  newEventData.put(newKey, entry.getValue());\n}\nctx.winlog.event_data = newEventData;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def newEventData = new HashMap();\nfor (entry in ctx.winlog.event_data.entrySet()) {\n  def newKey = /\\s/.matcher(entry.getKey().toString()).replaceAll(\"\");\n  newEventData.put(newKey, entry.getValue());\n}\nctx.winlog.event_data = newEventData;"#
                    ),
                )?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { event.has_value("winlog.event_data") };
            if _cond {
                // Painless script
                // Source: ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))"#
                    ),
                )?;
            }

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

            event.set("event.kind", json!("event"))?;

            event.set(
                "event.code",
                json!(
                    event
                        .get("winlog.event_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("event.category", Value::Array(vec![json!("process")]))?;

            let _cond = { event.get_str("event.code") == Some("4105") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = { event.get_str("event.code") == Some("4106") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.event_data.SequenceNumber") {
                    if let Some(val) = event.get("winlog.event_data.SequenceNumber") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.event_data.SequenceNumber".into(),
                                message,
                            }
                        })?;
                        event.set("event.sequence", converted)?;
                    }
                }
                Ok(())
            })();

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

            let _cond = { event.get_str("winlog.event_data.HostID") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.HostID") {
                        event.rename("winlog.event_data.HostID", "process.entity_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.HostApplication") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.HostApplication") {
                        event
                            .rename("winlog.event_data.HostApplication", "process.command_line")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.HostName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.HostName") {
                        event.rename("winlog.event_data.HostName", "process.title")?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.process.pid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pid", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.user.identifier")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.user.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.user.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.event_data.ConnectedUser") };
            if _cond {
                if let Some(s) = event.get_string("winlog.event_data.ConnectedUser") {
                    let parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("_temp.connected_user_parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("_temp.connected_user_parts") && event.get("_temp.connected_user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.connected_user_parts.0")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("source.user.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("_temp.connected_user_parts") && event.get("_temp.connected_user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.connected_user_parts.1")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("source.user.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("user.domain") {
                        event.rename("user.domain", "destination.user.domain")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("user.name") {
                        event.rename("user.name", "destination.user.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("source.user.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("source.user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("winlog.event_data._MemberUserName") {
                    event.rename("winlog.event_data._MemberUserName", "user.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("winlog.event_data._MemberDomain") {
                    event.rename("winlog.event_data._MemberDomain", "user.domain")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("winlog.event_data._MemberAccountType") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("winlog.event_data._MemberAccountType")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("user.roles")
                    && event.has_value("winlog.event_data._MemberAccountType")
                    && event.get("user.roles").is_some_and(|v| {
                        match (v, event.get("winlog.event_data._MemberAccountType")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("winlog.event_data._MemberAccountType");
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.event_data.MessageNumber") {
                    if let Some(val) = event.get("winlog.event_data.MessageNumber") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.event_data.MessageNumber".into(),
                                message,
                            }
                        })?;
                        event.set("powershell.sequence", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.event_data.MessageTotal") {
                    if let Some(val) = event.get("winlog.event_data.MessageTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.event_data.MessageTotal".into(),
                                message,
                            }
                        })?;
                        event.set("powershell.total", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("winlog.event_data.ShellID") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ShellID") {
                        event.rename("winlog.event_data.ShellID", "powershell.id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.EngineVersion") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.EngineVersion") {
                        event.rename(
                            "winlog.event_data.EngineVersion",
                            "powershell.engine.version",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.PipelineID") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.PipelineID") {
                        event.rename("winlog.event_data.PipelineID", "powershell.pipeline_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.RunspaceID") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.RunspaceID") {
                        event.rename("winlog.event_data.RunspaceID", "powershell.runspace_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.RunspaceId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.RunspaceId") {
                        event.rename("winlog.event_data.RunspaceId", "powershell.runspace_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.HostVersion") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.HostVersion") {
                        event.rename(
                            "winlog.event_data.HostVersion",
                            "powershell.process.executable_version",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandLine") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CommandLine") {
                        event
                            .rename("winlog.event_data.CommandLine", "powershell.command.value")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandPath") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CommandPath") {
                        event.rename("winlog.event_data.CommandPath", "powershell.command.path")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CommandName") {
                        event.rename("winlog.event_data.CommandName", "powershell.command.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandType") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CommandType") {
                        event.rename("winlog.event_data.CommandType", "powershell.command.type")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.ScriptBlockId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ScriptBlockId") {
                        event.rename(
                            "winlog.event_data.ScriptBlockId",
                            "powershell.file.script_block_id",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.ScriptBlockText") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ScriptBlockText") {
                        event.rename(
                            "winlog.event_data.ScriptBlockText",
                            "powershell.file.script_block_text",
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("powershell.file.script_block_text") {
                map_strings(
                    event,
                    "powershell.file.script_block_text",
                    "powershell.file.script_block_text",
                    |s| s.trim().to_string(),
                )?;
            }

            if event.has_value("powershell.file.script_block_text") {
                gsub_field(
                    event,
                    "powershell.file.script_block_text",
                    "_temp.script_block_no_space",
                    cached_regex!("\\s"),
                    "",
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("_temp.script_block_no_space") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "powershell.file.script_block_hash",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            if event.has_value("powershell.file.script_block_text") {
                gsub_field(
                    event,
                    "powershell.file.script_block_text",
                    "_temp.script_block_no_signature",
                    cached_regex!("(?s)# SIG # Begin signature block.+"),
                    "",
                )?;
            }

            let _cond = { event.has_value("_temp.script_block_no_signature") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: // Entropy Variance from: https://github.com/elastic/toutoumomoma/blob/be287c9c0d0e435572e3889a6584199983c688f0/toutoumomoma.go#L326-L363.\nString script = ctx._temp.script_block_no_signature;\n\nint length = script.length();\nif (length == 0) {\n  return;\n}\n\n// Skip signature only scripts:\n// - Inspect only line 2 (line 1 can be truncated mid-signature).\n// - Match \"# \" + base64-ish content at the fixed signature line length (64 chars).\nint lf = 10;    // '\\n'\nint cr = 13;    // '\\r'\nint hash = 35;  // '#'\nint space = 32; // ' '\nint sigLineLen = 64; // Content length, excluding \"# \" prefix.\n\nint firstLineEnd = -1;\nfor (int idx = 0; idx < length; idx++) {\n  if (script.charAt(idx) == lf) {\n    firstLineEnd = idx;\n    break;\n  }\n}\n\nif (firstLineEnd > 0) {\n  int secondLineStart = firstLineEnd + 1;\n  if (secondLineStart < length) {\n    int secondLineEnd = length;\n    for (int idx = secondLineStart; idx < length; idx++) {\n      if (script.charAt(idx) == lf) {\n        secondLineEnd = idx;\n        break;\n      }\n    }\n\n    if (secondLineEnd > secondLineStart && script.charAt(secondLineEnd - 1) == cr) {\n      secondLineEnd--;\n    }\n\n    if (secondLineStart < secondLineEnd && script.charAt(secondLineStart) == hash) {\n      int contentStart = secondLineStart + 1;\n      if (contentStart < secondLineEnd && script.charAt(contentStart) == space) {\n        contentStart++;\n      }\n      int lineLen = secondLineEnd - contentStart;\n      if (lineLen == sigLineLen) {\n        boolean base64Line = true;\n        for (int i = contentStart; i < secondLineEnd; i++) {\n          int c = (int) script.charAt(i);\n          if (!((c >= 65 && c <= 90) || (c >= 97 && c <= 122) ||\n                (c >= 48 && c <= 57) || c == 43 || c == 47 || c == 61)) {\n            base64Line = false;\n            break;\n          }\n        }\n        if (base64Line) {\n          return;\n        }\n      }\n    }\n  }\n}\n\nscript = java.text.Normalizer.normalize(script, java.text.Normalizer.Form.NFC);\n\nlength = script.length();\nif (length == 0) {\n  return;\n}\n\nint[] counts = new int[65536];\nint[] seen = new int[length];\nint seenCount = 0;\nint uniqueSymbols = 0;\nfor (int i = 0; i < length; i++) {\n    int ch = script.charAt(i);\n    if (counts[ch] == 0) {\n        counts[ch] = 1;\n        seen[seenCount++] = ch;\n        uniqueSymbols++;\n    } else {\n        counts[ch]++;\n    }\n}\n\ndouble invLog2 = 1.0 / Math.log(2.0);\ndouble entropy = 0.0;\ndouble surprisalVar = 0.0;\ndouble pSum = 0.0;\n\nfor (int i = 0; i < seenCount; i++) {\n    int code = seen[i];\n    double cnt = (double) counts[code];\n    double p = cnt / (double) length;\n    double l2p = Math.log(p) * invLog2;\n\n    pSum += p;\n    double tmp = entropy;\n    entropy = tmp + (p / pSum) * (l2p - tmp);\n    surprisalVar += p * (l2p - tmp) * (l2p - entropy);\n}\n\nsurprisalVar = Math.max(0.0, surprisalVar);\ndouble surprisalSd = Math.sqrt(surprisalVar);\ndouble entropyBits = -entropy;\nif (entropyBits == -0.0) {\n    entropyBits = 0.0;\n}\n\ndouble normalizedEntropy = 0.0;\nif (length > 1) {\n    double maxEntropy = Math.log((double) length) * invLog2; // max bits if every character is unique\n    normalizedEntropy = entropyBits / maxEntropy;           // scale 0..1 against script length\n    if (normalizedEntropy < 0.0) normalizedEntropy = 0.0;\n    else if (normalizedEntropy > 1.0) normalizedEntropy = 1.0;\n}\n\nctx.powershell.file.script_block_entropy_bits = entropyBits;\nctx.powershell.file.script_block_entropy_normalized = normalizedEntropy;\nctx.powershell.file.script_block_surprisal_stdev = surprisalSd;\nctx.powershell.file.script_block_length = length;\nctx.powershell.file.script_block_unique_symbols = uniqueSymbols;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r##"// Entropy Variance from: https://github.com/elastic/toutoumomoma/blob/be287c9c0d0e435572e3889a6584199983c688f0/toutoumomoma.go#L326-L363.\nString script = ctx._temp.script_block_no_signature;\n\nint length = script.length();\nif (length == 0) {\n  return;\n}\n\n// Skip signature only scripts:\n// - Inspect only line 2 (line 1 can be truncated mid-signature).\n// - Match \"# \" + base64-ish content at the fixed signature line length (64 chars).\nint lf = 10;    // '\\n'\nint cr = 13;    // '\\r'\nint hash = 35;  // '#'\nint space = 32; // ' '\nint sigLineLen = 64; // Content length, excluding \"# \" prefix.\n\nint firstLineEnd = -1;\nfor (int idx = 0; idx < length; idx++) {\n  if (script.charAt(idx) == lf) {\n    firstLineEnd = idx;\n    break;\n  }\n}\n\nif (firstLineEnd > 0) {\n  int secondLineStart = firstLineEnd + 1;\n  if (secondLineStart < length) {\n    int secondLineEnd = length;\n    for (int idx = secondLineStart; idx < length; idx++) {\n      if (script.charAt(idx) == lf) {\n        secondLineEnd = idx;\n        break;\n      }\n    }\n\n    if (secondLineEnd > secondLineStart && script.charAt(secondLineEnd - 1) == cr) {\n      secondLineEnd--;\n    }\n\n    if (secondLineStart < secondLineEnd && script.charAt(secondLineStart) == hash) {\n      int contentStart = secondLineStart + 1;\n      if (contentStart < secondLineEnd && script.charAt(contentStart) == space) {\n        contentStart++;\n      }\n      int lineLen = secondLineEnd - contentStart;\n      if (lineLen == sigLineLen) {\n        boolean base64Line = true;\n        for (int i = contentStart; i < secondLineEnd; i++) {\n          int c = (int) script.charAt(i);\n          if (!((c >= 65 && c <= 90) || (c >= 97 && c <= 122) ||\n                (c >= 48 && c <= 57) || c == 43 || c == 47 || c == 61)) {\n            base64Line = false;\n            break;\n          }\n        }\n        if (base64Line) {\n          return;\n        }\n      }\n    }\n  }\n}\n\nscript = java.text.Normalizer.normalize(script, java.text.Normalizer.Form.NFC);\n\nlength = script.length();\nif (length == 0) {\n  return;\n}\n\nint[] counts = new int[65536];\nint[] seen = new int[length];\nint seenCount = 0;\nint uniqueSymbols = 0;\nfor (int i = 0; i < length; i++) {\n    int ch = script.charAt(i);\n    if (counts[ch] == 0) {\n        counts[ch] = 1;\n        seen[seenCount++] = ch;\n        uniqueSymbols++;\n    } else {\n        counts[ch]++;\n    }\n}\n\ndouble invLog2 = 1.0 / Math.log(2.0);\ndouble entropy = 0.0;\ndouble surprisalVar = 0.0;\ndouble pSum = 0.0;\n\nfor (int i = 0; i < seenCount; i++) {\n    int code = seen[i];\n    double cnt = (double) counts[code];\n    double p = cnt / (double) length;\n    double l2p = Math.log(p) * invLog2;\n\n    pSum += p;\n    double tmp = entropy;\n    entropy = tmp + (p / pSum) * (l2p - tmp);\n    surprisalVar += p * (l2p - tmp) * (l2p - entropy);\n}\n\nsurprisalVar = Math.max(0.0, surprisalVar);\ndouble surprisalSd = Math.sqrt(surprisalVar);\ndouble entropyBits = -entropy;\nif (entropyBits == -0.0) {\n    entropyBits = 0.0;\n}\n\ndouble normalizedEntropy = 0.0;\nif (length > 1) {\n    double maxEntropy = Math.log((double) length) * invLog2; // max bits if every character is unique\n    normalizedEntropy = entropyBits / maxEntropy;           // scale 0..1 against script length\n    if (normalizedEntropy < 0.0) normalizedEntropy = 0.0;\n    else if (normalizedEntropy > 1.0) normalizedEntropy = 1.0;\n}\n\nctx.powershell.file.script_block_entropy_bits = entropyBits;\nctx.powershell.file.script_block_entropy_normalized = normalizedEntropy;\nctx.powershell.file.script_block_surprisal_stdev = surprisalSd;\nctx.powershell.file.script_block_length = length;\nctx.powershell.file.script_block_unique_symbols = uniqueSymbols;"##
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("4103") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.Payload") {
                        if let Some(s) = event.get_string("winlog.event_data.Payload") {
                            let parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                            event.set("winlog.event_data.Payload", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("4103") };
            if _cond {
                // Painless script
                // Source: def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}"#
                    ),
                    cached_params!("{\"field\":\"Payload\"}"),
                )?;
            }

            let _cond = {
                event.has_value("_temp.details") && event.get("_temp.details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                event.rename("_temp.details", "powershell.command.invocation_details")?;
            }

            let _cond = {
                event.has_value("process.command_line")
                    && event.get_str("process.command_line") != Some("")
            };
            if _cond {
                // Painless script
                // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;"#
                    ),
                )?;
            }

            let _cond = { event.get_str("winlog.event_data.Path") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Path") {
                        event.rename("winlog.event_data.Path", "winlog.event_data.ScriptName")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ScriptName")
                    && event
                        .get_as_string("winlog.event_data.ScriptName")
                        .is_some_and(|s| s.len() > 1)
            };
            if _cond {
                // Painless script
                // Source: def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("winlog.event_data.ScriptName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ScriptName") {
                        event.rename("winlog.event_data.ScriptName", "file.path")?;
                    }
                    Ok(())
                })();
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp");
                event.remove("winlog.event_data.SequenceNumber");
                event.remove("winlog.event_data.User");
                event.remove("winlog.event_data.ConnectedUser");
                event.remove("winlog.event_data.ContextInfo");
                event.remove("winlog.event_data.Severity");
                event.remove("winlog.event_data.MessageTotal");
                event.remove("winlog.event_data.MessageNumber");
                event.remove("winlog.event_data.Payload");
                event.remove("winlog.time_created");
                event.remove("winlog.level");
                Ok(())
            })();

            let _cond = {
                event.has_value("winlog.event_data") && event.get("winlog.event_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("winlog.event_data");
                    Ok(())
                })();
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
