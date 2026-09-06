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

            event.set("ecs.version", json!("8.17.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("_collection_id") {
                event.rename("_collection_id", "ti_socradar_feeds.feed.collection_id")?;
            }

            if event.has_value("_collection_name") {
                event.rename("_collection_name", "ti_socradar_feeds.feed.collection_name")?;
            }

            let _cond = { !event.has_value("ti_socradar_feeds.feed.collection_id") };
            if _cond {
                if event.has_value("json._collection_metadata.collection_id") {
                    event.rename(
                        "json._collection_metadata.collection_id",
                        "ti_socradar_feeds.feed.collection_id",
                    )?;
                }
            }

            let _cond = { !event.has_value("ti_socradar_feeds.feed.collection_name") };
            if _cond {
                if event.has_value("json._collection_metadata.collection_name") {
                    event.rename(
                        "json._collection_metadata.collection_name",
                        "ti_socradar_feeds.feed.collection_name",
                    )?;
                }
            }

            event.remove("json._collection_metadata");

            if let Some(v) = event
                .get("json.feed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ti_socradar_feeds.feed.value", v)?;
            }

            if let Some(v) = event
                .get("json.feed_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ti_socradar_feeds.feed.type", v)?;
            }

            if let Some(v) = event
                .get("json.maintainer_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ti_socradar_feeds.feed.maintainer_name", v)?;
            }

            let _cond = { event.has_value("json.extra_info") };
            if _cond {
                if let Some(v) = event.get("json.extra_info").cloned() {
                    event.set("ti_socradar_feeds.feed.extra_info", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.first_seen_date") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss", "yyyy-MM-dd HH:mm:ss"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("ti_socradar_feeds.feed.first_seen_date", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.first_seen_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.latest_seen_date") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss", "yyyy-MM-dd HH:mm:ss"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("ti_socradar_feeds.feed.latest_seen_date", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.latest_seen_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.set("event.kind", json!("enrichment"))?;

            event.append_unique("event.category", json!("threat"))?;

            event.append_unique("event.type", json!("indicator"))?;

            event.set("threat.indicator.provider", json!("SOCRadar"))?;

            event.set("threat.feed.name", json!("SOCRadar Threat Feeds"))?;

            event.set(
                "threat.feed.reference",
                json!("https://platform.socradar.com"),
            )?;

            // Painless script
            // Source: double score = -1;\nif (ctx.ti_socradar_feeds?.feed?.extra_info?.score != null) {\n  score = ((Number) ctx.ti_socradar_feeds.feed.extra_info.score).doubleValue();\n}\nif (score < 0) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (score == 0) {\n  ctx.threat.indicator.confidence = 'None';\n} else if (score <= 25) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (score <= 50) {\n  ctx.threat.indicator.confidence = 'Medium';\n} else {\n  ctx.threat.indicator.confidence = 'High';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"double score = -1;\nif (ctx.ti_socradar_feeds?.feed?.extra_info?.score != null) {\n  score = ((Number) ctx.ti_socradar_feeds.feed.extra_info.score).doubleValue();\n}\nif (score < 0) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (score == 0) {\n  ctx.threat.indicator.confidence = 'None';\n} else if (score <= 25) {\n  ctx.threat.indicator.confidence = 'Low';\n} else if (score <= 50) {\n  ctx.threat.indicator.confidence = 'Medium';\n} else {\n  ctx.threat.indicator.confidence = 'High';\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.latest_seen_date") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss", "yyyy-MM-dd HH:mm:ss"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.latest_seen_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.first_seen_date") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss", "yyyy-MM-dd HH:mm:ss"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.first_seen_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event.get("@timestamp").cloned() {
                if !event.has("threat.indicator.last_seen") {
                    event.set("threat.indicator.last_seen", v)?;
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                if !event.has("threat.indicator.modified_at") {
                    event.set("threat.indicator.modified_at", v)?;
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                if !event.has("event.created") {
                    event.set("event.created", v)?;
                }
            }

            let _cond = {
                event.has_value("ti_socradar_feeds.feed.value")
                    && event.has_value("ti_socradar_feeds.feed.type")
            };
            if _cond {
                // Painless script
                // Source: def feedType = ctx.ti_socradar_feeds.feed.type.toLowerCase();\ndef value = ctx.ti_socradar_feeds.feed.value;\n\nif (ctx.threat == null) ctx.threat = new HashMap();\nif (ctx.threat.indicator == null) ctx.threat.indicator = new HashMap();\n\nif (feedType == 'ipv6') {\n  ctx.threat.indicator.type = 'ipv6-addr';\n  ctx.threat.indicator.ip = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.ip == null) ctx.related.ip = new ArrayList();\n  if (!ctx.related.ip.contains(value)) ctx.related.ip.add(value);\n} else if (feedType == 'ip' || feedType == 'ipv4-addr' || feedType == 'ipv4') {\n  ctx.threat.indicator.type = 'ipv4-addr';\n  ctx.threat.indicator.ip = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.ip == null) ctx.related.ip = new ArrayList();\n  if (!ctx.related.ip.contains(value)) ctx.related.ip.add(value);\n} else if (feedType == 'domain' || feedType == 'hostname' || feedType == 'domain-name') {\n  ctx.threat.indicator.type = 'domain-name';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.domain = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hosts == null) ctx.related.hosts = new ArrayList();\n  if (!ctx.related.hosts.contains(value)) ctx.related.hosts.add(value);\n} else if (feedType == 'hash' || feedType == 'file') {\n  ctx.threat.indicator.type = 'file';\n  if (ctx.threat.indicator.file == null) ctx.threat.indicator.file = new HashMap();\n  if (ctx.threat.indicator.file.hash == null) ctx.threat.indicator.file.hash = new HashMap();\n  if (value.length() == 32) {\n    ctx.threat.indicator.file.hash.md5 = value;\n  } else if (value.length() == 40) {\n    ctx.threat.indicator.file.hash.sha1 = value;\n  } else if (value.length() == 64) {\n    ctx.threat.indicator.file.hash.sha256 = value;\n  } else if (value.length() == 128) {\n    ctx.threat.indicator.file.hash.sha512 = value;\n  }\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hash == null) ctx.related.hash = new ArrayList();\n  if (!ctx.related.hash.contains(value)) ctx.related.hash.add(value);\n} else if (feedType == 'url') {\n  ctx.threat.indicator.type = 'url';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.full = value;\n  ctx.threat.indicator.url.original = value;\n} else if (feedType == 'email' || feedType == 'email-addr') {\n  ctx.threat.indicator.type = 'email-addr';\n  if (ctx.threat.indicator.email == null) ctx.threat.indicator.email = new HashMap();\n  ctx.threat.indicator.email.address = value;\n} else {\n  ctx.threat.indicator.type = 'domain-name';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.domain = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hosts == null) ctx.related.hosts = new ArrayList();\n  if (!ctx.related.hosts.contains(value)) ctx.related.hosts.add(value);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def feedType = ctx.ti_socradar_feeds.feed.type.toLowerCase();\ndef value = ctx.ti_socradar_feeds.feed.value;\n\nif (ctx.threat == null) ctx.threat = new HashMap();\nif (ctx.threat.indicator == null) ctx.threat.indicator = new HashMap();\n\nif (feedType == 'ipv6') {\n  ctx.threat.indicator.type = 'ipv6-addr';\n  ctx.threat.indicator.ip = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.ip == null) ctx.related.ip = new ArrayList();\n  if (!ctx.related.ip.contains(value)) ctx.related.ip.add(value);\n} else if (feedType == 'ip' || feedType == 'ipv4-addr' || feedType == 'ipv4') {\n  ctx.threat.indicator.type = 'ipv4-addr';\n  ctx.threat.indicator.ip = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.ip == null) ctx.related.ip = new ArrayList();\n  if (!ctx.related.ip.contains(value)) ctx.related.ip.add(value);\n} else if (feedType == 'domain' || feedType == 'hostname' || feedType == 'domain-name') {\n  ctx.threat.indicator.type = 'domain-name';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.domain = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hosts == null) ctx.related.hosts = new ArrayList();\n  if (!ctx.related.hosts.contains(value)) ctx.related.hosts.add(value);\n} else if (feedType == 'hash' || feedType == 'file') {\n  ctx.threat.indicator.type = 'file';\n  if (ctx.threat.indicator.file == null) ctx.threat.indicator.file = new HashMap();\n  if (ctx.threat.indicator.file.hash == null) ctx.threat.indicator.file.hash = new HashMap();\n  if (value.length() == 32) {\n    ctx.threat.indicator.file.hash.md5 = value;\n  } else if (value.length() == 40) {\n    ctx.threat.indicator.file.hash.sha1 = value;\n  } else if (value.length() == 64) {\n    ctx.threat.indicator.file.hash.sha256 = value;\n  } else if (value.length() == 128) {\n    ctx.threat.indicator.file.hash.sha512 = value;\n  }\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hash == null) ctx.related.hash = new ArrayList();\n  if (!ctx.related.hash.contains(value)) ctx.related.hash.add(value);\n} else if (feedType == 'url') {\n  ctx.threat.indicator.type = 'url';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.full = value;\n  ctx.threat.indicator.url.original = value;\n} else if (feedType == 'email' || feedType == 'email-addr') {\n  ctx.threat.indicator.type = 'email-addr';\n  if (ctx.threat.indicator.email == null) ctx.threat.indicator.email = new HashMap();\n  ctx.threat.indicator.email.address = value;\n} else {\n  ctx.threat.indicator.type = 'domain-name';\n  if (ctx.threat.indicator.url == null) ctx.threat.indicator.url = new HashMap();\n  ctx.threat.indicator.url.domain = value;\n  if (ctx.related == null) ctx.related = new HashMap();\n  if (ctx.related.hosts == null) ctx.related.hosts = new ArrayList();\n  if (!ctx.related.hosts.contains(value)) ctx.related.hosts.add(value);\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("ti_socradar_feeds.feed.collection_name") };
            if _cond {
                event.set(
                    "threat.indicator.description",
                    json!(format!(
                        "SOCRadar feed: {}",
                        event
                            .get("ti_socradar_feeds.feed.collection_name")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // Painless script
                // Source: ZonedDateTime _now = ZonedDateTime.ofInstant(Instant.ofEpochMilli(new Date().getTime()), ZoneId.of('Z'));\nString dur = ctx._conf.ioc_expiration_duration.trim();\nlong amount = Long.parseLong(dur.substring(0, dur.length() - 1));\nString unit = dur.substring(dur.length() - 1);\nZonedDateTime expiry;\nif (unit == 'd') {\n  expiry = _now.plusDays(amount);\n} else if (unit == 'h') {\n  expiry = _now.plusHours(amount);\n} else if (unit == 'm') {\n  expiry = _now.plusMinutes(amount);\n} else {\n  expiry = _now.plusDays(90);\n}\nif (ctx.ti_socradar_feeds == null) ctx.ti_socradar_feeds = new HashMap();\nif (ctx.ti_socradar_feeds.feed == null) ctx.ti_socradar_feeds.feed = new HashMap();\nctx.ti_socradar_feeds.feed.ioc_expiration_date = expiry;\nctx.ti_socradar_feeds.feed.ioc_expiration_duration = ctx._conf.ioc_expiration_duration;\nctx.ti_socradar_feeds.feed.ioc_expiration_reason = 'Expiration set by configuration';\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ZonedDateTime _now = ZonedDateTime.ofInstant(Instant.ofEpochMilli(new Date().getTime()), ZoneId.of('Z'));\nString dur = ctx._conf.ioc_expiration_duration.trim();\nlong amount = Long.parseLong(dur.substring(0, dur.length() - 1));\nString unit = dur.substring(dur.length() - 1);\nZonedDateTime expiry;\nif (unit == 'd') {\n  expiry = _now.plusDays(amount);\n} else if (unit == 'h') {\n  expiry = _now.plusHours(amount);\n} else if (unit == 'm') {\n  expiry = _now.plusMinutes(amount);\n} else {\n  expiry = _now.plusDays(90);\n}\nif (ctx.ti_socradar_feeds == null) ctx.ti_socradar_feeds = new HashMap();\nif (ctx.ti_socradar_feeds.feed == null) ctx.ti_socradar_feeds.feed = new HashMap();\nctx.ti_socradar_feeds.feed.ioc_expiration_date = expiry;\nctx.ti_socradar_feeds.feed.ioc_expiration_duration = ctx._conf.ioc_expiration_duration;\nctx.ti_socradar_feeds.feed.ioc_expiration_reason = 'Expiration set by configuration';\n"#
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("ti_socradar_feeds.feed.collection_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ti_socradar_feeds.feed.type") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ti_socradar_feeds.feed.value") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "_id",
                        json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                            TransformError::ParseError {
                                path: "_id".into(),
                                message,
                            }
                        })?),
                    )?;
                }
            }

            event.remove("json");
            event.remove("_conf");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null) {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> dropEmptyFields(v));\n    return ((Map) object).isEmpty();\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> dropEmptyFields(v));\n    return ((List) object).isEmpty();\n  } else if (object instanceof String) {\n    return ((String) object).isEmpty();\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
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
                        "Processor '{}' {}failed with message '{}'",
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
