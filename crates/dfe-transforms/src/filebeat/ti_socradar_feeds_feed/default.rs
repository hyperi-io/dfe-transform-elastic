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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "c32cc74e45d7cce966d2fe6c3d78a2acecd635eb50841f13681f65b59cfb1021",
                    r#"{"params":null,"known":[{"GuardedCopy":{"statements":[{"If":{"test":[[{"Compare":{"path":"ti_socradar_feeds.feed.extra_info.score","wanted":"Null","negated":true,"lowered":false}}]],"then":[],"alt":[]}},{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"threat.indicator.confidence","value":{"Literal":"Low"}}}}],"alt":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"threat.indicator.confidence","value":{"Literal":"None"}}}}],"alt":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"threat.indicator.confidence","value":{"Literal":"Low"}}}}],"alt":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"threat.indicator.confidence","value":{"Literal":"Medium"}}}}],"alt":[{"Literal":{"Set":{"path":"threat.indicator.confidence","value":{"Literal":"High"}}}}]}}]}}]}}]}}],"whole":false}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "c7a8e8cfcc7c6a1c44fe6edb44b5a0679acacb786a901a939fc730582fdd2685",
                        r#"{"params":null,"known":[{"EqualityLadder":{"subject":"ti_socradar_feeds.feed.type","fold_case":true,"test":"Equals","arms":[{"literals":["ipv6"],"writes":[["threat.indicator.type","ipv6-addr"]],"removes":[]},{"literals":["ip","ipv4-addr","ipv4"],"writes":[["threat.indicator.type","ipv4-addr"]],"removes":[]},{"literals":["domain","hostname","domain-name"],"writes":[["threat.indicator.type","domain-name"]],"removes":[]},{"literals":["hash","file"],"writes":[["threat.indicator.type","file"]],"removes":[]},{"literals":["url"],"writes":[["threat.indicator.type","url"]],"removes":[]},{"literals":["email","email-addr"],"writes":[["threat.indicator.type","email-addr"]],"removes":[]}]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "daba4304391af0936d260eedf4719c3e36b44c4493e57d7f412b777c0aaa30cd",
                        r#"{"params":null,"known":["JoinOptional"]}"#
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

            // The pipeline's script, emitted as the runtime's equivalent
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
