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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.ocsp")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("network.transport", json!("tcp"))?;

            if event.has_value("zeek.ocsp.id") {
                event.rename("zeek.ocsp.id", "zeek.ocsp.file_id")?;
            }

            if event.has_value("zeek.ocsp.hashAlgorithm") {
                event.rename("zeek.ocsp.hashAlgorithm", "zeek.ocsp.hash.algorithm")?;
            }

            if event.has_value("zeek.ocsp.issuerNameHash") {
                event.rename("zeek.ocsp.issuerNameHash", "zeek.ocsp.hash.issuer.name")?;
            }

            if event.has_value("zeek.ocsp.issuerKeyHash") {
                event.rename("zeek.ocsp.issuerKeyHash", "zeek.ocsp.hash.issuer.key")?;
            }

            if event.has_value("zeek.ocsp.serialNumber") {
                event.rename("zeek.ocsp.serialNumber", "zeek.ocsp.serial_number")?;
            }

            if event.has_value("zeek.ocsp.certStatus") {
                event.rename("zeek.ocsp.certStatus", "zeek.ocsp.status")?;
            }

            if event.has_value("zeek.ocsp.revoketime") {
                event.rename("zeek.ocsp.revoketime", "zeek.ocsp.revoke.date")?;
            }

            if event.has_value("zeek.ocsp.revokereason") {
                event.rename("zeek.ocsp.revokereason", "zeek.ocsp.revoke.reason")?;
            }

            if event.has_value("zeek.ocsp.thisUpdate") {
                event.rename("zeek.ocsp.thisUpdate", "zeek.ocsp.update.this")?;
            }

            if event.has_value("zeek.ocsp.nextUpdate") {
                event.rename("zeek.ocsp.nextUpdate", "zeek.ocsp.update.next")?;
            }

            if let Some(date_str) = event.get_as_string("zeek.ocsp.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.ocsp.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.ocsp.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.ocsp.ts".into(),
                });
            }

            let _cond = { event.has_value("zeek.ocsp.revoke.date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.ocsp.revoke.date") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.ocsp.revoke.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.ocsp.revoke.date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("zeek.ocsp.update.this") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.ocsp.update.this") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.ocsp.update.this", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.ocsp.update.this".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("zeek.ocsp.update.next") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.ocsp.update.next") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.ocsp.update.next", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.ocsp.update.next".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("zeek.ocsp.issuerNameHash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zeek.ocsp.issuerNameHash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zeek.ocsp.issuerKeyHash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zeek.ocsp.issuerKeyHash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
