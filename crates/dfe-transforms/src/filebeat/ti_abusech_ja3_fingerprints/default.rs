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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = {
                event.has_value("_conf.interval") && event.get_str("_conf.interval") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_conf.interval").cloned() {
                    event.set("labels.interval", v)?;
                }
            }

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

            parse_json_field(event, "event.original", "abusech.ja3_fingerprints")?;

            event.set("threat.indicator.type", json!("software"))?;

            let _cond = { event.has_value("abusech.ja3_fingerprints.first_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.first_ts") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.first_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.first_seen") };
            if _cond {
                if event.remove("abusech.ja3_fingerprints.first_ts").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "abusech.ja3_fingerprints.first_ts".into(),
                    });
                }
            }

            let _cond = { event.has_value("abusech.ja3_fingerprints.last_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.last_ts") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.last_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if event.remove("abusech.ja3_fingerprints.last_ts").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "abusech.ja3_fingerprints.last_ts".into(),
                    });
                }
            }

            if let Some(v) = event.get("abusech.ja3_fingerprints.ja3").cloned() {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event.get("abusech.ja3_fingerprints.reason").cloned() {
                event.set("threat.indicator.description", v)?;
            }

            let _cond = { event.has_value("abusech.ja3_fingerprints.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("abusech.ja3_fingerprints.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event.get("_ingest.timestamp").cloned() {
                event.set("event.ingested", v)?;
            }

            // Painless script
            // Source: def dur = ctx.labels.interval; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_created_at = ctx.event.ingested; long max_ingest_time_in_sec = 30L; long transform_max_age_in_min = 1L; String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_created_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_created_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_created_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_created_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} // Subtract transform's retention max_age and ingest time so that indicators are expire time slightly before next interval. // This ensures the indicator is deleted when the next interval starts.\n_tmp_deleted_at = _tmp_deleted_at.minusMinutes(transform_max_age_in_min).minusSeconds(max_ingest_time_in_sec); ctx.abusech.ja3_fingerprints.deleted_at = _tmp_deleted_at;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def dur = ctx.labels.interval; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_created_at = ctx.event.ingested; long max_ingest_time_in_sec = 30L; long transform_max_age_in_min = 1L; String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_created_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_created_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_created_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_created_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} // Subtract transform's retention max_age and ingest time so that indicators are expire time slightly before next interval. // This ensures the indicator is deleted when the next interval starts.\n_tmp_deleted_at = _tmp_deleted_at.minusMinutes(transform_max_age_in_min).minusSeconds(max_ingest_time_in_sec); ctx.abusech.ja3_fingerprints.deleted_at = _tmp_deleted_at;\n"#
                ),
            )?;

            let _cond = { event.has_value("abusech.ja3_fingerprints.deleted_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("abusech.ja3_fingerprints.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
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

            event.remove("abusech.ja3_fingerprints.ja3");
            event.remove("abusech.ja3_fingerprints.reason");
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
