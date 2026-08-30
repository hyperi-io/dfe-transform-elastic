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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "snyk.issues")?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", Value::Array(vec![json!("alert")]))?;

            if let Some(date_str) = event.get_as_string("snyk.issues.attributes.updated_at") {
                match parse_date_out(
                    &date_str,
                    &["yyyy-MM-dd'T'HH:mm:ss[.SSS][.SS][.S]'Z'"],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "snyk.issues.attributes.updated_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("snyk.issues.attributes.ignored") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("snyk.issues.attributes.status") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("snyk.issues.attributes.updated_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("snyk.issues.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("vulnerability.scanner.vendor", json!("Snyk"))?;

            if let Some(v) = event
                .get("snyk.issues.relationships.organization.data.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("snyk.issues.attributes.effective_severity_level")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.severity", v)?;
            }

            let _cond = { event.has_value("snyk.issues.attributes.classes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "snyk.issues.attributes.classes", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "vulnerability.enumeration",
                                json!(
                                    event
                                        .get("_ingest._value.source")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("snyk.issues.attributes.problems") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "snyk.issues.attributes.problems", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "vulnerability.id",
                                json!(
                                    event
                                        .get("_ingest._value.id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("snyk.issues.attributes.problems") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "snyk.issues.attributes.problems", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "vulnerability.enumeration",
                                json!(
                                    event
                                        .get("_ingest._value.source")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("snyk.issues.attributes.problems") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "snyk.issues.attributes.problems", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "vulnerability.reference",
                                json!(
                                    event
                                        .get("_ingest._value.url")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            let _cond = {
                event
                    .get("snyk.issues.relationships.scan_item.data.attributes.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def result = [];\nfor (def tag : ctx.snyk.issues.relationships.scan_item.data.attributes.tags) {\n  if (tag instanceof Map) {\n    result.add([tag.key, tag.value].join(\":\"))\n  } else if (tag instanceof String) {\n    result.add(tag);\n  }\n}\nctx.snyk.issues.relationships.scan_item.data.attributes.tags = result;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def result = [];\nfor (def tag : ctx.snyk.issues.relationships.scan_item.data.attributes.tags) {\n  if (tag instanceof Map) {\n    result.add([tag.key, tag.value].join(\":\"))\n  } else if (tag instanceof String) {\n    result.add(tag);\n  }\n}\nctx.snyk.issues.relationships.scan_item.data.attributes.tags = result;\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("snyk.issues.relationships.scan_item.data._enrich_error_message")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append_unique(
                    "error.message",
                    json!(
                        event
                            .get("snyk.issues.relationships.scan_item.data._enrich_error_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get("snyk.issues.relationships.scan_item.data.relationships.importer.links.related").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("snyk.issues.relationships.scan_item.data.relationships.importer.links.related", "snyk.issues.relationships.scan_item.data.relationships.importer.links.related.href")?;
            }

            let _cond = {
                event.get("snyk.issues.relationships.scan_item.data.relationships.organization.links.related").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("snyk.issues.relationships.scan_item.data.relationships.organization.links.related", "snyk.issues.relationships.scan_item.data.relationships.organization.links.related.href")?;
            }

            let _cond = {
                event.get("snyk.issues.relationships.scan_item.data.relationships.owner.links.related.related").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("snyk.issues.relationships.scan_item.data.relationships.owner.links.related.related", "snyk.issues.relationships.scan_item.data.relationships.owner.links.related.related.href")?;
            }

            let _cond = {
                event.get("snyk.issues.relationships.scan_item.data.relationships.target.links.related").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("snyk.issues.relationships.scan_item.data.relationships.target.links.related", "snyk.issues.relationships.scan_item.data.relationships.target.links.related.href")?;
            }

            event.remove("message");
            event.remove("snyk.issues.cvssScore");
            event.remove("snyk.issues.type");
            event.remove("snyk.issues.relationships.scan_item.data._enrich_error_message");

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
