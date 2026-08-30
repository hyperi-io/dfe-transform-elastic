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
            event.set("ecs.version", json!("9.3.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("message") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" : TTY=") else {
                                break 'dissect false;
                            };
                            captured.push(("forescout.event.service", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" : TTY=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ; PWD=") else {
                                break 'dissect false;
                            };
                            captured.push(("forescout.event.tty", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ; PWD=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ; USER=") else {
                                break 'dissect false;
                            };
                            captured.push(("forescout.event.pwd", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ; USER=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ; COMMAND=") else {
                                break 'dissect false;
                            };
                            captured.push(("forescout.event.user", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ; COMMAND=") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("forescout.event.command", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("forescout.event.message", v)?;
            }

            if let Some(v) = event
                .get("forescout.event.pwd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event
                .get("forescout.event.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("forescout.event.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            if let Some(v) = event
                .get("forescout.event.command")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            let _cond = { event.has_value("forescout.event.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("log.syslog.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("log.syslog.hostname")
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
                event.remove("forescout.event.message");
                event.remove("forescout.event.pwd");
                event.remove("forescout.event.user");
                event.remove("forescout.event.command");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n\tmap.values().removeIf(v -> {\n\t\tif (v instanceof Map) {\n\t\t\thandleMap(v);\n\t\t} else if (v instanceof List) {\n\t\t\thandleList(v);\n\t\t}\n\t\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n\t});\n}\nvoid handleList(List list) {\n\tlist.removeIf(v -> {\n\t\tif (v instanceof Map) {\n\t\t\thandleMap(v);\n\t\t} else if (v instanceof List) {\n\t\t\thandleList(v);\n\t\t}\n\t\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n\t});\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n\tmap.values().removeIf(v -> {\n\t\tif (v instanceof Map) {\n\t\t\thandleMap(v);\n\t\t} else if (v instanceof List) {\n\t\t\thandleList(v);\n\t\t}\n\t\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n\t});\n}\nvoid handleList(List list) {\n\tlist.removeIf(v -> {\n\t\tif (v instanceof Map) {\n\t\t\thandleMap(v);\n\t\t} else if (v instanceof List) {\n\t\t\thandleList(v);\n\t\t}\n\t\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n\t});\n}\nhandleMap(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
