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

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            event.set("threat.indicator.provider", json!("OTX"))?;

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

            parse_json_field(event, "event.original", "otx")?;

            let _cond = {
                event.has_value("otx.created")
                    && !event.has_value("otx.expiration")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dur = ctx._conf.ioc_expiration_duration; String created = ctx.otx.created; ZonedDateTime _tmp_expiration; if (dur instanceof String) {\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0) {\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusMinutes(Long.parseLong(time_value));\n  } else {\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(90L);\n  }\n  ctx.otx.expiration = _tmp_expiration;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def dur = ctx._conf.ioc_expiration_duration; String created = ctx.otx.created; ZonedDateTime _tmp_expiration; if (dur instanceof String) {\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0) {\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusMinutes(Long.parseLong(time_value));\n  } else {\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n    _tmp_expiration = ZonedDateTime.parse(created, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(90L);\n  }\n  ctx.otx.expiration = _tmp_expiration;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-default-expiration",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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

            let _cond = { event.has_value("otx.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("otx.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("otx.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "otx.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("otx.created") };
            if _cond {
                if let Some(v) = event.get("otx.created").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("otx.expiration") };
            if _cond {
                if let Some(date_str) = event.get_as_string("otx.expiration") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("otx.expiration", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "otx.expiration".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("otx.id") };
            if _cond {
                if let Some(val) = event.get("otx.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.id".into(),
                            message,
                        }
                    })?;
                    event.set("otx.id", converted)?;
                }
            }

            let _cond = { event.has_value("otx.count") };
            if _cond {
                if let Some(val) = event.get("otx.count") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.count".into(),
                            message,
                        }
                    })?;
                    event.set("otx.count", converted)?;
                }
            }

            let _cond = { event.has_value("otx.t") };
            if _cond {
                if let Some(val) = event.get("otx.t") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.t".into(),
                            message,
                        }
                    })?;
                    event.set("otx.t", converted)?;
                }
            }

            let _cond = { event.has_value("otx.t2") };
            if _cond {
                if let Some(val) = event.get("otx.t2") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.t2".into(),
                            message,
                        }
                    })?;
                    event.set("otx.t2", converted)?;
                }
            }

            let _cond = { event.has_value("otx.t3") };
            if _cond {
                if let Some(val) = event.get("otx.t3") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.t3".into(),
                            message,
                        }
                    })?;
                    event.set("otx.t3", converted)?;
                }
            }

            let _cond = { event.has_value("otx.revision") };
            if _cond {
                if let Some(val) = event.get("otx.revision") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "otx.revision".into(),
                            message,
                        }
                    })?;
                    event.set("otx.revision", converted)?;
                }
            }

            parse_json_field(event, "otx.pulse_raw", "otx.pulse")?;

            let _cond = { event.has_value("otx.pulse.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("otx.pulse.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("otx.pulse.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "otx.pulse.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("otx.pulse.modified") };
            if _cond {
                if let Some(date_str) = event.get_as_string("otx.pulse.modified") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("otx.pulse.modified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "otx.pulse.modified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get_str("otx.type")
                    .is_some_and(|s| s.starts_with("FileHash"))
                    || event.get_str("otx.type") == Some("filepath")
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-MD5") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.md5")?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-SHA1") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.sha1")?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-SHA256") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.sha256")?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-PEHASH") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.pehash")?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-IMPHASH") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.imphash")?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("IPv4") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("otx.type") == Some("IPv6") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                event.has_value("threat.indicator.type")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.ip")?;
                }
            }

            let _cond = {
                !event.has_value("threat.indicator.type")
                    && ["URL", "URI"].contains(&event.get_str("otx.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                uri_parts(event, "otx.indicator", "threat.indicator.url", true, true)?;
            }

            let _cond = { event.get_str("otx.type") == Some("URL") };
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

            let _cond = { event.get_str("otx.type") == Some("email") };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.email.address")?;
                }
            }

            let _cond = {
                !event.has_value("threat.indicator.type")
                    && ["domain", "hostname"].contains(&event.get_str("otx.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("domain-name")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.url.domain")?;
                }
            }

            let _cond = {
                event.get("otx.pulse.tags").is_some_and(|v| v.is_array()) && event.get("otx.pulse.tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "otx.pulse.tags", |event| {
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

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            let _cond = { event.has_value("otx") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            let _cond = { event.get_str("otx.content") == Some("") };
            if _cond {
                event.remove("otx.content");
            }

            let _cond = { event.get_str("otx.description") == Some("") };
            if _cond {
                event.remove("otx.description");
            }

            let _cond = { event.get_str("otx.title") == Some("") };
            if _cond {
                event.remove("otx.title");
            }

            let _cond = { event.get_str("otx.pulse.adversary") == Some("") };
            if _cond {
                event.remove("otx.pulse.adversary");
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                event.remove("otx.type");
                event.remove("otx.pulse.tags");
                event.remove("otx.pulse_raw");
                event.remove("_conf");
            }

            let _cond = { !event.has_value("otx") };
            if _cond {
                event.remove("otx");
            }

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
            }
        }

        Ok(TransformResult::Continue)
    }
}
