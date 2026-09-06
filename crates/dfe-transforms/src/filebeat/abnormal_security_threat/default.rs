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
                if let Some(v) = event.get("json.abxMessageIdStr") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.internetMessageId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.receivedTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.threatId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("indicator"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Abnormal"))?;

            event.set("observer.product", json!("Inbound Email Security"))?;

            if event.has_value("json.abxMessageIdStr") {
                event.rename(
                    "json.abxMessageIdStr",
                    "abnormal_security.threat.abx_message_id",
                )?;
            }

            let _cond = { !event.has_value("abnormal_security.threat.abx_message_id") };
            if _cond {
                if event.has_value("json.abxMessageId") {
                    if let Some(val) = event.get("json.abxMessageId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.abxMessageId".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.abx_message_id", converted)?;
                    }
                }
            }

            if let Some(v) = event
                .get("abnormal_security.threat.abx_message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.abxPortalUrl") {
                event.rename(
                    "json.abxPortalUrl",
                    "abnormal_security.threat.abx_portal_url",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.abx_portal_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.abx_portal_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.reference", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attachmentCount") {
                    if let Some(val) = event.get("json.attachmentCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attachmentCount".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.attachment_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attachmentCount_to_long",
                )?;
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

            if event.has_value("json.attachmentNames") {
                event.rename(
                    "json.attachmentNames",
                    "abnormal_security.threat.attachment_names",
                )?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("json.attachments") {
                    foreach_array(event, "json.attachments", |event| {
                        event.append_unique(
                            "abnormal_security.threat.attachment_names",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("abnormal_security.threat.attachment_names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def attachmentList = new ArrayList();\nfor (name in ctx.abnormal_security.threat.attachment_names) {\n  def attachment = new HashMap();\n  attachment.put('file', new HashMap());\n  attachment.file.put('name', name);\n  String[] tokenList = name.splitOnToken('.');\n  if(tokenList.length > 1){\n    attachment.file.put('extension', tokenList[tokenList.length - 1]);\n  }\n  attachmentList.add(attachment);\n}\nctx.put('email',new HashMap());\nctx.email.attachments = attachmentList;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def attachmentList = new ArrayList();\nfor (name in ctx.abnormal_security.threat.attachment_names) {\n  def attachment = new HashMap();\n  attachment.put('file', new HashMap());\n  attachment.file.put('name', name);\n  String[] tokenList = name.splitOnToken('.');\n  if(tokenList.length > 1){\n    attachment.file.put('extension', tokenList[tokenList.length - 1]);\n  }\n  attachmentList.add(attachment);\n}\nctx.put('email',new HashMap());\nctx.email.attachments = attachmentList;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_email_attachments_field",
                    )?;
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

            if event.has_value("json.links") {
                event.rename("json.links", "abnormal_security.threat.links")?;
            }

            if event.has_value("json.attackStrategy") {
                event.rename(
                    "json.attackStrategy",
                    "abnormal_security.threat.attack.strategy",
                )?;
            }

            let _cond = { event.has_value("abnormal_security.threat.attack.strategy") };
            if _cond {
                event.append_unique(
                    "threat.technique.name",
                    json!(
                        event
                            .get("abnormal_security.threat.attack.strategy")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attackType") {
                event.rename("json.attackType", "abnormal_security.threat.attack.type")?;
            }

            let _cond = { event.has_value("abnormal_security.threat.attack.type") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("abnormal_security.threat.attack.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attackVector") {
                event.rename(
                    "json.attackVector",
                    "abnormal_security.threat.attack.vector",
                )?;
            }

            if event.has_value("json.attackedParty") {
                event.rename(
                    "json.attackedParty",
                    "abnormal_security.threat.attacked_party",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.autoRemediated") {
                    if let Some(val) = event.get("json.autoRemediated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.autoRemediated".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.auto_remediated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_autoRemediated_to_boolean",
                )?;
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

            if event.has_value("json.ccEmails") {
                event.rename("json.ccEmails", "abnormal_security.threat.cc_emails")?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.cc_emails")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.cc.address", v)?;
            }

            let _cond = {
                event
                    .get("abnormal_security.threat.cc_emails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "abnormal_security.threat.cc_emails", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.fromAddress") {
                event.rename("json.fromAddress", "abnormal_security.threat.from_address")?;
            }

            let _cond = { event.has_value("abnormal_security.threat.from_address") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("abnormal_security.threat.from_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.from_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.email.address", v)?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.from_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = { event.has_value("abnormal_security.threat.from_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.threat.from_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set("threat.indicator.type", json!("email-addr"))?;

            if event.has_value("json.fromName") {
                event.rename("json.fromName", "abnormal_security.threat.from_name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("abnormal_security.threat.from_name") {
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

            let _cond = { event.has_value("abnormal_security.threat.from_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.threat.from_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.threatId") {
                event.rename("json.threatId", "abnormal_security.threat.id")?;
            }

            if event.has_value("json.impersonatedParty") {
                event.rename(
                    "json.impersonatedParty",
                    "abnormal_security.threat.impersonated_party",
                )?;
            }

            if event.has_value("json.internetMessageId") {
                event.rename(
                    "json.internetMessageId",
                    "abnormal_security.threat.internet_message_id",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.internet_message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.isRead") {
                    if let Some(val) = event.get("json.isRead") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isRead".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.is_read", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isRead_to_boolean",
                )?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.postRemediated") {
                    if let Some(val) = event.get("json.postRemediated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.postRemediated".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.post_remediated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_postRemediated_to_boolean",
                )?;
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

            let _cond = {
                event.has_value("json.receivedTime")
                    && event.get_str("json.receivedTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.receivedTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("abnormal_security.threat.received_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.receivedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_receivedTime")?;
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
                .get("abnormal_security.threat.received_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.received_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.delivery_timestamp", v)?;
            }

            if event.has_value("json.recipientAddress") {
                event.rename(
                    "json.recipientAddress",
                    "abnormal_security.threat.recipient_address",
                )?;
            }

            let _cond = { event.has_value("abnormal_security.threat.recipient_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.threat.recipient_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.remediationStatus") {
                event.rename(
                    "json.remediationStatus",
                    "abnormal_security.threat.remediation_status",
                )?;
            }

            let _cond = {
                event.has_value("json.remediationTimestamp")
                    && event.get_str("json.remediationTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.remediationTimestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("abnormal_security.threat.remediation_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.remediationTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_remediationTimestamp",
                    )?;
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

            if event.has_value("json.replyToEmails") {
                event.rename(
                    "json.replyToEmails",
                    "abnormal_security.threat.reply_to_emails",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.reply_to_emails")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.reply_to.address", v)?;
            }

            let _cond = {
                event
                    .get("abnormal_security.threat.reply_to_emails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "abnormal_security.threat.reply_to_emails", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.returnPath") {
                event.rename("json.returnPath", "abnormal_security.threat.return_path")?;
            }

            let _cond = { event.has_value("abnormal_security.threat.return_path") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("abnormal_security.threat.return_path")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.senderDomain") {
                event.rename(
                    "json.senderDomain",
                    "abnormal_security.threat.sender_domain",
                )?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.sender_domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("abnormal_security.threat.sender_domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("abnormal_security.threat.sender_domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.senderIpAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.senderIpAddress") {
                        if let Some(val) = event.get("json.senderIpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.senderIpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("abnormal_security.threat.sender_ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_senderIpAddress_to_ip",
                    )?;
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
                .get("abnormal_security.threat.sender_ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("abnormal_security.threat.sender_ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("abnormal_security.threat.sender_ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.sentTime") && event.get_str("json.sentTime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.sentTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("abnormal_security.threat.sent_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.sentTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_sentTime")?;
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
                .get("abnormal_security.threat.sent_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.origination_timestamp", v)?;
            }

            if event.has_value("json.subject") {
                event.rename("json.subject", "abnormal_security.threat.subject")?;
            }

            if let Some(v) = event
                .get("abnormal_security.threat.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if event.has_value("json.summaryInsights") {
                event.rename(
                    "json.summaryInsights",
                    "abnormal_security.threat.summary_insights",
                )?;
            }

            let _cond = { event.get("json.toAddresses").is_some_and(|v| v.is_string()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.toAddresses") {
                        if let Some(s) = event.get_string("json.toAddresses") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set(
                                "abnormal_security.threat.to_addresses",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.get("json.toAddresses").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("json.toAddresses") {
                    event.rename("json.toAddresses", "abnormal_security.threat.to_addresses")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("abnormal_security.threat.to_addresses") {
                    map_strings(
                        event,
                        "abnormal_security.threat.to_addresses",
                        "abnormal_security.threat.to_addresses",
                        |s| s.trim().to_string(),
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "trim")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "trim_threat_to_addresses",
                )?;
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

            if let Some(v) = event
                .get("abnormal_security.threat.to_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.to.address", v)?;
            }

            let _cond = {
                event
                    .get("abnormal_security.threat.to_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "abnormal_security.threat.to_addresses", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.urlCount") {
                    if let Some(val) = event.get("json.urlCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.urlCount".into(),
                                message,
                            }
                        })?;
                        event.set("abnormal_security.threat.url_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_urlCount_to_long",
                )?;
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

            if event.has_value("json.urls") {
                event.rename("json.urls", "abnormal_security.threat.urls")?;
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
                event.remove("abnormal_security.threat.abx_message_id");
                event.remove("abnormal_security.threat.abx_portal_url");
                event.remove("abnormal_security.threat.attack.type");
                event.remove("abnormal_security.threat.from_address");
                event.remove("abnormal_security.threat.from_name");
                event.remove("abnormal_security.threat.internet_message_id");
                event.remove("abnormal_security.threat.received_time");
                event.remove("abnormal_security.threat.sender_domain");
                event.remove("abnormal_security.threat.sent_time");
                event.remove("abnormal_security.threat.subject");
                event.remove("abnormal_security.threat.attachment_names");
                event.remove("abnormal_security.threat.cc_emails");
                event.remove("abnormal_security.threat.reply_to_emails");
                event.remove("abnormal_security.threat.to_addresses");
                event.remove("abnormal_security.threat.sender_ip_address");
                event.remove("abnormal_security.threat.attack.strategy");
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
