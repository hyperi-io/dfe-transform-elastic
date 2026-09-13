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
            // Source: ctx.hadoop.node_manager.temp = ctx.hadoop.node_manager.beans[0]; ctx.hadoop.node_manager.remove(\"beans\")\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.hadoop.node_manager.temp = ctx.hadoop.node_manager.beans[0]; ctx.hadoop.node_manager.remove(\"beans\")\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainerLaunchDurationAvgTime") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainerLaunchDurationAvgTime",
                        "hadoop.node_manager.container_launch_duration_avg_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainerLaunchDurationNumOps") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainerLaunchDurationNumOps",
                        "hadoop.node_manager.container_launch_duration_num_ops",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.AllocatedContainers") {
                    event.rename(
                        "hadoop.node_manager.temp.AllocatedContainers",
                        "hadoop.node_manager.allocated_containers",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersCompleted") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersCompleted",
                        "hadoop.node_manager.containers.completed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersFailed") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersFailed",
                        "hadoop.node_manager.containers.failed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersIniting") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersIniting",
                        "hadoop.node_manager.containers.initing",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersKilled") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersKilled",
                        "hadoop.node_manager.containers.killed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersLaunched") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersLaunched",
                        "hadoop.node_manager.containers.launched",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.node_manager.temp.ContainersRunning") {
                    event.rename(
                        "hadoop.node_manager.temp.ContainersRunning",
                        "hadoop.node_manager.containers.running",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("hadoop.node_manager.temp");
                Ok(())
            })();

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
