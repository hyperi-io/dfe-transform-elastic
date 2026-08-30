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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (%{POSINT:process.pid:long}:(?P<redis_log_role>(?:[a-zA-Z])) )?((?P<redis_log_timestamp>(?:%{MONTHDAY} %{MONTH} %{TIME}))||(?P<redis_log_timestamp>(?:%{MONTHDAY} %{MONTH} %{YEAR} %{TIME}))) (?P<log_level>(?:[.\\-*#])) %{GREEDYDATA:message}
                // Grok pattern: %{POSINT:process.pid:long}:signal-handler \\(%{POSINT:redis.log.timestamp}\\) %{GREEDYDATA:message}
                let _ = extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "(%{POSINT:process.pid:long}:(?P<redis_log_role>(?:[a-zA-Z])) )?((?P<redis_log_timestamp>(?:%{MONTHDAY} %{MONTH} %{TIME}))||(?P<redis_log_timestamp>(?:%{MONTHDAY} %{MONTH} %{YEAR} %{TIME}))) (?P<log_level>(?:[.\\-*#])) %{GREEDYDATA:message}",
                            [
                                ("redis_log_role", "redis.log.role"),
                                ("redis_log_timestamp", "redis.log.timestamp"),
                                ("redis_log_timestamp", "redis.log.timestamp"),
                                ("log_level", "log.level")
                            ]
                        ),
                        cached_grok!(
                            "%{POSINT:process.pid:long}:signal-handler \\(%{POSINT:redis.log.timestamp}\\) %{GREEDYDATA:message}"
                        ),
                    ],
                    &input,
                    event,
                )?;
            }

            let _cond = { event.has_value("log.level") };
            if _cond {
                // Painless script
                // Source: if (ctx.log.level == params.dot) {\n  ctx.log.level = params.debug;\n} else if (ctx.log.level == params.dash) {\n  ctx.log.level = params.verbose;\n} else if (ctx.log.level == params.asterisk) {\n  ctx.log.level = params.notice;\n} else if (ctx.log.level == params.hash) {\n  ctx.log.level = params.warning;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.log.level == params.dot) {\n  ctx.log.level = params.debug;\n} else if (ctx.log.level == params.dash) {\n  ctx.log.level = params.verbose;\n} else if (ctx.log.level == params.asterisk) {\n  ctx.log.level = params.notice;\n} else if (ctx.log.level == params.hash) {\n  ctx.log.level = params.warning;\n}"#
                    ),
                    cached_params!(
                        "{\"dot\":\".\",\"debug\":\"debug\",\"dash\":\"-\",\"verbose\":\"verbose\",\"asterisk\":\"*\",\"notice\":\"notice\",\"hash\":\"#\",\"warning\":\"warning\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("redis.log.role") };
            if _cond {
                // Painless script
                // Source: if (ctx.redis.log.role == params.master_abbrev) {\n  ctx.redis.log.role = params.master;\n} else if (ctx.redis.log.role == params.slave_abbrev) {\n  ctx.redis.log.role = params.slave;\n} else if (ctx.redis.log.role == params.child_abbrev) {\n  ctx.redis.log.role = params.child;\n} else if (ctx.redis.log.role == params.sentinel_abbrev) {\n  ctx.redis.log.role = params.sentinel;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.redis.log.role == params.master_abbrev) {\n  ctx.redis.log.role = params.master;\n} else if (ctx.redis.log.role == params.slave_abbrev) {\n  ctx.redis.log.role = params.slave;\n} else if (ctx.redis.log.role == params.child_abbrev) {\n  ctx.redis.log.role = params.child;\n} else if (ctx.redis.log.role == params.sentinel_abbrev) {\n  ctx.redis.log.role = params.sentinel;\n}"#
                    ),
                    cached_params!(
                        "{\"master_abbrev\":\"M\",\"master\":\"master\",\"slave_abbrev\":\"S\",\"slave\":\"slave\",\"child_abbrev\":\"C\",\"child\":\"child\",\"sentinel_abbrev\":\"X\",\"sentinel\":\"sentinel\"}"
                    ),
                )?;
            }

            event.rename("@timestamp", "event.created")?;

            if let Some(date_str) = event.get_as_string("redis.log.timestamp") {
                match parse_date_out(
                    &date_str,
                    &[
                        "dd MMM yyyy H:m:s.SSS",
                        "dd MMM H:m:s.SSS",
                        "dd MMM H:m:s",
                        "UNIX",
                    ],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "redis.log.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("redis.log.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "redis.log.timestamp".into(),
                    });
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
