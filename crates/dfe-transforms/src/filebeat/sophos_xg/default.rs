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
            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(v) = event.get("message").cloned() {
                if !event.has("event.original") {
                    event.set("event.original", v)?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^%{SYSLOG5424PRI}(%{SYSLOGTIMESTAMP} %{NOTSPACE} )?%{GREEDYDATA:message}$
                // Grok pattern: ^%{SYSLOG5424PRI}%{GREEDYDATA:message}$
                // Grok pattern: ^%{SYSLOGTIMESTAMP} %{HOSTNAME:observer.hostname} %{GREEDYDATA:message}$
                // Grok pattern: %{GREEDYDATA:message}$
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^%{SYSLOG5424PRI}(%{SYSLOGTIMESTAMP} %{NOTSPACE} )?%{GREEDYDATA:message}$"
                        ),
                        cached_grok!("^%{SYSLOG5424PRI}%{GREEDYDATA:message}$"),
                        cached_grok!(
                            "^%{SYSLOGTIMESTAMP} %{HOSTNAME:observer.hostname} %{GREEDYDATA:message}$"
                        ),
                        cached_grok!("%{GREEDYDATA:message}$"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if event.has_value("message") {
                if let Some(kv_str) = event.get_string("message") {
                    for pair in cached_regex!(" (?=[a-zA-Z0-9_]+=)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\"".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("sophos.xg.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("sophos.xg") };
            if _cond {
                // Painless script
                // Source: def lowercaseMap = [:];\nfor(def entry : ctx.sophos.xg.entrySet()){\n  lowercaseMap.put(entry.getKey().toLowerCase(), entry.getValue());\n}\nctx.sophos.xg = lowercaseMap;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def lowercaseMap = [:];\nfor(def entry : ctx.sophos.xg.entrySet()){\n  lowercaseMap.put(entry.getKey().toLowerCase(), entry.getValue());\n}\nctx.sophos.xg = lowercaseMap;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("sophos.xg.timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("sophos.xg.timestamp") {
                        // Grok pattern: %{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:_temp_.tz}?
                        if !cached_grok!("%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:_temp_.tz}?").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("sophos.xg.timezone") };
            if _cond {
                if let Some(v) = event.get("sophos.xg.timezone").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            let _cond = { event.get_str("_temp_.tz") == Some("Z") };
            if _cond {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            if !event.has("_temp_.tz") {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            let _cond = { event.has_value("_temp_.tz") && event.get_str("_temp_.tz") != Some("") };
            if _cond {
                // Painless script
                // Source: def conf = ctx['_conf']; if (conf == null) return;\ndef mappings = conf.tz_map; if (mappings == null) return;\ndef tz_log = ctx._temp_.tz; for (def item : mappings) {\n  if (item.tz_short == tz_log) {\n    ctx._temp_.tz = item.tz_long;\n    break;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def conf = ctx['_conf']; if (conf == null) return;\ndef mappings = conf.tz_map; if (mappings == null) return;\ndef tz_log = ctx._temp_.tz; for (def item : mappings) {\n  if (item.tz_short == tz_log) {\n    ctx._temp_.tz = item.tz_long;\n    break;\n  }\n}"#
                    ),
                )?;
            }

            if let Some(v) = event.get("_temp_.tz").cloned() {
                event.set("event.timezone", v)?;
            }

            if event.has_value("event.timezone") {
                gsub_field(
                    event,
                    "event.timezone",
                    "event.timezone",
                    cached_regex!("([+-][0-9]{2})([0-9]{2})"),
                    "$1:$2",
                )?;
            }

            if event.has_value("event.timezone") {
                gsub_field(
                    event,
                    "event.timezone",
                    "event.timezone",
                    cached_regex!("([+-])([0-9]):?([0-9]{2})"),
                    "$10$2:$3",
                )?;
            }

            let _cond = { event.has_value("sophos.xg.date") && event.has_value("sophos.xg.time") };
            if _cond {
                event.set(
                    "_temp_.time",
                    json!(format!(
                        "{} {}",
                        event
                            .get("sophos.xg.date")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("sophos.xg.time")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { !event.has_value("_temp_.time") };
            if _cond {
                if let Some(v) = event
                    .get("sophos.xg.timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("_temp_.time", v)?;
                }
            }

            let _cond = { event.has_value("_temp_.time") };
            if _cond {
                // on_failure: 4 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss",
                                "yyyy-MM-dd HH:mm:ss Z",
                                "yyyy-MM-dd HH:mm:ss z",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.time".into(),
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
                        "date__temp__time_2f23ff71",
                    )?;
                    event.remove("event.timezone");
                    if let Some(v) = event
                        .get("_conf.tz_offset")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.timezone", v)?;
                    }
                    let _cond = { event.has_value("event.timezone") };
                    if _cond {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_temp_.time") {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "yyyy-MM-dd HH:mm:ss",
                                        "yyyy-MM-dd HH:mm:ss Z",
                                        "yyyy-MM-dd HH:mm:ss z",
                                        "ISO8601",
                                    ],
                                    event.get_str("event.timezone"),
                                    None,
                                ) {
                                    Some(parsed) => event.set("@timestamp", parsed)?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "_temp_.time".into(),
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
                                "date_config_tz_fallback",
                            )?;
                            event.remove("event.timezone");
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_temp_.time") {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "yyyy-MM-dd HH:mm:ss",
                                            "yyyy-MM-dd HH:mm:ss Z",
                                            "yyyy-MM-dd HH:mm:ss z",
                                            "ISO8601",
                                        ],
                                        Some("UTC"),
                                        None,
                                    ) {
                                        Some(parsed) => event.set("@timestamp", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_temp_.time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_utc_config_tz_fallback",
                                )?;
                                event.append(
                                    "error.message",
                                    json!(format!(
                                        "fail-{}",
                                        event
                                            .get("_ingest.on_failure_processor_tag")
                                            .map_or_else(String::new, template_to_string)
                                    )),
                                )?;
                                return Err(TransformError::ParseError {
                                            path: "_fail".into(),
                                            message: (format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))).to_string(),
                                        });
                            }
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { !event.has_value("event.timezone") };
                    if _cond {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_temp_.time") {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "yyyy-MM-dd HH:mm:ss",
                                        "yyyy-MM-dd HH:mm:ss Z",
                                        "yyyy-MM-dd HH:mm:ss z",
                                        "ISO8601",
                                    ],
                                    Some("UTC"),
                                    None,
                                ) {
                                    Some(parsed) => event.set("@timestamp", parsed)?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "_temp_.time".into(),
                                            message: format!("unable to parse date [{date_str}]"),
                                        });
                                    }
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_utc_fallback")?;
                            event.append(
                                "error.message",
                                json!(format!(
                                    "fail-{}",
                                    event
                                        .get("_ingest.on_failure_processor_tag")
                                        .map_or_else(String::new, template_to_string)
                                )),
                            )?;
                            return Err(TransformError::ParseError {
                                    path: "_fail".into(),
                                    message: (format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))).to_string(),
                                });
                        }
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("sophos.xg.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Integer.parseInt(ctx.sophos.xg.duration) * 1000000000L; ctx.event.start = ctx['@timestamp']; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Integer.parseInt(ctx.sophos.xg.duration) * 1000000000L; ctx.event.start = ctx['@timestamp']; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);"#
                    ),
                )?;
            }

            // Painless script
            // Source: ctx.sophos?.xg.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.sophos?.xg.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));"#
                ),
                cached_params!("{\"values\":[\"\",\"-\",\"N/A\"]}"),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(
                    event,
                    "sophos.xg.log_id",
                    "event.severity",
                    cached_regex!("^.{6}(.).*$"),
                    "$1",
                )?;
                Ok(())
            })();

            if event.has_value("event.severity") {
                if let Some(val) = event.get("event.severity") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.severity".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(
                    event,
                    "sophos.xg.log_id",
                    "event.code",
                    cached_regex!("^.{7}(.{5})$"),
                    "$1",
                )?;
                Ok(())
            })();

            let _cond = { event.get_i64("event.severity") == Some(0) };
            if _cond {
                event.set("log.level", json!("unknown"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(1) };
            if _cond {
                event.set("log.level", json!("alert"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(2) };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(3) };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(4) };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(5) };
            if _cond {
                event.set("log.level", json!("notification"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(6) };
            if _cond {
                event.set("log.level", json!("informational"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(7) };
            if _cond {
                event.set("log.level", json!("debug"))?;
            }

            if let Some(v) = event
                .get("sophos.xg.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            event.set("observer.vendor", json!("Sophos"))?;

            event.set("observer.product", json!("XG"))?;

            event.set("observer.type", json!("firewall"))?;

            if event.has_value("sophos.xg.device_id") {
                event.rename("sophos.xg.device_id", "observer.serial_number")?;
            }

            if event.has_value("sophos.xg.device_serial_id") {
                event.rename("sophos.xg.device_serial_id", "observer.serial_number")?;
            }

            if event.has_value("sophos.xg.out_interface") {
                event.rename("sophos.xg.out_interface", "observer.egress.interface.name")?;
            }

            if event.has_value("sophos.xg.in_interface") {
                event.rename("sophos.xg.in_interface", "observer.ingress.interface.name")?;
            }

            if event.has_value("sophos.xg.srczone") {
                event.rename("sophos.xg.srczone", "observer.ingress.zone")?;
            }

            if event.has_value("sophos.xg.src_zone") {
                event.rename("sophos.xg.src_zone", "observer.ingress.zone")?;
            }

            if event.has_value("sophos.xg.dstzone") {
                event.rename("sophos.xg.dstzone", "observer.egress.zone")?;
            }

            if event.has_value("sophos.xg.dst_zone") {
                event.rename("sophos.xg.dst_zone", "observer.egress.zone")?;
            }

            if event.has_value("sophos.xg.srczonetype") {
                event.rename("sophos.xg.srczonetype", "sophos.xg.src_zone_type")?;
            }

            if event.has_value("sophos.xg.dstzonetype") {
                event.rename("sophos.xg.dstzonetype", "sophos.xg.dst_zone_type")?;
            }

            let _cond = { event.has_value("observer.serial_number") };
            if _cond {
                // Painless script
                // Source: def conf = ctx['_conf']; if (conf == null) return; def serial = ctx.observer.serial_number; def mappings = conf.mappings; if (mappings == null) return; def name = conf['default']; for (def item : mappings) {\n  if (item.serial_number == serial) {\n    name = item.hostname;\n    break;\n  }\n} if (ctx.host == null) {\n  ctx.host = new HashMap();\n} ctx.host.name = name;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def conf = ctx['_conf']; if (conf == null) return; def serial = ctx.observer.serial_number; def mappings = conf.mappings; if (mappings == null) return; def name = conf['default']; for (def item : mappings) {\n  if (item.serial_number == serial) {\n    name = item.hostname;\n    break;\n  }\n} if (ctx.host == null) {\n  ctx.host = new HashMap();\n} ctx.host.name = name;"#
                    ),
                )?;
            }

            event.remove("message");
            event.remove("_temp_");
            event.remove("_conf");
            event.remove("sophos.xg.date");
            event.remove("sophos.xg.time");
            event.remove("sophos.xg.timestamp");
            event.remove("sophos.xg.duration");
            event.remove("sophos.xg.dir_disp");
            event.remove("sophos.xg.log_occurrence");
            event.remove("sophos.xg.nat_rule_id");
            event.remove("sophos.xg.in_display_interface");
            event.remove("sophos.xg.out_display_interface");
            event.remove("syslog5424_pri");

            let _cond = { event.has_value("sophos.xg.sent_bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.sent_bytes") {
                        if let Some(val) = event.get("sophos.xg.sent_bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.sent_bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("source.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("sophos.xg.bytes_sent") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.bytes_sent") {
                        if let Some(val) = event.get("sophos.xg.bytes_sent") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.bytes_sent".into(),
                                    message,
                                }
                            })?;
                            event.set("source.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("sophos.xg.recv_bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.recv_bytes") {
                        if let Some(val) = event.get("sophos.xg.recv_bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.recv_bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("sophos.xg.bytes_received") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.bytes_received") {
                        if let Some(val) = event.get("sophos.xg.bytes_received") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.bytes_received".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("sophos.xg.src_mac", "source.mac")?;
                Ok(())
            })();

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[-:. ]"),
                    "",
                )?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("sophos.xg.dst_mac", "destination.mac")?;
                Ok(())
            })();

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Anti-Spam") };
            if _cond {
                // Begin nested pipeline: "antispam"
                event.set("event.kind", json!("event"))?;
                let v = json!(
                    event
                        .get("sophos.xg.log_subtype")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.action", v)?;
                }
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
                let _cond = {
                    [
                        "13001", "13002", "13004", "13005", "13006", "13009", "13012", "13014",
                        "14001", "14002", "15001", "15002",
                    ]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    [
                        "13001", "13002", "13004", "13005", "13006", "13009", "13014", "14001",
                        "14002", "15001", "15002",
                    ]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = { event.get_str("event.code") == Some("13012") };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    [
                        "13003", "13007", "13008", "13010", "13013", "14003", "15003", "18035",
                    ]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    [
                        "13001", "13002", "13004", "13005", "13006", "13009", "13012", "13014",
                        "14001", "14002", "15001", "15002",
                    ]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("sophos.xg.src_domainname") {
                    event.rename("sophos.xg.src_domainname", "source.domain")?;
                }
                if event.has_value("sophos.xg.from_email_address") {
                    event.rename("sophos.xg.from_email_address", "source.user.email")?;
                }
                if event.has_value("sophos.xg.to_email_address") {
                    event.rename("sophos.xg.to_email_address", "destination.user.email")?;
                }
                let _cond = { event.has_value("source.user.email") };
                if _cond {
                    event.append(
                        "email.from.address",
                        json!(
                            event
                                .get("source.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.user.email") };
                if _cond {
                    event.append(
                        "email.to.address",
                        json!(
                            event
                                .get("destination.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.email_subject") };
                if _cond {
                    if let Some(v) = event.get("sophos.xg.email_subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond =
                    { event.has_value("sophos.xg.subject") && !event.has_value("email.subject") };
                if _cond {
                    if let Some(v) = event.get("sophos.xg.subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                if event.has_value("sophos.xg.log_component") {
                    map_strings(
                        event,
                        "sophos.xg.log_component",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.from_email_address");
                event.remove("sophos.xg.to_email_address");
                // End nested pipeline: "antispam"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Anti-Virus") };
            if _cond {
                // Begin nested pipeline: "antivirus"
                event.set("event.kind", json!("alert"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Virus") };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Virus") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { ["09002"].contains(&event.get_str("event.code").unwrap_or("")) };
                if _cond {
                    event.set("event.kind", json!("event"))?;
                }
                let _cond = { ["09002"].contains(&event.get_str("event.code").unwrap_or("")) };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { ["09002"].contains(&event.get_str("event.code").unwrap_or("")) };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("sophos.xg.dstdomain", "destination.domain")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("sophos.xg.dst_domainname", "destination.domain")?;
                    Ok(())
                })();
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("sophos.xg.src_domainname", "source.domain")?;
                    Ok(())
                })();
                if event.has_value("sophos.xg.from_email_address") {
                    event.rename("sophos.xg.from_email_address", "source.user.email")?;
                }
                if event.has_value("sophos.xg.to_email_address") {
                    event.rename("sophos.xg.to_email_address", "destination.user.email")?;
                }
                let _cond = { event.has_value("source.user.email") };
                if _cond {
                    event.append(
                        "email.from.address",
                        json!(
                            event
                                .get("source.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.user.email") };
                if _cond {
                    event.append(
                        "email.to.address",
                        json!(
                            event
                                .get("destination.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.email_subject") };
                if _cond {
                    if let Some(v) = event.get("sophos.xg.email_subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond =
                    { event.has_value("sophos.xg.subject") && !event.has_value("email.subject") };
                if _cond {
                    if let Some(v) = event.get("sophos.xg.subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond = { !event.has_value("rule.id") };
                if _cond {
                    if event.has_value("sophos.xg.fw_rule_id") {
                        event.rename("sophos.xg.fw_rule_id", "rule.id")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.url") };
                if _cond {
                    if event.has_value("sophos.xg.url") {
                        event.rename("sophos.xg.url", "url.original")?;
                    }
                }
                let _cond = {
                    event.has_value("url.original")
                        && event.get("url.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("://"))
                            }
                            serde_json::Value::String(s) => s.contains("://"),
                            _ => false,
                        })
                };
                if _cond {
                    uri_parts(event, "url.original", "url", true, false)?;
                }
                let _cond = {
                    event.has_value("url.original")
                        && event.get("url.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("://"))
                            }
                            serde_json::Value::String(s) => s.contains("://"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("url.original")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.full", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("sophos.xg.domainname", "url.domain")?;
                    Ok(())
                })();
                let _cond = { event.has_value("sophos.xg.user_agent") };
                if _cond {
                    if event.has_value("sophos.xg.user_agent") {
                        event.rename("sophos.xg.user_agent", "user_agent.original")?;
                    }
                }
                let _cond = {
                    event.has_value("sophos.xg.status_code")
                        && event.get_str("sophos.xg.status_code") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.status_code") {
                            if let Some(val) = event.get("sophos.xg.status_code") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.status_code".into(),
                                        message,
                                    }
                                })?;
                                event.set("http.response.status_code", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.filename") };
                if _cond {
                    if event.has_value("sophos.xg.filename") {
                        event.rename("sophos.xg.filename", "file.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.file_size") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.file_size") {
                            if let Some(val) = event.get("sophos.xg.file_size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.file_size".into(),
                                        message,
                                    }
                                })?;
                                event.set("file.size", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.file_path") };
                if _cond {
                    if event.has_value("sophos.xg.file_path") {
                        event.rename("sophos.xg.file_path", "file.directory")?;
                    }
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                if event.has_value("sophos.xg.log_component") {
                    map_strings(
                        event,
                        "sophos.xg.log_component",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.info", "event.info", str::to_lowercase)?;
                    Ok(())
                })();
                event.remove("sophos.xg.domainname");
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.status_code");
                event.remove("sophos.xg.file_size");
                event.remove("sophos.xg.from_email_address");
                event.remove("sophos.xg.to_email_address");
                // End nested pipeline: "antivirus"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("ATP") };
            if _cond {
                // Begin nested pipeline: "atp"
                event.set("event.kind", json!("alert"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond =
                    { ["18009", "18010"].contains(&event.get_str("event.code").unwrap_or("")) };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond =
                    { ["18009", "18010"].contains(&event.get_str("event.code").unwrap_or("")) };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("sophos.xg.eventid") };
                if _cond {
                    if event.has_value("sophos.xg.eventid") {
                        event.rename("sophos.xg.eventid", "event.id")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.destinationip") };
                if _cond {
                    if event.has_value("sophos.xg.destinationip") {
                        event.rename("sophos.xg.destinationip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.sourceip") };
                if _cond {
                    if event.has_value("sophos.xg.sourceip") {
                        event.rename("sophos.xg.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }
                let _cond = { event.has_value("sophos.xg.url") };
                if _cond {
                    if event.has_value("sophos.xg.url") {
                        event.rename("sophos.xg.url", "url.original")?;
                    }
                }
                let _cond = {
                    event.has_value("url.original")
                        && event.get("url.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("://"))
                            }
                            serde_json::Value::String(s) => s.contains("://"),
                            _ => false,
                        })
                };
                if _cond {
                    uri_parts(event, "url.original", "url", true, false)?;
                }
                let _cond = {
                    event.has_value("url.original")
                        && event.get("url.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("://"))
                            }
                            serde_json::Value::String(s) => s.contains("://"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("url.original")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.full", v)?;
                    }
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.info", "event.info", str::to_lowercase)?;
                    Ok(())
                })();
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                // End nested pipeline: "atp"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Content Filtering") };
            if _cond {
                // Begin nested pipeline: "cfilter"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") != Some("Denied") };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond = {
                    ["Allowed", "Warned"]
                        .contains(&event.get_str("sophos.xg.log_subtype").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.user_gp") };
                if _cond {
                    if event.has_value("sophos.xg.user_gp") {
                        event.rename("sophos.xg.user_gp", "source.user.group.name")?;
                    }
                }
                if event.has_value("sophos.xg.url") {
                    event.rename("sophos.xg.url", "url.original")?;
                }
                let _cond = { event.has_value("url.original") };
                if _cond {
                    uri_parts(event, "url.original", "url", true, false)?;
                }
                if let Some(v) = event
                    .get("url.original")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
                let _cond = { !event.has_value("url.domain") };
                if _cond {
                    if event.has_value("sophos.xg.domain") {
                        event.rename("sophos.xg.domain", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.referer") };
                if _cond {
                    if event.has_value("sophos.xg.referer") {
                        event.rename("sophos.xg.referer", "http.request.referrer")?;
                    }
                }
                let _cond = {
                    event.has_value("sophos.xg.status_code")
                        && event.get_str("sophos.xg.status_code") != Some("")
                };
                if _cond {
                    if event.has_value("sophos.xg.status_code") {
                        if let Some(val) = event.get("sophos.xg.status_code") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.status_code".into(),
                                    message,
                                }
                            })?;
                            event.set("http.response.status_code", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("sophos.xg.http_status")
                        && event.get_str("sophos.xg.http_status") != Some("")
                        && event.get_str("sophos.xg.http_status") != Some("0")
                };
                if _cond {
                    if event.has_value("sophos.xg.http_status") {
                        if let Some(val) = event.get("sophos.xg.http_status") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.http_status".into(),
                                    message,
                                }
                            })?;
                            event.set("http.response.status_code", converted)?;
                        }
                    }
                }
                if event.has_value("sophos.xg.user_agent") {
                    event.rename("sophos.xg.user_agent", "user_agent.original")?;
                }
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                if let Some(v) = event
                    .get("url.scheme")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("network.protocol") {
                        event.set("network.protocol", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    Ok(())
                })();
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.domain");
                event.remove("sophos.xg.http_status");
                event.remove("sophos.xg.http_user_agent");
                // End nested pipeline: "cfilter"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Event") };
            if _cond {
                // Begin nested pipeline: "event"
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event.get_str("sophos.xg.log_subtype") == Some("Authentication")
                        && event.get_str("sophos.xg.status") == Some("Successful")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("sophos.xg.log_subtype") == Some("Authentication")
                        && event.get_str("sophos.xg.status") == Some("Failed")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("sophos.xg.log_subtype") == Some("Admin")
                        && event.get_str("sophos.xg.status") == Some("Successful")
                        && event.get_str("event.code") == Some("17507")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("sophos.xg.log_subtype") == Some("Admin")
                        && event.get_str("sophos.xg.status") == Some("Failed")
                        && event.get_str("event.code") == Some("17507")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    ["17701", "17704", "17707", "17710", "17713"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    ["17703", "17706", "17709", "17712", "17715"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                let _cond = {
                    ["SSLVPN", "IPSec", "Thin Client", "Radius SSO"]
                        .contains(&event.get_str("sophos.xg.auth_client").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    ["SSLVPN", "IPSec", "Thin Client", "Radius SSO"]
                        .contains(&event.get_str("sophos.xg.auth_client").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = { event.get_str("event.code") == Some("17819") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.get_str("event.code") == Some("17819") };
                if _cond {
                    event.append("event.category", json!("host"))?;
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.localinterfaceip") };
                if _cond {
                    if event.has_value("sophos.xg.localinterfaceip") {
                        event.rename("sophos.xg.localinterfaceip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.remoteinterfaceip") };
                if _cond {
                    if event.has_value("sophos.xg.remoteinterfaceip") {
                        event.rename("sophos.xg.remoteinterfaceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.name") };
                if _cond {
                    event.set(
                        "source.user.name",
                        json!(
                            event
                                .get("sophos.xg.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") };
                if _cond {
                    let v = json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.usergroupname") };
                if _cond {
                    if event.has_value("sophos.xg.usergroupname") {
                        event.rename("sophos.xg.usergroupname", "source.user.group.name")?;
                    }
                }
                if event.has_value("sophos.xg.message") {
                    event.rename("sophos.xg.message", "message")?;
                }
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.name");
                // End nested pipeline: "event"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Firewall") };
            if _cond {
                // Begin nested pipeline: "firewall"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    ["03001", "05001", "05151", "00003", "00004"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    ["03001", "05001", "05151", "00003", "00004"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    ["Start", "Interim"]
                        .contains(&event.get_str("sophos.xg.connevent").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.connevent") == Some("Stop") };
                if _cond {
                    event.append("event.type", json!("end"))?;
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.status") == Some("Deny") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.tran_dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.tran_dst_ip") {
                        event.rename("sophos.xg.tran_dst_ip", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.destinationip") };
                if _cond {
                    if event.has_value("sophos.xg.destinationip") {
                        event.rename("sophos.xg.destinationip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.tran_dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.tran_dst_port") {
                            if let Some(val) = event.get("sophos.xg.tran_dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.tran_dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.recv_pkts") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.recv_pkts") {
                            if let Some(val) = event.get("sophos.xg.recv_pkts") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.recv_pkts".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.packets_received") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.packets_received") {
                            if let Some(val) = event.get("sophos.xg.packets_received") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.packets_received".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.tran_src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.tran_src_ip") {
                        event.rename("sophos.xg.tran_src_ip", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_trans_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_trans_ip") {
                        event.rename("sophos.xg.src_trans_ip", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.sourceip") };
                if _cond {
                    if event.has_value("sophos.xg.sourceip") {
                        event.rename("sophos.xg.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.tran_src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.tran_src_port") {
                            if let Some(val) = event.get("sophos.xg.tran_src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.tran_src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.src_mac") };
                if _cond {
                    if event.has_value("sophos.xg.src_mac") {
                        event.rename("sophos.xg.src_mac", "source.mac")?;
                    }
                }
                if event.has_value("sophos.xg.sent_pkts") {
                    map_strings(event, "sophos.xg.sent_pkts", "sophos.xg.sent_pkts", |s| {
                        s.trim().to_string()
                    })?;
                }
                if event.has_value("sophos.xg.packets_sent") {
                    map_strings(
                        event,
                        "sophos.xg.packets_sent",
                        "sophos.xg.packets_sent",
                        |s| s.trim().to_string(),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.sent_pkts") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.sent_pkts") {
                            if let Some(val) = event.get("sophos.xg.sent_pkts") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.sent_pkts".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.packets_sent") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.packets_sent") {
                            if let Some(val) = event.get("sophos.xg.packets_sent") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.packets_sent".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.user_gp") };
                if _cond {
                    if event.has_value("sophos.xg.user_gp") {
                        event.rename("sophos.xg.user_gp", "source.user.group.name")?;
                    }
                }
                let _cond = { !event.has_value("rule.id") };
                if _cond {
                    if event.has_value("sophos.xg.fw_rule_id") {
                        event.rename("sophos.xg.fw_rule_id", "rule.id")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.policy_type") };
                if _cond {
                    if event.has_value("sophos.xg.policy_type") {
                        event.rename("sophos.xg.policy_type", "rule.ruleset")?;
                    }
                }
                if event.has_value("sophos.xg.application") {
                    event.rename("sophos.xg.application", "network.protocol")?;
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                let _cond = {
                    ["LAN", "DMZ", "VPN", "WiFi"]
                        .contains(&event.get_str("observer.egress.zone").unwrap_or(""))
                        && event.get_str("observer.ingress.zone") == Some("WAN")
                };
                if _cond {
                    event.set("network.direction", json!("inbound"))?;
                }
                let _cond = {
                    ["LAN", "DMZ", "VPN", "WiFi"]
                        .contains(&event.get_str("observer.ingress.zone").unwrap_or(""))
                        && event.get_str("observer.egress.zone") == Some("WAN")
                };
                if _cond {
                    event.set("network.direction", json!("outbound"))?;
                }
                let _cond = {
                    ["LAN", "DMZ", "VPN", "WiFi"]
                        .contains(&event.get_str("observer.ingress.zone").unwrap_or(""))
                        && ["LAN", "DMZ", "VPN", "WiFi"]
                            .contains(&event.get_str("observer.egress.zone").unwrap_or(""))
                };
                if _cond {
                    event.set("network.direction", json!("internal"))?;
                }
                let _cond = {
                    event.get_str("observer.ingress.zone") == Some("WAN")
                        && event.get_str("observer.egress.zone") == Some("WAN")
                };
                if _cond {
                    event.set("network.direction", json!("external"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    Ok(())
                })();
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.tran_dst_port");
                event.remove("sophos.xg.recv_pkts");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.tran_src_port");
                event.remove("sophos.xg.sent_pkts");
                event.remove("sophos.xg.packets_received");
                event.remove("sophos.xg.packets_sent");
                // End nested pipeline: "firewall"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("IDP") };
            if _cond {
                // Begin nested pipeline: "idp"
                event.set("event.kind", json!("alert"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    ["06001", "06002", "07001", "07002"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = {
                    ["06001", "06002", "07001", "07002"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("sophos.xg.dst_ip") };
                if _cond {
                    if event.has_value("sophos.xg.dst_ip") {
                        event.rename("sophos.xg.dst_ip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.dst_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.dst_port") {
                            if let Some(val) = event.get("sophos.xg.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.src_port") {
                            if let Some(val) = event.get("sophos.xg.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.signature_id") };
                if _cond {
                    if event.has_value("sophos.xg.signature_id") {
                        event.rename("sophos.xg.signature_id", "rule.id")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.signature_msg") };
                if _cond {
                    if event.has_value("sophos.xg.signature_msg") {
                        event.rename("sophos.xg.signature_msg", "rule.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.classification") };
                if _cond {
                    if event.has_value("sophos.xg.classification") {
                        event.rename("sophos.xg.classification", "rule.category")?;
                    }
                }
                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.info", "event.info", str::to_lowercase)?;
                    Ok(())
                })();
                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                // End nested pipeline: "idp"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Sandbox") };
            if _cond {
                // Begin nested pipeline: "sandstorm"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(
                            event
                                .get("sophos.xg.log_subtype")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("sophos.xg.log_subtype") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") != Some("Denied") };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond =
                    { ["Allowed"].contains(&event.get_str("sophos.xg.log_subtype").unwrap_or("")) };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond =
                    { ["pending"].contains(&event.get_str("sophos.xg.reason").unwrap_or("")) };
                if _cond {
                    event.append("event.type", json!("start"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") == Some("eligible") };
                if _cond {
                    event.append("event.type", json!("end"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.log_component") == Some("Web") };
                if _cond {
                    if event.has_value("sophos.xg.source") {
                        event.rename("sophos.xg.source", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.src_ip") };
                if _cond {
                    if event.has_value("sophos.xg.src_ip") {
                        event.rename("sophos.xg.src_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("url.domain") {
                        if let Some(val) = event.get("url.domain") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "url.domain".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_url_domain_to_destination_ip_01f5ca51",
                    )?;
                    if let Some(v) = event
                        .get("url.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.domain", v)?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = { event.has_value("sophos.xg.filename") };
                if _cond {
                    if event.has_value("sophos.xg.filename") {
                        event.rename("sophos.xg.filename", "file.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.filesize") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.filesize") {
                            if let Some(val) = event.get("sophos.xg.filesize") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.filesize".into(),
                                        message,
                                    }
                                })?;
                                event.set("file.size", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.filetype") };
                if _cond {
                    if event.has_value("sophos.xg.filetype") {
                        event.rename("sophos.xg.filetype", "file.mime_type")?;
                    }
                }
                let _cond = {
                    event.has_value("sophos.xg.sha1sum")
                        && event
                            .get_as_string("sophos.xg.sha1sum")
                            .is_some_and(|s| s.len() == 40)
                };
                if _cond {
                    if event.has_value("sophos.xg.sha1sum") {
                        event.rename("sophos.xg.sha1sum", "file.hash.sha1")?;
                    }
                }
                let _cond = {
                    event.has_value("sophos.xg.sha1sum")
                        && event
                            .get_as_string("sophos.xg.sha1sum")
                            .is_some_and(|s| s.len() == 64)
                };
                if _cond {
                    if event.has_value("sophos.xg.sha1sum") {
                        event.rename("sophos.xg.sha1sum", "file.hash.sha256")?;
                    }
                }
                event.remove("sophos.xg.filesize");
                event.remove("sophos.xg.sha1sum");
                // End nested pipeline: "sandstorm"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("System Health") };
            if _cond {
                // Begin nested pipeline: "systemhealth"
                event.set("event.kind", json!("event"))?;
                if event.has_value("sophos.xg.idle") {
                    event.rename("sophos.xg.idle", "sophos.xg.idle_cpu")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.idle_cpu") {
                        gsub_field(
                            event,
                            "sophos.xg.idle_cpu",
                            "sophos.xg.idle_cpu",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.idle_cpu") {
                        if let Some(val) = event.get("sophos.xg.idle_cpu") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.idle_cpu".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.idle_cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_idle_cpu_db95818d",
                    )?;
                    if event.remove("sophos.xg.idle_cpu").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.idle_cpu".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("sophos.xg.system") {
                    event.rename("sophos.xg.system", "sophos.xg.system_cpu")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.system_cpu") {
                        gsub_field(
                            event,
                            "sophos.xg.system_cpu",
                            "sophos.xg.system_cpu",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.system_cpu") {
                        if let Some(val) = event.get("sophos.xg.system_cpu") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.system_cpu".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.system_cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_system_cpu_ec01adad",
                    )?;
                    if event.remove("sophos.xg.system_cpu").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.system_cpu".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("sophos.xg.user") {
                    event.rename("sophos.xg.user", "sophos.xg.user_cpu")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.user_cpu") {
                        gsub_field(
                            event,
                            "sophos.xg.user_cpu",
                            "sophos.xg.user_cpu",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.user_cpu") {
                        if let Some(val) = event.get("sophos.xg.user_cpu") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.user_cpu".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.user_cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_user_cpu_c2564909",
                    )?;
                    if event.remove("sophos.xg.user_cpu").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.user_cpu".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.used") {
                        if let Some(val) = event.get("sophos.xg.used") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.used".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.used", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_used_8dbe6b69",
                    )?;
                    if event.remove("sophos.xg.used").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.used".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.total_memory") {
                        if let Some(val) = event.get("sophos.xg.total_memory") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.total_memory".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.total_memory", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_total_memory_e8eb0879",
                    )?;
                    if event.remove("sophos.xg.total_memory").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.total_memory".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.free") {
                        if let Some(val) = event.get("sophos.xg.free") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.free".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.free", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_free_6cba3b75",
                    )?;
                    if event.remove("sophos.xg.free").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.free".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.configuration") {
                        gsub_field(
                            event,
                            "sophos.xg.configuration",
                            "sophos.xg.configuration",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.configuration") {
                        if let Some(val) = event.get("sophos.xg.configuration") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.configuration".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.configuration", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_configuration_e9a7a27f",
                    )?;
                    if event.remove("sophos.xg.configuration").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.configuration".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.reports") {
                        gsub_field(
                            event,
                            "sophos.xg.reports",
                            "sophos.xg.reports",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.reports") {
                        if let Some(val) = event.get("sophos.xg.reports") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.reports".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.reports", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_reports_1dfbe06d",
                    )?;
                    if event.remove("sophos.xg.reports").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.reports".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.temp") {
                        gsub_field(
                            event,
                            "sophos.xg.temp",
                            "sophos.xg.temp",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.temp") {
                        if let Some(val) = event.get("sophos.xg.temp") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.temp".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.temp", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_temp_9bb56e85",
                    )?;
                    if event.remove("sophos.xg.temp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.temp".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sophos.xg.signature") {
                        gsub_field(
                            event,
                            "sophos.xg.signature",
                            "sophos.xg.signature",
                            cached_regex!("%$"),
                            "",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.signature") {
                        if let Some(val) = event.get("sophos.xg.signature") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.signature".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.signature", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_signature_f1ca0097",
                    )?;
                    if event.remove("sophos.xg.signature").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.signature".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.users") {
                        if let Some(val) = event.get("sophos.xg.users") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.users".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.users", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_users_389670cf",
                    )?;
                    if event.remove("sophos.xg.users").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.users".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.transmittedkbits") {
                        if let Some(val) = event.get("sophos.xg.transmittedkbits") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.transmittedkbits".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.transmittedkbits", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_transmittedkbits_01bd9d3d",
                    )?;
                    if event.remove("sophos.xg.transmittedkbits").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.transmittedkbits".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.receivedkbits") {
                        if let Some(val) = event.get("sophos.xg.receivedkbits") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.receivedkbits".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.receivedkbits", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_receivedkbits_0f95c677",
                    )?;
                    if event.remove("sophos.xg.receivedkbits").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.receivedkbits".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.collisions") {
                        if let Some(val) = event.get("sophos.xg.collisions") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.collisions".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.collisions", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_collisions_68b4412d",
                    )?;
                    if event.remove("sophos.xg.collisions").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.collisions".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.receiveddrops") {
                        if let Some(val) = event.get("sophos.xg.receiveddrops") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.receiveddrops".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.receiveddrops", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_receiveddrops_50710095",
                    )?;
                    if event.remove("sophos.xg.receiveddrops").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.receiveddrops".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.transmitteddrops") {
                        if let Some(val) = event.get("sophos.xg.transmitteddrops") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.transmitteddrops".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.transmitteddrops", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_transmitteddrops_c502223d",
                    )?;
                    if event.remove("sophos.xg.transmitteddrops").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.transmitteddrops".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // End nested pipeline: "systemhealth"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("WAF") };
            if _cond {
                // Begin nested pipeline: "waf"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.get_str("sophos.xg.reason") == Some("-") };
                if _cond {
                    event.set("event.action", json!("allowed"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
                if _cond {
                    event.set("event.action", json!("denied"))?;
                }
                let _cond = { event.has_value("sophos.xg.reason") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") == Some("Antivirus") };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = {
                    event.get_str("sophos.xg.reason") != Some("Antivirus")
                        && event.get_str("sophos.xg.reason") != Some("-")
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                    event.append("event.category", json!("network"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") == Some("-") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.responsetime") {
                        if let Some(val) = event.get("sophos.xg.responsetime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.responsetime".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.responsetime", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_responsetime_04a3f9c7",
                    )?;
                    if event.remove("sophos.xg.responsetime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.responsetime".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // Painless script
                // Source: if (ctx.sophos?.xg?.responsetime != null && ctx.sophos.xg.responsetime > 0) {\n  ctx.event.duration = ctx.sophos.xg.responsetime * 1000; \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.sophos?.xg?.responsetime != null && ctx.sophos.xg.responsetime > 0) {\n  ctx.event.duration = ctx.sophos.xg.responsetime * 1000; \n}\n"#
                    ),
                )?;
                let _cond = { event.has_value("sophos.xg.localip") };
                if _cond {
                    if event.has_value("sophos.xg.localip") {
                        event.rename("sophos.xg.localip", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.bytessent") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.bytessent") {
                            if let Some(val) = event.get("sophos.xg.bytessent") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.bytessent".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.sourceip") };
                if _cond {
                    if event.has_value("sophos.xg.sourceip") {
                        event.rename("sophos.xg.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.bytesrcv") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.bytesrcv") {
                            if let Some(val) = event.get("sophos.xg.bytesrcv") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.bytesrcv".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.user_name") };
                if _cond {
                    if event.has_value("sophos.xg.user_name") {
                        event.rename("sophos.xg.user_name", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.user_gp") };
                if _cond {
                    if event.has_value("sophos.xg.user_gp") {
                        event.rename("sophos.xg.user_gp", "source.user.group.name")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.url") };
                if _cond {
                    if event.has_value("sophos.xg.url") {
                        event.rename("sophos.xg.url", "url.full")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.domain") };
                if _cond {
                    if event.has_value("sophos.xg.domain") {
                        event.rename("sophos.xg.domain", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.referer") };
                if _cond {
                    if event.has_value("sophos.xg.referer") {
                        event.rename("sophos.xg.referer", "http.request.referrer")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.httpstatus") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("sophos.xg.httpstatus") {
                            if let Some(val) = event.get("sophos.xg.httpstatus") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "sophos.xg.httpstatus".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("sophos.xg.method") };
                if _cond {
                    if event.has_value("sophos.xg.method") {
                        event.rename("sophos.xg.method", "http.request.method")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.ws_protocol") };
                if _cond {
                    if event.has_value("sophos.xg.ws_protocol") {
                        event.rename("sophos.xg.ws_protocol", "http.version")?;
                    }
                }
                let _cond = { event.has_value("sophos.xg.useragent") };
                if _cond {
                    if event.has_value("sophos.xg.useragent") {
                        event.rename("sophos.xg.useragent", "user_agent.original")?;
                    }
                }
                if event.has_value("sophos.xg.SQLi") {
                    event.rename("sophos.xg.SQLi", "sophos.xg.sqli")?;
                }
                if event.has_value("sophos.xg.XSS") {
                    event.rename("sophos.xg.XSS", "sophos.xg.xss")?;
                }
                event.remove("sophos.xg.bytesrcv");
                event.remove("sophos.xg.bytessent");
                event.remove("sophos.xg.httpstatus");
                event.remove("sophos.xg.responsetime");
                // End nested pipeline: "waf"
            }

            let _cond = { event.get_str("sophos.xg.log_type") == Some("Wireless Protection") };
            if _cond {
                // Begin nested pipeline: "wifi"
                event.set("event.kind", json!("event"))?;
                event.set("event.outcome", json!("success"))?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sophos.xg.clients_conn_ssid") {
                        if let Some(val) = event.get("sophos.xg.clients_conn_ssid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sophos.xg.clients_conn_ssid".into(),
                                    message,
                                }
                            })?;
                            event.set("sophos.xg.clients_conn_ssid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sophos_xg_clients_conn_ssid_772f218b",
                    )?;
                    if event.remove("sophos.xg.clients_conn_ssid").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.clients_conn_ssid".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // End nested pipeline: "wifi"
            }

            let _cond = { event.has_value("sophos.xg.eventtime") };
            if _cond {
                gsub_field(
                    event,
                    "sophos.xg.eventtime",
                    "sophos.xg.eventtime",
                    cached_regex!("\\s[A-Z]{2,4}$"),
                    "",
                )?;
            }

            let _cond =
                { event.has_value("sophos.xg.eventtime") && !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sophos.xg.eventtime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss",
                                "yyyy-MM-dd HH:mm:ss Z",
                                "yyyy-MM-dd HH:mm:ss z",
                                "ISO8601",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("sophos.xg.eventtime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sophos.xg.eventtime".into(),
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
                        "date_sophos_xg_eventtime_to_sophos_xg_eventtime_7e1f1a67",
                    )?;
                    if event.remove("sophos.xg.eventtime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.eventtime".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond =
                { event.has_value("sophos.xg.eventtime") && event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sophos.xg.eventtime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss",
                                "yyyy-MM-dd HH:mm:ss Z",
                                "yyyy-MM-dd HH:mm:ss z",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("sophos.xg.eventtime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sophos.xg.eventtime".into(),
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
                        "date_sophos_xg_eventtime_to_sophos_xg_eventtime_fd4e5f38",
                    )?;
                    if event.remove("sophos.xg.eventtime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sophos.xg.eventtime".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond =
                { !event.has_value("source.geo") && event.get_str("source.ip") != Some("") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("destination.geo") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("source.ip") != Some("") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("destination.ip") != Some("") };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond =
                { !event.has_value("source.geo") && event.get_str("source.nat.ip") != Some("") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("destination.geo")
                    && event.get_str("destination.nat.ip") != Some("")
            };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond =
                { !event.has_value("source.as") && event.get_str("source.nat.ip") != Some("") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("destination.as")
                    && event.get_str("destination.nat.ip") != Some("")
            };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
                Ok(())
            })();

            let _cond = { event.get_str("network.protocol") == Some("pops") };
            if _cond {
                event.set("network.protocol", json!("pop3s"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("source.bytes") && event.has_value("destination.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                    sum_directions(event, &["bytes"]);
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("source.packets") && event.has_value("destination.packets") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.network.packets = ctx.source.packets + ctx.destination.packets
                    sum_directions(event, &["packets"]);
                    Ok(())
                })();
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

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") && !event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("source.user.email")
                    && event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("source.user.name", "source.user.email")?;
            }

            let _cond =
                { !event.has_value("source.user.name") && !event.has_value("source.user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.user.email") {
                        if let Some(input) = event.get_string("source.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("source.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("source.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("sophos.xg.reason", "event.reason")?;
                Ok(())
            })();

            event.remove("sophos.xg.bytes_received");
            event.remove("sophos.xg.bytes_sent");
            event.remove("sophos.xg.dst_country");
            event.remove("sophos.xg.in_display_interface");
            event.remove("sophos.xg.out_display_interface");
            event.remove("sophos.xg.recv_bytes");
            event.remove("sophos.xg.sent_bytes");
            event.remove("sophos.xg.severity");
            event.remove("sophos.xg.src_country");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
