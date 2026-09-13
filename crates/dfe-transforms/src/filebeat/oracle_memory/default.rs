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
                event.remove("sql.driver");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("sql.query");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("sql") {
                    event.rename("sql", "oracle")?;
                }
                Ok(())
            })();

            if event.has_value("oracle.metrics") {
                event.rename("oracle.metrics", "oracle.memory")?;
            }

            if event.has_value("oracle.memory") {
                foreach_array(event, "oracle.memory", |event| {
                    gsub_field(
                        event,
                        "_ingest._key",
                        "_ingest._key",
                        cached_regex!(" "),
                        "_",
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.cache_hit_percentage") {
                    event.rename(
                        "oracle.memory.cache_hit_percentage",
                        "oracle.memory.pga.cache_hit_pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.aggregate_pga_auto_target") {
                    event.rename(
                        "oracle.memory.aggregate_pga_auto_target",
                        "oracle.memory.pga.aggregate_auto_target",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.aggregate_pga_target_parameter") {
                    event.rename(
                        "oracle.memory.aggregate_pga_target_parameter",
                        "oracle.memory.pga.aggregate_target_parameter",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.total_pga_allocated") {
                    event.rename(
                        "oracle.memory.total_pga_allocated",
                        "oracle.memory.pga.total_allocated",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.total_pga_used_for_auto_workareas") {
                    event.rename(
                        "oracle.memory.total_pga_used_for_auto_workareas",
                        "oracle.memory.pga.total_used_for_auto_workareas",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.global_memory_bound") {
                    event.rename(
                        "oracle.memory.global_memory_bound",
                        "oracle.memory.pga.global_memory_bound",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.total_pga_inuse") {
                    event.rename(
                        "oracle.memory.total_pga_inuse",
                        "oracle.memory.pga.total_inuse",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.total_freeable_pga_memory") {
                    event.rename(
                        "oracle.memory.total_freeable_pga_memory",
                        "oracle.memory.pga.total_freeable_memory",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.maximum_pga_allocated") {
                    event.rename(
                        "oracle.memory.maximum_pga_allocated",
                        "oracle.memory.pga.maximum_allocated",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.sga_total_memory") {
                    event.rename(
                        "oracle.memory.sga_total_memory",
                        "oracle.memory.sga.total_memory",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.memory.sga_free_memory") {
                    event.rename(
                        "oracle.memory.sga_free_memory",
                        "oracle.memory.sga.free_memory",
                    )?;
                }
                Ok(())
            })();

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
