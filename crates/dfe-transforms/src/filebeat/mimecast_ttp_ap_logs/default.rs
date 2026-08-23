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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "mimecast")?;

            let _cond = {
                !event.has_value("mimecast.date")
                    || (event.has_value("mimecast.data")
                        && event.get("mimecast.data").is_none_or(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        }))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.messageId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.date") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            let _cond = { event.get_str("mimecast.result") == Some("malicious") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if let Some(date_str) = event.get_as_string("mimecast.date") {
                if let Some(parsed) =
                    parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ssZ"], Some("UTC"), None)
                {
                    event.set("@timestamp", parsed)?;
                }
            }

            let _cond = { event.has_value("mimecast.senderAddress") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "email.from.address",
                        json!(
                            event
                                .get("mimecast.senderAddress")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("mimecast.recipientAddress") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "email.to.address",
                        json!(
                            event
                                .get("mimecast.recipientAddress")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.from.address") {
                    if let Some(input) = event.get_string("email.from.address") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("<") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(">") else {
                                break 'dissect false;
                            };
                            captured.push(("email.from.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(">") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.from.address") {
                    if let Some(s) = event.get_string("email.from.address") {
                        let re = cached_regex!("<>");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("email.from.address", replaced)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.to.address") {
                    if let Some(input) = event.get_string("email.to.address") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("<") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(">") else {
                                break 'dissect false;
                            };
                            captured.push(("email.to.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(">") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.to.address") {
                    if let Some(s) = event.get_string("email.to.address") {
                        let re = cached_regex!("<>");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("email.to.address", replaced)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("mimecast.actionTriggered") {
                if let Some(s) = event.get_string("mimecast.actionTriggered") {
                    let re = cached_regex!(",");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("mimecast.actionTriggered", replaced)?;
                }
            }

            if event.has_value("mimecast.actionTriggered") {
                if let Some(s) = event.get_string("mimecast.actionTriggered") {
                    let re = cached_regex!(" ");
                    let replaced = re.replace_all(&s, "_").into_owned();
                    event.set("mimecast.actionTriggered", replaced)?;
                }
            }

            let _cond = { event.has_value("mimecast.actionTriggered") };
            if _cond {
                if event.has("mimecast.actionTriggered") {
                    event.rename("mimecast.actionTriggered", "event.action")?;
                }
            }

            let _cond = { event.has_value("mimecast.subject") };
            if _cond {
                if event.has("mimecast.subject") {
                    event.rename("mimecast.subject", "email.subject")?;
                }
            }

            let _cond = { event.has_value("mimecast.messageId") };
            if _cond {
                if event.has("mimecast.messageId") {
                    event.rename("mimecast.messageId", "email.message_id")?;
                }
            }

            let _cond = { event.has_value("mimecast.route") };
            if _cond {
                if event.has("mimecast.route") {
                    event.rename("mimecast.route", "email.direction")?;
                }
            }

            let _cond = { event.has_value("mimecast.fileName") };
            if _cond {
                if event.has("mimecast.fileName") {
                    event.rename("mimecast.fileName", "email.attachments.file.name")?;
                }
            }

            let _cond = { event.has_value("mimecast.definition") };
            if _cond {
                if event.has("mimecast.definition") {
                    event.rename("mimecast.definition", "rule.name")?;
                }
            }

            let _cond = {
                event.has_value("mimecast.fileHash")
                    && event
                        .get_as_string("mimecast.fileHash")
                        .is_some_and(|s| s.len() == 64)
            };
            if _cond {
                if event.has("mimecast.fileHash") {
                    event.rename("mimecast.fileHash", "email.attachments.file.hash.sha256")?;
                }
            }

            let _cond = { event.has_value("mimecast.fileType") };
            if _cond {
                if event.has("mimecast.fileType") {
                    event.rename("mimecast.fileType", "email.attachments.file.mime_type")?;
                }
            }

            let _cond = { event.has_value("mimecast.date") };
            if _cond {
                event.set(
                    "event.created",
                    json!(
                        event
                            .get("mimecast.date")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("email.attachments.file.name") };
            if _cond {
                if let Some(s) = event.get_string("email.attachments.file.name") {
                    let parts: Vec<Value> = cached_regex!("\\.")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("file.parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("file.parts") && event.get("file.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                // Painless script
                // Source: ctx.email.attachments.file.extension = ctx.file.parts[ctx.file.parts.length-1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.email.attachments.file.extension = ctx.file.parts[ctx.file.parts.length-1];\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("email.attachments.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("email.attachments.file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("email.direction") {
                if let Some(s) = event.get_string("email.direction") {
                    let lowered = s.to_lowercase();
                    event.set("email.direction", lowered)?;
                }
            }

            event.remove("mimecast.date");
            event.remove("file.parts");
            event.remove("file");
            event.remove("mimecast.senderAddress");
            event.remove("mimecast.recipientAddress");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
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
                    json!(format!(
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
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
