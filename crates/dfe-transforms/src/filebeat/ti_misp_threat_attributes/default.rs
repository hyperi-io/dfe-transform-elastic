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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

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

            parse_json_field(event, "event.original", "misp.attribute")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("misp.attribute.Event.uuid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("misp.attribute.first_seen") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("misp.attribute.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("misp.attribute.uuid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("threat.indicator.provider", json!("misp"))?;

            let _cond = { event.has_value("misp.attribute.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.attribute.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.attribute.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("misp.attribute.Event") {
                event.rename("misp.attribute.Event", "misp.event")?;
            }

            if event.has_value("misp.attribute.Tag") {
                event.rename("misp.attribute.Tag", "misp.tag")?;
            }

            if event.has_value("misp.attribute.Object") {
                event.rename("misp.attribute.Object", "misp.object")?;
            }

            if event.has_value("misp.object.Attribute") {
                event.rename("misp.object.Attribute", "misp.object.attribute")?;
            }

            if event.has_value("misp.object.meta-category") {
                event.rename("misp.object.meta-category", "misp.object.meta_category")?;
            }

            event.set(
                "_tmp.event_ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.event_ingested") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("_tmp.event_ingested", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.event_ingested".into(),
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
                    "date-misp_event_ingested",
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
                            .get("_ingest.pipeline")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.misp?.attribute?.first_seen != null && ctx.misp.attribute.first_seen.length() == 16) {\n    ctx.misp.attribute.first_seen = ctx.misp.attribute.first_seen.substring(0, ctx.misp.attribute.first_seen.length() - 3)\n}\nif (ctx.misp?.attribute?.last_seen != null && ctx.misp.attribute.last_seen.length() == 16) {\n    ctx.misp.attribute.last_seen = ctx.misp.attribute.last_seen.substring(0, ctx.misp.attribute.last_seen.length() - 3)\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.misp?.attribute?.first_seen != null && ctx.misp.attribute.first_seen.length() == 16) {\n    ctx.misp.attribute.first_seen = ctx.misp.attribute.first_seen.substring(0, ctx.misp.attribute.first_seen.length() - 3)\n}\nif (ctx.misp?.attribute?.last_seen != null && ctx.misp.attribute.last_seen.length() == 16) {\n    ctx.misp.attribute.last_seen = ctx.misp.attribute.last_seen.substring(0, ctx.misp.attribute.last_seen.length() - 3)\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("misp.attribute.first_seen") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.attribute.first_seen") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set("misp.attribute.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.attribute.first_seen".into(),
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
                        "date_attribute_first_seen",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("misp.attribute.last_seen") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.attribute.last_seen") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                            Some(parsed) => event.set("misp.attribute.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.attribute.last_seen".into(),
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
                        "date_attribute_last_seen",
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.get("misp.attribute.decay_score").is_some_and(|v| v.is_array()) && event.get("misp.attribute.decay_score").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def decayed_count = \n    ctx.misp.attribute.decay_score.stream()\n    .filter(t -> t.decayed == true)\n    .count();\ndef decayed = ((float) decayed_count >= (ctx.misp.attribute.decay_score.size())/2.0);\nctx.misp.attribute.decayed = decayed;\nif (decayed == true) {\n    ctx.misp.attribute.decayed_at = ZonedDateTime.parse(ctx._tmp.event_ingested).minusMinutes(2L);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def decayed_count = \n    ctx.misp.attribute.decay_score.stream()\n    .filter(t -> t.decayed == true)\n    .count();\ndef decayed = ((float) decayed_count >= (ctx.misp.attribute.decay_score.size())/2.0);\nctx.misp.attribute.decayed = decayed;\nif (decayed == true) {\n    ctx.misp.attribute.decayed_at = ZonedDateTime.parse(ctx._tmp.event_ingested).minusMinutes(2L);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script-misp_decayed")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                !event.has_value("misp.attribute.decayed_at")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dur = ctx._conf.ioc_expiration_duration; def ts = ctx.misp.attribute.timestamp; long tsMillis = ts instanceof Number ? ts.longValue() : Long.parseLong(ts); ZonedDateTime _tmp_decayed_at; ZonedDateTime _tmp_timestamp = ZonedDateTime.ofInstant(Instant.ofEpochMilli(tsMillis * 1000L), ZoneId.of('Z')); ZonedDateTime _tmp_max_time = _tmp_timestamp; if (ctx.misp.attribute.last_seen != null) {\n    ZonedDateTime _tmp_last_seen = ZonedDateTime.parse(ctx.misp.attribute.last_seen);\n    if (_tmp_max_time.isBefore(_tmp_last_seen)) {\n        _tmp_max_time = _tmp_last_seen;\n    }\n} if (dur instanceof String){\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0){\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_decayed_at = _tmp_max_time.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_decayed_at = _tmp_max_time.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_decayed_at = _tmp_max_time.plusMinutes(Long.parseLong(time_value));\n  } else {\n    _tmp_decayed_at = _tmp_max_time.plusDays(90L);\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n  }\n  ctx.misp.attribute.decayed_at = _tmp_decayed_at;\n} ctx.misp.attribute.decayed = _tmp_decayed_at.isBefore(ZonedDateTime.parse(ctx._tmp.event_ingested))? true : false;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def dur = ctx._conf.ioc_expiration_duration; def ts = ctx.misp.attribute.timestamp; long tsMillis = ts instanceof Number ? ts.longValue() : Long.parseLong(ts); ZonedDateTime _tmp_decayed_at; ZonedDateTime _tmp_timestamp = ZonedDateTime.ofInstant(Instant.ofEpochMilli(tsMillis * 1000L), ZoneId.of('Z')); ZonedDateTime _tmp_max_time = _tmp_timestamp; if (ctx.misp.attribute.last_seen != null) {\n    ZonedDateTime _tmp_last_seen = ZonedDateTime.parse(ctx.misp.attribute.last_seen);\n    if (_tmp_max_time.isBefore(_tmp_last_seen)) {\n        _tmp_max_time = _tmp_last_seen;\n    }\n} if (dur instanceof String){\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0){\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_decayed_at = _tmp_max_time.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_decayed_at = _tmp_max_time.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_decayed_at = _tmp_max_time.plusMinutes(Long.parseLong(time_value));\n  } else {\n    _tmp_decayed_at = _tmp_max_time.plusDays(90L);\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n  }\n  ctx.misp.attribute.decayed_at = _tmp_decayed_at;\n} ctx.misp.attribute.decayed = _tmp_decayed_at.isBefore(ZonedDateTime.parse(ctx._tmp.event_ingested))? true : false;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-default-decayed_at",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("misp.event.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.event.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.event.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.event.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("misp.event.publish_timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.event.publish_timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.event.publish_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.event.publish_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("misp.event.sighting_timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.event.sighting_timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.event.sighting_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.event.sighting_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("misp.object.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.object.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.object.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.object.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("threat.feed.name", json!("MISP"))?;

            if let Some(v) = event
                .get("misp.attribute.first_seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.first_seen", v)?;
            }

            if let Some(v) = event
                .get("misp.attribute.last_seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.last_seen", v)?;
            }

            if event.has_value("misp.event.analysis") {
                if let Some(val) = event.get("misp.event.analysis") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.analysis".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.scanner_stats", converted)?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ([
                        "md5",
                        "impfuzzy",
                        "imphash",
                        "pehash",
                        "sha1",
                        "sha224",
                        "sha256",
                        "sha3-224",
                        "sha3-256",
                        "sha3-384",
                        "sha3-512",
                        "sha384",
                        "sha512",
                        "sha512/224",
                        "sha512/256",
                        "ssdeep",
                        "tlsh",
                        "vhash",
                    ]
                    .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
                        || event
                            .get_str("misp.attribute.type")
                            .is_some_and(|s| s.starts_with("filename")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("file")
                    && event.has_value("misp.attribute.type")
                    && !(event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename")))
            };
            if _cond {
                if let Some(from) = resolve_path(event, "misp.attribute.value")
                    && let Some(to) = resolve_path(
                        event,
                        "threat.indicator.file.hash.{{{misp.attribute.type}}}",
                    )
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("file")
                    && event.get_str("misp.attribute.type") == Some("filename")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.file.name")?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
            };
            if _cond {
                if event.has_value("misp.attribute.type") {
                    if let Some(input) = event.get_string("misp.attribute.type") {
                        // Grok pattern: %{WORD}\\|%{WORD:_tmp.hashtype}
                        if !cached_grok!("%{WORD}\\|%{WORD:_tmp.hashtype}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.file.name}\\|%{GREEDYDATA:_tmp.hashvalue}
                        if !cached_grok!(
                            "%{DATA:threat.indicator.file.name}\\|%{GREEDYDATA:_tmp.hashvalue}"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
                    && event.has_value("_tmp.hashvalue")
                    && event.has_value("_tmp.hashtype")
            };
            if _cond {
                set_templated(
                    event,
                    "threat.indicator.file.hash.{{{_tmp.hashtype}}}",
                    json!(
                        event
                            .get("_tmp.hashvalue")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["url", "link", "uri"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(
                        event,
                        "misp.attribute.value",
                        "threat.indicator.url",
                        true,
                        true,
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("url")
                    && event.has_value("threat.indicator.url.original")
            };
            if _cond {
                let v = json!(
                    event
                        .get("threat.indicator.url.original")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("url")
                    && !event.has_value("threat.indicator.url.original")
                    && event.has_value("misp.attribute.value")
            };
            if _cond {
                let v = json!(
                    event
                        .get("misp.attribute.value")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.original", v)?;
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("url")
                    && !event.has_value("threat.indicator.url.full")
                    && event.has_value("misp.attribute.value")
            };
            if _cond {
                let v = json!(
                    event
                        .get("misp.attribute.value")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("regkey"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("windows-registry-key"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("windows-registry-key")
                    && event.get_str("misp.attribute.type") == Some("regkey")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.registry.key")?;
                }
            }

            let _cond = { event.get_str("misp.attribute.type") == Some("regkey|value") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.registry.key}\\|%{DATA:threat.indicator.registry.value}
                        if !cached_grok!("%{DATA:threat.indicator.registry.key}\\|%{DATA:threat.indicator.registry.value}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("misp.attribute.type") == Some("AS")
            };
            if _cond {
                event.set("threat.indicator.type", json!("autonomous-system"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("autonomous-system") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(val) = event.get("misp.attribute.value") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "misp.attribute.value".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.as.number", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && (event.get_str("misp.attribute.type") == Some("hostname")
                        || event
                            .get_str("misp.attribute.type")
                            .is_some_and(|s| s.starts_with("domain")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["ip-src", "ip-src|port", "ip-dst", "ip-dst|port"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("threat.indicator.type") == Some("domain-name")
                    && event.get_str("misp.attribute.type") != Some("domain|ip")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.url.domain")?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("threat.indicator.type") == Some("ipv4-addr")
                    && !(["domain|ip", "ip-src|port", "ip-dst|port"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or("")))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.ip")?;
                }
            }

            let _cond = {
                event.get_str("misp.attribute.type") == Some("domain|ip")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.url.domain}\\|%{IP:threat.indicator.ip}
                        if !cached_grok!(
                            "%{DATA:threat.indicator.url.domain}\\|%{IP:threat.indicator.ip}"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                ["ip-src|port", "ip-dst|port"]
                    .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{IP:threat.indicator.ip}\\|%{NUMBER:threat.indicator.port}
                        if !cached_grok!(
                            "%{IP:threat.indicator.ip}\\|%{NUMBER:threat.indicator.port}"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("email"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = {
                event.get_str("misp.attribute.type") == Some("email-dst")
                    || event.get_str("misp.attribute.type") == Some("email-src")
            };
            if _cond {
                event.set(
                    "threat.indicator.email.address",
                    json!(
                        event
                            .get("misp.attribute.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("misp.attribute.value")
                    && event.get_str("misp.attribute.type") == Some("email-subject")
            };
            if _cond {
                event.set(
                    "threat.indicator.email.subject",
                    json!(
                        event
                            .get("misp.attribute.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("misp.event.event_creator_email") {
                event.rename("misp.event.event_creator_email", "user.email")?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append("user.roles", json!("reporting_user"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["mac-address", "mac-eui-64"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("mac-addr"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("mac-addr") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.mac")?;
                }
            }

            let _cond = { event.get_str("misp.attribute.type") == Some("mime-type") };
            if _cond {
                event.set("threat.indicator.type", json!("artifact"))?;
            }

            let _cond = { event.get_str("misp.attribute.type") == Some("mutex") };
            if _cond {
                event.set("threat.indicator.type", json!("mutex"))?;
            }

            let _cond = { event.get_str("misp.attribute.type") == Some("cpe") };
            if _cond {
                event.set("threat.indicator.type", json!("software"))?;
            }

            let _cond = { event.has_value("misp.tag") };
            if _cond {
                // Painless script
                // Source: def tags = ctx.misp.tag.stream()\n   .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   .filter(t -> t.startsWith('tlp:'))\n   .map(t -> t.replace('tlp:', '').toUpperCase())\n   .collect(Collectors.toList());\n\nctx.temp_tags = tags;\nctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def tags = ctx.misp.tag.stream()\n   .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   .filter(t -> t.startsWith('tlp:'))\n   .map(t -> t.replace('tlp:', '').toUpperCase())\n   .collect(Collectors.toList());\n\nctx.temp_tags = tags;\nctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("temp_tags") };
            if _cond {
                foreach_array(event, "temp_tags", |event| {
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

            if event.has_value("misp.event.org_id") {
                event.rename("misp.event.org_id", "organization.id")?;
            }

            if event.has_value("misp.attribute.distribution") {
                if let Some(val) = event.get("misp.attribute.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.attribute.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.attribute.distribution", converted)?;
                }
            }

            if event.has_value("misp.object.distribution") {
                if let Some(val) = event.get("misp.object.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.object.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.object.distribution", converted)?;
                }
            }

            if event.has_value("misp.event.distribution") {
                if let Some(val) = event.get("misp.event.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.distribution", converted)?;
                }
            }

            if event.has_value("threat.indicator.port") {
                if let Some(val) = event.get("threat.indicator.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "threat.indicator.port".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.port", converted)?;
                }
            }

            if event.has_value("misp.event.attribute_count") {
                if let Some(val) = event.get("misp.event.attribute_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.attribute_count".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.attribute_count", converted)?;
                }
            }

            if event.has_value("misp.event.threat_level_id") {
                if let Some(val) = event.get("misp.event.threat_level_id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.threat_level_id".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.threat_level_id", converted)?;
                }
            }

            let _cond = { event.has_value("misp.object.attribute") };
            if _cond {
                foreach_array(event, "misp.object.attribute", |event| {
                    let _cond = { false };
                    if _cond {
                        event.append(
                            "debug_timestamp",
                            json!(
                                event
                                    .get("_ingest._value.timestamp")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("misp.object.attribute") };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("misp.object.attribute").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.timestamp")
                                {
                                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.timestamp", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.timestamp".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "misp.object.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("misp") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                event.remove("misp.attribute.value");
            }

            event.remove("temp_tags");
            event.remove("misp.attribute.timestamp");
            event.remove("misp.tag");
            event.remove("misp.event.analysis");
            event.remove("_tmp");
            event.remove("json");
            event.remove("_conf");

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
