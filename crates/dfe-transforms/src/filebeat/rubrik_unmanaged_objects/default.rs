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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("event.kind", json!("metric"))?;

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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.id") {
                event.rename("json.id", "rubrik.unmanaged_objects.id")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "rubrik.unmanaged_objects.name")?;
            }

            if event.has_value("json.objectType") {
                event.rename("json.objectType", "rubrik.unmanaged_objects.object_type")?;
            }

            if event.has_value("json.snapshotCount") {
                event.rename(
                    "json.snapshotCount",
                    "rubrik.unmanaged_objects.snapshot.count",
                )?;
            }

            if event.has_value("json.unmanagedStatus") {
                event.rename(
                    "json.unmanagedStatus",
                    "rubrik.unmanaged_objects.unmanaged_status",
                )?;
            }

            if event.has_value("json.localStorage") {
                event.rename(
                    "json.localStorage",
                    "rubrik.unmanaged_objects.local_storage.bytes",
                )?;
            }

            if event.has_value("json.archiveStorage") {
                event.rename(
                    "json.archiveStorage",
                    "rubrik.unmanaged_objects.archive_storage.bytes",
                )?;
            }

            if event.has_value("json.retentionSlaDomainId") {
                event.rename(
                    "json.retentionSlaDomainId",
                    "rubrik.unmanaged_objects.retention_sla_domain.id",
                )?;
            }

            if event.has_value("json.retentionSlaDomainName") {
                event.rename(
                    "json.retentionSlaDomainName",
                    "rubrik.unmanaged_objects.retention_sla_domain.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            event.remove("json");

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
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
