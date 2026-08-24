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
                    for pair in kv_str.split("\n\t") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "winlog.event_data.param2".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| "\n\t".contains(c));
                            let value = value.trim_matches(|c| "\n\t".contains(c));
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
                // Painless script
                // Source: def p = ctx.winlog?.event_data[params[\"field\"]];\n// Define the pattern that will match all keys\ndef pat = /(^|(^[\\n]?))?\\t([^\\s\\W]+)=/m;\ndef m = pat.matcher(p);\n\n// we position ourselves in the first matching key\nm.find();\ndef key = m.group(3).trim();\ndef previousEnd = m.end();\n\n// while new keys are found, we add everything between one key and the next\n// as the value, regardless of its contents\nwhile(m.find())\n{\n    ctx.winlog.event_data[key] = p.substring(previousEnd, m.start()).trim();\n    previousEnd = m.end();\n    key = m.group(3).trim();\n}\n\n// add remaining value\nctx.winlog.event_data[key] = p.substring(previousEnd).trim();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def p = ctx.winlog?.event_data[params[\"field\"]];\n// Define the pattern that will match all keys\ndef pat = /(^|(^[\\n]?))?\\t([^\\s\\W]+)=/m;\ndef m = pat.matcher(p);\n\n// we position ourselves in the first matching key\nm.find();\ndef key = m.group(3).trim();\ndef previousEnd = m.end();\n\n// while new keys are found, we add everything between one key and the next\n// as the value, regardless of its contents\nwhile(m.find())\n{\n    ctx.winlog.event_data[key] = p.substring(previousEnd, m.start()).trim();\n    previousEnd = m.end();\n    key = m.group(3).trim();\n}\n\n// add remaining value\nctx.winlog.event_data[key] = p.substring(previousEnd).trim();"#
                    ),
                    cached_params!("{\"field\":\"param3\"}"),
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
                    let parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
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
                            let parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                            event.set("winlog.event_data.param3", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("800") };
            if _cond {
                // Painless script
                // Source: def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^:(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^:(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}"#
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
