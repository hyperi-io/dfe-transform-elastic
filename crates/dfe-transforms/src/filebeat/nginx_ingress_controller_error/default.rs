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
            event.set("ecs.version", json!("8.4.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?P<log_level>(?:[A-Z]{1}))%{MONTHNUM2:timestamp_month}%{MONTHDAY:timestamp_day} %{HOUR:timestamp_hour}:%{MINUTE:timestamp_minute}:%{SECOND:timestamp_second}.(?P<timestamp_nano>(?:[0-9]{6}))%{SPACE}%{NUMBER:nginx_ingress_controller.error.thread_id} (?P<nginx_ingress_controller_error_source_file>(?:[^:]+)):%{NUMBER:nginx_ingress_controller.error.source.line_number}\\] (?P<message>(?:(.|\\n)*))
                    let _ = cached_grok_mapped!("(?P<log_level>(?:[A-Z]{1}))%{MONTHNUM2:timestamp_month}%{MONTHDAY:timestamp_day} %{HOUR:timestamp_hour}:%{MINUTE:timestamp_minute}:%{SECOND:timestamp_second}.(?P<timestamp_nano>(?:[0-9]{6}))%{SPACE}%{NUMBER:nginx_ingress_controller.error.thread_id} (?P<nginx_ingress_controller_error_source_file>(?:[^:]+)):%{NUMBER:nginx_ingress_controller.error.source.line_number}\\] (?P<message>(?:(.|\\n)*))", [("log_level", "log.level"), ("nginx_ingress_controller_error_source_file", "nginx_ingress_controller.error.source.file")]).extract_into(&input, event)?;
                }
            }

            event.rename("@timestamp", "event.created")?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

            let _cond = {
                event.get_str("log.level") == Some("E") || event.get_str("log.level") == Some("F")
            };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = {
                event.get_str("log.level") != Some("E") && event.get_str("log.level") != Some("F")
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            if let Some(val) = event.get("nginx_ingress_controller.error.thread_id") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "nginx_ingress_controller.error.thread_id".into(),
                        message,
                    }
                })?;
                event.set("nginx_ingress_controller.error.thread_id", converted)?;
            }

            if let Some(val) = event.get("nginx_ingress_controller.error.source.line_number") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "nginx_ingress_controller.error.source.line_number".into(),
                        message,
                    }
                })?;
                event.set(
                    "nginx_ingress_controller.error.source.line_number",
                    converted,
                )?;
            }

            if let Some(val) = event.get("timestamp_month") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_month".into(),
                        message,
                    }
                })?;
                event.set("timestamp_month", converted)?;
            }

            if let Some(val) = event.get("timestamp_day") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_day".into(),
                        message,
                    }
                })?;
                event.set("timestamp_day", converted)?;
            }

            if let Some(val) = event.get("timestamp_hour") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_hour".into(),
                        message,
                    }
                })?;
                event.set("timestamp_hour", converted)?;
            }

            if let Some(val) = event.get("timestamp_minute") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_minute".into(),
                        message,
                    }
                })?;
                event.set("timestamp_minute", converted)?;
            }

            if let Some(val) = event.get("timestamp_second") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_second".into(),
                        message,
                    }
                })?;
                event.set("timestamp_second", converted)?;
            }

            if let Some(val) = event.get("timestamp_nano") {
                let converted = convert_value(val, "integer").map_err(|message| {
                    TransformError::ParseError {
                        path: "timestamp_nano".into(),
                        message,
                    }
                })?;
                event.set("timestamp_nano", converted)?;
            }

            // Painless script
            // Source: ZoneId zid = ZoneId.of(ctx.event.timezone); ZonedDateTime zdt = ZonedDateTime.parse(ctx.event.created)\n         .withMonth(ctx.timestamp_month)\n         .withDayOfMonth(ctx.timestamp_day)\n         .withHour(ctx.timestamp_hour)\n         .withMinute(ctx.timestamp_minute)\n         .withSecond(ctx.timestamp_second)\n         .withNano(ctx.timestamp_nano * 1000)\n         .withZoneSameLocal(zid);\nctx.timestamp = zdt;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ZoneId zid = ZoneId.of(ctx.event.timezone); ZonedDateTime zdt = ZonedDateTime.parse(ctx.event.created)\n         .withMonth(ctx.timestamp_month)\n         .withDayOfMonth(ctx.timestamp_day)\n         .withHour(ctx.timestamp_hour)\n         .withMinute(ctx.timestamp_minute)\n         .withSecond(ctx.timestamp_second)\n         .withNano(ctx.timestamp_nano * 1000)\n         .withZoneSameLocal(zid);\nctx.timestamp = zdt;"#
                ),
            )?;

            event.rename("timestamp", "@timestamp")?;

            if event.remove("timestamp_month").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_month".into(),
                });
            }
            if event.remove("timestamp_day").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_day".into(),
                });
            }
            if event.remove("timestamp_hour").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_hour".into(),
                });
            }
            if event.remove("timestamp_minute").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_minute".into(),
                });
            }
            if event.remove("timestamp_second").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_second".into(),
                });
            }
            if event.remove("timestamp_nano").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp_nano".into(),
                });
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
