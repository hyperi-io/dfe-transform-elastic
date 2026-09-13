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
                event.rename("oracle.metrics", "oracle.tablespace")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.file_id") {
                    event.rename(
                        "oracle.tablespace.file_id",
                        "oracle.tablespace.data_file.id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.file_name") {
                    event.rename(
                        "oracle.tablespace.file_name",
                        "oracle.tablespace.data_file.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.status") {
                    event.rename(
                        "oracle.tablespace.status",
                        "oracle.tablespace.data_file.status",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.online_status") {
                    event.rename(
                        "oracle.tablespace.online_status",
                        "oracle.tablespace.data_file.online_status",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.bytes") {
                    event.rename(
                        "oracle.tablespace.bytes",
                        "oracle.tablespace.data_file.size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.maxbytes") {
                    event.rename(
                        "oracle.tablespace.maxbytes",
                        "oracle.tablespace.data_file.size.max.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.user_bytes") {
                    event.rename(
                        "oracle.tablespace.user_bytes",
                        "oracle.tablespace.data_file.size.free.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tb_size_free") {
                    event.rename(
                        "oracle.tablespace.tb_size_free",
                        "oracle.tablespace.space.free.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tb_size_used") {
                    event.rename(
                        "oracle.tablespace.tb_size_used",
                        "oracle.tablespace.space.used.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tablespace_name") {
                    event.rename(
                        "oracle.tablespace.tablespace_name",
                        "oracle.tablespace.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.total_bytes") {
                    event.rename(
                        "oracle.tablespace.total_bytes",
                        "oracle.tablespace.space.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tb_extended_total") {
                    event.rename(
                        "oracle.tablespace.tb_extended_total",
                        "oracle.tablespace.extended_space.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tb_extended_free") {
                    event.rename(
                        "oracle.tablespace.tb_extended_free",
                        "oracle.tablespace.extended_space.free.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("oracle.tablespace.tb_extended_used") {
                    event.rename(
                        "oracle.tablespace.tb_extended_used",
                        "oracle.tablespace.extended_space.used.bytes",
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
                            "oracle.tablespace.query_id",
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
