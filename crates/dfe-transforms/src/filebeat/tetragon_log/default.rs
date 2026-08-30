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

            if let Some(v) = event.get("message").cloned() {
                event.set("originalMessage", v)?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("originalMessage") {
                    event.rename("originalMessage", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "cilium_tetragon.log")?;

            parse_json_field(event, "event.original", "_tmp_")?;

            // Painless script
            // Source: void run(Map map) {\n  for (def k : map?.cilium_tetragon?.log?.keySet()) {\n    /* these tetragon objects have \"process\" */\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\" ||\n        k == \"process_tracepoint\" ||\n        k == \"process_loader\" ||\n        k == \"process_lsm\" ||\n        k == \"process_uprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"process\"] = map.cilium_tetragon.log[k].process;\n    }\n\n    /* these tetragon objects have \"parent\" */\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\" ||\n        k == \"process_tracepoint\" ||\n        k == \"process_lsm\" ||\n        k == \"process_uprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"parent\"] = map.cilium_tetragon.log[k].parent;\n    }\n  }\n}\n\nrun(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void run(Map map) {\n  for (def k : map?.cilium_tetragon?.log?.keySet()) {\n    /* these tetragon objects have \"process\" */\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\" ||\n        k == \"process_tracepoint\" ||\n        k == \"process_loader\" ||\n        k == \"process_lsm\" ||\n        k == \"process_uprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"process\"] = map.cilium_tetragon.log[k].process;\n    }\n\n    /* these tetragon objects have \"parent\" */\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\" ||\n        k == \"process_tracepoint\" ||\n        k == \"process_lsm\" ||\n        k == \"process_uprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"parent\"] = map.cilium_tetragon.log[k].parent;\n    }\n  }\n}\n\nrun(ctx);\n"#
                ),
            )?;

            if event.has_value("_tmp_.process.arguments") {
                event.rename("_tmp_.process.arguments", "process.args")?;
            }

            if event.has_value("process.args") {
                if let Some(s) = event.get_string("process.args") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("process.args", Value::Array(parts))?;
                }
            }

            if event.has_value("_tmp_.process.binary") {
                event.rename("_tmp_.process.binary", "process.executable")?;
            }

            if event.has_value("_tmp_.process.cwd") {
                event.rename("_tmp_.process.cwd", "process.working_directory")?;
            }

            if event.has_value("_tmp_.process.pid") {
                event.rename("_tmp_.process.pid", "process.pid")?;
            }

            if event.has_value("_tmp_.process.exec_id") {
                event.rename("_tmp_.process.exec_id", "process.entity_id")?;
            }

            if event.has_value("_tmp_.process.tid") {
                event.rename("_tmp_.process.tid", "process.thread.id")?;
            }

            if event.has_value("_tmp_.process.uid") {
                event.rename("_tmp_.process.uid", "process.user.id")?;
            }

            if event.has_value("process.user.id") {
                if let Some(val) = event.get("process.user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.user.id".into(),
                            message,
                        }
                    })?;
                    event.set("process.user.id", converted)?;
                }
            }

            if event.has_value("_tmp_.process.start_time") {
                event.rename("_tmp_.process.start_time", "process.start")?;
            }

            let _cond = { event.has_value("cilium_tetragon.log.process_exec") };
            if _cond {
                event.set("event.action", json!("executed"))?;
            }

            let _cond = { event.has_value("cilium_tetragon.log.process_exit") };
            if _cond {
                event.set("event.action", json!("end"))?;
            }

            let _cond = { event.has_value("cilium_tetragon.log.process_exit.status") };
            if _cond {
                if let Some(v) = event
                    .get("cilium_tetragon.log.process_exit.status")
                    .cloned()
                {
                    event.set("process.exit_code", v)?;
                }
            }

            if event.has_value("_tmp_.parent.arguments") {
                event.rename("_tmp_.parent.arguments", "process.parent.args")?;
            }

            if event.has_value("process.parent.args") {
                if let Some(s) = event.get_string("process.parent.args") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("process.parent.args", Value::Array(parts))?;
                }
            }

            if event.has_value("_tmp_.parent.binary") {
                event.rename("_tmp_.parent.binary", "process.parent.executable")?;
            }

            if event.has_value("_tmp_.parent.cwd") {
                event.rename("_tmp_.parent.cwd", "process.parent.working_directory")?;
            }

            if event.has_value("_tmp_.parent.pid") {
                event.rename("_tmp_.parent.pid", "process.parent.pid")?;
            }

            if event.has_value("_tmp_.parent.exec_id") {
                event.rename("_tmp_.parent.exec_id", "process.parent.entity_id")?;
            }

            if event.has_value("_tmp_.parent.tid") {
                event.rename("_tmp_.parent.tid", "process.parent.thread.id")?;
            }

            if event.has_value("_tmp_.parent.uid") {
                event.rename("_tmp_.parent.uid", "process.parent.user.id")?;
            }

            if event.has_value("process.parent.user.id") {
                if let Some(val) = event.get("process.parent.user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.parent.user.id".into(),
                            message,
                        }
                    })?;
                    event.set("process.parent.user.id", converted)?;
                }
            }

            if event.has_value("_tmp_.parent.start_time") {
                event.rename("_tmp_.parent.start_time", "process.parent.start")?;
            }

            if event.has_value("_tmp_.process.pod.container.name") {
                event.rename("_tmp_.process.pod.container.name", "container.name")?;
            }

            if event.has_value("_tmp_.process.pod.container.id") {
                event.rename("_tmp_.process.pod.container.id", "container.id")?;
            }

            if event.has_value("_tmp_.process.pod.container.image.name") {
                event.rename(
                    "_tmp_.process.pod.container.image.name",
                    "container.image.name",
                )?;
            }

            if event.has_value("_tmp_.process.pod.name") {
                event.rename("_tmp_.process.pod.name", "orchestrator.resource.name")?;
            }

            if event.has_value("_tmp_.process.pod.namespace") {
                event.rename("_tmp_.process.pod.namespace", "orchestrator.namespace")?;
            }

            if event.has_value("_tmp_.process.pod.workload_kind") {
                event.rename(
                    "_tmp_.process.pod.workload_kind",
                    "orchestrator.resource.parent.type",
                )?;
            }

            let _cond = { event.has_value("cilium_tetragon.log.node_name") };
            if _cond {
                if let Some(v) = event.get("cilium_tetragon.log.node_name").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("cilium_tetragon.log.cluster_name") };
            if _cond {
                if let Some(v) = event.get("cilium_tetragon.log.cluster_name").cloned() {
                    event.set("orchestrator.cluster.name", v)?;
                }
            }

            if event.remove("_tmp_").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "_tmp_".into(),
                });
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                if event.remove("_tmp_").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp_".into(),
                    });
                }
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
