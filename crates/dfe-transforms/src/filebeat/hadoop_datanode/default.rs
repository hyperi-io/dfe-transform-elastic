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

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http") {
                    event.rename("http", "hadoop")?;
                }
                Ok(())
            })();

            // Painless script
            // Source: ctx.hadoop.datanode.temp = ctx.hadoop.datanode.beans[0]; ctx.hadoop.datanode.remove(\"beans\")\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.hadoop.datanode.temp = ctx.hadoop.datanode.beans[0]; ctx.hadoop.datanode.remove(\"beans\")\n"#
                ),
            )?;

            // SKIPPED: condition not transpiled: ctx.hadoop.datanode.temp["tag.Context"] == 'FSDatasetState'
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Begin nested pipeline: "pipeline-dataset"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.CacheCapacity") {
                        event.rename(
                            "hadoop.datanode.temp.CacheCapacity",
                            "hadoop.datanode.cache.capacity",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.CacheUsed") {
                        event.rename(
                            "hadoop.datanode.temp.CacheUsed",
                            "hadoop.datanode.cache.used",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.NumBlocksCached") {
                        event.rename(
                            "hadoop.datanode.temp.NumBlocksCached",
                            "hadoop.datanode.blocks.cached",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.EstimatedCapacityLostTotal") {
                        event.rename(
                            "hadoop.datanode.temp.EstimatedCapacityLostTotal",
                            "hadoop.datanode.estimated_capacity_lost_total",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.Capacity") {
                        event.rename(
                            "hadoop.datanode.temp.Capacity",
                            "hadoop.datanode.disk_space.capacity",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.Remaining") {
                        event.rename(
                            "hadoop.datanode.temp.Remaining",
                            "hadoop.datanode.disk_space.remaining",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.DfsUsed") {
                        event.rename("hadoop.datanode.temp.DfsUsed", "hadoop.datanode.dfs_used")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.NumBlocksFailedToCache") {
                        event.rename(
                            "hadoop.datanode.temp.NumBlocksFailedToCache",
                            "hadoop.datanode.blocks.failed.to_cache",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.NumBlocksFailedToUnCache") {
                        event.rename(
                            "hadoop.datanode.temp.NumBlocksFailedToUnCache",
                            "hadoop.datanode.blocks.failed.to_uncache",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.NumFailedVolumes") {
                        event.rename(
                            "hadoop.datanode.temp.NumFailedVolumes",
                            "hadoop.datanode.volumes.failed",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("hadoop.datanode.temp.LastVolumeFailureDate")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("hadoop.datanode.last_volume_failure_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "hadoop.datanode.temp.LastVolumeFailureDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline-dataset"
            }

            // SKIPPED: condition not transpiled: ctx.hadoop.datanode.temp["tag.Context"] == 'dfs'
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Begin nested pipeline: "pipeline-dfs"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.BytesRead") {
                        event.rename(
                            "hadoop.datanode.temp.BytesRead",
                            "hadoop.datanode.bytes.read",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("hadoop.datanode.temp.BytesWritten") {
                        event.rename(
                            "hadoop.datanode.temp.BytesWritten",
                            "hadoop.datanode.bytes.written",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline-dfs"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("hadoop.datanode.temp");
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
