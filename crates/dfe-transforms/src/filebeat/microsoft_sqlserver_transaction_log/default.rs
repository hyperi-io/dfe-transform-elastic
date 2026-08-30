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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics.log_since_last_checkpoint_mb") {
                    event.rename(
                        "mssql.metrics.log_since_last_checkpoint_mb",
                        "mssql.metrics.log_since_last_checkpoint",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics.log_recovery_size_mb") {
                    event.rename(
                        "mssql.metrics.log_recovery_size_mb",
                        "mssql.metrics.log_recovery_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics.total_log_size_mb") {
                    event.rename(
                        "mssql.metrics.total_log_size_mb",
                        "mssql.metrics.total_log_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics.active_log_size_mb") {
                    event.rename(
                        "mssql.metrics.active_log_size_mb",
                        "mssql.metrics.active_log_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mssql.metrics.log_since_last_log_backup_mb") {
                    event.rename(
                        "mssql.metrics.log_since_last_log_backup_mb",
                        "mssql.metrics.log_since_last_log_backup",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("mssql.query") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "mssql.metrics.query_id",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
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

            let _cond = { event.has_value("mssql.metrics.log_since_last_checkpoint") };
            if _cond {
                // Painless script
                // Source: ctx.mssql.metrics.log_since_last_checkpoint = Math.round(ctx.mssql.metrics.log_since_last_checkpoint * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mssql.metrics.log_since_last_checkpoint = Math.round(ctx.mssql.metrics.log_since_last_checkpoint * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1048576}"),
                )?;
            }

            let _cond = { event.has_value("mssql.metrics.log_recovery_size") };
            if _cond {
                // Painless script
                // Source: ctx.mssql.metrics.log_recovery_size = Math.round(ctx.mssql.metrics.log_recovery_size * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mssql.metrics.log_recovery_size = Math.round(ctx.mssql.metrics.log_recovery_size * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1048576}"),
                )?;
            }

            let _cond = { event.has_value("mssql.metrics.total_log_size") };
            if _cond {
                // Painless script
                // Source: ctx.mssql.metrics.total_log_size = Math.round(ctx.mssql.metrics.total_log_size * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mssql.metrics.total_log_size = Math.round(ctx.mssql.metrics.total_log_size * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1048576}"),
                )?;
            }

            let _cond = { event.has_value("mssql.metrics.active_log_size") };
            if _cond {
                // Painless script
                // Source: ctx.mssql.metrics.active_log_size = Math.round(ctx.mssql.metrics.active_log_size * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mssql.metrics.active_log_size = Math.round(ctx.mssql.metrics.active_log_size * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1048576}"),
                )?;
            }

            let _cond = { event.has_value("mssql.metrics.log_since_last_log_backup") };
            if _cond {
                // Painless script
                // Source: ctx.mssql.metrics.log_since_last_log_backup = Math.round(ctx.mssql.metrics.log_since_last_log_backup * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mssql.metrics.log_since_last_log_backup = Math.round(ctx.mssql.metrics.log_since_last_log_backup * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1048576}"),
                )?;
            }

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
