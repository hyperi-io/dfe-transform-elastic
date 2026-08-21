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

            if let Some(s) = event.get_string("event.original") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "event.original".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("mimecast", parsed)?;
            }

            let _cond = {
                !event.has_value("mimecast.createTime")
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
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                if let Some(v) = event.get("mimecast.id") {
                    hasher.update(v.to_string().as_bytes());
                }
                let hash = format!("{:x}", hasher.finalize());
                event.set("_id", json!(hash))?;
            }

            if let Some(date_str) = event.get_as_string("mimecast.createTime") {
                if let Some(parsed) = parse_date_out(
                    &date_str,
                    &[
                        "yyyy-MM-dd'T'HH:mm:ssz",
                        "yyyy-MM-dd'T'HH:mm:ssZ",
                        "yyyy-MM-dd'T'HH:mm:ss.Sz",
                        "yyyy-MM-dd'T'HH:mm:ss.SZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSSSSSSSZ",
                        "yyyy-MM-dd'T'HH:mm:ss z",
                    ],
                    Some("UTC"),
                    None,
                ) {
                    event.set("@timestamp", parsed)?;
                }
            }

            if event.has("mimecast.searchText") {
                event.rename("mimecast.searchText", "mimecast.search_details.text")?;
            }

            if event.has("mimecast.description") {
                event.rename(
                    "mimecast.description",
                    "mimecast.search_details.description",
                )?;
            }

            if event.has("mimecast.source") {
                event.rename("mimecast.source", "mimecast.search_details.source")?;
            }

            if event.has("mimecast.searchPath") {
                event.rename("mimecast.searchPath", "mimecast.search_details.path")?;
            }

            if event.has("mimecast.searchReason") {
                event.rename("mimecast.searchReason", "mimecast.search_details.reason")?;
            }

            let _cond = { event.get_str("mimecast.emailAddr") == Some("<>") };
            if _cond {
                if event.remove("mimecast.emailAddr").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "mimecast.emailAddr".into(),
                    });
                }
            }

            if event.has("mimecast.emailAddr") {
                event.rename("mimecast.emailAddr", "user.email")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.email") {
                    if let Some(input) = event.get_string("user.email") {
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
                            captured.push(("user.email", &remaining[..pos]));
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                if let Some(s) = event.get_string("user.email") {
                    let parts: Vec<Value> = s.split("@").map(|p| json!(p)).collect();
                    event.set("user.parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("user.parts.length") && event.get("user.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                if let Some(v) = event.get("user.parts.0").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("user.parts.length") && event.get("user.parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                if let Some(v) = event.get("user.parts.1").cloned() {
                    event.set("user.domain", v)?;
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("api"))?;

            event.append("event.type", json!("admin"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            event.remove("mimecast.createTime");
            event.remove("mimecast.searchPath");
            event.remove("mimecast.searchText");
            event.remove("mimecast.description");
            event.remove("mimecast.searchReason");
            event.remove("mimecast.source");
            event.remove("user.parts");

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
