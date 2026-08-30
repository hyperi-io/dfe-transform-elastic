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

            if event.has_value("event.original") {
                event.rename("event.original", "auditd.messages")?;
            }

            let _cond = {
                event.has_value("auditd.messages")
                    && event.get("auditd.messages").is_some_and(|v| v.is_array())
            };
            if _cond {
                let joined = event
                    .get("auditd.messages")
                    .and_then(|v| join_values(v, "\n"));
                if let Some(joined) = joined {
                    event.set("event.original", json!(joined))?;
                }
            }

            if event.has_value("error.message") {
                event.rename("error.message", "auditd.warnings")?;
            }

            let _cond = {
                event.has_value("auditd.warnings") && event.get("auditd.warnings").is_some_and(|v| v.is_array()) && event.get("auditd.warnings").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                let joined = event
                    .get("auditd.warnings")
                    .and_then(|v| join_values(v, "\n"));
                if let Some(joined) = joined {
                    event.set("error.message", json!(joined))?;
                }
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def k : map.keySet().toArray(new def[map.size()])) {\n    if (map[k] instanceof Map) {\n      handleMap(map[k]);\n    }\n\n    if (k.contains(\"-\")) {\n      map[k.replace(\"-\", \"_\")] = map[k];\n      map.remove(k);\n    }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def k : map.keySet().toArray(new def[map.size()])) {\n    if (map[k] instanceof Map) {\n      handleMap(map[k]);\n    }\n\n    if (k.contains(\"-\")) {\n      map[k.replace(\"-\", \"_\")] = map[k];\n      map.remove(k);\n    }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.path") {
                    event.rename("source.path", "source.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("destination.path") {
                    event.rename("destination.path", "destination.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.audit") {
                    event.rename("user.audit", "auditd.user.audit")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.new_auid") {
                    event.rename("user.new_auid", "auditd.user.new_auid")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.old_auid") {
                    event.rename("user.old_auid", "auditd.user.old_auid")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("file.selinux") {
                    event.rename("file.selinux", "auditd.file.selinux")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.selinux") {
                    event.rename("user.selinux", "auditd.user.selinux")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.saved") {
                    event.rename("user.saved", "auditd.user.saved")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.filesystem") {
                    event.rename("user.filesystem", "auditd.user.filesystem")?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("event.outcome") == Some("fail") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("auditd.sequence") {
                    event.rename("auditd.sequence", "event.sequence")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("auditd.data.id") {
                    event.rename("auditd.data.id", "event.id")?;
                }
                Ok(())
            })();

            if event.has_value("auditd.data.removed") {
                if let Some(val) = event.get("auditd.data.removed") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.removed".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.removed", converted)?;
                }
            }

            if event.has_value("auditd.data.items") {
                if let Some(val) = event.get("auditd.data.items") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.items".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.items", converted)?;
                }
            }

            if event.has_value("auditd.data.lport") {
                if let Some(val) = event.get("auditd.data.lport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.lport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.lport", converted)?;
                }
            }

            if event.has_value("auditd.data.rport") {
                if let Some(val) = event.get("auditd.data.rport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.rport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.rport", converted)?;
                }
            }

            if event.has_value("auditd.data.sport") {
                if let Some(val) = event.get("auditd.data.sport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.sport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.sport", converted)?;
                }
            }

            if event.has_value("auditd.data.dport") {
                if let Some(val) = event.get("auditd.data.dport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.dport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.dport", converted)?;
                }
            }

            if event.has_value("auditd.data.entries") {
                if let Some(val) = event.get("auditd.data.entries") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.entries".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.entries", converted)?;
                }
            }

            if event.has_value("auditd.data.argc") {
                if let Some(val) = event.get("auditd.data.argc") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.argc".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.argc", converted)?;
                }
            }

            if event.has_value("auditd.data.seqno") {
                if let Some(val) = event.get("auditd.data.seqno") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.seqno".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.seqno", converted)?;
                }
            }

            if event.has_value("auditd.data.nargs") {
                if let Some(val) = event.get("auditd.data.nargs") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.nargs".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.nargs", converted)?;
                }
            }

            if event.has_value("auditd.data.socket.port") {
                if let Some(val) = event.get("auditd.data.socket.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.socket.port".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.socket.port", converted)?;
                }
            }

            if event.has_value("auditd.data.old_vcpu") {
                if let Some(val) = event.get("auditd.data.old_vcpu") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.old_vcpu".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.old_vcpu", converted)?;
                }
            }

            if event.has_value("auditd.data.new_vcpu") {
                if let Some(val) = event.get("auditd.data.new_vcpu") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.new_vcpu".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.new_vcpu", converted)?;
                }
            }

            if event.has_value("auditd.data.changed") {
                if let Some(val) = event.get("auditd.data.changed") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.changed".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.changed", converted)?;
                }
            }

            if event.has_value("auditd.data.added") {
                if let Some(val) = event.get("auditd.data.added") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.added".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.added", converted)?;
                }
            }

            if event.has_value("destination.port") {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            if event.has_value("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            if event.has_value("auditd.data.spid") {
                if let Some(val) = event.get("auditd.data.spid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.spid".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.spid", converted)?;
                }
            }

            if event.has_value("auditd.data.opid") {
                if let Some(val) = event.get("auditd.data.opid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.opid".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.opid", converted)?;
                }
            }

            if event.has_value("auditd.data.nlnk_pid") {
                if let Some(val) = event.get("auditd.data.nlnk_pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.nlnk_pid".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.nlnk_pid", converted)?;
                }
            }

            if event.has_value("auditd.data.vm_pid") {
                if let Some(val) = event.get("auditd.data.vm_pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.vm_pid".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.vm_pid", converted)?;
                }
            }

            if event.has_value("auditd.data.audit_pid") {
                if let Some(val) = event.get("auditd.data.audit_pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.data.audit_pid".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.data.audit_pid", converted)?;
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

            if event.has_value("process.exit_code") {
                if let Some(val) = event.get("process.exit_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.exit_code".into(),
                            message,
                        }
                    })?;
                    event.set("process.exit_code", converted)?;
                }
            }

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("auditd.messages");
                    Ok(())
                })();
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
