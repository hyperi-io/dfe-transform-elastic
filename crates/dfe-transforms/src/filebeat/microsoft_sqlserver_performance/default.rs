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
                    event.rename("sql", "mssql")?;
                }
                Ok(())
            })();

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("preserve_sql_queries"))
                        }
                        serde_json::Value::String(s) => s.contains("preserve_sql_queries"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("mssql.query");
                    Ok(())
                })();
            }

            if event.has_value("mssql.metrics") {
                foreach_array(event, "mssql.metrics", |event| {
                    if event.has_value("_ingest._key") {
                        map_strings(event, "_ingest._key", "_ingest._key", |s| {
                            s.trim().to_string()
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("mssql.metrics") {
                foreach_array(event, "mssql.metrics", |event| {
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
                if event.has_value("mssql.metrics") {
                    foreach_array(event, "mssql.metrics", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("/"),
                            "_",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics") {
                    foreach_array(event, "mssql.metrics", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!(">"),
                            "_",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics") {
                    foreach_array(event, "mssql.metrics", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("\\("),
                            "",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics") {
                    foreach_array(event, "mssql.metrics", |event| {
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
                if event.has_value("mssql.metrics") {
                    foreach_array(event, "mssql.metrics", |event| {
                        gsub_field(
                            event,
                            "_ingest._key",
                            "_ingest._key",
                            cached_regex!("\\)"),
                            "",
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "")?;
            }

            if event.has_value("host.mac") {
                gsub_field(
                    event,
                    "host.mac",
                    "host.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

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
