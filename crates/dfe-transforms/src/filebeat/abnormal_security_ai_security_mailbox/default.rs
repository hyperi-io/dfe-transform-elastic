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
                if let Some(v) = event.get("json.campaignId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.judgementStatus") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastReported") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.messageId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("observer.vendor", json!("Abnormal"))?;

            event.set("observer.product", json!("Inbound Email Security"))?;

            if event.has_value("json.attackType") {
                event.rename(
                    "json.attackType",
                    "abnormal_security.ai_security_mailbox.attack.type",
                )?;
            }

            let _cond = { event.has_value("abnormal_security.ai_security_mailbox.attack.type") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.attack.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.campaignId") {
                event.rename(
                    "json.campaignId",
                    "abnormal_security.ai_security_mailbox.campaign_id",
                )?;
            }

            let _cond = {
                event.has_value("json.firstReported")
                    && event.get_str("json.firstReported") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstReported") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "abnormal_security.ai_security_mailbox.first_reported",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.firstReported".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstReported")?;
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

            if event.has_value("json.fromName") {
                event.rename(
                    "json.fromName",
                    "abnormal_security.ai_security_mailbox.from.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) =
                    event.get_string("abnormal_security.ai_security_mailbox.from.name")
                {
                    // Grok pattern: ^%{EMAILADDRESS:user.email}$
                    // Grok pattern: ^%{DATA:user.name}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{EMAILADDRESS:user.email}$"),
                            cached_grok!("^%{DATA:user.name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("abnormal_security.ai_security_mailbox.from.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.from.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.judgementStatus") {
                event.rename(
                    "json.judgementStatus",
                    "abnormal_security.ai_security_mailbox.judgement_status",
                )?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.kind", json!("enrichment"))?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            if event.has_value("json.fromAddress") {
                event.rename(
                    "json.fromAddress",
                    "abnormal_security.ai_security_mailbox.from.address",
                )?;
            }

            let _cond = { event.has_value("abnormal_security.ai_security_mailbox.from.address") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                if let Some(v) = event
                    .get("abnormal_security.ai_security_mailbox.from.address")
                    .cloned()
                {
                    event.set("threat.indicator.email.address", v)?;
                }
            }

            let _cond = {
                event.has_value("abnormal_security.ai_security_mailbox.judgement_status")
                    && event
                        .get_str("abnormal_security.ai_security_mailbox.judgement_status")
                        .is_some_and(|s| ["spam", "malicious"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                if let Some(v) = event
                    .get("abnormal_security.ai_security_mailbox.from.address")
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            let _cond = { event.has_value("abnormal_security.ai_security_mailbox.from.address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.lastReported")
                    && event.get_str("json.lastReported") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastReported") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "abnormal_security.ai_security_mailbox.last_reported",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastReported".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastReported")?;
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

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox.last_reported")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.messageId") {
                if let Some(val) = event.get("json.messageId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.messageId".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "abnormal_security.ai_security_mailbox.message_id",
                        converted,
                    )?;
                }
            }

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.overallStatus") {
                event.rename(
                    "json.overallStatus",
                    "abnormal_security.ai_security_mailbox.overall_status",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("abnormal_security.ai_security_mailbox.overall_status")
                    .is_some_and(|s| s.to_lowercase() == "no action needed"))
            };
            if _cond {
                if let Some(v) = event
                    .get("abnormal_security.ai_security_mailbox.overall_status")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("json.recipientAddress") {
                event.rename(
                    "json.recipientAddress",
                    "abnormal_security.ai_security_mailbox.recipient.address",
                )?;
            }

            let _cond =
                { event.has_value("abnormal_security.ai_security_mailbox.recipient.address") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.recipient.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("abnormal_security.ai_security_mailbox.recipient.address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.recipient.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.recipientName") {
                event.rename(
                    "json.recipientName",
                    "abnormal_security.ai_security_mailbox.recipient.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) =
                    event.get_string("abnormal_security.ai_security_mailbox.recipient.name")
                {
                    // Grok pattern: ^%{EMAILADDRESS:destination.user.email}$
                    // Grok pattern: ^%{DATA:destination.user.name}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{EMAILADDRESS:destination.user.email}$"),
                            cached_grok!("^%{DATA:destination.user.name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("abnormal_security.ai_security_mailbox.recipient.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.ai_security_mailbox.recipient.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.subject") {
                event.rename(
                    "json.subject",
                    "abnormal_security.ai_security_mailbox.subject",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.ai_security_mailbox.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
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
                event.remove("abnormal_security.ai_security_mailbox.attack.type");
                event.remove("abnormal_security.ai_security_mailbox.from.address");
                event.remove("abnormal_security.ai_security_mailbox.from.name");
                event.remove("abnormal_security.ai_security_mailbox.last_reported");
                event.remove("abnormal_security.ai_security_mailbox.message_id");
                event.remove("abnormal_security.ai_security_mailbox.recipient.address");
                event.remove("abnormal_security.ai_security_mailbox.recipient.name");
                event.remove("abnormal_security.ai_security_mailbox.subject");
                event.remove("abnormal_security.ai_security_mailbox.overall_status");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
