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

            event.append_unique("event.category", json!("threat"))?;

            event.append_unique("event.type", json!("indicator"))?;

            event.set("threat.indicator.provider", json!("eset"))?;

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

            parse_json_field(event, "event.original", "eti")?;

            let _cond = { event.get_str("eti.type") != Some("indicator") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("threat.feed.name", json!("ESET Botnet C&C stix 2.1"))?;

            let _cond = { event.has_value("eti.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("eti.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "eti.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("eti.modified") };
            if _cond {
                if let Some(date_str) = event.get_as_string("eti.modified") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "eti.modified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if let Some(v) = event.get("threat.indicator.last_seen").cloned() {
                    event.set("threat.indicator.modified_at", v)?;
                }
            }

            let _cond = { event.has_value("eti.valid_until") };
            if _cond {
                if let Some(date_str) = event.get_as_string("eti.valid_until") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("eset.valid_until", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "eti.valid_until".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("eti.id") {
                event.rename("eti.id", "eset.id")?;
            }

            if event.has_value("eti.description") {
                event.rename("eti.description", "threat.indicator.description")?;
            }

            let _cond = { event.has_value("eti.labels") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("eti.labels") {
                        foreach_array(event, "eti.labels", |event| {
                            event.append(
                                "eset.labels",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("eset.labels") };
            if _cond {
                // Painless script
                // Source: for (def label : ctx.eset.labels) {\n  if (params.containsKey(label)) {\n    ctx.threat.indicator.confidence = params.get(label);\n    break;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (def label : ctx.eset.labels) {\n  if (params.containsKey(label)) {\n    ctx.threat.indicator.confidence = params.get(label);\n    break;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"benign\":\"Low\",\"malicious-activity\":\"High\",\"phishing-activity\":\"High\",\"unwanted-activity\":\"Medium\"}"
                    ),
                )?;
            }

            if event.has_value("eti.pattern") {
                if let Some(s) = event.get_string("eti.pattern") {
                    let mut parts: Vec<Value> = s.split(" OR ").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("eti._patterns", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("eti._patterns") };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("eti._patterns").cloned();
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
                                if let Some(input) = event.get_string("_ingest._value") {
                                    // Grok pattern: ^\\[url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?
                                    if !cached_grok!("^\\[url:value%{SPACE}=%{SPACE}'%{DATA:threat.indicator.url.original}'\\]?").extract_into(&input, event)? {
                            return Err(TransformError::GrokNoMatch { value: input });
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
                            "eti._patterns",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            event.set("threat.indicator.type", json!("url"))?;

            let _cond = { event.has_value("eti.name") };
            if _cond {
                if let Some(v) = event.get("eti.name").cloned() {
                    event.set("threat.indicator.name", v)?;
                }
            }

            event.remove("eti");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

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
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
