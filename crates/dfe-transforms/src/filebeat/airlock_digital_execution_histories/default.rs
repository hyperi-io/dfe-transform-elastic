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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_a68ecd77",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
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
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("host"))?;

            event.set("event.kind", json!("event"))?;

            if event.has_value("json.checkpoint") {
                event.rename(
                    "json.checkpoint",
                    "airlock_digital.execution_histories.checkpoint",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.checkpoint")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.commandline") {
                event.rename(
                    "json.commandline",
                    "airlock_digital.execution_histories.commandline",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.commandline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            let _cond =
                { event.has_value("json.datetime") && event.get_str("json.datetime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.datetime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("airlock_digital.execution_histories.datetime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.datetime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_datetime_1c151f24")?;
                    event.append(
                        "error.message",
                        json!(format!(
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
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.filename") {
                event.rename(
                    "json.filename",
                    "airlock_digital.execution_histories.filename",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(event, "file.path", "file.path", cached_regex!("\\\\"), "/")?;
                Ok(())
            })();

            let _cond = { event.has_value("file.path") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("file.path") {
                        let mut parts: Vec<Value> = s.split("/").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("file.name", Value::Array(parts))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("file.name").is_some_and(|v| v.is_array()) && event.get("file.name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: ctx.file.name = ctx.file.name[ctx.file.name.length-1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.file.name = ctx.file.name[ctx.file.name.length-1];\n"#),
                )?;
            }

            if event.has_value("json.hostname") {
                event.rename(
                    "json.hostname",
                    "airlock_digital.execution_histories.hostname",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.md5") {
                event.rename("json.md5", "airlock_digital.execution_histories.md5")?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.netdomain") {
                event.rename(
                    "json.netdomain",
                    "airlock_digital.execution_histories.netdomain",
                )?;
            }

            if event.has_value("json.policyname") {
                event.rename(
                    "json.policyname",
                    "airlock_digital.execution_histories.policyname",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.policyname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.policyver") {
                event.rename(
                    "json.policyver",
                    "airlock_digital.execution_histories.policyver",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.policyver")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.version", v)?;
            }

            if event.has_value("json.ppolicy") {
                event.rename(
                    "json.ppolicy",
                    "airlock_digital.execution_histories.ppolicy",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.ppolicy")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if event.has_value("json.pprocess") {
                event.rename(
                    "json.pprocess",
                    "airlock_digital.execution_histories.pprocess",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.pprocess")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            if event.has_value("json.publisher") {
                event.rename(
                    "json.publisher",
                    "airlock_digital.execution_histories.publisher",
                )?;
            }

            if event.has_value("json.sha128") {
                event.rename("json.sha128", "airlock_digital.execution_histories.sha128")?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.sha128") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.sha128")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.sha256") {
                event.rename("json.sha256", "airlock_digital.execution_histories.sha256")?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.sha384") {
                event.rename("json.sha384", "airlock_digital.execution_histories.sha384")?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.sha384")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha384", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.sha384") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.sha384")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.sha512") {
                event.rename("json.sha512", "airlock_digital.execution_histories.sha512")?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.sha512")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha512", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.sha512") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.sha512")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.type") {
                    if let Some(val) = event.get("json.type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.type".into(),
                                message,
                            }
                        })?;
                        event.set("airlock_digital.execution_histories.type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_type_to_long_fd969e74",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
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
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.type") && event.get_str("json.type") != Some("") };
            if _cond {
                // Painless script
                // Source: ctx.airlock_digital.execution_histories.type_value = params[(ctx.json.type).toString()];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.airlock_digital.execution_histories.type_value = params[(ctx.json.type).toString()];"#
                    ),
                    cached_params!(
                        "{\"0\":\"Trusted Execution\",\"1\":\"Blocked Execution\",\"2\":\"Untrusted Execution [Audit]\",\"3\":\"Untrusted Execution [OTP]\",\"4\":\"Trusted Path Execution\",\"5\":\"Trusted Publisher Execution\",\"6\":\"Blocklist Execution\",\"7\":\"Blocklist Execution [Audit]\",\"8\":\"Trusted Process Execution\",\"9\":\"Constrained Execution\",\"10\":\"Trusted Metadata Execution\",\"11\":\"Trusted Browser Execution\",\"12\":\"Blocked Browser Execution\",\"13\":\"Untrusted Browser Execution [Audit]\",\"14\":\"Untrusted Browser Execution [OTP]\",\"15\":\"Blocklist Browser Execution [Audit]\",\"16\":\"Blocklist Browser Execution\",\"17\":\"Trusted Installer Execution\",\"18\":\"Trusted Browser Metadata Execution\"}"
                    ),
                )?;
            }

            if event.has_value("json.username") {
                event.rename(
                    "json.username",
                    "airlock_digital.execution_histories.username",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.execution_histories.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("airlock_digital.execution_histories.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("airlock_digital.execution_histories.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("airlock_digital.execution_histories.checkpoint");
                event.remove("airlock_digital.execution_histories.commandline");
                event.remove("airlock_digital.execution_histories.datetime");
                event.remove("airlock_digital.execution_histories.filename");
                event.remove("airlock_digital.execution_histories.hostname");
                event.remove("airlock_digital.execution_histories.md5");
                event.remove("airlock_digital.execution_histories.policyname");
                event.remove("airlock_digital.execution_histories.policyver");
                event.remove("airlock_digital.execution_histories.ppolicy");
                event.remove("airlock_digital.execution_histories.pprocess");
                event.remove("airlock_digital.execution_histories.sha256");
                event.remove("airlock_digital.execution_histories.sha384");
                event.remove("airlock_digital.execution_histories.sha512");
                event.remove("airlock_digital.execution_histories.username");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
