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
            let _cond = { event.get_str("winlog.event_id") == Some("800") };
            if _cond {
                if let Some(kv_str) = event.get_string("winlog.event_data.param2") {
                    let mut kv_gap = false;
                    for pair in kv_str.split("\n\t") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "winlog.event_data.param2".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            let key = key.trim_matches(|c: char| matches!(c, '\t' | '\n'));
                            let value = value.trim_matches(|c: char| matches!(c, '\t' | '\n'));
                            if !key.is_empty() {
                                kv_put(event, &format!("winlog.event_data.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.get_str("winlog.event_id") != Some("800")
                    && event.has_value("winlog.event_data.param3")
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "d7cca56295c3c6492887dd56a9dff4ab0883fa6f4b9dc00e98adaffe90d73879",
                        r#"{"params":"MatcherKv","known":[]}"#
                    ),
                    cached_params!("{\"field\":\"param3\"}"),
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

            let _cond = { event.get_str("event.code") == Some("400") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = { event.get_str("event.code") == Some("403") };
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

            let _cond = { event.get_str("winlog.event_data.HostId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.HostId") {
                        event.rename("winlog.event_data.HostId", "process.entity_id")?;
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

            let _cond = { event.has_value("winlog.event_data.UserId") };
            if _cond {
                if let Some(s) = event.get_string("winlog.event_data.UserId") {
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

            let _cond = { event.get_str("winlog.event_data.NewEngineState") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.NewEngineState") {
                        event.rename(
                            "winlog.event_data.NewEngineState",
                            "powershell.engine.new_state",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.PreviousEngineState") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.PreviousEngineState") {
                        event.rename(
                            "winlog.event_data.PreviousEngineState",
                            "powershell.engine.previous_state",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.NewProviderState") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.NewProviderState") {
                        event.rename(
                            "winlog.event_data.NewProviderState",
                            "powershell.provider.new_state",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.ProviderName") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.ProviderName") {
                        event
                            .rename("winlog.event_data.ProviderName", "powershell.provider.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.DetailTotal") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.DetailTotal") {
                        if let Some(val) = event.get("winlog.event_data.DetailTotal") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.DetailTotal".into(),
                                    message,
                                }
                            })?;
                            event.set("powershell.total", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("winlog.event_data.DetailSequence") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.DetailSequence") {
                        if let Some(val) = event.get("winlog.event_data.DetailSequence") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.DetailSequence".into(),
                                    message,
                                }
                            })?;
                            event.set("powershell.sequence", converted)?;
                        }
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

            let _cond = { event.get_str("winlog.event_data.PipelineId") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.PipelineId") {
                        event.rename("winlog.event_data.PipelineId", "powershell.pipeline_id")?;
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

            let _cond = { event.get_str("event.code") == Some("800") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.param3") {
                        if let Some(s) = event.get_string("winlog.event_data.param3") {
                            let mut parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set("winlog.event_data.param3", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("800") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "57bd13f937e51c59000546b49d76e7ce33e6c1143fcddb826a0735868e21be71",
                        r#"{"params":"InvocationDetails","known":[]}"#
                    ),
                    cached_params!("{\"field\":\"param3\"}"),
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
                event.remove("winlog.event_data.param1");
                event.remove("winlog.event_data.param2");
                event.remove("winlog.event_data.param3");
                event.remove("winlog.event_data.SequenceNumber");
                event.remove("winlog.event_data.DetailTotal");
                event.remove("winlog.event_data.DetailSequence");
                event.remove("winlog.event_data.UserId");
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
