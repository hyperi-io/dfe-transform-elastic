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

            event.set("observer.vendor", json!("ForgeRock Identity Platform"))?;

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

            event.rename("json.payload", "forgerock")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock._id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.transactionId").cloned() {
                    event.set("transaction.id", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("forgerock.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "forgerock.timestamp".into(),
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

            event.set("event.type", Value::Array(vec![json!("access")]))?;

            if event.has_value("forgerock.client.ip") {
                if let Some(val) = event.get("forgerock.client.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "forgerock.client.ip".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.client.port").cloned() {
                    event.set("client.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.server.host").cloned() {
                    event.set("server.domain", v)?;
                }
                Ok(())
            })();

            if event.has_value("forgerock.server.ip") {
                if let Some(val) = event.get("forgerock.server.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "forgerock.server.ip".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.userId").cloned() {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.http.request.method").cloned() {
                    event.set("http.request.method", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("forgerock.http.request.path").cloned() {
                    event.set("http.request.Path", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("forgerock.response.statusCode") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "forgerock.response.statusCode".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("forgerock.response.status") == Some("SUCCESSFUL") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("forgerock.response.status") == Some("FAILED") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("forgerock.response.elapsedTime") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "forgerock.response.elapsedTime".into(),
                            message,
                        }
                    })?;
                    event.set("forgerock.response.elapsedTime", converted)?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("forgerock.response.elapsedTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            let _cond = {
                event.has_value("event.duration")
                    && event.get_str("forgerock.response.elapsedTimeUnits") == Some("MILLISECONDS")
            };
            if _cond {
                // Painless script
                // Source: ctx.event.duration *= params.MS_TO_NS;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(r#"ctx.event.duration *= params.MS_TO_NS;"#),
                    cached_params!("{\"MS_TO_NS\":1000000}"),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("json.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.timestamp".into(),
                    });
                }
                if event.remove("json.type").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.type".into(),
                    });
                }
                Ok(())
            })();

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    event.remove("forgerock._id");
                    event.remove("forgerock.transactionId");
                    event.remove("forgerock.userId");
                    event.remove("forgerock.client.port");
                    event.remove("forgerock.client.ip");
                    event.remove("forgerock.server.port");
                    event.remove("forgerock.server.ip");
                    event.remove("forgerock.http.request.method");
                    event.remove("forgerock.http.request.path");
                    event.remove("forgerock.response.statusCode");
                    event.remove("forgerock.timestamp");
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
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
