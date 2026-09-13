// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_plaintext` pipeline.
pub struct PipelinePlaintext;

impl Transform for PipelinePlaintext {
    fn name(&self) -> &str {
        "pipeline_plaintext"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[%{TIMESTAMP_ISO8601:logstash.slowlog.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_slowlog_module>(?:[\\w\\.]+\\s*))\\] %{GREEDYDATA:message}
                    if !cached_grok_mapped!("\\[%{TIMESTAMP_ISO8601:logstash.slowlog.timestamp}\\]\\[(?P<log_level>(?:INFO|ERROR|DEBUG|FATAL|WARN|TRACE))\\s?\\]\\[(?P<logstash_slowlog_module>(?:[\\w\\.]+\\s*))\\] %{GREEDYDATA:message}", [("log_level", "log.level"), ("logstash_slowlog_module", "logstash.slowlog.module")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

                if let Some(input) = event.get_string("logstash.slowlog.module") {
                    // Grok pattern: slowlog.logstash.%{WORD:logstash.slowlog.plugin_type}.%{WORD:logstash.slowlog.plugin_name}
                    if !cached_grok!("slowlog.logstash.%{WORD:logstash.slowlog.plugin_type}.%{WORD:logstash.slowlog.plugin_name}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

                if let Some(input) = event.get_string("message") {
                    // Grok pattern: {:plugin_params=>%{GREEDYDATA:logstash.slowlog.plugin_params}, :took_in_nanos=>%{NUMBER:event.duration}, :took_in_millis=>%{NUMBER:logstash.slowlog.took_in_millis}, :event=>%{GREEDYDATA:logstash.slowlog.event}}
                    if !cached_grok!("{:plugin_params=>%{GREEDYDATA:logstash.slowlog.plugin_params}, :took_in_nanos=>%{NUMBER:event.duration}, :took_in_millis=>%{NUMBER:logstash.slowlog.took_in_millis}, :event=>%{GREEDYDATA:logstash.slowlog.event}}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("logstash.slowlog.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss,SSS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "logstash.slowlog.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("logstash.slowlog.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss,SSS"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "logstash.slowlog.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.remove("message").is_none() {
                    return Err(TransformError::FieldNotFound { path: "message".into() });
                }
                if event.remove("logstash.slowlog.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound { path: "logstash.slowlog.timestamp".into() });
                }

                if let Some(val) = event.get("event.duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "event.duration".into(),
                            message,
                        })?;
                    event.set("event.duration", converted)?;
                }

                if let Some(val) = event.get("logstash.slowlog.took_in_millis") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "logstash.slowlog.took_in_millis".into(),
                            message,
                        })?;
                    event.set("logstash.slowlog.took_in_millis", converted)?;
                }

            event.set("event.kind", json!("event"))?;

                // Painless script
                // Source: def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def errorLevels = [\"ERROR\", \"FATAL\"]; if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}"#))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
