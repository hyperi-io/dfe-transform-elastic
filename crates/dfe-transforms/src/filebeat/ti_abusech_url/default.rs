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
            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

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

            parse_json_field(event, "event.original", "abusech.url")?;

            event.set("threat.indicator.type", json!("url"))?;

            let _cond = { event.has_value("abusech.url.date_added") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.url.date_added") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss z", "yyyy-MM-dd HH:mm:ss Z"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.url.date_added".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("abusech.url.dateadded")
                    && !event.has_value("threat.indicator.first_seen")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.url.dateadded") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss z", "yyyy-MM-dd HH:mm:ss Z"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.url.dateadded".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("abusech.url.last_online") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.url.last_online") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss z", "yyyy-MM-dd HH:mm:ss Z"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.url.last_online".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if event.remove("abusech.url.last_online").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "abusech.url.last_online".into(),
                    });
                }
            }

            if let Some(v) = event.get("abusech.url.url").cloned() {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = { event.has_value("abusech.url.url") };
            if _cond {
                uri_parts(event, "abusech.url.url", "threat.indicator.url", true, true)?;
            }

            let v = json!(
                event
                    .get("threat.indicator.url.original")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("threat.indicator.url.full", v)?;
            }

            if event.has_value("abusech.url.urlhaus_reference") {
                event.rename(
                    "abusech.url.urlhaus_reference",
                    "threat.indicator.reference",
                )?;
            }

            let _cond = {
                event.has_value("abusech.url.urlhaus_link")
                    && !event.has_value("threat.indicator.reference")
            };
            if _cond {
                if event.has_value("abusech.url.urlhaus_link") {
                    event.rename("abusech.url.urlhaus_link", "threat.indicator.reference")?;
                }
            }

            let _cond = { event.has_value("abusech.url.host") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("abusech.url.host") {
                        // Grok pattern: (?:%{IP:threat.indicator.ip}|%{GREEDYDATA:threat.indicator.url.domain})
                        if !cached_grok!("(?:%{IP:threat.indicator.ip}|%{GREEDYDATA:threat.indicator.url.domain})").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            if event.has_value("abusech.url.reporter") {
                event.rename("abusech.url.reporter", "threat.indicator.provider")?;
            }

            let _cond = { event.get("abusech.url.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "abusech.url.tags", |event| {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            event.remove("abusech.url.tags");

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { !event.has_value("abusech.url.deleted_at") };
            if _cond {
                // Painless script
                // Source: def dur = (ctx._conf?.ioc_expiration_duration != null && ctx._conf.ioc_expiration_duration instanceof String && ctx._conf.ioc_expiration_duration != '') ? ctx._conf.ioc_expiration_duration : '90d'; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_updated_at; if (ctx.threat.indicator.last_seen != null) {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_updated_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_updated_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} ctx.abusech.url.deleted_at = _tmp_deleted_at;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def dur = (ctx._conf?.ioc_expiration_duration != null && ctx._conf.ioc_expiration_duration instanceof String && ctx._conf.ioc_expiration_duration != '') ? ctx._conf.ioc_expiration_duration : '90d'; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_updated_at; if (ctx.threat.indicator.last_seen != null) {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_updated_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_updated_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} ctx.abusech.url.deleted_at = _tmp_deleted_at;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("abusech.url.deleted_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.url.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("abusech.url.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.url.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("_conf.ioc_expiration_duration") {
                event.rename(
                    "_conf.ioc_expiration_duration",
                    "abusech.url.ioc_expiration_duration",
                )?;
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            if event.has_value("abusech.url.larted") {
                if let Some(val) = event.get("abusech.url.larted") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "abusech.url.larted".into(),
                            message,
                        }
                    })?;
                    event.set("abusech.url.larted", converted)?;
                }
            }

            let _cond = { event.has_value("abusech") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            event.remove("abusech.url.date_added");
            event.remove("abusech.url.dateadded");
            event.remove("abusech.url.url");
            event.remove("abusech.url.host");
            event.remove("message");
            event.remove("_conf");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} {}in pipeline {} failed with message {}",
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
                                "with tag {} ",
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
