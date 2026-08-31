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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "event.ingested",
                    json!(
                        event
                            .get("_ingest.timestamp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("ibmmq");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("event");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("error")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            gsub_field(
                event,
                "message",
                "message",
                cached_regex!("^[\\-]{5}[a-z0-9\\. :]*[\\-]{5,}"),
                "",
            )?;

            gsub_field(event, "message", "message", cached_regex!("\n"), " ")?;

            gsub_field(event, "message", "message", cached_regex!("[ ]{2,}"), " ")?;

            map_strings(event, "message", "message", |s| s.trim().to_string())?;

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^%{DATA:log_timestamp} -
                if !cached_grok!("^%{DATA:log_timestamp} -").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if let Some(input) = event.get_string("message") {
                // Grok pattern: Process\\(%{DATA:process.pid}\\) User\\(%{WORD:user.name}\\) Program\\(%{DATA:process.title}\\) Host\\(%{DATA:host.hostname}\\) Installation\\(%{WORD:ibmmq.errorlog.installation}\\) VRMF\\(%{DATA:service.version}\\)( QMgr\\(%{DATA:ibmmq.errorlog.queue_manager}\\))?( Time\\(%{TIMESTAMP_ISO8601:log_timestamp}\\))?( RemoteHost\\(%{DATA:destination.address}\\))?( ArithInsert1\\(%{DATA:ibmmq.errorlog.arithinsert1}\\))?( ArithInsert2\\(%{DATA:ibmmq.errorlog.arithinsert2}\\))?( CommentInsert1\\(%{DATA:ibmmq.errorlog.commentinsert1}\\))?( CommentInsert2\\(%{DATA:ibmmq.errorlog.commentinsert2}\\))?( CommentInsert3\\(%{DATA:ibmmq.errorlog.commentinsert3}\\))? (?=AMQ[0-9]{4})%{DATA:ibmmq.errorlog.error.code}((?<=AMQ[0-9]{4}[A-Z])%{DATA:log.level})?: %{DATA:ibmmq.errorlog.error.description}( EXPLANATION: %{DATA:ibmmq.errorlog.error.explanation})?( ACTION: %{DATA:ibmmq.errorlog.error.action})?$
                if !cached_grok!("Process\\(%{DATA:process.pid}\\) User\\(%{WORD:user.name}\\) Program\\(%{DATA:process.title}\\) Host\\(%{DATA:host.hostname}\\) Installation\\(%{WORD:ibmmq.errorlog.installation}\\) VRMF\\(%{DATA:service.version}\\)( QMgr\\(%{DATA:ibmmq.errorlog.queue_manager}\\))?( Time\\(%{TIMESTAMP_ISO8601:log_timestamp}\\))?( RemoteHost\\(%{DATA:destination.address}\\))?( ArithInsert1\\(%{DATA:ibmmq.errorlog.arithinsert1}\\))?( ArithInsert2\\(%{DATA:ibmmq.errorlog.arithinsert2}\\))?( CommentInsert1\\(%{DATA:ibmmq.errorlog.commentinsert1}\\))?( CommentInsert2\\(%{DATA:ibmmq.errorlog.commentinsert2}\\))?( CommentInsert3\\(%{DATA:ibmmq.errorlog.commentinsert3}\\))? (?=AMQ[0-9]{4})%{DATA:ibmmq.errorlog.error.code}((?<=AMQ[0-9]{4}[A-Z])%{DATA:log.level})?: %{DATA:ibmmq.errorlog.error.description}( EXPLANATION: %{DATA:ibmmq.errorlog.error.explanation})?( ACTION: %{DATA:ibmmq.errorlog.error.action})?$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "float").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("log_timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "MM/dd/yyyy hh:mm:ss a",
                            "dd/MM/yyyy HH:mm:ss",
                            "dd.MM.yyyy HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "log_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append(
                    "ibmmq.errorlog.insert.comment",
                    json!(
                        event
                            .get("ibmmq.errorlog.commentinsert1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append(
                    "ibmmq.errorlog.insert.comment",
                    json!(
                        event
                            .get("ibmmq.errorlog.commentinsert2")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append(
                    "ibmmq.errorlog.insert.comment",
                    json!(
                        event
                            .get("ibmmq.errorlog.commentinsert3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append(
                    "ibmmq.errorlog.insert.arith",
                    json!(
                        event
                            .get("ibmmq.errorlog.arithinsert1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append(
                    "ibmmq.errorlog.insert.arith",
                    json!(
                        event
                            .get("ibmmq.errorlog.arithinsert2")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("log_timestamp");
                event.remove("ibmmq.errorlog.arithinsert1");
                event.remove("ibmmq.errorlog.arithinsert2");
                event.remove("ibmmq.errorlog.commentinsert1");
                event.remove("ibmmq.errorlog.commentinsert2");
                event.remove("ibmmq.errorlog.commentinsert3");
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
