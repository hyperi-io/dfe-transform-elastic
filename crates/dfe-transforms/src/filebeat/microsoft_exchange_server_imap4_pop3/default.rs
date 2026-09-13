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
                event
                    .get_str("message")
                    .is_some_and(|s| cached_regex!(r"^[^0-9]").is_match(s))
                    || event
                        .get_str("message")
                        .is_some_and(|s| cached_regex!(r"^#").is_match(s))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("event.original") {
                    if let Some(csv_str) = event.get_string("event.original") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                if !val.is_empty() {
                                    event.set("@timestamp", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.sessionid", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.seqnumber", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.sip", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.cip", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.user", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.duration", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.rqsize", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.rpsize", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.command", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.parameters", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.context", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("microsoft.exchange.puid", val)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("microsoft.exchange.cip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("microsoft.exchange.cip") {
                        if let Some(input) = event.get_string("microsoft.exchange.cip") {
                            // Grok pattern: %{NOTSPACE:source.ip}:%{NUMBER}
                            if !cached_grok!("%{NOTSPACE:source.ip}:%{NUMBER}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get_str("log.file.path")
                    .is_some_and(|s| cached_regex!(r"Imap4").is_match(s))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("imap4");
                    if !painless_is_empty_value(&v) {
                        event.set("microsoft.exchange.logtype", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get_str("log.file.path")
                    .is_some_and(|s| cached_regex!(r"Pop3").is_match(s))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("pop3");
                    if !painless_is_empty_value(&v) {
                        event.set("microsoft.exchange.logtype", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("microsoft.exchange.duration") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.exchange.duration".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.exchange.duration", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("microsoft.exchange.rpsize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.exchange.rpsize".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.exchange.rpsize", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("microsoft.exchange.rqsize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.exchange.rqsize".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.exchange.rqsize", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("microsoft.exchange.seqnumber") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.exchange.seqnumber".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.exchange.seqnumber", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("event.ingested", v)?;
                }
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
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
