// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `http_event` pipeline.
pub struct HttpEvent;

impl Transform for HttpEvent {
    fn name(&self) -> &str {
        "http_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("arista.httpRequestEvent.timeStamp") };
            if _cond {
                event.remove("arista.httpRequestEvent.timeStamp");
            }

            let _cond = { event.has_value("arista.httpRequestEvent.sessionEvent.timeStamp") };
            if _cond {
                event.remove("arista.httpRequestEvent.sessionEvent.timeStamp");
            }

            let _cond = { event.has_value("arista.httpRequestEvent.sessionEvent") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("arista.httpRequestEvent.sessionEvent").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(from) = resolve_path(event, "_ingest._value")
                            && let Some(to) = resolve_path(event, "arista.{{{_ingest._key}}}")
                            && event.has(&from)
                            {
                            event.rename(&from, &to)?;
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "rename")?;
                            if event.remove("_ingest._key").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._key".into() });
                            }
                            event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("arista.httpRequestEvent.sessionEvent", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

                event.remove("arista.httpRequestEvent.contentLength");

                if event.has_value("arista.httpRequestEvent.domain") {
                    event.rename("arista.httpRequestEvent.domain", "destination.domain")?;
                }

                if event.has_value("arista.httpRequestEvent.method") {
                    event.rename("arista.httpRequestEvent.method", "http.request.method")?;
                }

                if event.has_value("arista.httpRequestEvent.requestId") {
                    event.rename("arista.httpRequestEvent.requestId", "arista.requestId")?;
                }

                if event.has_value("arista.httpRequestEvent.requestUri") {
                    event.rename("arista.httpRequestEvent.requestUri", "arista.requestUri")?;
                }

                if event.has_value("arista.httpRequestEvent.timeStamp") {
                    event.rename("arista.httpRequestEvent.timeStamp", "arista.timeStamp")?;
                }

                if event.has_value("arista.domain") {
                    event.rename("arista.domain", "destination.domain")?;
                }

                if event.has_value("arista.method") {
                    event.rename("arista.method", "http.request.method")?;
                }

            let _cond = { event.get_str("arista.class") == Some("class com.untangle.app.http.HttpRequestEvent") };
            if _cond {
                if event.has_value("arista.contentLength") {
                    event.rename("arista.contentLength", "http.request.bytes")?;
                }
            }

            let _cond = { event.get_str("arista.class") == Some("class com.untangle.app.http.HttpResponseEvent") };
            if _cond {
                if event.has_value("arista.contentLength") {
                    event.rename("arista.contentLength", "http.response.bytes")?;
                }
            }

                if event.has_value("arista.requestUri") {
                    event.rename("arista.requestUri", "url.path")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.contentFilename", "file.name")?;
                Ok(())
            })();

            let _cond = { event.has_value("arista.requestLine") && event.get("arista.requestLine").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(" ")), serde_json::Value::String(s) => s.contains(" "), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("arista.requestLine") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("http.request.method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("_temp.url_full", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "arista.requestLine".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("_temp.url_full") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "_temp.url_full", "url", true, false)?;
                Ok(())
            })();
            }

                if event.has_value("_temp.url_full") {
                    event.rename("_temp.url_full", "url.full")?;
                }

                event.remove("arista.contentType");
                event.remove("arista.host");
                event.remove("arista.httpRequestEvent.host");
                event.remove("arista.requestId");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
