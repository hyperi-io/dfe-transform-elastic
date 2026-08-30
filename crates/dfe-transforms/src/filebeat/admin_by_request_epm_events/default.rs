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
        let _cond = { event.has_value("error.statuscode") };
        if _cond {
            return Ok(TransformResult::Drop);
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            parse_json_field(event, "message", "admin_by_request_epm.events")?;
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "json")?;
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
            {
                let mut values = Vec::new();
                if let Some(v) = event.get("admin_by_request_epm.events.id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "admin_by_request_epm.events.id".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "fingerprint")?;
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

        // Painless script
        // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec_plan(
            event,
            cached_painless!(
                r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
            ),
        )?;

        // Painless script
        // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.admin_by_request_epm.events != null) {\n  ctx.admin_by_request_epm.events = keysToSnakeCase(ctx.admin_by_request_epm.events);\n}\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec_plan(
            event,
            cached_painless!(
                r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.admin_by_request_epm.events != null) {\n  ctx.admin_by_request_epm.events = keysToSnakeCase(ctx.admin_by_request_epm.events);\n}\n"#
            ),
        )?;

        event.set("ecs.version", json!("8.11.0"))?;

        event.set("event.kind", json!("event"))?;

        event.append("event.category", json!("configuration"))?;

        event.append("event.type", json!("info"))?;

        event.set("event.dataset", json!("admin_by_request_epm.events"))?;

        event.set("event.module", json!("admin_by_request_epm"))?;

        if let Some(v) = event
            .get("admin_by_request_epm.events.computer_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.hostname", v)?;
        }

        let _cond = { event.has_value("admin_by_request_epm.events.computer_name") };
        if _cond {
            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("admin_by_request_epm.events.computer_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("admin_by_request_epm.events.user_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("user.name", v)?;
        }

        if let Some(v) = event
            .get("admin_by_request_epm.events.audit_log_url")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("url.original", v)?;
        }

        if let Some(v) = event
            .get("admin_by_request_epm.events.application.file")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.name", v)?;
        }

        if let Some(v) = event
            .get("admin_by_request_epm.events.application.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.path", v)?;
        }

        if let Some(v) = event
            .get("admin_by_request_epm.events.application.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("hash.sha256", v)?;
        }

        let _cond = { event.has_value("admin_by_request_epm.events.application.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("admin_by_request_epm.events.application.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        event.remove("message");

        Ok(TransformResult::Continue)
    }
}
