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

            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| cached_regex!(r"^\d").is_match(s))
            };
            if _cond {
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
                                        event.set("client.ip", val)?;
                                    }
                                }
                                if let Some(val) = record.get(2) {
                                    if !val.is_empty() {
                                        event.set("client.domain", val)?;
                                    }
                                }
                                if let Some(val) = record.get(3) {
                                    if !val.is_empty() {
                                        event.set("server.ip", val)?;
                                    }
                                }
                                if let Some(val) = record.get(4) {
                                    if !val.is_empty() {
                                        event.set("server.domain", val)?;
                                    }
                                }
                                if let Some(val) = record.get(5) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.sourcecontext", val)?;
                                    }
                                }
                                if let Some(val) = record.get(6) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.connectorid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(7) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.source", val)?;
                                    }
                                }
                                if let Some(val) = record.get(8) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.eventid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(9) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.internalmessageid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(10) {
                                    if !val.is_empty() {
                                        event.set("email.message_id", val)?;
                                    }
                                }
                                if let Some(val) = record.get(11) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.networkmessageid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(12) {
                                    if !val.is_empty() {
                                        event.set("email.to.address", val)?;
                                    }
                                }
                                if let Some(val) = record.get(13) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.recipientstatus", val)?;
                                    }
                                }
                                if let Some(val) = record.get(14) {
                                    if !val.is_empty() {
                                        event.set("network.bytes", val)?;
                                    }
                                }
                                if let Some(val) = record.get(15) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.recipientcount", val)?;
                                    }
                                }
                                if let Some(val) = record.get(16) {
                                    if !val.is_empty() {
                                        event.set(
                                            "microsoft.exchange.relatedrecipientaddress",
                                            val,
                                        )?;
                                    }
                                }
                                if let Some(val) = record.get(17) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.reference", val)?;
                                    }
                                }
                                if let Some(val) = record.get(18) {
                                    if !val.is_empty() {
                                        event.set("email.subject", val)?;
                                    }
                                }
                                if let Some(val) = record.get(19) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.senderaddress", val)?;
                                    }
                                }
                                if let Some(val) = record.get(20) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.returnpath", val)?;
                                    }
                                }
                                if let Some(val) = record.get(21) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.messageinfo", val)?;
                                    }
                                }
                                if let Some(val) = record.get(22) {
                                    if !val.is_empty() {
                                        event.set("email.direction", val)?;
                                    }
                                }
                                if let Some(val) = record.get(23) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.tenantid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(24) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.originalclientip", val)?;
                                    }
                                }
                                if let Some(val) = record.get(25) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.originalserverip", val)?;
                                    }
                                }
                                if let Some(val) = record.get(26) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.customdata", val)?;
                                    }
                                }
                                if let Some(val) = record.get(27) {
                                    if !val.is_empty() {
                                        event
                                            .set("microsoft.exchange.transporttraffictype", val)?;
                                    }
                                }
                                if let Some(val) = record.get(28) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.logid", val)?;
                                    }
                                }
                                if let Some(val) = record.get(29) {
                                    if !val.is_empty() {
                                        event.set("microsoft.exchange.schemaversion", val)?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.to.address") {
                    if let Some(s) = event.get_string("email.to.address") {
                        let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                        event.set("email.to.address", Value::Array(parts))?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft.exchange.networkmessageid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append(
                    "email.sender.address",
                    json!(
                        event
                            .get("microsoft.exchange.senderaddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("microsoft.exchange.customdata") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("S:OriginalFromAddress=") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("S:OriginalFromAddress=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(";") else {
                            break 'dissect false;
                        };
                        captured.push(("_tmp.email.from.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(";") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("_tmp.email.from.address")
                    && event.get_str("_tmp.email.from.address") != Some("<>")
            };
            if _cond {
                event.append(
                    "email.from.address",
                    json!(
                        event
                            .get("_tmp.email.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("microsoft.exchange.senderaddress")
                    && event.get_str("microsoft.exchange.senderaddress") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "email.from.address",
                        json!(
                            event
                                .get("microsoft.exchange.senderaddress")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("microsoft.exchange.senderaddress");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft.exchange.recipientcount") {
                    if let Some(val) = event.get("microsoft.exchange.recipientcount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft.exchange.recipientcount".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft.exchange.recipientcount", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("network.bytes") {
                    if let Some(val) = event.get("network.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "network.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("network.bytes", converted)?;
                    }
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

            event.remove("_tmp");

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
