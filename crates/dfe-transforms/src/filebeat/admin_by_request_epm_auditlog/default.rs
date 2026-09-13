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
            let _cond = { event.has_value("error.statuscode") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "message", "admin_by_request_epm.auditlog")?;
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
                    if let Some(v) = event.get("admin_by_request_epm.auditlog") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "admin_by_request_epm.auditlog".into(),
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

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.admin_by_request_epm.auditlog != null) {\n  ctx.admin_by_request_epm.auditlog = keysToSnakeCase(ctx.admin_by_request_epm.auditlog);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.admin_by_request_epm.auditlog != null) {\n  ctx.admin_by_request_epm.auditlog = keysToSnakeCase(ctx.admin_by_request_epm.auditlog);\n}\n"#
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.dataset", json!("admin_by_request_epm.auditlog"))?;

            event.set("event.module", json!("admin_by_request_epm"))?;

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.user.full_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("admin_by_request_epm.auditlog.user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("admin_by_request_epm.auditlog.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.computer.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.computer.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("admin_by_request_epm.auditlog.computer.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.computer.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("os.platform", v)?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.application.file")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.application.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.application.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("hash.sha256", v)?;
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.application.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("admin_by_request_epm.auditlog.application.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("admin_by_request_epm.auditlog.request_time_utc")
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.response_time") };
            if _cond {
                if let Some(input) = event.get_string("admin_by_request_epm.auditlog.response_time")
                {
                    // Grok pattern: %{HOUR:hours}:%{MINUTE:minutes}:%{SECOND:seconds}.(?P<nanoseconds>(?:\\d{7}))
                    if !cached_grok!("%{HOUR:hours}:%{MINUTE:minutes}:%{SECOND:seconds}.(?P<nanoseconds>(?:\\d{7}))").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("admin_by_request_epm.auditlog.response_time") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.admin_by_request_epm.auditlog.response_time_in_seconds = (Integer.parseInt(ctx.hours) * 3600) + \n                      (Integer.parseInt(ctx.minutes) * 60) + \n                      Integer.parseInt(ctx.seconds) + \n                      (Integer.parseInt(ctx.nanoseconds) / 10000000.0);\n
                scalar_expression(
                    event,
                    &ScalarExpression::new(
                        "admin_by_request_epm.auditlog.response_time_in_seconds",
                        Expr::Binary(
                            Box::new(Expr::Binary(
                                Box::new(Expr::Binary(
                                    Box::new(Expr::Binary(
                                        Box::new(Expr::ParseNumber(Box::new(Expr::Field(
                                            "hours".into(),
                                        )))),
                                        Op::Mul,
                                        Box::new(Expr::Int(3600)),
                                    )),
                                    Op::Add,
                                    Box::new(Expr::Binary(
                                        Box::new(Expr::ParseNumber(Box::new(Expr::Field(
                                            "minutes".into(),
                                        )))),
                                        Op::Mul,
                                        Box::new(Expr::Int(60)),
                                    )),
                                )),
                                Op::Add,
                                Box::new(Expr::ParseNumber(Box::new(Expr::Field(
                                    "seconds".into(),
                                )))),
                            )),
                            Op::Add,
                            Box::new(Expr::Binary(
                                Box::new(Expr::ParseNumber(Box::new(Expr::Field(
                                    "nanoseconds".into(),
                                )))),
                                Op::Div,
                                Box::new(Expr::Float(FloatLit::new(10000000.0))),
                            )),
                        ),
                    ),
                );
            }

            event.remove("admin_by_request_epm.auditlog.request_time");
            event.remove("admin_by_request_epm.auditlog.start_time");
            event.remove("admin_by_request_epm.auditlog.end_time");
            event.remove("hours");
            event.remove("minutes");
            event.remove("seconds");
            event.remove("nanoseconds");
            event.remove("message");

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
