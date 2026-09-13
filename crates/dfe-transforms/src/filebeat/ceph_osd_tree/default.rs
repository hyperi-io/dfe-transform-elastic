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

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("ceph"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
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

            if event.has_value("json.exists") {
                if let Some(val) = event.get("json.exists") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.exists".into(),
                            message,
                        }
                    })?;
                    event.set("json.exists", converted)?;
                }
            }

            let _cond = { event.get_str("json.exists") == Some("1") };
            if _cond {
                event.set("json.exists", json!(true))?;
            }

            let _cond = { event.get_str("json.exists") == Some("0") };
            if _cond {
                event.set("json.exists", json!(false))?;
            }

            if event.has_value("json.children") {
                if let Some(val) = event.get("json.children") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.children".into(),
                            message,
                        }
                    })?;
                    event.set("json.children", converted)?;
                }
            }

            if event.has_value("json.children") {
                event.rename("json.children", "ceph.osd_tree.children")?;
            }

            if event.has_value("json.crush_weight") {
                event.rename("json.crush_weight", "ceph.osd_tree.crush_weight")?;
            }

            if event.has_value("json.depth") {
                event.rename("json.depth", "ceph.osd_tree.depth")?;
            }

            if event.has_value("json.device_class") {
                event.rename("json.device_class", "ceph.osd_tree.device_class")?;
            }

            if event.has_value("json.exists") {
                event.rename("json.exists", "ceph.osd_tree.exists")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "ceph.osd_tree.node_osd_id")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "ceph.osd_tree.node_osd_name")?;
            }

            if event.has_value("json.primary_affinity") {
                event.rename(
                    "json.primary_affinity",
                    "ceph.osd_tree.primary_affinity.count",
                )?;
            }

            if event.has_value("json.reweight") {
                event.rename("json.reweight", "ceph.osd_tree.reweight")?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "ceph.osd_tree.status")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "ceph.osd_tree.type.name")?;
            }

            if event.has_value("json.type_id") {
                event.rename("json.type_id", "ceph.osd_tree.type.id")?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
