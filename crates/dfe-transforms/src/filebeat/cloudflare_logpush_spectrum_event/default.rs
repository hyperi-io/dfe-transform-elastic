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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond = {
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Timestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.Timestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ConnectTimestamp")
                    && event.get_str("json.ConnectTimestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.ConnectTimestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ConnectTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.ConnectTimestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.DisconnectTimestamp")
                    && event.get_str("json.DisconnectTimestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.DisconnectTimestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.DisconnectTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.DisconnectTimestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.Timestamp != null && ctx.json.Timestamp instanceof Number) {\n  ctx.json.Timestamp = convertToMillis(ctx.json.Timestamp);\n}\nif (ctx.json?.ConnectTimestamp != null && ctx.json.ConnectTimestamp instanceof Number) {\n  if (ctx.json.ConnectTimestamp == 0) {\n    ctx.json.ConnectTimestamp = null;\n  } else {\n    ctx.json.ConnectTimestamp = convertToMillis(ctx.json.ConnectTimestamp);\n  }\n}\nif (ctx.json?.DisconnectTimestamp != null && ctx.json.DisconnectTimestamp instanceof Number) {\n  if (ctx.json.DisconnectTimestamp == 0) {\n    ctx.json.DisconnectTimestamp = null;\n  } else {\n    ctx.json.DisconnectTimestamp = convertToMillis(ctx.json.DisconnectTimestamp);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.Timestamp != null && ctx.json.Timestamp instanceof Number) {\n  ctx.json.Timestamp = convertToMillis(ctx.json.Timestamp);\n}\nif (ctx.json?.ConnectTimestamp != null && ctx.json.ConnectTimestamp instanceof Number) {\n  if (ctx.json.ConnectTimestamp == 0) {\n    ctx.json.ConnectTimestamp = null;\n  } else {\n    ctx.json.ConnectTimestamp = convertToMillis(ctx.json.ConnectTimestamp);\n  }\n}\nif (ctx.json?.DisconnectTimestamp != null && ctx.json.DisconnectTimestamp instanceof Number) {\n  if (ctx.json.DisconnectTimestamp == 0) {\n    ctx.json.DisconnectTimestamp = null;\n  } else {\n    ctx.json.DisconnectTimestamp = convertToMillis(ctx.json.DisconnectTimestamp);\n  }\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "painless_timestamp_to_milli",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_Timestamp_70be028a",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.spectrum_event.timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.ConnectTimestamp")
                    && event.get_str("json.ConnectTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ConnectTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "uuuu-MM-dd'T'HH:mm:ssX",
                                "uuuu-MM-dd'T'HH:mm:ss.SSSX",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "UNIX_MS",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.spectrum_event.connect.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.ConnectTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_ConnectTimestamp_to_cloudflare_logpush_spectrum_event_connect_time_2d3e71a3")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.connect.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.has_value("json.DisconnectTimestamp")
                    && event.get_str("json.DisconnectTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.DisconnectTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "uuuu-MM-dd'T'HH:mm:ssX",
                                "uuuu-MM-dd'T'HH:mm:ss.SSSX",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "UNIX_MS",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.spectrum_event.disconnect.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.DisconnectTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_DisconnectTimestamp_to_cloudflare_logpush_spectrum_event_disconnect_time_73e74811")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.disconnect.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("event.start")
                    && event.has_value("event.end")
                    && !event.has_value("event.duration")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\nZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\nctx.event.duration = ChronoUnit.NANOS.between(start, end);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\nZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\nctx.event.duration = ChronoUnit.NANOS.between(start, end);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_efba3c4b")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.Event") {
                event.rename("json.Event", "cloudflare_logpush.spectrum_event.action")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("json.OriginBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginBytes") {
                        if let Some(val) = event.get("json.OriginBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginBytes".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.spectrum_event.origin.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginBytes_to_cloudflare_logpush_spectrum_event_origin_bytes_6396e1ea")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.origin.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.bytes", v)?;
            }

            let _cond =
                { event.has_value("json.OriginIP") && event.get_str("json.OriginIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginIP") {
                        if let Some(val) = event.get("json.OriginIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.spectrum_event.origin.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginIP_to_cloudflare_logpush_spectrum_event_origin_ip_b269aecb")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.origin.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.get_str("json.OriginPort") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginPort") {
                        if let Some(val) = event.get("json.OriginPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginPort".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.spectrum_event.origin.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginPort_to_cloudflare_logpush_spectrum_event_origin_port_1a731398")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.origin.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.Application") {
                event.rename(
                    "json.Application",
                    "cloudflare_logpush.spectrum_event.application",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.application")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.get_str("json.Status") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Status") {
                        if let Some(val) = event.get("json.Status") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Status".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.spectrum_event.status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_Status_to_cloudflare_logpush_spectrum_event_status_82e7a265",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            let _cond = { event.get_str("json.ClientAsn") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientAsn") {
                        if let Some(val) = event.get("json.ClientAsn") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientAsn".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.spectrum_event.client.asn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientAsn_to_cloudflare_logpush_spectrum_event_client_asn_00481746")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.asn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.number", v)?;
            }

            let _cond = { event.get_str("json.ClientBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientBytes") {
                        if let Some(val) = event.get("json.ClientBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientBytes".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.spectrum_event.client.bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientBytes_to_cloudflare_logpush_spectrum_event_client_bytes_d6f6bef3")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            if event.has_value("json.ClientCountry") {
                event.rename(
                    "json.ClientCountry",
                    "cloudflare_logpush.spectrum_event.client.country",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            let _cond =
                { event.has_value("json.ClientIP") && event.get_str("json.ClientIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientIP") {
                        if let Some(val) = event.get("json.ClientIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.spectrum_event.client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientIP_to_cloudflare_logpush_spectrum_event_client_ip_8e63b9ca")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get_str("json.ClientPort") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientPort") {
                        if let Some(val) = event.get("json.ClientPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientPort".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.spectrum_event.client.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientPort_to_cloudflare_logpush_spectrum_event_client_port_97bd15e1")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.ClientMatchedIpFirewall") {
                event.rename(
                    "json.ClientMatchedIpFirewall",
                    "cloudflare_logpush.spectrum_event.client.matched_ip_firewall",
                )?;
            }

            if event.has_value("json.ClientProto") {
                event.rename(
                    "json.ClientProto",
                    "cloudflare_logpush.spectrum_event.client.protocol",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("json.ClientTcpRtt") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientTcpRtt") {
                        if let Some(val) = event.get("json.ClientTcpRtt") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientTcpRtt".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.spectrum_event.client.tcp_rtt",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientTcpRtt_to_cloudflare_logpush_spectrum_event_client_tcp_rtt_69f1bb02")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.ClientTlsCipher") {
                event.rename(
                    "json.ClientTlsCipher",
                    "cloudflare_logpush.spectrum_event.client.tls.cipher",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.tls.cipher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.cipher", v)?;
            }

            if event.has_value("json.ClientTlsClientHelloServerName") {
                event.rename(
                    "json.ClientTlsClientHelloServerName",
                    "cloudflare_logpush.spectrum_event.client.tls.client_hello_server_name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.client.tls.client_hello_server_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.server_name", v)?;
            }

            if event.has_value("json.ClientTlsProtocol") {
                event.rename(
                    "json.ClientTlsProtocol",
                    "cloudflare_logpush.spectrum_event.client.tls.protocol",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.spectrum_event.client.tls.protocol")
                    && event.get_str("cloudflare_logpush.spectrum_event.client.tls.protocol")
                        != Some("none")
                    && event.get_str("cloudflare_logpush.spectrum_event.client.tls.protocol")
                        != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.spectrum_event.client.tls.protocol") {
                        if let Some(input) = event
                            .get_string("cloudflare_logpush.spectrum_event.client.tls.protocol")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("v") else {
                                    break 'dissect false;
                                };
                                captured.push(("tls.version_protocol", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("v") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("tls.version", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "cloudflare_logpush.spectrum_event.client.tls.protocol"
                                        .into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_client_tls_protocol",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.ClientTlsStatus") {
                event.rename(
                    "json.ClientTlsStatus",
                    "cloudflare_logpush.spectrum_event.client.tls.status",
                )?;
            }

            if event.has_value("json.ColoCode") {
                event.rename(
                    "json.ColoCode",
                    "cloudflare_logpush.spectrum_event.colo.code",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.IpFirewall") {
                    if let Some(val) = event.get("json.IpFirewall") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.IpFirewall".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.spectrum_event.ip_firewall", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ipfirewall_to_boolean",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.OriginProto") {
                event.rename(
                    "json.OriginProto",
                    "cloudflare_logpush.spectrum_event.origin.protocol",
                )?;
            }

            let _cond = { event.get_str("json.OriginTcpRtt") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginTcpRtt") {
                        if let Some(val) = event.get("json.OriginTcpRtt") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginTcpRtt".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.spectrum_event.origin.tcp_rtt",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginTcpRtt_to_cloudflare_logpush_spectrum_event_origin_tcp_rtt_7c870d0d")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.OriginTlsCipher") {
                event.rename(
                    "json.OriginTlsCipher",
                    "cloudflare_logpush.spectrum_event.origin.tls.cipher",
                )?;
            }

            if event.has_value("json.OriginTlsFingerprint") {
                event.rename(
                    "json.OriginTlsFingerprint",
                    "cloudflare_logpush.spectrum_event.origin.tls.fingerprint",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.spectrum_event.origin.tls.fingerprint")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.hash.sha256", v)?;
            }

            if event.has_value("json.OriginTlsMode") {
                event.rename(
                    "json.OriginTlsMode",
                    "cloudflare_logpush.spectrum_event.origin.tls.mode",
                )?;
            }

            if event.has_value("json.OriginTlsProtocol") {
                event.rename(
                    "json.OriginTlsProtocol",
                    "cloudflare_logpush.spectrum_event.origin.tls.protocol",
                )?;
            }

            if event.has_value("json.OriginTlsStatus") {
                event.rename(
                    "json.OriginTlsStatus",
                    "cloudflare_logpush.spectrum_event.origin.tls.status",
                )?;
            }

            if event.has_value("json.ProxyProtocol") {
                event.rename(
                    "json.ProxyProtocol",
                    "cloudflare_logpush.spectrum_event.proxy.protocol",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.spectrum_event.client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.spectrum_event.client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.spectrum_event.origin.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.spectrum_event.origin.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.spectrum_event.origin.tls.fingerprint") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.spectrum_event.origin.tls.fingerprint")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("json");
            event.remove("_conf");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("cloudflare_logpush.spectrum_event.timestamp");
                event.remove("cloudflare_logpush.spectrum_event.origin.bytes");
                event.remove("cloudflare_logpush.spectrum_event.origin.ip");
                event.remove("cloudflare_logpush.spectrum_event.origin.port");
                event.remove("cloudflare_logpush.spectrum_event.application");
                event.remove("cloudflare_logpush.spectrum_event.action");
                event.remove("cloudflare_logpush.spectrum_event.status");
                event.remove("cloudflare_logpush.spectrum_event.client.asn");
                event.remove("cloudflare_logpush.spectrum_event.client.bytes");
                event.remove("cloudflare_logpush.spectrum_event.client.country");
                event.remove("cloudflare_logpush.spectrum_event.client.ip");
                event.remove("cloudflare_logpush.spectrum_event.client.port");
                event.remove("cloudflare_logpush.spectrum_event.client.tls.cipher");
                event.remove(
                    "cloudflare_logpush.spectrum_event.client.tls.client_hello_server_name",
                );
                event.remove("cloudflare_logpush.spectrum_event.origin.tls.fingerprint");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
