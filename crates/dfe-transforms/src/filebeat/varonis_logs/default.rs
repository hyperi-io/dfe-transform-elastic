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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "2faab67f39077cee83c4c744655747c4ae40cc2e709435b645afd0d6c50496aa",
                    r#"{"params":"FirstContainedMember","known":[{"LabelledKeyRename":{"container":"cef.extensions","suffix":"Label"}}]}"#,
                    r#"void changeLabelsToKeys(Map input) {\n    List keys = new ArrayList(input.keySet()); // To avoid conflicts\n    for (String key : keys) {\n        if (key.endsWith(\"Label\")) {\n            String valueKey = key.substring(0, key.length() - 5);\n            if (input.containsKey(valueKey) && input.get(valueKey) != null && !input.get(valueKey).toString().isEmpty()) {\n                input.put(input.get(key), input.get(valueKey));\n                input.remove(valueKey);\n                input.remove(key);\n            } else {\n                input.remove(key);\n            }\n        }\n    }\n}\nchangeLabelsToKeys(ctx.cef.extensions);\n"#
                ),
            )?;

            event.rename("cef.extensions", "varonis.logs")?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "fd9a882dca5ba0618a3e5263c42d1b564d88475ca533775e59aad5a42b9e6be8",
                    r#"{"params":null,"known":[{"KeysToSnakeCase":["varonis.logs","CamelBreak"]}]}"#
                ),
            )?;

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            let _cond = { event.has_value("event.severity") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "60df504f134fbc74932664f63bd0d43d86c19f62da138b3425834cddd76bd615",
                        r#"{"params":{"TableLookupOrLiteral":{"source":"event.severity","table":"descriptions","target":"event.severity_label","default":{"Literal":"unknown"},"gate":null}},"known":[]}"#
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
