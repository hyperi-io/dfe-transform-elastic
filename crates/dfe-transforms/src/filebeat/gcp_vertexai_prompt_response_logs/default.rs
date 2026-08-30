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
            if event.has_value("gcp.vertexai_logs") {
                event.rename("gcp.vertexai_logs", "gcp.vertexai.prompt_response_logs")?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("cloud.provider", json!("gcp"))?;

            event.set("cloud.service.name", json!("vertex-ai"))?;

            event.set("service.name", json!("vertex-ai"))?;

            event.set("service.type", json!("ai-platform"))?;

            let _cond = { event.has_value("gcp.vertexai.prompt_response_logs.api_method") };
            if _cond {
                if let Some(v) = event
                    .get("gcp.vertexai.prompt_response_logs.api_method")
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n    def regex = /_?([a-z])([A-Z]+)/;\n    def snakeCaseMap = [:];\n    for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n        v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n        for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n            v.set(i, keysToSnakeCase(item));\n        }\n        }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n    }\n    return snakeCaseMap;\n}\n\nif (ctx.gcp?.vertexai?.prompt_response_logs?.full_request != null) {\n    ctx.gcp.vertexai.prompt_response_logs.full_request = keysToSnakeCase(ctx.gcp.vertexai.prompt_response_logs.full_request);\n}\nif (ctx.gcp?.vertexai?.prompt_response_logs?.full_response != null) {\n    ctx.gcp.vertexai.prompt_response_logs.full_response = keysToSnakeCase(ctx.gcp.vertexai.prompt_response_logs.full_response);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n    def regex = /_?([a-z])([A-Z]+)/;\n    def snakeCaseMap = [:];\n    for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n        v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n        for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n            v.set(i, keysToSnakeCase(item));\n        }\n        }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n    }\n    return snakeCaseMap;\n}\n\nif (ctx.gcp?.vertexai?.prompt_response_logs?.full_request != null) {\n    ctx.gcp.vertexai.prompt_response_logs.full_request = keysToSnakeCase(ctx.gcp.vertexai.prompt_response_logs.full_request);\n}\nif (ctx.gcp?.vertexai?.prompt_response_logs?.full_response != null) {\n    ctx.gcp.vertexai.prompt_response_logs.full_response = keysToSnakeCase(ctx.gcp.vertexai.prompt_response_logs.full_response);\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                    ),
                )?;
                Ok(())
            })();

            event.remove("gcp.vertexai_logs");

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
