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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("observer.vendor", json!("Abnormal"))?;

            event.set("observer.product", json!("Inbound Email Security"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.kind", json!("event"))?;

            if event.has_value("json.abx_message_id") {
                if let Some(val) = event.get("json.abx_message_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.abx_message_id".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "abnormal_security.ai_security_mailbox_not_analyzed.abx_message_id",
                        converted,
                    )?;
                }
            }

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox_not_analyzed.abx_message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.reported_datetime") {
                event.rename(
                    "json.reported_datetime",
                    "abnormal_security.ai_security_mailbox_not_analyzed.reported_time",
                )?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox_not_analyzed.reported_time")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox_not_analyzed.reported_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "abnormal_security.ai_security_mailbox_not_analyzed.reported_time",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "abnormal_security.ai_security_mailbox_not_analyzed.reported_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_reported_datetime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.subject") {
                event.rename(
                    "json.subject",
                    "abnormal_security.ai_security_mailbox_not_analyzed.subject",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox_not_analyzed.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            let _cond = {
                event.has_value("json.recipient.email")
                    && event.get_str("json.recipient.email") != Some("")
            };
            if _cond {
                event.rename(
                    "json.recipient.email",
                    "abnormal_security.ai_security_mailbox_not_analyzed.recipient.address",
                )?;
            }

            let _cond = {
                event.has_value(
                    "abnormal_security.ai_security_mailbox_not_analyzed.recipient.address",
                )
            };
            if _cond {
                event.append_unique("email.to.address", json!(event.get("abnormal_security.ai_security_mailbox_not_analyzed.recipient.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "abnormal_security.ai_security_mailbox_not_analyzed.recipient.address",
                )
            };
            if _cond {
                event.append_unique("related.user", json!(event.get("abnormal_security.ai_security_mailbox_not_analyzed.recipient.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("json.recipient.name")
                    && event.get_str("json.recipient.name") != Some("")
            };
            if _cond {
                event.rename(
                    "json.recipient.name",
                    "abnormal_security.ai_security_mailbox_not_analyzed.recipient.name",
                )?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox_not_analyzed.recipient.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get(
                                "abnormal_security.ai_security_mailbox_not_analyzed.recipient.name"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.reporter.email")
                    && event.get_str("json.reporter.email") != Some("")
            };
            if _cond {
                event.rename(
                    "json.reporter.email",
                    "abnormal_security.ai_security_mailbox_not_analyzed.reporter.address",
                )?;
            }

            let _cond = {
                event.has_value(
                    "abnormal_security.ai_security_mailbox_not_analyzed.reporter.address",
                )
            };
            if _cond {
                event.append_unique("related.user", json!(event.get("abnormal_security.ai_security_mailbox_not_analyzed.reporter.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("json.reporter.name")
                    && event.get_str("json.reporter.name") != Some("")
            };
            if _cond {
                event.rename(
                    "json.reporter.name",
                    "abnormal_security.ai_security_mailbox_not_analyzed.reporter.name",
                )?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox_not_analyzed.reporter.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox_not_analyzed.reporter.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.not_analyzed_reason")
                    && event.get_str("json.not_analyzed_reason") != Some("")
            };
            if _cond {
                event.rename(
                    "json.not_analyzed_reason",
                    "abnormal_security.ai_security_mailbox_not_analyzed.reason",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox_not_analyzed.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

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
                event.remove("abnormal_security.ai_security_mailbox_not_analyzed.reason");
                event
                    .remove("abnormal_security.ai_security_mailbox_not_analyzed.recipient.address");
                event.remove("abnormal_security.ai_security_mailbox_not_analyzed.reported_time");
                event.remove("abnormal_security.ai_security_mailbox_not_analyzed.subject");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
