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
                if !cached_grok!("^(?P<first_char>(?:.))").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("first_char") != Some("{") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-json"
                if event.has_value("message") {
                    event.rename("message", "_ecs_json_message")?;
                }
                // SKIPPED: condition not transpiled: ctx.containsKey('_ecs_json_message')
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field_to_root(event, "_ecs_json_message", true)?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        if event.has_value("_ecs_json_message") {
                            event.rename("_ecs_json_message", "message")?;
                        }
                        if !event.has("error.message") {
                            event.set("error.message", json!("Error while parsing JSON"))?;
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                event.remove("_ecs_json_message");
                dot_expand(event, "", "*")?;
                // SKIPPED: condition not transpiled: ctx.error?.stack_trace instanceof Collection
                #[allow(unreachable_code, unused_variables)]
                if false {
                    let joined = event
                        .get("error.stack_trace")
                        .and_then(|v| join_values(v, "\n"));
                    if let Some(joined) = joined {
                        event.set("error.stack_trace", json!(joined))?;
                    }
                }
                let _cond = { event.get_str("event.dataset") != Some("elasticsearch.server") };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                event.set("service.type", json!("elasticsearch"))?;
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:\\[gc\\]\\[%{NUMBER:elasticsearch.server.gc.overhead_seq}\\] overhead, spent \\[%{NUMBER:elasticsearch.server.gc.collection_duration.time:float}%{DATA:elasticsearch.server.gc.collection_duration.unit}\\] collecting in the last \\[%{NUMBER:elasticsearch.server.gc.observation_duration.time:float}%{DATA:elasticsearch.server.gc.observation_duration.unit}\\])
                    // Grok pattern: (?:\\[gc\\]\\[young\\]\\[%{NUMBER:elasticsearch.server.gc.young.one}\\]\\[%{NUMBER:elasticsearch.server.gc.young.two}\\]%{SPACE}(?P<message>(?:(.|\n)*)))
                    // Grok pattern: ((\\[(?P<_parsed_index_name>(?:[a-zA-Z0-9_.-]*))\\]|\\[(?P<_parsed_index_name>(?:[a-zA-Z0-9_.-]*))\\/%{DATA:_parsed_index_id}\\]))?%{SPACE}(?:(.|\n)*)
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "(?:\\[gc\\]\\[%{NUMBER:elasticsearch.server.gc.overhead_seq}\\] overhead, spent \\[%{NUMBER:elasticsearch.server.gc.collection_duration.time:float}%{DATA:elasticsearch.server.gc.collection_duration.unit}\\] collecting in the last \\[%{NUMBER:elasticsearch.server.gc.observation_duration.time:float}%{DATA:elasticsearch.server.gc.observation_duration.unit}\\])"
                            ),
                            cached_grok!(
                                "(?:\\[gc\\]\\[young\\]\\[%{NUMBER:elasticsearch.server.gc.young.one}\\]\\[%{NUMBER:elasticsearch.server.gc.young.two}\\]%{SPACE}(?P<message>(?:(.|\n)*)))"
                            ),
                            cached_grok!(
                                "((\\[(?P<_parsed_index_name>(?:[a-zA-Z0-9_.-]*))\\]|\\[(?P<_parsed_index_name>(?:[a-zA-Z0-9_.-]*))\\/%{DATA:_parsed_index_id}\\]))?%{SPACE}(?:(.|\n)*)"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("@timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "@timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { !event.has_value("elasticsearch.index.name") };
                if _cond {
                    if event.has_value("_parsed_index_name") {
                        event.rename("_parsed_index_name", "elasticsearch.index.name")?;
                    }
                }
                event.remove("_parsed_index_name");
                let _cond = { !event.has_value("elasticsearch.index.id") };
                if _cond {
                    if event.has_value("_parsed_index_id") {
                        event.rename("_parsed_index_id", "elasticsearch.index.id")?;
                    }
                }
                event.remove("_parsed_index_id");
                // End nested pipeline: "pipeline-json"
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            // Painless script
            // Source: def gc = ctx?.elasticsearch?.server?.gc; def observationDuration = gc?.observation_duration; if (observationDuration != null) {\n  if (observationDuration.unit == params.seconds_unit) {\n    observationDuration.ms = observationDuration.time * params.ms_in_one_s;\n  }\n  if (observationDuration.unit == params.milliseconds_unit) {\n    observationDuration.ms = observationDuration.time;\n  }\n  if (observationDuration.unit == params.minutes_unit) {\n    observationDuration.ms = observationDuration.time * params.ms_in_one_m;\n  }\n} def collectionDuration = gc?.collection_duration; if (collectionDuration != null) {\n  if (collectionDuration.unit == params.seconds_unit) {\n    collectionDuration.ms = collectionDuration.time * params.ms_in_one_s;\n  }\n  if (collectionDuration.unit == params.milliseconds_unit) {\n    collectionDuration.ms = collectionDuration.time;\n  }\n  if (collectionDuration.unit == params.minutes_unit) {\n    collectionDuration.ms = collectionDuration.time * params.ms_in_one_m;\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def gc = ctx?.elasticsearch?.server?.gc; def observationDuration = gc?.observation_duration; if (observationDuration != null) {\n  if (observationDuration.unit == params.seconds_unit) {\n    observationDuration.ms = observationDuration.time * params.ms_in_one_s;\n  }\n  if (observationDuration.unit == params.milliseconds_unit) {\n    observationDuration.ms = observationDuration.time;\n  }\n  if (observationDuration.unit == params.minutes_unit) {\n    observationDuration.ms = observationDuration.time * params.ms_in_one_m;\n  }\n} def collectionDuration = gc?.collection_duration; if (collectionDuration != null) {\n  if (collectionDuration.unit == params.seconds_unit) {\n    collectionDuration.ms = collectionDuration.time * params.ms_in_one_s;\n  }\n  if (collectionDuration.unit == params.milliseconds_unit) {\n    collectionDuration.ms = collectionDuration.time;\n  }\n  if (collectionDuration.unit == params.minutes_unit) {\n    collectionDuration.ms = collectionDuration.time * params.ms_in_one_m;\n  }\n}"#
                ),
                cached_params!(
                    "{\"minutes_unit\":\"m\",\"seconds_unit\":\"s\",\"milliseconds_unit\":\"ms\",\"ms_in_one_s\":1000,\"ms_in_one_m\":60000}"
                ),
            )?;

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

            if event.has_value("tags") {
                event.rename("tags", "elasticsearch.server.tags")?;
            }

            event.remove("elasticsearch.server.gc.collection_duration.time");
            event.remove("elasticsearch.server.gc.collection_duration.unit");
            event.remove("elasticsearch.server.gc.observation_duration.time");
            event.remove("elasticsearch.server.gc.observation_duration.unit");

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
