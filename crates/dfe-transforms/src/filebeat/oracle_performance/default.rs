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
                if event.has_value("sql") {
                    event.rename("sql", "oracle")?;
                }
                Ok(())
            })();

            if event.has_value("oracle.metrics") {
                event.rename("oracle.metrics", "oracle.performance")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.hit_ratio") {
                    event.rename(
                        "oracle.performance.hit_ratio",
                        "oracle.performance.cache.buffer.hit.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.consistent_gets") {
                    event.rename(
                        "oracle.performance.consistent_gets",
                        "oracle.performance.cache.get.consistent",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.db_block_gets") {
                    event.rename(
                        "oracle.performance.db_block_gets",
                        "oracle.performance.cache.get.db_blocks",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.physical_reads") {
                    event.rename(
                        "oracle.performance.physical_reads",
                        "oracle.performance.cache.physical_reads",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.name") {
                    event.rename("oracle.performance.name", "oracle.performance.buffer_pool")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.avg_cur") {
                    event.rename(
                        "oracle.performance.avg_cur",
                        "oracle.performance.cursors.avg",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.max_cur") {
                    event.rename(
                        "oracle.performance.max_cur",
                        "oracle.performance.cursors.max",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.total_cur") {
                    event.rename(
                        "oracle.performance.total_cur",
                        "oracle.performance.cursors.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.cachehits_totalcursors_ratio") {
                    event.rename(
                        "oracle.performance.cachehits_totalcursors_ratio",
                        "oracle.performance.cursors.cache_hit.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.current_cursors") {
                    event.rename(
                        "oracle.performance.current_cursors",
                        "oracle.performance.cursors.opened.current",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.total_cursors") {
                    event.rename(
                        "oracle.performance.total_cursors",
                        "oracle.performance.cursors.opened.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.real_parses") {
                    event.rename(
                        "oracle.performance.real_parses",
                        "oracle.performance.cursors.parse.real",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.parse_count_total") {
                    event.rename(
                        "oracle.performance.parse_count_total",
                        "oracle.performance.cursors.parse.total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.sess_cur_cache_hits") {
                    event.rename(
                        "oracle.performance.sess_cur_cache_hits",
                        "oracle.performance.cursors.session.cache_hits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.active_session_count") {
                    event.rename(
                        "oracle.performance.active_session_count",
                        "oracle.performance.session_count.active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.inactive_morethan_onehr") {
                    event.rename(
                        "oracle.performance.inactive_morethan_onehr",
                        "oracle.performance.session_count.inactive_morethan_onehr",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.inactive_session_count") {
                    event.rename(
                        "oracle.performance.inactive_session_count",
                        "oracle.performance.session_count.inactive",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.pct_time") {
                    event.rename(
                        "oracle.performance.pct_time",
                        "oracle.performance.wait.pct_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.pct_waits") {
                    event.rename(
                        "oracle.performance.pct_waits",
                        "oracle.performance.wait.pct_waits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.time_waited_secs") {
                    event.rename(
                        "oracle.performance.time_waited_secs",
                        "oracle.performance.wait.time_waited_secs",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.total_waits") {
                    event.rename(
                        "oracle.performance.total_waits",
                        "oracle.performance.wait.total_waits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance.wait_class") {
                    event.rename(
                        "oracle.performance.wait_class",
                        "oracle.performance.wait.wait_class",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("oracle.query") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "oracle.performance.query_id",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("oracle.query");
                Ok(())
            })();

            if event.has_value("oracle.performance") {
                foreach_array(event, "oracle.performance", |event| {
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
                if event.has_value("oracle.performance") {
                    foreach_array(event, "oracle.performance", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("\\(%\\)"),
                            "pct",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance") {
                    foreach_array(event, "oracle.performance", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("%"),
                            "pct",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.performance") {
                    foreach_array(event, "oracle.performance", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("/"),
                            "",
                        )?;
                        Ok(())
                    })?;
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
