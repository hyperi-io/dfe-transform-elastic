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
                    let mut kv_gap = false;
                    for pair in cached_regex!("\\n(?!\\n)\\s+").split(&kv_str).into_iter() {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = ({
                            let parts = cached_regex!("[:=]").splitn(&pair, 2);
                            match (parts.first(), parts.get(1)) {
                                (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                _ => None,
                            }
                        })
                        .filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "winlog.event_data.ContextInfo".into(),
                                split: "[:=]".into(),
                            });
                        };
                        {
                            let key = &key[..];
                            let key = key.trim_matches(|c: char| matches!(c, '\t' | '\n' | ' '));
                            let value =
                                value.trim_matches(|c: char| matches!(c, '\t' | '\n' | ' '));
                            if !key.is_empty() {
                                kv_put(event, &format!("winlog.event_data.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("winlog.event_data") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "f11af2ec88e99fd289cab3db2073274e9d7f640919db8824b0418dd5fb8f3b2d",
                        r#"{"params":null,"known":[{"KeysStripWhitespace":"winlog.event_data"}]}"#
                    ),
                )?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { event.has_value("winlog.event_data") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "cb6e7a51906a4b9d3d1af5385b0d3b046068f8045e61f40b9cd1e7f94a6b6b75",
                        r#"{"params":"SentinelRemoval","known":["SentinelRemovalLiteral"]}"#,
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
                    if event.has_value("winlog.event_data.HostID") {
                        event.rename("winlog.event_data.HostID", "process.entity_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.HostApplication") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.HostApplication") {
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
                    if event.has_value("winlog.event_data.HostName") {
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
                    let mut parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
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
                    if event.has_value("user.domain") {
                        event.rename("user.domain", "destination.user.domain")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.name") {
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
                if event.has_value("winlog.event_data._MemberUserName") {
                    event.rename("winlog.event_data._MemberUserName", "user.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.event_data._MemberDomain") {
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
                    if event.has_value("winlog.event_data.ShellID") {
                        event.rename("winlog.event_data.ShellID", "powershell.id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.EngineVersion") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.EngineVersion") {
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
                    if event.has_value("winlog.event_data.PipelineID") {
                        event.rename("winlog.event_data.PipelineID", "powershell.pipeline_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.RunspaceID") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.RunspaceID") {
                        event.rename("winlog.event_data.RunspaceID", "powershell.runspace_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.RunspaceId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.RunspaceId") {
                        event.rename("winlog.event_data.RunspaceId", "powershell.runspace_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.HostVersion") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.HostVersion") {
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
                    if event.has_value("winlog.event_data.CommandLine") {
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
                    if event.has_value("winlog.event_data.CommandPath") {
                        event.rename("winlog.event_data.CommandPath", "powershell.command.path")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.CommandName") {
                        event.rename("winlog.event_data.CommandName", "powershell.command.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.CommandType") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.CommandType") {
                        event.rename("winlog.event_data.CommandType", "powershell.command.type")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.ScriptBlockId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.ScriptBlockId") {
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
                    if event.has_value("winlog.event_data.ScriptBlockText") {
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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "174d337c83cb17c1562dedf0c328beabeaa7dbfaa035cd93bc540afa447ed840",
                            r#"{"params":null,"known":[{"ScriptBlockEntropy":"_temp.script_block_no_signature"}]}"#
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
                            let mut parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set("winlog.event_data.Payload", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("4103") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "d8a41995902f7ce0972b9380b2dfce506900c0bcc8164a665c1622e231b3f494",
                        r#"{"params":"InvocationDetails","known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "76736a193e916d5b301db678b4db23e1b25aa81353168f6003070b37ff3f3f9a",
                        r#"{"params":null,"known":[{"SplitCommandLine":{"sites":[{"source":"process.command_line","base":"process","count":true,"executable":false}],"skip_empty":false}}]}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("winlog.event_data.Path") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.Path") {
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "f356df944933dc6c889ca1b40b02da3be56165159bc116dbd39ff8146ba709f0",
                        r#"{"params":null,"known":[{"FileInfo":"winlog.event_data.ScriptName"}]}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("winlog.event_data.ScriptName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.ScriptName") {
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
