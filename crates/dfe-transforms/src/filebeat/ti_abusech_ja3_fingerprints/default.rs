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
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = {
                event.has_value("_conf.interval") && event.get_str("_conf.interval") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_conf.interval").cloned() {
                    event.set("labels.interval", v)?;
                }
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

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

            parse_json_field(event, "event.original", "abusech.ja3_fingerprints")?;

            event.set("threat.indicator.type", json!("software"))?;

            let _cond = { event.has_value("abusech.ja3_fingerprints.first_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.first_ts") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.first_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.first_seen") };
            if _cond {
                if event.remove("abusech.ja3_fingerprints.first_ts").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "abusech.ja3_fingerprints.first_ts".into(),
                    });
                }
            }

            let _cond = { event.has_value("abusech.ja3_fingerprints.last_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.last_ts") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.last_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if event.remove("abusech.ja3_fingerprints.last_ts").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "abusech.ja3_fingerprints.last_ts".into(),
                    });
                }
            }

            if let Some(v) = event.get("abusech.ja3_fingerprints.ja3").cloned() {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event.get("abusech.ja3_fingerprints.reason").cloned() {
                event.set("threat.indicator.description", v)?;
            }

            let _cond = { event.has_value("abusech.ja3_fingerprints.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("abusech.ja3_fingerprints.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event.get("_ingest.timestamp").cloned() {
                event.set("event.ingested", v)?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "9c45003feb1b31b4346c952df7a294999199d612dec96468491ef641c2216493",
                    r#"{"params":null,"known":[{"IocExpiry":{"duration":"labels.interval","default_duration":null,"bases":["event.ingested"],"default_days":90,"settle_seconds":90,"invalid_message":"invalid ioc_expiration_duration: using default 90 days","target":"abusech.ja3_fingerprints.deleted_at"}}]}"#
                ),
            )?;

            let _cond = { event.has_value("abusech.ja3_fingerprints.deleted_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("abusech.ja3_fingerprints.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("abusech.ja3_fingerprints.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "abusech.ja3_fingerprints.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            let _cond = { event.has_value("abusech") };
            if _cond {
                // The pipeline's script, emitted as the runtime's equivalent
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            event.remove("abusech.ja3_fingerprints.ja3");
            event.remove("abusech.ja3_fingerprints.reason");
            event.remove("message");
            event.remove("_conf");

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} {}in pipeline {} failed with message {}",
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
                                "with tag {} ",
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
