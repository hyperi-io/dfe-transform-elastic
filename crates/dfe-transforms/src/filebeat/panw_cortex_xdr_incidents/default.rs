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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.append_unique("event.category", json!("malware"))?;

            event.append_unique("event.type", json!("info"))?;

            parse_json_field(event, "event.original", "panw_cortex.xdr")?;

            let _cond = { event.get_i64("panw_cortex.xdr.reply.result_count") == Some(0) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("panw_cortex.xdr.creation_time") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("panw_cortex.xdr.incident_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("panw_cortex.xdr.modification_time") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.creation_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.creation_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.creation_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.creation_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.resolved_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.resolved_timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.resolved_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.resolved_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.modification_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.modification_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.modification_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.modification_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("panw_cortex.xdr.creation_time").cloned() {
                event.set("event.created", v)?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.modification_time") };
            if _cond {
                if let Some(v) = event.get("panw_cortex.xdr.modification_time").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("unknown") };
            if _cond {
                event.set("event.severity", json!(0))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("informational") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("low") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("medium") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("high") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("critical") };
            if _cond {
                event.set("event.severity", json!(5))?;
            }

            if event.has_value("panw_cortex.xdr.incident_id") {
                event.rename_over("panw_cortex.xdr.incident_id", "event.id")?;
            }

            let _cond = {
                event
                    .get("panw_cortex.xdr.description")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("panw_cortex.xdr.description") {
                    event.rename_over("panw_cortex.xdr.description", "event.reason")?;
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.hosts") };
            if _cond {
                if let Some(v) = event.get("panw_cortex.xdr.hosts").cloned() {
                    event.set("related.hosts", v)?;
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.users") };
            if _cond {
                if let Some(v) = event.get("panw_cortex.xdr.users").cloned() {
                    event.set("related.user", v)?;
                }
            }

            let _cond = { event.has_value("related.hosts") };
            if _cond {
                gsub_field(
                    event,
                    "related.hosts",
                    "related.hosts",
                    cached_regex!(":.*"),
                    "",
                )?;
            }

            let _cond = { event.has_value("related.user") };
            if _cond {
                gsub_field(
                    event,
                    "related.user",
                    "related.user",
                    cached_regex!(".*\\\\"),
                    "",
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.mitre_techniques_ids_and_names") };
            if _cond {
                // Painless script
                // Source: void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_techniques_ids_and_names) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_techniques_ids_and_names) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.mitre_tactics_ids_and_names") };
            if _cond {
                // Painless script
                // Source: void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n  ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n  ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n  ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n  ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n  ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n  ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactics_ids_and_names) {\n    addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n  ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n  ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n  ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n  ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n  ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n  ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactics_ids_and_names) {\n    addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.technique") || event.has_value("threat.tactic") };
            if _cond {
                event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.host",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.tags") };
            if _cond {
                foreach_array(event, "panw_cortex.xdr.tags", |event| {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
                event.remove("panw_cortex.xdr.severity");
                event.remove("panw_cortex.xdr.tags");
                event.remove("panw_cortex.xdr.mitre_techniques_id_and_names");
                event.remove("panw_cortex.xdr.mitre_tactics_id_and_names");
            }

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
