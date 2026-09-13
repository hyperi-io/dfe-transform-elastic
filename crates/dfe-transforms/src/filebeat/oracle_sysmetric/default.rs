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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.metrics") {
                    event.rename("oracle.metrics", "oracle.sysmetric")?;
                }
                Ok(())
            })();

            if event.has_value("oracle.sysmetric") {
                foreach_array(event, "oracle.sysmetric", |event| {
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
                if event.has_value("oracle.sysmetric") {
                    foreach_array(event, "oracle.sysmetric", |event| {
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
                if event.has_value("oracle.sysmetric") {
                    foreach_array(event, "oracle.sysmetric", |event| {
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
                if event.has_value("oracle.sysmetric") {
                    foreach_array(event, "oracle.sysmetric", |event| {
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
