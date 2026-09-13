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

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("configuration"))?;

            event.append_unique("event.type", json!("info"))?;

            event.set("event.dataset", json!("varonis.logs"))?;

            event.set("event.module", json!("varonis"))?;

            // Painless script
            // Source: void changeLabelsToKeys(Map input) {\n    List keys = new ArrayList(input.keySet()); // To avoid conflicts\n    for (String key : keys) {\n        if (key.endsWith(\"Label\")) {\n            String valueKey = key.substring(0, key.length() - 5);\n            if (input.containsKey(valueKey) && input.get(valueKey) != null && !input.get(valueKey).toString().isEmpty()) {\n                input.put(input.get(key), input.get(valueKey));\n                input.remove(valueKey);\n                input.remove(key);\n            } else {\n                input.remove(key);\n            }\n        }\n    }\n}\nchangeLabelsToKeys(ctx.cef.extensions);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void changeLabelsToKeys(Map input) {\n    List keys = new ArrayList(input.keySet()); // To avoid conflicts\n    for (String key : keys) {\n        if (key.endsWith(\"Label\")) {\n            String valueKey = key.substring(0, key.length() - 5);\n            if (input.containsKey(valueKey) && input.get(valueKey) != null && !input.get(valueKey).toString().isEmpty()) {\n                input.put(input.get(key), input.get(valueKey));\n                input.remove(valueKey);\n                input.remove(key);\n            } else {\n                input.remove(key);\n            }\n        }\n    }\n}\nchangeLabelsToKeys(ctx.cef.extensions);\n"#
                ),
            )?;

            event.rename("cef.extensions", "varonis.logs")?;

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.varonis.logs != null) {\n  ctx.varonis.logs = keysToSnakeCase(ctx.varonis.logs);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.varonis.logs != null) {\n  ctx.varonis.logs = keysToSnakeCase(ctx.varonis.logs);\n}\n"#
                ),
            )?;

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            let _cond = { event.has_value("event.severity") };
            if _cond {
                // Painless script
                // Source: String severity = String.valueOf(ctx.event.severity);\nctx.event.severity_label = params.descriptions.getOrDefault(\n  severity,\n  \"unknown\"\n);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String severity = String.valueOf(ctx.event.severity);\nctx.event.severity_label = params.descriptions.getOrDefault(\n  severity,\n  \"unknown\"\n);\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"emergency\",\"1\":\"alert\",\"2\":\"critical\",\"3\":\"error\",\"4\":\"warning\",\"5\":\"notice\",\"6\":\"informational\",\"7\":\"debug\"}}"
                    ),
                )?;
            }

            event.remove("cef");
            event.remove("log");
            event.remove("varonis.logs.destination_host_name");
            event.remove("varonis.logs.destination_user_name");
            event.remove("varonis.logs.source_user_name");
            event.remove("varonis.logs.destination_user_privileges");
            event.remove("varonis.logs.device_action");
            event.remove("varonis.logs.end_time");
            event.remove("varonis.logs.start_time");
            event.remove("varonis.logs.event_outcome");
            event.remove("varonis.logs.file_path");
            event.remove("varonis.logs.file_permission");
            event.remove("varonis.logs.filename");
            event.remove("varonis.logs.message");
            event.remove("varonis.logs.file_type");
            event.remove("varonis.logs.device_receipt_time");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
