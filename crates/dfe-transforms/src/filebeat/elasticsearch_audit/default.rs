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
                parse_json_field(event, "message", "elasticsearch.audit")?;
                dot_expand(event, "elasticsearch.audit", "event.type")?;
                let _cond = {
                    event.has("elasticsearch.audit.type")
                        && event.get_str("elasticsearch.audit.type") != Some("audit")
                };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                let _cond = {
                    !(event.has("elasticsearch.audit.type"))
                        && !(["rest", "transport", "ip_filter", "security_config_change"].contains(
                            &event
                                .get_str("elasticsearch.audit.event.type")
                                .unwrap_or(""),
                        ))
                };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                event.remove("elasticsearch.audit.type");
                // SKIPPED: condition not transpiled: ctx.elasticsearch.audit['@timestamp'] != null && ctx.event.timezone != null
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("elasticsearch.audit.@timestamp")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy-MM-dd'T'HH:mm:ss,SSS", "yyyy-MM-dd'T'HH:mm:ss,SSSZ"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("elasticsearch.audit.@timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "elasticsearch.audit.@timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                // SKIPPED: condition not transpiled: ctx.elasticsearch.audit['@timestamp'] == null && ctx.event.timezone != null
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if event.remove("event.timezone").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.timezone".into(),
                        });
                    }
                }
                if event.has_value("elasticsearch.audit.timestamp") {
                    event.rename(
                        "elasticsearch.audit.timestamp",
                        "elasticsearch.audit.@timestamp",
                    )?;
                }
                dot_expand(event, "elasticsearch.audit", "event.action")?;
                event.remove("event.action");
                if event.has_value("elasticsearch.audit.event.action") {
                    event.rename("elasticsearch.audit.event.action", "event.action")?;
                }
                if event.has_value("elasticsearch.audit.event.type") {
                    event.rename(
                        "elasticsearch.audit.event.type",
                        "elasticsearch.audit.layer",
                    )?;
                }
                dot_expand(event, "elasticsearch.audit", "origin.address")?;
                if event.has_value("elasticsearch.audit.origin.address") {
                    if let Some(input) = event.get_string("elasticsearch.audit.origin.address") {
                        // Grok pattern: \\[%{IPORHOST:source.ip}\\]:%{INT:source.port:int}
                        // Grok pattern: %{IPORHOST:source.ip}:%{INT:source.port:int}
                        if !extract_first_match(
                            &[
                                cached_grok!("\\[%{IPORHOST:source.ip}\\]:%{INT:source.port:int}"),
                                cached_grok!("%{IPORHOST:source.ip}:%{INT:source.port:int}"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                event.remove("source.address");
                if event.has_value("elasticsearch.audit.origin.address") {
                    event.rename("elasticsearch.audit.origin.address", "source.address")?;
                }
                dot_expand(event, "elasticsearch.audit", "url.path")?;
                dot_expand(event, "elasticsearch.audit", "url.query")?;
                let _cond = { !event.has_value("elasticsearch.audit.url.query") };
                if _cond {
                    let v = json!(
                        event
                            .get("elasticsearch.audit.url.path")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.has_value("elasticsearch.audit.url.path")
                        && event.has_value("elasticsearch.audit.url.query")
                };
                if _cond {
                    event.set(
                        "url.original",
                        json!(format!(
                            "{}?{}",
                            event
                                .get("elasticsearch.audit.url.path")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("elasticsearch.audit.url.query")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.has_value("elasticsearch.audit.url.path") };
                if _cond {
                    if event.remove("elasticsearch.audit.url.path").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "elasticsearch.audit.url.path".into(),
                        });
                    }
                }
                let _cond = { event.has_value("elasticsearch.audit.url.query") };
                if _cond {
                    if event.remove("elasticsearch.audit.url.query").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "elasticsearch.audit.url.query".into(),
                        });
                    }
                }
                dot_expand(event, "elasticsearch.audit", "node.id")?;
                dot_expand(event, "elasticsearch.audit", "node.name")?;
                event.remove("elasticsearch.node");
                event.rename("elasticsearch.audit.node", "elasticsearch.node")?;
                if event.has_value("elasticsearch.audit.change.disable.user.name") {
                    event.rename("elasticsearch.audit.change.disable.user.name", "user.name")?;
                }
                if event.has_value("elasticsearch.audit.change.enable.user.name") {
                    event.rename("elasticsearch.audit.change.enable.user.name", "user.name")?;
                }
                if event.has_value("elasticsearch.audit.delete.user.name") {
                    event.rename("elasticsearch.audit.delete.user.name", "user.name")?;
                }
                if event.has_value("elasticsearch.audit.put.user.name") {
                    event.rename("elasticsearch.audit.put.user.name", "user.name")?;
                }
                if event.has_value("elasticsearch.audit.put.user.full_name") {
                    event.rename("elasticsearch.audit.put.user.full_name", "user.full_name")?;
                }
                if event.has_value("elasticsearch.audit.put.user.email") {
                    event.rename("elasticsearch.audit.put.user.email", "user.email")?;
                }
                event.remove("elasticsearch.audit.put");
                if event.has_value("elasticsearch.audit.invalidate.apikeys.user.name") {
                    event.rename(
                        "elasticsearch.audit.invalidate.apikeys.user.name",
                        "user.name",
                    )?;
                }
                if event.has_value("elasticsearch.audit.invalidate.apikeys.user.realm") {
                    event.rename(
                        "elasticsearch.audit.invalidate.apikeys.user.realm",
                        "elasticsearch.audit.user.realm",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    dot_expand(event, "elasticsearch.audit", "user.run_as.name")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    dot_expand(event, "elasticsearch.audit", "user.run_as.realm")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("elasticsearch.audit.user.run_as.name") {
                        if let Some(val) = event.get("elasticsearch.audit.user.run_as.name") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "elasticsearch.audit.user.run_as.name".into(),
                                    message,
                                }
                            })?;
                            event.set("user.effective.name", converted)?;
                        }
                    }
                    Ok(())
                })();
                dot_expand(event, "elasticsearch.audit", "user.name")?;
                if event.has_value("elasticsearch.audit.user.name") {
                    event.rename("elasticsearch.audit.user.name", "user.name")?;
                }
                dot_expand(event, "elasticsearch.audit", "user.email")?;
                dot_expand(event, "elasticsearch.audit", "request.method")?;
                if event.has_value("elasticsearch.audit.request.method") {
                    event.rename("elasticsearch.audit.request.method", "http.request.method")?;
                }
                dot_expand(event, "elasticsearch.audit", "request.body")?;
                if event.has_value("elasticsearch.audit.request.body") {
                    event.rename(
                        "elasticsearch.audit.request.body",
                        "http.request.body.content",
                    )?;
                }
                dot_expand(event, "elasticsearch.audit", "request.id")?;
                let v = json!(
                    event
                        .get("elasticsearch.audit.request.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("http.request.id", v)?;
                }
                dot_expand(event, "elasticsearch.audit", "cluster.name")?;
                dot_expand(event, "elasticsearch.audit", "cluster.uuid")?;
                if event.has_value("elasticsearch.audit.cluster.name") {
                    event.rename(
                        "elasticsearch.audit.cluster.name",
                        "elasticsearch.cluster.name",
                    )?;
                }
                if event.has_value("elasticsearch.audit.cluster.uuid") {
                    event.rename(
                        "elasticsearch.audit.cluster.uuid",
                        "elasticsearch.cluster.uuid",
                    )?;
                }
                if event.has_value("elasticsearch.audit.level") {
                    event.rename("elasticsearch.audit.level", "log.level")?;
                }
                if !event.has("log.level") {
                    event.set("log.level", json!("info"))?;
                }
                dot_expand(event, "elasticsearch.audit", "trace.id")?;
                if event.has_value("elasticsearch.audit.trace.id") {
                    event.rename("elasticsearch.audit.trace.id", "trace.id")?;
                }
                event.remove("elasticsearch.audit.trace.id");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("elasticsearch.audit.@timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "elasticsearch.audit.@timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                event.set("service.type", json!("elasticsearch"))?;
                // End nested pipeline: "pipeline-json"
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("event.category", json!("database"))?;

            let _cond = { event.has_value("elasticsearch.audit.event_type") };
            if _cond {
                event.set("event.type", json!("access"))?;
            }

            // Painless script
            // Source: def successEvents = ['authentication_success', 'access_granted', 'run_as_granted', 'connection_granted']; if (ctx?.elasticsearch?.audit?.event_type != null && successEvents.contains(ctx.elasticsearch.audit.event_type)) {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'failure';\n} if (ctx?.event.action != null && successEvents.contains(ctx.event.action)) {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'failure';\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def successEvents = ['authentication_success', 'access_granted', 'run_as_granted', 'connection_granted']; if (ctx?.elasticsearch?.audit?.event_type != null && successEvents.contains(ctx.elasticsearch.audit.event_type)) {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'failure';\n} if (ctx?.event.action != null && successEvents.contains(ctx.event.action)) {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'failure';\n}"#
                ),
            )?;

            let v = json!(
                event
                    .get("elasticsearch.node.id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.id", v)?;
            }

            let v = json!(
                event
                    .get("elasticsearch.node.name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.effective.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.effective.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.remove("elasticsearch.audit.@timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "elasticsearch.audit.@timestamp".into(),
                });
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
