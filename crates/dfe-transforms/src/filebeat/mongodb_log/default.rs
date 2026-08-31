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

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.rename("@timestamp", "event.created")?;

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^(?P<first_char>(?:.))
                if !cached_grok!("^(?P<first_char>(?:.))").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("first_char") != Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-plaintext"
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: %{TIMESTAMP_ISO8601:mongodb.log.timestamp}%{SPACE}%{MONGO3_SEVERITY:log.level}%{SPACE}%{MONGO3_COMPONENT:mongodb.log.component}%{SPACE}(?:\\[%{DATA:mongodb.log.context}\\])?%{SPACE}%{GREEDYDATA:message}
                        if !cached_grok!("%{TIMESTAMP_ISO8601:mongodb.log.timestamp}%{SPACE}%{MONGO3_SEVERITY:log.level}%{SPACE}%{MONGO3_COMPONENT:mongodb.log.component}%{SPACE}(?:\\[%{DATA:mongodb.log.context}\\])?%{SPACE}%{GREEDYDATA:message}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                if let Some(date_str) = event.get_as_string("mongodb.log.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSZZ"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mongodb.log.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                if event.remove("mongodb.log.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "mongodb.log.timestamp".into(),
                    });
                }
                let _cond = { event.get_str("mongodb.log.component") == Some("ACCESS") };
                if _cond {
                    event.append("event.type", json!("access"))?;
                }
                let _cond = { event.get_str("mongodb.log.component") == Some("WRITE") };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.get_str("mongodb.log.component") != Some("WRITE")
                        && event.get_str("mongodb.log.component") != Some("ACCESS")
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event.get_str("log.level") == Some("F")
                        || event.get_str("log.level") == Some("E")
                };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                // End nested pipeline: "pipeline-plaintext"
            }

            let _cond = { event.get_str("first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "pipeline-json"
                parse_json_field(event, "message", "mongodb.log")?;
                if let Some(date_str) = event.get_as_string("mongodb.log.t.$date") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSZZZZZ"], None, None)
                    {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mongodb.log.t.$date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                event.rename("mongodb.log.s", "log.level")?;
                event.rename("mongodb.log.c", "mongodb.log.component")?;
                event.rename("mongodb.log.ctx", "mongodb.log.context")?;
                let _cond = { !event.has_value("event.original") };
                if _cond {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                }
                event.rename("mongodb.log.msg", "message")?;
                let _cond = { event.get_str("mongodb.log.component") == Some("ACCESS") };
                if _cond {
                    event.append("event.type", json!("access"))?;
                }
                let _cond = { event.get_str("mongodb.log.component") == Some("WRITE") };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.get_str("mongodb.log.component") != Some("WRITE")
                        && event.get_str("mongodb.log.component") != Some("ACCESS")
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event.get_str("log.level") == Some("F")
                        || event.get_str("log.level") == Some("E")
                };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                event.remove("mongodb.log.t");
                event.remove("mongodb.log.tags");
                event.remove("mongodb.log.truncated");
                event.remove("mongodb.log.size");
                // End nested pipeline: "pipeline-json"
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.effective.name")
                    && event.get_str("user.effective.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.effective.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
