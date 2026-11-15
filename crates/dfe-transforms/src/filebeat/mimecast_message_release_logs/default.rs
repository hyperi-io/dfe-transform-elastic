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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "mimecast")?;

            let _cond = { event.has_value("mimecast.released") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mimecast.released") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ssZZZZZ"],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mimecast.released".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("@timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("mimecast.fromEnv.emailAddress") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.fromEnv.emailAddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.fromHdr.emailAddress") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.fromHdr.emailAddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("mimecast.to").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "mimecast.to", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.emailAddress")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("mimecast.route")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.direction", v)?;
            }

            if let Some(v) = event
                .get("mimecast.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            if let Some(v) = event
                .get("mimecast.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            let _cond = { event.get_str("email.direction") == Some("outbound") };
            if _cond {
                if let Some(v) = event
                    .get("email.from.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.get_str("email.direction") == Some("outbound")
                    && event.has_value("mimecast.fromEnv.emailAddress")
            };
            if _cond {
                event.append(
                    "user.full_name",
                    json!(
                        event
                            .get("mimecast.fromEnv.displayableName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("email.direction") == Some("inbound") };
            if _cond {
                if let Some(v) = event
                    .get("email.to.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.get_str("email.direction") == Some("inbound")
                    && event.get("mimecast.to").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "mimecast.to", |event| {
                    event.append_unique(
                        "user.full_name",
                        json!(
                            event
                                .get("_ingest._value.displayableName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "58c3c69c6e46ce62cf9fa99177604ef57f1240f11d1f809c61734e2cbcde9f41",
                        r#"{"params":null,"known":[{"MailRelated":{"names":["mimecast.fromEnv.displayableName","mimecast.fromHdr.displayableName"],"addresses":["mimecast.operator","mimecast.fromEnv.emailAddress","mimecast.fromHdr.emailAddress"],"lists":["mimecast.to"],"lift":"mimecast.operator"}}]}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mimecast.spamProcessingDetail.verdict") {
                    event.rename(
                        "mimecast.spamProcessingDetail.verdict",
                        "mimecast.spamProcessingDetail.spamVerdict",
                    )?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("mimecast.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            if let Some(v) = event.get("mimecast.id").cloned() {
                event.set("event.id", v)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("mimecast.rejectReason")
                    && event.get_str("mimecast.rejectReason") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if let Some(v) = event
                .get("mimecast.rejectReason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event
                .get("mimecast.spamScore")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            event.rename("mimecast", "mimecast.message_release_logs")?;

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
                        "Processor '{}' {}failed with message '{}'",
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
