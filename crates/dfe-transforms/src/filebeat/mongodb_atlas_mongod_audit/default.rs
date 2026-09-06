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
            let _cond = {
                event.get_str("message")
                    == Some("No data for given time period or host is unreachable")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.module", json!("mongodb_atlas"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("network"), json!("authentication")]),
            )?;

            event.set(
                "event.type",
                Value::Array(vec![json!("access"), json!("info")]),
            )?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("error message set and no data to process.").to_string(),
                });
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("host_name") {
                event.rename("host_name", "mongodb_atlas.mongod_audit.hostname")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ts.$date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.ts.$date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.local.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.local.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.remote.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.remote.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.atype") {
                event.rename("json.atype", "event.action")?;
            }

            if event.has_value("json.local.port") {
                event.rename("json.local.port", "mongodb_atlas.mongod_audit.local.port")?;
            }

            if event.has_value("json.local.ip") {
                event.rename("json.local.ip", "mongodb_atlas.mongod_audit.local.ip")?;
            }

            if event.has_value("json.local.isSystemUser") {
                event.rename(
                    "json.local.isSystemUser",
                    "mongodb_atlas.mongod_audit.local.is_system_user",
                )?;
            }

            if event.has_value("json.local.unix") {
                event.rename("json.local.unix", "mongodb_atlas.mongod_audit.local.unix")?;
            }

            if event.has_value("json.remote.port") {
                event.rename("json.remote.port", "mongodb_atlas.mongod_audit.remote.port")?;
            }

            if event.has_value("json.remote.ip") {
                event.rename("json.remote.ip", "mongodb_atlas.mongod_audit.remote.ip")?;
            }

            if event.has_value("json.remote.isSystemUser") {
                event.rename(
                    "json.remote.isSystemUser",
                    "mongodb_atlas.mongod_audit.remote.is_system_user",
                )?;
            }

            if event.has_value("json.remote.unix") {
                event.rename("json.remote.unix", "mongodb_atlas.mongod_audit.remote.unix")?;
            }

            if event.has_value("json.param") {
                event.rename("json.param", "mongodb_atlas.mongod_audit.param")?;
            }

            if event.has_value("json.users") {
                event.rename("json.users", "mongodb_atlas.mongod_audit.user.names")?;
            }

            if event.has_value("json.roles") {
                event.rename("json.roles", "mongodb_atlas.mongod_audit.user.roles")?;
            }

            if event.has_value("json.uuid.$binary") {
                event.rename(
                    "json.uuid.$binary",
                    "mongodb_atlas.mongod_audit.uuid.binary",
                )?;
            }

            if event.has_value("json.uuid.$type") {
                event.rename("json.uuid.$type", "mongodb_atlas.mongod_audit.uuid.type")?;
            }

            if event.has_value("json.result") {
                if let Some(val) = event.get("json.result") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.result".into(),
                            message,
                        }
                    })?;
                    event.set("json.result", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: String value = ctx.json?.result;\nif (value != null) {\n  ctx.mongodb_atlas.mongod_audit.result = params.error_codes.getOrDefault(value, null);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String value = ctx.json?.result;\nif (value != null) {\n  ctx.mongodb_atlas.mongod_audit.result = params.error_codes.getOrDefault(value, null);\n}\n"#
                    ),
                    cached_params!(
                        "{\"error_codes\":{\"0\":\"Success\",\"13\":\"Unauthorized to perform the operation\",\"18\":\"Authentication Failed\",\"26\":\"Namespace Not Found\",\"276\":\"Index build aborted\",\"334\":\"Unauthorized to perform the operation\"}}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "informative_error_code")?;
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

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
                event.remove("event.original");
            }

            event.remove("json");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
