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

            parse_json_field(event, "event.original", "abusech.threatfox")?;

            if event.has_value("abusech.threatfox.id") {
                event.rename("abusech.threatfox.id", "event.id")?;
            }

            let _cond = { event.has_value("abusech.threatfox.first_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.threatfox.first_seen") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss z",
                            "yyyy-MM-dd HH:mm:ss Z",
                            "yyyy-MM-dd HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.threatfox.first_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("abusech.threatfox.last_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.threatfox.last_seen") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss z",
                            "yyyy-MM-dd HH:mm:ss Z",
                            "yyyy-MM-dd HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.threatfox.last_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("abusech.threatfox.ioc").cloned() {
                event.set("threat.indicator.name", v)?;
            }

            if event.has_value("abusech.threatfox.ioc_type_desc") {
                event.rename(
                    "abusech.threatfox.ioc_type_desc",
                    "threat.indicator.description",
                )?;
            }

            if event.has_value("abusech.threatfox.reporter") {
                event.rename("abusech.threatfox.reporter", "threat.indicator.provider")?;
            }

            if event.has_value("abusech.threatfox.malware_alias") {
                if let Some(s) = event.get_string("abusech.threatfox.malware_alias") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("threat.software.alias", Value::Array(parts))?;
                }
            }

            if event.has_value("abusech.threatfox.reporter") {
                event.rename("abusech.threatfox.reporter", "threat.indicator.provider")?;
            }

            if event.has_value("abusech.threatfox.reference") {
                event.rename("abusech.threatfox.reference", "threat.indicator.reference")?;
            }

            if event.has_value("abusech.threatfox.malware_printable") {
                event.rename(
                    "abusech.threatfox.malware_printable",
                    "threat.software.name",
                )?;
            }

            if event.has_value("abusech.threatfox.malware_malpedia") {
                event.rename(
                    "abusech.threatfox.malware_malpedia",
                    "threat.software.reference",
                )?;
            }

            event.set("threat.indicator.marking.tlp", json!("WHITE"))?;

            let _cond = { event.has_value("abusech.threatfox.confidence_level") };
            if _cond {
                // Painless script
                // Source: def value = ctx.abusech.threatfox.confidence_level; def confidence = \"None\"; if (value > 0 && value < 30) {\n  confidence = \"Low\";\n} if (value >= 30.0 && value < 70) {\n  confidence = \"Medium\";\n} else if (value >= 70 && value <= 100) {\n  confidence = \"High\";\n} ctx.threat.indicator.put(\"confidence\", confidence)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def value = ctx.abusech.threatfox.confidence_level; def confidence = \"None\"; if (value > 0 && value < 30) {\n  confidence = \"Low\";\n} if (value >= 30.0 && value < 70) {\n  confidence = \"Medium\";\n} else if (value >= 70 && value <= 100) {\n  confidence = \"High\";\n} ctx.threat.indicator.put(\"confidence\", confidence)\n"#
                    ),
                )?;
            }

            if !event.has("threat.indicator.confidence") {
                event.set("threat.indicator.confidence", json!("Not Specified"))?;
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && ["url"].contains(&event.get_str("abusech.threatfox.ioc_type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("abusech.threatfox.ioc_type") == Some("url") };
            if _cond {
                uri_parts(
                    event,
                    "abusech.threatfox.ioc",
                    "threat.indicator.url",
                    true,
                    true,
                )?;
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && ["domain"]
                        .contains(&event.get_str("abusech.threatfox.ioc_type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && ["domain"]
                        .contains(&event.get_str("abusech.threatfox.ioc_type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("abusech.threatfox.ioc") {
                    event.rename("abusech.threatfox.ioc", "threat.indicator.url.domain")?;
                }
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && ["ip:port"]
                        .contains(&event.get_str("abusech.threatfox.ioc_type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("abusech.threatfox.ioc_type") == Some("ip:port") };
            if _cond {
                if event.has_value("abusech.threatfox.ioc") {
                    if let Some(input) = event.get_string("abusech.threatfox.ioc") {
                        // Grok pattern: %{IP:threat.indicator.ip}:%{NUMBER:threat.indicator.port:long}
                        if !cached_grok!(
                            "%{IP:threat.indicator.ip}:%{NUMBER:threat.indicator.port:long}"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && event.get("threat.indicator.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && ["md5_hash", "sha1_hash", "sha256_hash"]
                        .contains(&event.get_str("abusech.threatfox.ioc_type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && event
                        .get_str("abusech.threatfox.ioc_type")
                        .is_some_and(|s| s.ends_with("_hash"))
            };
            if _cond {
                if event.has_value("abusech.threatfox.ioc_type") {
                    if let Some(input) = event.get_string("abusech.threatfox.ioc_type") {
                        // Grok pattern: %{DATA:_tmp.hashtype}_hash
                        if !cached_grok!("%{DATA:_tmp.hashtype}_hash")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("abusech.threatfox.ioc_type")
                    && event
                        .get_str("abusech.threatfox.ioc_type")
                        .is_some_and(|s| s.ends_with("_hash"))
                    && event.has_value("_tmp.hashtype")
            };
            if _cond {
                if let Some(from) = resolve_path(event, "abusech.threatfox.ioc")
                    && let Some(to) =
                        resolve_path(event, "threat.indicator.file.hash.{{{_tmp.hashtype}}}")
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                event
                    .get("abusech.threatfox.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "abusech.threatfox.tags", |event| {
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

            event.remove("abusech.threatfox.tags");

            let _cond = { !event.has_value("abusech.threatfox.deleted_at") };
            if _cond {
                // Painless script
                // Source: def dur = (ctx._conf?.ioc_expiration_duration != null && ctx._conf.ioc_expiration_duration instanceof String && ctx._conf.ioc_expiration_duration != '') ? ctx._conf.ioc_expiration_duration : '90d'; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_updated_at; if (ctx.threat.indicator.last_seen != null) {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_updated_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_updated_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} ctx.abusech.threatfox.deleted_at = _tmp_deleted_at;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def dur = (ctx._conf?.ioc_expiration_duration != null && ctx._conf.ioc_expiration_duration instanceof String && ctx._conf.ioc_expiration_duration != '') ? ctx._conf.ioc_expiration_duration : '90d'; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_updated_at; if (ctx.threat.indicator.last_seen != null) {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} String time_unit = dur.substring(dur.length() -  1, dur.length()); String time_value = dur.substring(0, dur.length() - 1); if (time_unit == 'd') {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(Long.parseLong(time_value));\n} else if (time_unit == 'h') {\n  _tmp_deleted_at = _tmp_updated_at.plusHours(Long.parseLong(time_value));\n} else if (time_unit == 'm') {\n  _tmp_deleted_at = _tmp_updated_at.plusMinutes(Long.parseLong(time_value));\n} else {\n  _tmp_deleted_at = _tmp_updated_at.plusDays(90L);\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n} ctx.abusech.threatfox.deleted_at = _tmp_deleted_at;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("abusech.threatfox.deleted_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.threatfox.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("abusech.threatfox.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.threatfox.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("_conf.ioc_expiration_duration") {
                event.rename(
                    "_conf.ioc_expiration_duration",
                    "abusech.threatfox.ioc_expiration_duration",
                )?;
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            let _cond = { event.has_value("abusech") };
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

            event.remove("abusech.threatfox.first_seen");
            event.remove("abusech.threatfox.last_seen");
            event.remove("threatintel_indicator_confidence");
            event.remove("abusech.threatfox.malware_alias");
            event.remove("abusech.threatfox.ioc_type");
            event.remove("abusech.threatfox.ioc");
            event.remove("message");
            event.remove("_tmp");
            event.remove("_conf");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
