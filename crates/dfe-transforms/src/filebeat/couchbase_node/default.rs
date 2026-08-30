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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("database")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("couchbase");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "message", "message")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.cmd_get") {
                    event.rename(
                        "message.interestingStats.cmd_get",
                        "couchbase.node.commands.get.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_docs_data_size") {
                    event.rename(
                        "message.interestingStats.couch_docs_data_size",
                        "couchbase.node.couch.docs.data_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_docs_actual_disk_size") {
                    event.rename(
                        "message.interestingStats.couch_docs_actual_disk_size",
                        "couchbase.node.couch.docs.disk_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_spatial_data_size") {
                    event.rename(
                        "message.interestingStats.couch_spatial_data_size",
                        "couchbase.node.couch.spatial.data_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_spatial_disk_size") {
                    event.rename(
                        "message.interestingStats.couch_spatial_disk_size",
                        "couchbase.node.couch.spatial.disk_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_views_data_size") {
                    event.rename(
                        "message.interestingStats.couch_views_data_size",
                        "couchbase.node.couch.views.data_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.couch_views_actual_disk_size") {
                    event.rename(
                        "message.interestingStats.couch_views_actual_disk_size",
                        "couchbase.node.couch.views.disk_size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.systemStats.cpu_utilization_rate") {
                    event.rename(
                        "message.systemStats.cpu_utilization_rate",
                        "couchbase.node.cpu_utilization_rate.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.curr_items_tot") {
                    event.rename(
                        "message.interestingStats.curr_items_tot",
                        "couchbase.node.current_items.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.curr_items") {
                    event.rename(
                        "message.interestingStats.curr_items",
                        "couchbase.node.current_items.value",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.ep_bg_fetched") {
                    event.rename(
                        "message.interestingStats.ep_bg_fetched",
                        "couchbase.node.ep_bg_fetched",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.get_hits") {
                    event.rename(
                        "message.interestingStats.get_hits",
                        "couchbase.node.get.hits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.hostname") {
                    event.rename("message.hostname", "couchbase.node.hostname")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.mcdMemoryAllocated") {
                    event.rename(
                        "message.mcdMemoryAllocated",
                        "couchbase.node.memcached.allocated.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.mcdMemoryReserved") {
                    event.rename(
                        "message.mcdMemoryReserved",
                        "couchbase.node.memcached.reserved.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.memoryFree") {
                    event.rename("message.memoryFree", "couchbase.node.memory.free.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.memoryTotal") {
                    event.rename("message.memoryTotal", "couchbase.node.memory.total.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.mem_used") {
                    event.rename(
                        "message.interestingStats.mem_used",
                        "couchbase.node.memory.used.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.ops") {
                    event.rename(
                        "message.interestingStats.ops",
                        "couchbase.node.operations.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.systemStats.swap_total") {
                    event.rename(
                        "message.systemStats.swap_total",
                        "couchbase.node.swap.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.systemStats.swap_used") {
                    event.rename(
                        "message.systemStats.swap_used",
                        "couchbase.node.swap.used.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.uptime") {
                    if let Some(val) = event.get("message.uptime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "message.uptime".into(),
                                message,
                            }
                        })?;
                        event.set("couchbase.node.uptime.sec", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message.interestingStats.vb_replica_curr_items") {
                    event.rename(
                        "message.interestingStats.vb_replica_curr_items",
                        "couchbase.node.vb_replica.items.current",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("message");
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
