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
                    if event.has("winlog.level") {
                        event.rename("winlog.level", "log.level")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.time_created") };
            if _cond {
                // on_failure: 3 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("winlog.time_created") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("event.created", parsed)?;
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

            let _cond = { event.has_value("winlog.event_data.UtcTime") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("winlog.event_data.UtcTime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss.SSS"],
                            Some("UTC"),
                            None,
                        ) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })();
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

            // Painless script
            // Source: if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"1\":{\"action\":\"Process creation\",\"category\":[\"process\"],\"type\":[\"start\"]},\"10\":{\"action\":\"ProcessAccess\",\"category\":[\"process\"],\"type\":[\"access\"]},\"11\":{\"action\":\"FileCreate\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"12\":{\"action\":\"RegistryEvent (Object create and delete)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"13\":{\"action\":\"RegistryEvent (Value Set)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"14\":{\"action\":\"RegistryEvent (Key and Value Rename)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"15\":{\"action\":\"FileCreateStreamHash\",\"category\":[\"file\"],\"type\":[\"access\"]},\"16\":{\"action\":\"ServiceConfigurationChange\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"17\":{\"action\":\"PipeEvent (Pipe Created)\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"18\":{\"action\":\"PipeEvent (Pipe Connected)\",\"category\":[\"file\"],\"type\":[\"access\"]},\"19\":{\"action\":\"WmiEvent (WmiEventFilter activity detected)\",\"category\":[\"process\"],\"type\":[\"info\"]},\"2\":{\"action\":\"A process changed a file creation time\",\"category\":[\"file\"],\"type\":[\"change\"]},\"20\":{\"action\":\"WmiEvent (WmiEventConsumer activity detected)\",\"category\":[\"process\"],\"type\":[\"change\"]},\"21\":{\"action\":\"WmiEvent (WmiEventConsumerToFilter activity detected)\",\"category\":[\"process\"],\"type\":[\"access\"]},\"22\":{\"action\":\"DNSEvent (DNS query)\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\",\"info\"]},\"23\":{\"action\":\"FileDelete (File Delete archived)\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"24\":{\"action\":\"ClipboardChange (New content in the clipboard)\",\"type\":[\"change\"]},\"25\":{\"action\":\"ProcessTampering (Process image change)\",\"category\":[\"process\"],\"type\":[\"change\"]},\"255\":{\"action\":\"Error\",\"category\":[\"process\"],\"outcome\":[\"failure\"]},\"26\":{\"action\":\"FileDeleteDetected (File Delete logged)\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"27\":{\"action\":\"FileBlockExecutable\",\"category\":[\"file\"],\"outcome\":[\"failure\"],\"type\":[\"creation\"]},\"28\":{\"action\":\"FileBlockShredding\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"29\":{\"action\":\"FileExecutableDetected\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"3\":{\"action\":\"Network connection\",\"category\":[\"network\"],\"type\":[\"start\",\"connection\",\"protocol\"]},\"4\":{\"action\":\"Sysmon service state changed\",\"category\":[\"process\"],\"type\":[\"change\"]},\"5\":{\"action\":\"Process terminated\",\"category\":[\"process\"],\"type\":[\"end\"]},\"6\":{\"action\":\"Driver loaded\",\"category\":[\"driver\"],\"type\":[\"start\"]},\"7\":{\"action\":[\"Image loaded\",\"load\"],\"category\":[\"process\",\"library\"],\"type\":[\"change\",\"start\"]},\"8\":{\"action\":\"CreateRemoteThread\",\"category\":[\"process\"],\"type\":[\"change\"]},\"9\":{\"action\":\"RawAccessRead\",\"category\":[\"process\"],\"type\":[\"access\"]}}"
                ),
            )?;

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

            let _cond = {
                event.get_str("event.code") == Some("255")
                    && event.has_value("winlog.event_data.ID")
                    && event.get_str("winlog.event_data.ID") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ID") {
                        event.rename("winlog.event_data.ID", "error.code")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.RuleName")
                    && event.get_str("winlog.event_data.RuleName") != Some("")
                    && event.get_str("winlog.event_data.RuleName") != Some("-")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.RuleName") {
                        event.rename("winlog.event_data.RuleName", "rule.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code") == Some("25")
                    && event.has_value("winlog.event_data.Type")
                    && event.get_str("winlog.event_data.Type") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Type") {
                        event.rename("winlog.event_data.Type", "message")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Hash")
                    && event.get_str("winlog.event_data.Hash") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Hash") {
                        event.rename("winlog.event_data.Hash", "winlog.event_data.Hashes")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.event_data.Hashes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("winlog.event_data.Hashes") {
                        for pair in kv_str.split(",") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "winlog.event_data.Hashes".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("_temp.hashes.{}", key), value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.hashes") };
            if _cond {
                // Painless script
                // Source: def hashIsEmpty(String hash) {\n  if (hash == \"\") {\n    return true;\n  }\n  \n  Pattern emptyHashRegex = /^0*$/;\n  def matcher = emptyHashRegex.matcher(hash);\n  \n  return matcher.matches(); \n}\n\ndef hashes = new HashMap();\ndef related = [\n  \"hash\": new ArrayList()\n];\nfor (entry in ctx._temp.hashes.entrySet()) {\n  def key = entry.getKey().toString().toLowerCase();\n  def value = entry.getValue().toString().toLowerCase();\n\n  if (hashIsEmpty(value)) {\n    continue;\n  }\n\n  hashes[key] = value;\n  related.hash.add(value);\n}\n\nctx._temp.hashes = hashes;\nif (related.hash.length > 0) {\n  ctx.related = related;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def hashIsEmpty(String hash) {\n  if (hash == \"\") {\n    return true;\n  }\n  \n  Pattern emptyHashRegex = /^0*$/;\n  def matcher = emptyHashRegex.matcher(hash);\n  \n  return matcher.matches(); \n}\n\ndef hashes = new HashMap();\ndef related = [\n  \"hash\": new ArrayList()\n];\nfor (entry in ctx._temp.hashes.entrySet()) {\n  def key = entry.getKey().toString().toLowerCase();\n  def value = entry.getValue().toString().toLowerCase();\n\n  if (hashIsEmpty(value)) {\n    continue;\n  }\n\n  hashes[key] = value;\n  related.hash.add(value);\n}\n\nctx._temp.hashes = hashes;\nif (related.hash.length > 0) {\n  ctx.related = related;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp.hashes")
                    && ["1", "23", "24", "25"].contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.rename("_temp.hashes", "process.hash")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("process.hash.imphash") {
                    event.rename("process.hash.imphash", "process.pe.imphash")?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("winlog.event_data.ProcessGuid")
                    && event.get_str("winlog.event_data.ProcessGuid") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ProcessGuid") {
                        event.rename("winlog.event_data.ProcessGuid", "process.entity_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ProcessId")
                    && event.get_str("winlog.event_data.ProcessId") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.ProcessId") {
                        if let Some(val) = event.get("winlog.event_data.ProcessId") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.ProcessId".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Image")
                    && event.get_str("winlog.event_data.Image") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Image") {
                        event.rename("winlog.event_data.Image", "process.executable")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceProcessGuid")
                    && event.get_str("winlog.event_data.SourceProcessGuid") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.SourceProcessGuid") {
                        event.rename("winlog.event_data.SourceProcessGuid", "process.entity_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceProcessGUID")
                    && event.get_str("winlog.event_data.SourceProcessGUID") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.SourceProcessGUID") {
                        event.rename("winlog.event_data.SourceProcessGUID", "process.entity_id")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceProcessId")
                    && event.get_str("winlog.event_data.SourceProcessId") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SourceProcessId") {
                        if let Some(val) = event.get("winlog.event_data.SourceProcessId") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SourceProcessId".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceThreadId")
                    && event.get_str("winlog.event_data.SourceThreadId") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SourceThreadId") {
                        if let Some(val) = event.get("winlog.event_data.SourceThreadId") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SourceThreadId".into(),
                                    message,
                                }
                            })?;
                            event.set("process.thread.id", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceImage")
                    && event.get_str("winlog.event_data.SourceImage") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.SourceImage") {
                        event.rename("winlog.event_data.SourceImage", "process.executable")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Destination")
                    && event.get_str("winlog.event_data.Destination") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Destination") {
                        event.rename("winlog.event_data.Destination", "process.executable")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.CommandLine")
                    && event.get_str("winlog.event_data.CommandLine") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CommandLine") {
                        event.rename("winlog.event_data.CommandLine", "process.command_line")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.CurrentDirectory")
                    && event.get_str("winlog.event_data.CurrentDirectory") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.CurrentDirectory") {
                        event.rename(
                            "winlog.event_data.CurrentDirectory",
                            "process.working_directory",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ParentProcessGuid")
                    && event.get_str("winlog.event_data.ParentProcessGuid") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ParentProcessGuid") {
                        event.rename(
                            "winlog.event_data.ParentProcessGuid",
                            "process.parent.entity_id",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ParentProcessId")
                    && event.get_str("winlog.event_data.ParentProcessId") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.ParentProcessId") {
                        if let Some(val) = event.get("winlog.event_data.ParentProcessId") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.ParentProcessId".into(),
                                    message,
                                }
                            })?;
                            event.set("process.parent.pid", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ParentImage")
                    && event.get_str("winlog.event_data.ParentImage") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ParentImage") {
                        event
                            .rename("winlog.event_data.ParentImage", "process.parent.executable")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ParentCommandLine")
                    && event.get_str("winlog.event_data.ParentCommandLine") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ParentCommandLine") {
                        event.rename(
                            "winlog.event_data.ParentCommandLine",
                            "process.parent.command_line",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code") != Some("7")
                    && event.has_value("winlog.event_data.OriginalFileName")
                    && event.get_str("winlog.event_data.OriginalFileName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.OriginalFileName") {
                        event.rename(
                            "winlog.event_data.OriginalFileName",
                            "process.pe.original_file_name",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") != Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Company")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.company", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") != Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Description")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.description", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") != Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.FileVersion")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.file_version", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") != Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Product")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.product", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                (event.has_value("process.command_line")
                    && event.get_str("process.command_line") != Some(""))
                    || (event.has_value("process.parent.command_line")
                        && event.get_str("process.parent.command_line") != Some(""))
            };
            if _cond {
                // Painless script
                // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}"#
                    ),
                )?;
            }

            let _cond = {
                (event.has_value("process.executable")
                    && event
                        .get_as_string("process.executable")
                        .is_some_and(|s| s.len() > 1))
                    || (event.has_value("process.parent.executable")
                        && event
                            .get_as_string("process.parent.executable")
                            .is_some_and(|s| s.len() > 1))
            };
            if _cond {
                // Painless script
                // Source: def getProcessName(def path) {\n  def idx = path.lastIndexOf(\"\\\\\");\n  if (idx > -1) {\n      return path.substring(idx+1);\n  }\n  return \"\";\n}\n\ndef cmd = ctx.process?.executable;\nif (cmd != null && cmd != \"\" && ctx.process?.name == null) {\n  def name = getProcessName(cmd);\n  if (name != \"\") {\n    ctx.process.name = name;\n  }\n}\n\ndef parentCmd = ctx.process?.parent?.executable;\nif (parentCmd != null && parentCmd != \"\" && ctx.process?.parent?.name == null) {\n  def name = getProcessName(parentCmd);\n  if (name != \"\") {\n    ctx.process.parent.name = name;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def getProcessName(def path) {\n  def idx = path.lastIndexOf(\"\\\\\");\n  if (idx > -1) {\n      return path.substring(idx+1);\n  }\n  return \"\";\n}\n\ndef cmd = ctx.process?.executable;\nif (cmd != null && cmd != \"\" && ctx.process?.name == null) {\n  def name = getProcessName(cmd);\n  if (name != \"\") {\n    ctx.process.name = name;\n  }\n}\n\ndef parentCmd = ctx.process?.parent?.executable;\nif (parentCmd != null && parentCmd != \"\" && ctx.process?.parent?.name == null) {\n  def name = getProcessName(parentCmd);\n  if (name != \"\") {\n    ctx.process.parent.name = name;\n  }\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp.hashes")
                    && ["6", "7", "15", "26", "29"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(v) = event.get("_temp.hashes").cloned() {
                    event.set("file.hash", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("file.hash.imphash") {
                    event.rename("file.hash.imphash", "file.pe.imphash")?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("winlog.event_data.TargetFilename")
                    && event.get_str("winlog.event_data.TargetFilename") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.TargetFilename") {
                        event.rename("winlog.event_data.TargetFilename", "file.path")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Device")
                    && event.get_str("winlog.event_data.Device") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Device") {
                        event.rename("winlog.event_data.Device", "file.path")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.PipeName")
                    && event.get_str("winlog.event_data.PipeName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.PipeName") {
                        event.rename("winlog.event_data.PipeName", "file.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.ImageLoaded")
                    && event.get_str("winlog.event_data.ImageLoaded") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.ImageLoaded") {
                        event.rename("winlog.event_data.ImageLoaded", "file.path")?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.event_data.Signature")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.code_signature.subject_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.event_data.SignatureStatus")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.code_signature.status", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.get_str("event.code") == Some("7")
                    && event.has_value("winlog.event_data.OriginalFileName")
                    && event.get_str("winlog.event_data.OriginalFileName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.OriginalFileName") {
                        event.rename(
                            "winlog.event_data.OriginalFileName",
                            "file.pe.original_file_name",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Company")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.pe.company", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Description")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.pe.description", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.FileVersion")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.pe.file_version", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Product")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.pe.product", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Signed")
                    && event.get_str("winlog.event_data.Signed") == Some("true")
            };
            if _cond {
                event.set("file.code_signature.trusted", json!(true))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.Signed")
                    && event.get_str("winlog.event_data.Signed") != Some("true")
            };
            if _cond {
                event.set("file.code_signature.trusted", json!(false))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.SignatureStatus")
                    && event.get_str("winlog.event_data.SignatureStatus") == Some("Valid")
            };
            if _cond {
                event.set("file.code_signature.valid", json!(true))?;
            }

            let _cond = {
                event.has_value("file.path")
                    && event
                        .get_as_string("file.path")
                        .is_some_and(|s| s.len() > 1)
            };
            if _cond {
                // Painless script
                // Source: def path = ctx.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def path = ctx.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("file.path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.path", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Signature")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.code_signature.subject_name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code") == Some("7")
                    && event.get_str("winlog.event_data.Signed") == Some("true")
            };
            if _cond {
                event.set("dll.code_signature.trusted", json!(true))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("7")
                    && event.has_value("winlog.event_data.Signed")
                    && event.get_str("winlog.event_data.Signed") == Some("false")
            };
            if _cond {
                event.set("dll.code_signature.trusted", json!(false))?;
            }

            let _cond = { event.get_str("event.code") == Some("7") };
            if _cond {
                // Painless script
                // Source: if (ctx.winlog?.event_data?.Signed == 'true') {\n  ctx.dll.code_signature.status = \"trusted\";\n} else if (ctx.winlog?.event_data?.SignatureStatus == \"Unavailable\") {\n  ctx.dll.code_signature.status = \"Unavailable\";\n} else if (ctx.winlog?.event_data?.Signed instanceof String && ctx.winlog.event_data.Signed.startsWith(\"failed:\")) {\n  ctx.dll.code_signature.status = ctx.winlog.event_data.Signed;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.winlog?.event_data?.Signed == 'true') {\n  ctx.dll.code_signature.status = \"trusted\";\n} else if (ctx.winlog?.event_data?.SignatureStatus == \"Unavailable\") {\n  ctx.dll.code_signature.status = \"Unavailable\";\n} else if (ctx.winlog?.event_data?.Signed instanceof String && ctx.winlog.event_data.Signed.startsWith(\"failed:\")) {\n  ctx.dll.code_signature.status = ctx.winlog.event_data.Signed;\n}"#
                    ),
                )?;
            }

            let _cond =
                { event.has_value("_temp.hashes") && event.get_str("event.code") == Some("7") };
            if _cond {
                if let Some(v) = event.get("_temp.hashes").cloned() {
                    event.set("dll.hash", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("dll.hash.imphash") {
                    event.rename("dll.hash.imphash", "dll.pe.imphash")?;
                }
                Ok(())
            })();

            let _cond = {
                event.get_str("event.code") == Some("7")
                    && event.has_value("file.pe.original_file_name")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("file.pe.original_file_name").cloned() {
                        event.set("dll.pe.original_file_name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Protocol")
                    && event.get_str("winlog.event_data.Protocol") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.Protocol") {
                        event.rename("winlog.event_data.Protocol", "network.transport")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code") != Some("22")
                    && event.has_value("winlog.event_data.DestinationPortName")
                    && event.get_str("winlog.event_data.DestinationPortName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.DestinationPortName") {
                        event
                            .rename("winlog.event_data.DestinationPortName", "network.protocol")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code") != Some("22")
                    && event.has_value("winlog.event_data.SourcePortName")
                    && event.get_str("winlog.event_data.SourcePortName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.SourcePortName") {
                        event.rename("winlog.event_data.SourcePortName", "network.protocol")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("event.code") == Some("22") };
            if _cond {
                event.set("network.protocol", json!("dns"))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceIp")
                    && event.get_str("winlog.event_data.SourceIp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SourceIp") {
                        if let Some(val) = event.get("winlog.event_data.SourceIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SourceIp".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceHostname")
                    && event.get_str("winlog.event_data.SourceHostname") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.SourceHostname") {
                        event.rename("winlog.event_data.SourceHostname", "source.domain")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.SourcePort")
                    && event.get_str("winlog.event_data.SourcePort") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SourcePort") {
                        if let Some(val) = event.get("winlog.event_data.SourcePort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SourcePort".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.DestinationIp")
                    && event.get_str("winlog.event_data.DestinationIp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.DestinationIp") {
                        if let Some(val) = event.get("winlog.event_data.DestinationIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.DestinationIp".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.DestinationHostname")
                    && event.get_str("winlog.event_data.DestinationHostname") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.DestinationHostname") {
                        event.rename(
                            "winlog.event_data.DestinationHostname",
                            "destination.domain",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.DestinationPort")
                    && event.get_str("winlog.event_data.DestinationPort") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.DestinationPort") {
                        if let Some(val) = event.get("winlog.event_data.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.QueryName")
                    && event.get_str("winlog.event_data.QueryName") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.QueryName") {
                        event.rename("winlog.event_data.QueryName", "dns.question.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.Initiated")
                    && event.get_str("winlog.event_data.Initiated") == Some("true")
            };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.Initiated")
                    && event.get_str("winlog.event_data.Initiated") == Some("false")
            };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceIsIpv6")
                    && event.get_str("winlog.event_data.SourceIsIpv6") == Some("false")
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.SourceIsIpv6")
                    && event.get_str("winlog.event_data.SourceIsIpv6") == Some("true")
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = {
                event.has_value("winlog.event_data.QueryResults")
                    && event.get_str("winlog.event_data.QueryResults") != Some("")
            };
            if _cond {
                // Painless script
                // Source: def results = /;/.split(ctx.winlog.event_data.QueryResults);\ndef answers = new ArrayList();\ndef ips = new ArrayList();\ndef relatedHosts = new ArrayList();\nfor (def i = 0; i < results.length; i++) {\n  def answer = results[i];\n  if (answer == \"\") {\n    continue;\n  }\n\n  if (answer.startsWith(\"type:\")) {\n    def parts = /\\s+/.split(answer);\n    if (parts.length < 2) {\n      throw new Exception(\"unexpected QueryResult format\");\n    }\n    if (parts.length == 3) {\n      answers.add([\n        \"type\": params[parts[1]],\n        \"data\": parts[2]\n      ]);\n      relatedHosts.add(parts[2]);\n    } else {\n      answers.add([\n        \"type\": params[parts[1]]\n      ]);\n    }\n  } else {\n    ips.add(answer);\n  }\n}\n\nif (answers.length > 0) {\n  ctx.dns.answers = answers;\n}\nif (ips.length > 0) {\n  ctx.dns.resolved_ip = ips;\n}\nif (relatedHosts.length > 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  ctx.related.hosts = relatedHosts;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def results = /;/.split(ctx.winlog.event_data.QueryResults);\ndef answers = new ArrayList();\ndef ips = new ArrayList();\ndef relatedHosts = new ArrayList();\nfor (def i = 0; i < results.length; i++) {\n  def answer = results[i];\n  if (answer == \"\") {\n    continue;\n  }\n\n  if (answer.startsWith(\"type:\")) {\n    def parts = /\\s+/.split(answer);\n    if (parts.length < 2) {\n      throw new Exception(\"unexpected QueryResult format\");\n    }\n    if (parts.length == 3) {\n      answers.add([\n        \"type\": params[parts[1]],\n        \"data\": parts[2]\n      ]);\n      relatedHosts.add(parts[2]);\n    } else {\n      answers.add([\n        \"type\": params[parts[1]]\n      ]);\n    }\n  } else {\n    ips.add(answer);\n  }\n}\n\nif (answers.length > 0) {\n  ctx.dns.answers = answers;\n}\nif (ips.length > 0) {\n  ctx.dns.resolved_ip = ips;\n}\nif (relatedHosts.length > 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  ctx.related.hosts = relatedHosts;\n}"#
                    ),
                    cached_params!(
                        "{\"1\":\"A\",\"10\":\"NULL\",\"100\":\"UINFO\",\"101\":\"UID\",\"102\":\"GID\",\"103\":\"UNSPEC\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"2\":\"NS\",\"20\":\"ISDN\",\"21\":\"RT\",\"22\":\"NSAP\",\"23\":\"NSAPPTR\",\"24\":\"SIG\",\"248\":\"ADDRS\",\"249\":\"TKEY\",\"25\":\"KEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"255\":\"ANY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"3\":\"MD\",\"30\":\"NXT\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"38\":\"A6\",\"39\":\"DNAME\",\"4\":\"MF\",\"40\":\"SINK\",\"41\":\"OPT\",\"43\":\"DS\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"5\":\"CNAME\",\"6\":\"SOA\",\"65281\":\"WINS\",\"65282\":\"WINSR\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\"}"
                    ),
                )?;
            }

            let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "dns.answers", |event| {
                        gsub_field(
                            event,
                            "_ingest._value",
                            "_ingest._value",
                            cached_regex!(
                                "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                            ),
                            "$1",
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "dns.resolved_ip", |event| {
                        gsub_field(
                            event,
                            "_ingest._value",
                            "_ingest._value",
                            cached_regex!(
                                "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                            ),
                            "$1",
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("dns.resolved_ip") {
                if let Some(Value::Array(items)) = event.get("dns.resolved_ip").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            if event.remove("_ingest._value").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value".into(),
                                });
                            }
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("dns.resolved_ip", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("dns.resolved_ip") };
            if _cond {
                // Painless script
                // Source: if (ctx.dns.answers == null) {\n  ctx.dns.answers = new ArrayList();\n}\nfor (def i = 0; i < ctx.dns.resolved_ip.length; i++) {\n  def ip = ctx.dns.resolved_ip[i];\n  if (ip == null) {\n    ctx.dns.resolved_ip.remove(i);\n    continue;\n  }\n\n  // Synthesize record type based on IP address type.\n  def type = \"A\";\n  if (ip.indexOf(\":\") != -1) {\n    type = \"AAAA\";\n  }\n  ctx.dns.answers.add([\n    \"type\": type,\n    \"data\": ip\n  ]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.dns.answers == null) {\n  ctx.dns.answers = new ArrayList();\n}\nfor (def i = 0; i < ctx.dns.resolved_ip.length; i++) {\n  def ip = ctx.dns.resolved_ip[i];\n  if (ip == null) {\n    ctx.dns.resolved_ip.remove(i);\n    continue;\n  }\n\n  // Synthesize record type based on IP address type.\n  def type = \"A\";\n  if (ip.indexOf(\":\") != -1) {\n    type = \"AAAA\";\n  }\n  ctx.dns.answers.add([\n    \"type\": type,\n    \"data\": ip\n  ]);\n}"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("dns.question.name") {
                    if let Some(domain_str) = event.get_string("dns.question.name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("dns.question.name")
                    && event.get_str("dns.question.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("dns.question.domain");
                Ok(())
            })();

            if event.has_value("dns.resolved_ip") {
                foreach_array(event, "dns.resolved_ip", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
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

            let _cond = { event.has_value("winlog.event_data.User") };
            if _cond {
                if let Some(s) = event.get_string("winlog.event_data.User") {
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

            let _cond = {
                event.has_value("winlog.event_data.QueryStatus")
                    && event.get_str("winlog.event_data.QueryStatus") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data.QueryStatus") {
                        event.rename("winlog.event_data.QueryStatus", "sysmon.dns.status")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("sysmon.dns.status")
                    && event.get_str("sysmon.dns.status") != Some("")
            };
            if _cond {
                // Painless script
                // Source: def status = params[ctx.sysmon.dns.status];\nif (status != null) {\n  ctx.sysmon.dns.status = status;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def status = params[ctx.sysmon.dns.status];\nif (status != null) {\n  ctx.sysmon.dns.status = status;\n}"#
                    ),
                    cached_params!(
                        "{\"0\":\"SUCCESS\",\"10054\":\"WSAECONNRESET\",\"10055\":\"WSAENOBUFS\",\"10060\":\"WSAETIMEDOUT\",\"1214\":\"ERROR_INVALID_NETNAME\",\"1223\":\"ERROR_CANCELLED\",\"123\":\"ERROR_INVALID_NAME\",\"13\":\"ERROR_INVALID_DATA\",\"14\":\"ERROR_OUTOFMEMORY\",\"1460\":\"ERROR_TIMEOUT\",\"4312\":\"ERROR_OBJECT_NOT_FOUND\",\"5\":\"ERROR_ACCESS_DENIED\",\"8\":\"ERROR_NOT_ENOUGH_MEMORY\",\"9001\":\"DNS_ERROR_RCODE_FORMAT_ERROR\",\"9002\":\"DNS_ERROR_RCODE_SERVER_FAILURE\",\"9003\":\"DNS_ERROR_RCODE_NAME_ERROR\",\"9004\":\"DNS_ERROR_RCODE_NOT_IMPLEMENTED\",\"9005\":\"DNS_ERROR_RCODE_REFUSED\",\"9006\":\"DNS_ERROR_RCODE_YXDOMAIN\",\"9007\":\"DNS_ERROR_RCODE_YXRRSET\",\"9008\":\"DNS_ERROR_RCODE_NXRRSET\",\"9009\":\"DNS_ERROR_RCODE_NOTAUTH\",\"9010\":\"DNS_ERROR_RCODE_NOTZONE\",\"9016\":\"DNS_ERROR_RCODE_BADSIG\",\"9017\":\"DNS_ERROR_RCODE_BADKEY\",\"9018\":\"DNS_ERROR_RCODE_BADTIME\",\"9101\":\"DNS_ERROR_KEYMASTER_REQUIRED\",\"9102\":\"DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE\",\"9103\":\"DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1\",\"9104\":\"DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS\",\"9105\":\"DNS_ERROR_UNSUPPORTED_ALGORITHM\",\"9106\":\"DNS_ERROR_INVALID_KEY_SIZE\",\"9107\":\"DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE\",\"9108\":\"DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION\",\"9109\":\"DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR\",\"9110\":\"DNS_ERROR_UNEXPECTED_CNG_ERROR\",\"9111\":\"DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION\",\"9112\":\"DNS_ERROR_KSP_NOT_ACCESSIBLE\",\"9113\":\"DNS_ERROR_TOO_MANY_SKDS\",\"9114\":\"DNS_ERROR_INVALID_ROLLOVER_PERIOD\",\"9115\":\"DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET\",\"9116\":\"DNS_ERROR_ROLLOVER_IN_PROGRESS\",\"9117\":\"DNS_ERROR_STANDBY_KEY_NOT_PRESENT\",\"9118\":\"DNS_ERROR_NOT_ALLOWED_ON_ZSK\",\"9119\":\"DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD\",\"9120\":\"DNS_ERROR_ROLLOVER_ALREADY_QUEUED\",\"9121\":\"DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE\",\"9122\":\"DNS_ERROR_BAD_KEYMASTER\",\"9123\":\"DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD\",\"9124\":\"DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT\",\"9125\":\"DNS_ERROR_DNSSEC_IS_DISABLED\",\"9126\":\"DNS_ERROR_INVALID_XML\",\"9127\":\"DNS_ERROR_NO_VALID_TRUST_ANCHORS\",\"9128\":\"DNS_ERROR_ROLLOVER_NOT_POKEABLE\",\"9129\":\"DNS_ERROR_NSEC3_NAME_COLLISION\",\"9130\":\"DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1\",\"9501\":\"DNS_INFO_NO_RECORDS\",\"9502\":\"DNS_ERROR_BAD_PACKET\",\"9503\":\"DNS_ERROR_NO_PACKET\",\"9504\":\"DNS_ERROR_RCODE\",\"9505\":\"DNS_ERROR_UNSECURE_PACKET\",\"9506\":\"DNS_REQUEST_PENDING\",\"9551\":\"DNS_ERROR_INVALID_TYPE\",\"9552\":\"DNS_ERROR_INVALID_IP_ADDRESS\",\"9553\":\"DNS_ERROR_INVALID_PROPERTY\",\"9554\":\"DNS_ERROR_TRY_AGAIN_LATER\",\"9555\":\"DNS_ERROR_NOT_UNIQUE\",\"9556\":\"DNS_ERROR_NON_RFC_NAME\",\"9557\":\"DNS_STATUS_FQDN\",\"9558\":\"DNS_STATUS_DOTTED_NAME\",\"9559\":\"DNS_STATUS_SINGLE_PART_NAME\",\"9560\":\"DNS_ERROR_INVALID_NAME_CHAR\",\"9561\":\"DNS_ERROR_NUMERIC_NAME\",\"9562\":\"DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER\",\"9563\":\"DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION\",\"9564\":\"DNS_ERROR_CANNOT_FIND_ROOT_HINTS\",\"9565\":\"DNS_ERROR_INCONSISTENT_ROOT_HINTS\",\"9566\":\"DNS_ERROR_DWORD_VALUE_TOO_SMALL\",\"9567\":\"DNS_ERROR_DWORD_VALUE_TOO_LARGE\",\"9568\":\"DNS_ERROR_BACKGROUND_LOADING\",\"9569\":\"DNS_ERROR_NOT_ALLOWED_ON_RODC\",\"9570\":\"DNS_ERROR_NOT_ALLOWED_UNDER_DNAME\",\"9571\":\"DNS_ERROR_DELEGATION_REQUIRED\",\"9572\":\"DNS_ERROR_INVALID_POLICY_TABLE\",\"9573\":\"DNS_ERROR_ADDRESS_REQUIRED\",\"9601\":\"DNS_ERROR_ZONE_DOES_NOT_EXIST\",\"9602\":\"DNS_ERROR_NO_ZONE_INFO\",\"9603\":\"DNS_ERROR_INVALID_ZONE_OPERATION\",\"9604\":\"DNS_ERROR_ZONE_CONFIGURATION_ERROR\",\"9605\":\"DNS_ERROR_ZONE_HAS_NO_SOA_RECORD\",\"9606\":\"DNS_ERROR_ZONE_HAS_NO_NS_RECORDS\",\"9607\":\"DNS_ERROR_ZONE_LOCKED\",\"9608\":\"DNS_ERROR_ZONE_CREATION_FAILED\",\"9609\":\"DNS_ERROR_ZONE_ALREADY_EXISTS\",\"9610\":\"DNS_ERROR_AUTOZONE_ALREADY_EXISTS\",\"9611\":\"DNS_ERROR_INVALID_ZONE_TYPE\",\"9612\":\"DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP\",\"9613\":\"DNS_ERROR_ZONE_NOT_SECONDARY\",\"9614\":\"DNS_ERROR_NEED_SECONDARY_ADDRESSES\",\"9615\":\"DNS_ERROR_WINS_INIT_FAILED\",\"9616\":\"DNS_ERROR_NEED_WINS_SERVERS\",\"9617\":\"DNS_ERROR_NBSTAT_INIT_FAILED\",\"9618\":\"DNS_ERROR_SOA_DELETE_INVALID\",\"9619\":\"DNS_ERROR_FORWARDER_ALREADY_EXISTS\",\"9620\":\"DNS_ERROR_ZONE_REQUIRES_MASTER_IP\",\"9621\":\"DNS_ERROR_ZONE_IS_SHUTDOWN\",\"9622\":\"DNS_ERROR_ZONE_LOCKED_FOR_SIGNING\",\"9651\":\"DNS_ERROR_PRIMARY_REQUIRES_DATAFILE\",\"9652\":\"DNS_ERROR_INVALID_DATAFILE_NAME\",\"9653\":\"DNS_ERROR_DATAFILE_OPEN_FAILURE\",\"9654\":\"DNS_ERROR_FILE_WRITEBACK_FAILED\",\"9655\":\"DNS_ERROR_DATAFILE_PARSING\",\"9701\":\"DNS_ERROR_RECORD_DOES_NOT_EXIST\",\"9702\":\"DNS_ERROR_RECORD_FORMAT\",\"9703\":\"DNS_ERROR_NODE_CREATION_FAILED\",\"9704\":\"DNS_ERROR_UNKNOWN_RECORD_TYPE\",\"9705\":\"DNS_ERROR_RECORD_TIMED_OUT\",\"9706\":\"DNS_ERROR_NAME_NOT_IN_ZONE\",\"9707\":\"DNS_ERROR_CNAME_LOOP\",\"9708\":\"DNS_ERROR_NODE_IS_CNAME\",\"9709\":\"DNS_ERROR_CNAME_COLLISION\",\"9710\":\"DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT\",\"9711\":\"DNS_ERROR_RECORD_ALREADY_EXISTS\",\"9712\":\"DNS_ERROR_SECONDARY_DATA\",\"9713\":\"DNS_ERROR_NO_CREATE_CACHE_DATA\",\"9714\":\"DNS_ERROR_NAME_DOES_NOT_EXIST\",\"9715\":\"DNS_WARNING_PTR_CREATE_FAILED\",\"9716\":\"DNS_WARNING_DOMAIN_UNDELETED\",\"9717\":\"DNS_ERROR_DS_UNAVAILABLE\",\"9718\":\"DNS_ERROR_DS_ZONE_ALREADY_EXISTS\",\"9719\":\"DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE\",\"9720\":\"DNS_ERROR_NODE_IS_DNAME\",\"9721\":\"DNS_ERROR_DNAME_COLLISION\",\"9722\":\"DNS_ERROR_ALIAS_LOOP\",\"9751\":\"DNS_INFO_AXFR_COMPLETE\",\"9752\":\"DNS_ERROR_AXFR\",\"9753\":\"DNS_INFO_ADDED_LOCAL_WINS\",\"9801\":\"DNS_STATUS_CONTINUE_NEEDED\",\"9851\":\"DNS_ERROR_NO_TCPIP\",\"9852\":\"DNS_ERROR_NO_DNS_SERVERS\",\"9901\":\"DNS_ERROR_DP_DOES_NOT_EXIST\",\"9902\":\"DNS_ERROR_DP_ALREADY_EXISTS\",\"9903\":\"DNS_ERROR_DP_NOT_ENLISTED\",\"9904\":\"DNS_ERROR_DP_ALREADY_ENLISTED\",\"9905\":\"DNS_ERROR_DP_NOT_AVAILABLE\",\"9906\":\"DNS_ERROR_DP_FSMO_ERROR\",\"9911\":\"DNS_ERROR_RRL_NOT_ENABLED\",\"9912\":\"DNS_ERROR_RRL_INVALID_WINDOW_SIZE\",\"9913\":\"DNS_ERROR_RRL_INVALID_IPV4_PREFIX\",\"9914\":\"DNS_ERROR_RRL_INVALID_IPV6_PREFIX\",\"9915\":\"DNS_ERROR_RRL_INVALID_TC_RATE\",\"9916\":\"DNS_ERROR_RRL_INVALID_LEAK_RATE\",\"9917\":\"DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE\",\"9921\":\"DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS\",\"9922\":\"DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST\",\"9923\":\"DNS_ERROR_VIRTUALIZATION_TREE_LOCKED\",\"9924\":\"DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME\",\"9925\":\"DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE\",\"9951\":\"DNS_ERROR_ZONESCOPE_ALREADY_EXISTS\",\"9952\":\"DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST\",\"9953\":\"DNS_ERROR_DEFAULT_ZONESCOPE\",\"9954\":\"DNS_ERROR_INVALID_ZONESCOPE_NAME\",\"9955\":\"DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES\",\"9956\":\"DNS_ERROR_LOAD_ZONESCOPE_FAILED\",\"9957\":\"DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED\",\"9958\":\"DNS_ERROR_INVALID_SCOPE_NAME\",\"9959\":\"DNS_ERROR_SCOPE_DOES_NOT_EXIST\",\"9960\":\"DNS_ERROR_DEFAULT_SCOPE\",\"9961\":\"DNS_ERROR_INVALID_SCOPE_OPERATION\",\"9962\":\"DNS_ERROR_SCOPE_LOCKED\",\"9963\":\"DNS_ERROR_SCOPE_ALREADY_EXISTS\",\"9971\":\"DNS_ERROR_POLICY_ALREADY_EXISTS\",\"9972\":\"DNS_ERROR_POLICY_DOES_NOT_EXIST\",\"9973\":\"DNS_ERROR_POLICY_INVALID_CRITERIA\",\"9974\":\"DNS_ERROR_POLICY_INVALID_SETTINGS\",\"9975\":\"DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED\",\"9976\":\"DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST\",\"9977\":\"DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS\",\"9978\":\"DNS_ERROR_SUBNET_DOES_NOT_EXIST\",\"9979\":\"DNS_ERROR_SUBNET_ALREADY_EXISTS\",\"9980\":\"DNS_ERROR_POLICY_LOCKED\",\"9981\":\"DNS_ERROR_POLICY_INVALID_WEIGHT\",\"9982\":\"DNS_ERROR_POLICY_INVALID_NAME\",\"9983\":\"DNS_ERROR_POLICY_MISSING_CRITERIA\",\"9984\":\"DNS_ERROR_INVALID_CLIENT_SUBNET_NAME\",\"9985\":\"DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID\",\"9986\":\"DNS_ERROR_POLICY_SCOPE_MISSING\",\"9987\":\"DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED\",\"9988\":\"DNS_ERROR_SERVERSCOPE_IS_REFERENCED\",\"9989\":\"DNS_ERROR_ZONESCOPE_IS_REFERENCED\",\"9990\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET\",\"9991\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL\",\"9992\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL\",\"9993\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE\",\"9994\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN\",\"9995\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE\",\"9996\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY\"}"
                    ),
                )?;
            }

            let _cond = {
                event.has_value("winlog.event_data.Archived")
                    && event.get_str("winlog.event_data.Archived") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.Archived") {
                        if let Some(val) = event.get("winlog.event_data.Archived") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.Archived".into(),
                                    message,
                                }
                            })?;
                            event.set("sysmon.file.archived", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.IsExecutable")
                    && event.get_str("winlog.event_data.IsExecutable") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.IsExecutable") {
                        if let Some(val) = event.get("winlog.event_data.IsExecutable") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.IsExecutable".into(),
                                    message,
                                }
                            })?;
                            event.set("sysmon.file.is_executable", converted)?;
                        }
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

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("winlog.event_data.TargetObject")
                    && event.get_str("winlog.event_data.TargetObject") != Some("")
                    && ["12", "13", "14"].contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                // Painless script
                // Source: ctx.registry = new HashMap();\nPattern qwordRegex = /(?i)QWORD \\(((0x[0-9A-F]{8})-(0x[0-9A-F]{8}))\\)/;\nPattern dwordRegex = /(?i)DWORD \\((0x[0-9A-F]{8})\\)/;\nPattern binDataRegex = /Binary Data/;\n\ndef path = ctx.winlog.event_data.TargetObject;\nctx.registry.path = path;\n\ndef pathTokens = Arrays.asList(/\\\\/.split(path));\ndef hive = params[pathTokens[0]];\nif (hive != null) {\n  ctx.registry.hive = hive;\n  if (pathTokens.length > 1) {\n    ctx.registry.key = pathTokens.subList(1, pathTokens.length).join(\"\\\\\");\n  }\n}\n\ndef value = pathTokens[pathTokens.length - 1];\nctx.registry.value = value;\n\ndef data = ctx.winlog?.event_data?.Details;\nif (data != null && data != \"\") {\n  def prefixLen = 2; // to remove 0x prefix\n  def dataValue = \"\";\n  def dataType = \"\";\n  def matcher = qwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedHighByte = Long.parseLong(matcher.group(2).substring(prefixLen), 16);\n    def parsedLowByte = Long.parseLong(matcher.group(3).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedHighByte) && !Double.isNaN(parsedLowByte)) {\n      dataType = \"SZ_QWORD\";\n      dataValue = Long.toString(((parsedHighByte << 8) + parsedLowByte));\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = dwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedValue = Long.parseLong(matcher.group(1).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedValue)) {\n      dataType = \"SZ_DWORD\";\n      dataValue = Long.toString(parsedValue);\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = binDataRegex.matcher(data);\n  if (matcher.matches()) {\n    // Data type could be REG_BINARY or REG_MULTI_SZ\n    ctx.registry.data = [\n      \"strings\": [data],\n      \"type\": \"REG_BINARY\"\n    ];\n    return;\n  }\n\n  // REG_SZ or REG_EXPAND_SZ\n  ctx.registry.data = [\n    \"strings\": [data],\n    \"type\": \"REG_SZ\"\n  ];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.registry = new HashMap();\nPattern qwordRegex = /(?i)QWORD \\(((0x[0-9A-F]{8})-(0x[0-9A-F]{8}))\\)/;\nPattern dwordRegex = /(?i)DWORD \\((0x[0-9A-F]{8})\\)/;\nPattern binDataRegex = /Binary Data/;\n\ndef path = ctx.winlog.event_data.TargetObject;\nctx.registry.path = path;\n\ndef pathTokens = Arrays.asList(/\\\\/.split(path));\ndef hive = params[pathTokens[0]];\nif (hive != null) {\n  ctx.registry.hive = hive;\n  if (pathTokens.length > 1) {\n    ctx.registry.key = pathTokens.subList(1, pathTokens.length).join(\"\\\\\");\n  }\n}\n\ndef value = pathTokens[pathTokens.length - 1];\nctx.registry.value = value;\n\ndef data = ctx.winlog?.event_data?.Details;\nif (data != null && data != \"\") {\n  def prefixLen = 2; // to remove 0x prefix\n  def dataValue = \"\";\n  def dataType = \"\";\n  def matcher = qwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedHighByte = Long.parseLong(matcher.group(2).substring(prefixLen), 16);\n    def parsedLowByte = Long.parseLong(matcher.group(3).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedHighByte) && !Double.isNaN(parsedLowByte)) {\n      dataType = \"SZ_QWORD\";\n      dataValue = Long.toString(((parsedHighByte << 8) + parsedLowByte));\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = dwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedValue = Long.parseLong(matcher.group(1).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedValue)) {\n      dataType = \"SZ_DWORD\";\n      dataValue = Long.toString(parsedValue);\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = binDataRegex.matcher(data);\n  if (matcher.matches()) {\n    // Data type could be REG_BINARY or REG_MULTI_SZ\n    ctx.registry.data = [\n      \"strings\": [data],\n      \"type\": \"REG_BINARY\"\n    ];\n    return;\n  }\n\n  // REG_SZ or REG_EXPAND_SZ\n  ctx.registry.data = [\n    \"strings\": [data],\n    \"type\": \"REG_SZ\"\n  ];\n}"#
                    ),
                    cached_params!(
                        "{\"HKCC\":\"HKCC\",\"HKCR\":\"HKCR\",\"HKCU\":\"HKCU\",\"HKDD\":\"HKDD\",\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_CONFIG\":\"HKCC\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_DYN_DATA\":\"HKDD\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_PERFORMANCE_DATA\":\"HKPD\",\"HKEY_USERS\":\"HKU\",\"HKLM\":\"HKLM\",\"HKPD\":\"HKPD\",\"HKU\":\"HKU\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("winlog.event_data.TargetProcessGuid") };
            if _cond {
                event.rename(
                    "winlog.event_data.TargetProcessGuid",
                    "winlog.event_data.TargetProcessGUID",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp");
                event.remove("winlog.event_data.ProcessId");
                event.remove("winlog.event_data.ParentProcessId");
                event.remove("winlog.event_data.SourceProcessId");
                event.remove("winlog.event_data.SourceThreadId");
                event.remove("winlog.event_data.SourceIp");
                event.remove("winlog.event_data.SourcePort");
                event.remove("winlog.event_data.SourcePortName");
                event.remove("winlog.event_data.DestinationIp");
                event.remove("winlog.event_data.DestinationPort");
                event.remove("winlog.event_data.DestinationPortName");
                event.remove("winlog.event_data.RuleName");
                event.remove("winlog.event_data.User");
                event.remove("winlog.event_data.Initiated");
                event.remove("winlog.event_data.SourceIsIpv6");
                event.remove("winlog.event_data.DestinationIsIpv6");
                event.remove("winlog.event_data.QueryStatus");
                event.remove("winlog.event_data.Archived");
                event.remove("winlog.event_data.IsExecutable");
                event.remove("winlog.event_data.QueryResults");
                event.remove("winlog.event_data.UtcTime");
                event.remove("winlog.event_data.Hash");
                event.remove("winlog.event_data.Hashes");
                event.remove("winlog.event_data.TargetObject");
                event.remove("winlog.event_data.Details");
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

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
