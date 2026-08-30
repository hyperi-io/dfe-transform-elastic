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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^(?P<first_char>(?:.))
                let _ = cached_grok!("^(?P<first_char>(?:.))").extract_into(&input, event)?;
            }

            let _cond = { event.get_str("first_char") != Some("{") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-json"
                parse_json_field_to_root(event, "message", false)?;
                dot_expand(event, "", "*")?;
                let _cond = {
                    !([
                        "elasticsearch.slowlog",
                        "elasticsearch.index_indexing_slowlog",
                        "elasticsearch.index_search_slowlog",
                    ]
                    .contains(&event.get_str("event.dataset").unwrap_or("")))
                };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                if event.has_value("elasticsearch.slowlog.took_millis") {
                    if let Some(val) = event.get("elasticsearch.slowlog.took_millis") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "elasticsearch.slowlog.took_millis".into(),
                                message,
                            }
                        })?;
                        event.set("elasticsearch.slowlog.took_millis", converted)?;
                    }
                }
                if event.has_value("elasticsearch.slowlog.took_millis") {
                    event.rename(
                        "elasticsearch.slowlog.took_millis",
                        "elasticsearch.slowlog.duration",
                    )?;
                }
                let _cond = { !event.has_value("elasticsearch.slowlog.id") };
                if _cond {
                    if event.remove("elasticsearch.slowlog.id").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "elasticsearch.slowlog.id".into(),
                        });
                    }
                }
                if let Some(input) = event.get_string("elasticsearch.slowlog.message") {
                    // Grok pattern: (\\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\]\\[%{NUMBER:elasticsearch.shard.id}\\])?(%{SPACE})(\\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\/%{DATA:elasticsearch.index.id}\\])?(%{SPACE})%{SPACE}(took\\[%{DATA:elasticsearch.slowlog.took}\\],)?%{SPACE}(took_millis\\[%{NUMBER:elasticsearch.slowlog.duration:long}\\],)?%{SPACE}(type\\[%{DATA:elasticsearch.slowlog.type}\\],)?%{SPACE}(id\\[%{DATA:elasticsearch.slowlog.id}\\],)?%{SPACE}(routing\\[%{DATA:elasticsearch.slowlog.routing}\\],)?%{SPACE}(total_hits\\[%{NUMBER:elasticsearch.slowlog.total_hits:int}\\],)?%{SPACE}(types\\[%{DATA:elasticsearch.slowlog.types}\\],)?%{SPACE}(stats\\[%{DATA:elasticsearch.slowlog.stats}\\],)?%{SPACE}(search_type\\[%{DATA:elasticsearch.slowlog.search_type}\\],)?%{SPACE}(total_shards\\[%{NUMBER:elasticsearch.slowlog.total_shards:int}\\],)?%{SPACE}(source\\[(?P<elasticsearch_slowlog_source_query>(?:(.|\n)*))\\])?,?%{SPACE}(extra_source\\[%{DATA:elasticsearch.slowlog.extra_source}\\])?,?
                    // Grok pattern: \\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\]\\[%{NUMBER:elasticsearch.shard.id}\\]
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(\\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\]\\[%{NUMBER:elasticsearch.shard.id}\\])?(%{SPACE})(\\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\/%{DATA:elasticsearch.index.id}\\])?(%{SPACE})%{SPACE}(took\\[%{DATA:elasticsearch.slowlog.took}\\],)?%{SPACE}(took_millis\\[%{NUMBER:elasticsearch.slowlog.duration:long}\\],)?%{SPACE}(type\\[%{DATA:elasticsearch.slowlog.type}\\],)?%{SPACE}(id\\[%{DATA:elasticsearch.slowlog.id}\\],)?%{SPACE}(routing\\[%{DATA:elasticsearch.slowlog.routing}\\],)?%{SPACE}(total_hits\\[%{NUMBER:elasticsearch.slowlog.total_hits:int}\\],)?%{SPACE}(types\\[%{DATA:elasticsearch.slowlog.types}\\],)?%{SPACE}(stats\\[%{DATA:elasticsearch.slowlog.stats}\\],)?%{SPACE}(search_type\\[%{DATA:elasticsearch.slowlog.search_type}\\],)?%{SPACE}(total_shards\\[%{NUMBER:elasticsearch.slowlog.total_shards:int}\\],)?%{SPACE}(source\\[(?P<elasticsearch_slowlog_source_query>(?:(.|\n)*))\\])?,?%{SPACE}(extra_source\\[%{DATA:elasticsearch.slowlog.extra_source}\\])?,?",
                                [
                                    ("elasticsearch_index_name", "elasticsearch.index.name"),
                                    ("elasticsearch_index_name", "elasticsearch.index.name"),
                                    (
                                        "elasticsearch_slowlog_source_query",
                                        "elasticsearch.slowlog.source_query"
                                    )
                                ]
                            ),
                            cached_grok_mapped!(
                                "\\[(?P<elasticsearch_index_name>(?:[a-zA-Z0-9_.-]*))\\]\\[%{NUMBER:elasticsearch.shard.id}\\]",
                                [("elasticsearch_index_name", "elasticsearch.index.name")]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
                let v = json!(
                    event
                        .get("elasticsearch.slowlog.message")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("message", v)?;
                }
                if event.remove("elasticsearch.slowlog.message").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "elasticsearch.slowlog.message".into(),
                    });
                }
                event.set("service.type", json!("elasticsearch"))?;
                // End nested pipeline: "pipeline-json"
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.remove("elasticsearch.slowlog.timestamp");
            event.remove("elasticsearch.server.@timestamp");

            let _cond = { event.has_value("elasticsearch.slowlog.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.elasticsearch.slowlog.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx.elasticsearch.slowlog.duration * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000}"),
                )?;
            }

            event.remove("elasticsearch.slowlog.duration");

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            // Painless script
            // Source: def errorLevels = ['FATAL', 'ERROR']; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = ['error'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def errorLevels = ['FATAL', 'ERROR']; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = ['error'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}"#
                ),
            )?;

            let v = json!(
                event
                    .get("elasticsearch.node.name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.name", v)?;
            }

            let v = json!(
                event
                    .get("elasticsearch.node.id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.id", v)?;
            }

            if event.remove("first_char").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "first_char".into(),
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
