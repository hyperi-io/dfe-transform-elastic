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

            event.set("ecs.version", json!("8.16.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            event.append("event.type", json!("info"))?;

            event.set("event.kind", json!("alert"))?;

            let _cond = { event.get_str("json.source") == Some("Gmail phishing") };
            if _cond {
                event.append("event.category", json!("email"))?;
                event.append("event.category", json!("threat"))?;
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.get_str("json.source") != Some("Gmail phishing") };
            if _cond {
                event.append("event.category", json!("threat"))?;
                event.append("event.category", json!("malware"))?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.alertId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.createTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.customerId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.endTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.startTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.updateTime") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.createTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("google_workspace.alert.create_time", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.endTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.endTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("google_workspace.alert.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.endTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.end_time").cloned() {
                    event.set("event.end", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.startTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.startTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.startTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.start_time").cloned() {
                    event.set("event.start", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.source") {
                event.rename("json.source", "google_workspace.alert.source")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.source").cloned() {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.customerId") {
                event.rename("json.customerId", "google_workspace.alert.customer.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.customer.id").cloned() {
                    event.set("organization.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.metadata.assignee") {
                event.rename(
                    "json.metadata.assignee",
                    "google_workspace.alert.metadata.assignee",
                )?;
            }

            let _cond = { event.has_value("google_workspace.alert.metadata.assignee") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.email",
                        json!(
                            event
                                .get("google_workspace.alert.metadata.assignee")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.alertId") {
                event.rename("json.alertId", "google_workspace.alert.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.deleted") {
                    if let Some(val) = event.get("json.deleted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deleted".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.alert.deleted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "google_workspace.alert.etag")?;
            }

            if event.has_value("json.metadata.alertId") {
                event.rename(
                    "json.metadata.alertId",
                    "google_workspace.alert.metadata.alert.id",
                )?;
            }

            if event.has_value("json.metadata.customerId") {
                event.rename(
                    "json.metadata.customerId",
                    "google_workspace.alert.metadata.customer.id",
                )?;
            }

            if event.has_value("json.data.@type") {
                event.rename("json.data.@type", "google_workspace.alert.data.type")?;
            }

            if event.has_value("json.metadata.etag") {
                event.rename("json.metadata.etag", "google_workspace.alert.metadata.etag")?;
            }

            if event.has_value("json.metadata.severity") {
                event.rename(
                    "json.metadata.severity",
                    "google_workspace.alert.metadata.severity",
                )?;
            }

            if event.has_value("json.metadata.status") {
                event.rename(
                    "json.metadata.status",
                    "google_workspace.alert.metadata.status",
                )?;
            }

            let _cond = { event.has_value("json.metadata.updateTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.metadata.updateTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.metadata.update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.metadata.updateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.securityInvestigationToolLink") {
                event.rename(
                    "json.securityInvestigationToolLink",
                    "google_workspace.alert.security_investigation_tool_link",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "google_workspace.alert.type")?;
            }

            let _cond = { event.has_value("json.updateTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updateTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.data.email") {
                event.rename("json.data.email", "google_workspace.alert.data.email")?;
            }

            let _cond = { event.has_value("google_workspace.alert.data.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.email",
                        json!(
                            event
                                .get("google_workspace.alert.data.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.data.alertDetails") {
                event.rename(
                    "json.data.alertDetails",
                    "google_workspace.alert.data.alert_details",
                )?;
            }

            if event.has_value("json.data.takeoutRequestId") {
                event.rename(
                    "json.data.takeoutRequestId",
                    "google_workspace.alert.data.takeout.request.id",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.attachmentsSha256Hash") {
                            event.rename(
                                "_ingest._value.attachmentsSha256Hash",
                                "_ingest._value.attachments_sha256_hash",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.data.messages").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.date")
                                    {
                                        match parse_date_out(
                                            &date_str,
                                            &[
                                                "ISO8601",
                                                "yyyy-MM-dd'T'HH:mm:ss",
                                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                                "yyyy/MM/dd HH:mm:ss z",
                                            ],
                                            Some("UTC"),
                                            None,
                                        ) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.date", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.date".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.data.messages",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        event.append_unique(
                            "email.delivery_timestamp",
                            json!(
                                event
                                    .get("_ingest._value.date")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.md5HashSubject") {
                            event.rename(
                                "_ingest._value.md5HashSubject",
                                "_ingest._value.md5.hash.subject",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.messageBodySnippet") {
                            event.rename(
                                "_ingest._value.messageBodySnippet",
                                "_ingest._value.message_body_snippet",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.md5HashMessageBody") {
                            event.rename(
                                "_ingest._value.md5HashMessageBody",
                                "_ingest._value.md5.hash.message_body",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.messageId") {
                            event.rename("_ingest._value.messageId", "_ingest._value.id")?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        event.append_unique(
                            "email.message_id",
                            json!(
                                event
                                    .get("_ingest._value.id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.recipient") {
                            event.rename(
                                "_ingest._value.recipient",
                                "_ingest._value.recipient_email",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value.recipient_email")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        if event.has_value("_ingest._value.subjectText") {
                            event.rename(
                                "_ingest._value.subjectText",
                                "_ingest._value.subject_text",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.messages") {
                    foreach_array(event, "json.data.messages", |event| {
                        event.append_unique(
                            "email.subject",
                            json!(
                                event
                                    .get("_ingest._value.subject_text")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.messages") {
                event.rename("json.data.messages", "google_workspace.alert.data.messages")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("google_workspace.alert.data.messages") {
                    foreach_array(event, "google_workspace.alert.data.messages", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.attachments_sha256_hash") {
                                foreach_array(
                                    event,
                                    "_ingest._value.attachments_sha256_hash",
                                    |event| {
                                        // ignore_failure: true
                                        let _ =
                                            (|| -> Result<()> {
                                                event.append_unique(
                                                    "email.attachments.file.hash.sha256",
                                                    json!(event.get("_ingest._value").map_or_else(
                                                        String::new,
                                                        template_to_string
                                                    )),
                                                )?;
                                                Ok(())
                                            })();
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("google_workspace.alert.data.messages") {
                    foreach_array(event, "google_workspace.alert.data.messages", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.md5.hash.subject")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.maliciousEntity.entity.emailAddress") {
                event.rename(
                    "json.data.maliciousEntity.entity.emailAddress",
                    "google_workspace.alert.data.malicious_entity.entity.email_address",
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.alert.data.malicious_entity.entity.email_address")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("user.email", json!(event.get("google_workspace.alert.data.malicious_entity.entity.email_address").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })();
            }

            if event.has_value("json.data.maliciousEntity.entity.displayName") {
                event.rename(
                    "json.data.maliciousEntity.entity.displayName",
                    "google_workspace.alert.data.malicious_entity.entity.display_name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("google_workspace.alert.data.malicious_entity.entity.display_name")
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.data.domainId.customerPrimaryDomain") {
                event.rename(
                    "json.data.domainId.customerPrimaryDomain",
                    "google_workspace.alert.data.domain_id.customer_primary_domain",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("google_workspace.alert.data.domain_id.customer_primary_domain")
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.data.maliciousEntity.displayName") {
                event.rename(
                    "json.data.maliciousEntity.displayName",
                    "google_workspace.alert.data.malicious_entity.display_name",
                )?;
            }

            if event.has_value("json.data.maliciousEntity.fromHeader") {
                event.rename(
                    "json.data.maliciousEntity.fromHeader",
                    "google_workspace.alert.data.malicious_entity.from_header",
                )?;
            }

            if event.has_value("json.data.systemActionType") {
                event.rename(
                    "json.data.systemActionType",
                    "google_workspace.alert.data.system_action_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.isInternal") {
                    if let Some(val) = event.get("json.data.isInternal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.isInternal".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.alert.data.is_internal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.data.sourceIp") {
                    if let Some(val) = event.get("json.data.sourceIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.sourceIp".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.alert.data.source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
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

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.data.source.ip").cloned() {
                    event.set("source.ip", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.loginDetails.ipAddress") {
                    if let Some(val) = event.get("json.data.loginDetails.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.loginDetails.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.alert.data.login_details.ip_address",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.data.loginDetails.loginTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.loginDetails.loginTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "google_workspace.alert.data.login_details.login_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.loginDetails.loginTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.data.state") {
                event.rename("json.data.state", "google_workspace.alert.data.state")?;
            }

            if event.has_value("json.data.appealWindow") {
                event.rename(
                    "json.data.appealWindow",
                    "google_workspace.alert.data.appeal_window",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.suspensionDetails") {
                    foreach_array(event, "json.data.suspensionDetails", |event| {
                        if event.has_value("_ingest._value.abuseReason") {
                            event.rename(
                                "_ingest._value.abuseReason",
                                "_ingest._value.abuse_reason",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.suspensionDetails") {
                    foreach_array(event, "json.data.suspensionDetails", |event| {
                        if event.has_value("_ingest._value.productName") {
                            event.rename(
                                "_ingest._value.productName",
                                "_ingest._value.product_name",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.suspensionDetails") {
                event.rename(
                    "json.data.suspensionDetails",
                    "google_workspace.alert.data.suspension_details",
                )?;
            }

            if event.has_value("json.data.affectedUserEmails") {
                event.rename(
                    "json.data.affectedUserEmails",
                    "google_workspace.alert.data.affected.user_emails",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("google_workspace.alert.data.affected.user_emails") {
                    foreach_array(
                        event,
                        "google_workspace.alert.data.affected.user_emails",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "user.email",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                Ok(())
            })();

            if event.has_value("json.data.title") {
                event.rename("json.data.title", "google_workspace.alert.data.title")?;
            }

            let _cond = { event.get_str("event.action") == Some("Google Operations") };
            if _cond {
                if event.has_value("json.data.description") {
                    event.rename(
                        "json.data.description",
                        "google_workspace.alert.data.description",
                    )?;
                }
            }

            if event.has_value("json.data.attachmentData.csv.headers") {
                event.rename(
                    "json.data.attachmentData.csv.headers",
                    "google_workspace.alert.data.attachment.data.csv.headers",
                )?;
            }

            if event.has_value("json.data.attachmentData.csv.dataRows") {
                event.rename(
                    "json.data.attachmentData.csv.dataRows",
                    "google_workspace.alert.data.attachment.data.csv.data_rows",
                )?;
            }

            if event.has_value("json.data.header") {
                event.rename("json.data.header", "google_workspace.alert.data.header")?;
            }

            if event.has_value("json.data.domain") {
                event.rename("json.data.domain", "google_workspace.alert.data.domain")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.data.domain").cloned() {
                    event.set("user.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.deviceId") {
                            event.rename("_ingest._value.deviceId", "_ingest._value.device.id")?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.serialNumber") {
                            event.rename(
                                "_ingest._value.serialNumber",
                                "_ingest._value.serial.number",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.deviceType") {
                            event.rename(
                                "_ingest._value.deviceType",
                                "_ingest._value.device.type",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.deviceModel") {
                            event.rename(
                                "_ingest._value.deviceModel",
                                "_ingest._value.device.model",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.resourceId") {
                            event.rename(
                                "_ingest._value.resourceId",
                                "_ingest._value.resource.id",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.iosVendorId") {
                            event.rename(
                                "_ingest._value.iosVendorId",
                                "_ingest._value.ios_vendor.id",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.deviceCompromisedState") {
                            event.rename(
                                "_ingest._value.deviceCompromisedState",
                                "_ingest._value.device_compromised_state",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.deviceProperty") {
                            event.rename(
                                "_ingest._value.deviceProperty",
                                "_ingest._value.device.property",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.oldValue") {
                            event.rename("_ingest._value.oldValue", "_ingest._value.old_value")?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.events") {
                    foreach_array(event, "json.data.events", |event| {
                        if event.has_value("_ingest._value.newValue") {
                            event.rename("_ingest._value.newValue", "_ingest._value.new_value")?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.events") {
                event.rename("json.data.events", "google_workspace.alert.data.events")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.requestInfo") {
                    foreach_array(event, "json.data.requestInfo", |event| {
                        if event.has_value("_ingest._value.appKey") {
                            event.rename("_ingest._value.appKey", "_ingest._value.app.key")?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.requestInfo") {
                    foreach_array(event, "json.data.requestInfo", |event| {
                        if event.has_value("_ingest._value.appDeveloperEmail") {
                            event.rename(
                                "_ingest._value.appDeveloperEmail",
                                "_ingest._value.app.developer_email",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.requestInfo") {
                    foreach_array(event, "json.data.requestInfo", |event| {
                        if event.has_value("_ingest._value.numberOfRequests") {
                            event.rename(
                                "_ingest._value.numberOfRequests",
                                "_ingest._value.number_of_requests",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.requestInfo") {
                event.rename(
                    "json.data.requestInfo",
                    "google_workspace.alert.data.request.info",
                )?;
            }

            let _cond = { event.get_str("event.action") == Some("Security Center rules") };
            if _cond {
                if event.has_value("json.data.description") {
                    event.rename(
                        "json.data.description",
                        "google_workspace.alert.data.rule_description",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("google_workspace.alert.data.rule_description")
                    .cloned()
                {
                    event.set("rule.description", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.data.name") {
                event.rename("json.data.name", "google_workspace.alert.data.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("google_workspace.alert.data.name").cloned() {
                    event.set("rule.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.data.displayName") {
                event.rename(
                    "json.data.displayName",
                    "google_workspace.alert.data.display.name",
                )?;
            }

            if event.has_value("json.data.windowSize") {
                event.rename(
                    "json.data.windowSize",
                    "google_workspace.alert.data.window_size",
                )?;
            }

            if event.has_value("json.data.threshold") {
                event.rename(
                    "json.data.threshold",
                    "google_workspace.alert.data.threshold",
                )?;
            }

            let _cond = { event.has_value("json.data.createTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.createTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.data.create_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.createTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.data.updateTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.updateTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.data.update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.updateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.data.triggerSource") {
                event.rename(
                    "json.data.triggerSource",
                    "google_workspace.alert.data.trigger.source",
                )?;
            }

            if event.has_value("json.data.supersededAlerts") {
                event.rename(
                    "json.data.supersededAlerts",
                    "google_workspace.alert.data.superseded_alerts",
                )?;
            }

            if event.has_value("json.data.supersedingAlert") {
                event.rename(
                    "json.data.supersedingAlert",
                    "google_workspace.alert.data.superseding_alert",
                )?;
            }

            if event.has_value("json.data.actionNames") {
                event.rename(
                    "json.data.actionNames",
                    "google_workspace.alert.data.action.name",
                )?;
            }

            if event.has_value("json.data.query") {
                event.rename("json.data.query", "google_workspace.alert.data.query")?;
            }

            if event.has_value("json.data.ruleViolationInfo.ruleInfo.displayName") {
                event.rename(
                    "json.data.ruleViolationInfo.ruleInfo.displayName",
                    "google_workspace.alert.data.rule.violation_info.rule_info.display.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("google_workspace.alert.data.rule.violation_info.rule_info.display.name")
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.data.ruleViolationInfo.ruleInfo.resourceName") {
                event.rename(
                    "json.data.ruleViolationInfo.ruleInfo.resourceName",
                    "google_workspace.alert.data.rule.violation_info.rule_info.resource.name",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.dataSource") {
                event.rename(
                    "json.data.ruleViolationInfo.dataSource",
                    "google_workspace.alert.data.rule.violation_info.data.source",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.trigger") {
                event.rename(
                    "json.data.ruleViolationInfo.trigger",
                    "google_workspace.alert.data.rule.violation_info.trigger.value",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.triggeringUserEmail") {
                event.rename(
                    "json.data.ruleViolationInfo.triggeringUserEmail",
                    "google_workspace.alert.data.rule.violation_info.trigger.user.email",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.recipients") {
                event.rename(
                    "json.data.ruleViolationInfo.recipients",
                    "google_workspace.alert.data.rule.violation_info.recipients",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.resourceInfo.resourceTitle") {
                event.rename(
                    "json.data.ruleViolationInfo.resourceInfo.resourceTitle",
                    "google_workspace.alert.data.rule.violation_info.resource_info.resource.title",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.resourceInfo.documentId") {
                event.rename(
                    "json.data.ruleViolationInfo.resourceInfo.documentId",
                    "google_workspace.alert.data.rule.violation_info.resource_info.document.id",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.ruleViolationInfo.matchInfo") {
                    foreach_array(event, "json.data.ruleViolationInfo.matchInfo", |event| {
                        if event.has_value("_ingest._value.userDefinedDetector.resourceName") {
                            event.rename(
                                "_ingest._value.userDefinedDetector.resourceName",
                                "_ingest._value.user_defined_detector.resource.name",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.ruleViolationInfo.matchInfo") {
                    foreach_array(event, "json.data.ruleViolationInfo.matchInfo", |event| {
                        if event.has_value("_ingest._value.userDefinedDetector.displayName") {
                            event.rename(
                                "_ingest._value.userDefinedDetector.displayName",
                                "_ingest._value.user_defined_detector.display.name",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.data.ruleViolationInfo.matchInfo") {
                    foreach_array(event, "json.data.ruleViolationInfo.matchInfo", |event| {
                        if event.has_value("_ingest._value.predefinedDetector.detectorName") {
                            event.rename(
                                "_ingest._value.predefinedDetector.detectorName",
                                "_ingest._value.predefined_detector.name",
                            )?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.data.ruleViolationInfo.matchInfo") {
                event.rename(
                    "json.data.ruleViolationInfo.matchInfo",
                    "google_workspace.alert.data.rule.violation_info.match_info",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.triggeredActionTypes") {
                event.rename(
                    "json.data.ruleViolationInfo.triggeredActionTypes",
                    "google_workspace.alert.data.rule.violation_info.triggered.action.types",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.triggeredActionInfo") {
                event.rename(
                    "json.data.ruleViolationInfo.triggeredActionInfo",
                    "google_workspace.alert.data.rule.violation_info.triggered.action.info",
                )?;
            }

            if event.has_value("json.data.ruleViolationInfo.suppressedActionTypes") {
                event.rename(
                    "json.data.ruleViolationInfo.suppressedActionTypes",
                    "google_workspace.alert.data.rule.violation_info.suppressed.action.types",
                )?;
            }

            if event.has_value("json.data.products") {
                event.rename("json.data.products", "google_workspace.alert.data.products")?;
            }

            let _cond = { event.has_value("json.data.nextUpdateTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.nextUpdateTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.data.next_update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.nextUpdateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.data.resolutionTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.resolutionTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.data.resolution_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.resolutionTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.data.dashboardUri") {
                event.rename(
                    "json.data.dashboardUri",
                    "google_workspace.alert.data.dashboard.uri",
                )?;
            }

            if event.has_value("json.data.status") {
                event.rename("json.data.status", "google_workspace.alert.data.status")?;
            }

            if event.has_value("json.data.incidentTrackingId") {
                event.rename(
                    "json.data.incidentTrackingId",
                    "google_workspace.alert.data.incident_tracking.id",
                )?;
            }

            if event.has_value("json.data.mergeInfo.newIncidentTrackingId") {
                event.rename(
                    "json.data.mergeInfo.newIncidentTrackingId",
                    "google_workspace.alert.data.merge_info.new_incident_tracking.id",
                )?;
            }

            if event.has_value("json.data.mergeInfo.newAlertId") {
                event.rename(
                    "json.data.mergeInfo.newAlertId",
                    "google_workspace.alert.data.merge_info.new_alert.id",
                )?;
            }

            if event.has_value("json.data.actorEmail") {
                event.rename(
                    "json.data.actorEmail",
                    "google_workspace.alert.data.actor.email",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("google_workspace.alert.data.actor.email")
                    .cloned()
                {
                    event.set("source.user.email", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.data.eventTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.eventTime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.alert.data.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.eventTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.data.primaryAdminChangedEvent.domain") {
                event.rename(
                    "json.data.primaryAdminChangedEvent.domain",
                    "google_workspace.alert.data.primary.admin.changed_event.domain",
                )?;
            }

            if event.has_value("json.data.primaryAdminChangedEvent.previousAdminEmail") {
                event.rename(
                    "json.data.primaryAdminChangedEvent.previousAdminEmail",
                    "google_workspace.alert.data.primary.admin.changed_event.previous_admin_email",
                )?;
            }

            if event.has_value("json.data.primaryAdminChangedEvent.updatedAdminEmail") {
                event.rename(
                    "json.data.primaryAdminChangedEvent.updatedAdminEmail",
                    "google_workspace.alert.data.primary.admin.changed_event.updated_admin_email",
                )?;
            }

            if event.has_value("json.data.ssoProfileCreatedEvent.inboundSsoProfileName") {
                event.rename("json.data.ssoProfileCreatedEvent.inboundSsoProfileName", "google_workspace.alert.data.sso_profile.created_event.inbound_sso.profile_name")?;
            }

            if event.has_value("json.data.ssoProfileUpdatedEvent.inboundSsoProfileName") {
                event.rename("json.data.ssoProfileUpdatedEvent.inboundSsoProfileName", "google_workspace.alert.data.sso_profile.updated_event.inbound_sso.profile_name")?;
            }

            if event.has_value("json.data.ssoProfileUpdatedEvent.inboundSsoProfileChanges") {
                event.rename("json.data.ssoProfileUpdatedEvent.inboundSsoProfileChanges", "google_workspace.alert.data.sso_profile.updated_event.inbound_sso.profile_changes")?;
            }

            if event.has_value("json.data.ssoProfileDeletedEvent.inboundSsoProfileName") {
                event.rename("json.data.ssoProfileDeletedEvent.inboundSsoProfileName", "google_workspace.alert.data.sso_profile.deleted_event.inbound_sso.profile_name")?;
            }

            if event.has_value("json.data.superAdminPasswordResetEvent.userEmail") {
                event.rename(
                    "json.data.superAdminPasswordResetEvent.userEmail",
                    "google_workspace.alert.data.super_admin_password_reset_event.user.email",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_workspace.alert.data.login_details.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("email.attachments.file.hash.sha256") {
                    foreach_array(event, "email.attachments.file.hash.sha256", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("google_workspace.alert.data.messages.md5.hash.message_body") {
                    foreach_array(
                        event,
                        "google_workspace.alert.data.messages.md5.hash.message_body",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                Ok(())
            })();

            event.remove("json");

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("google_workspace.alert.create_time");
                    event.remove("google_workspace.alert.customer.id");
                    event.remove("google_workspace.alert.data.actor.email");
                    event.remove("google_workspace.alert.data.affected.user_emails");
                    event.remove("google_workspace.alert.data.rule_description");
                    event.remove("google_workspace.alert.data.email");
                    event
                        .remove("google_workspace.alert.data.malicious_entity.entity.display_name");
                    event.remove(
                        "google_workspace.alert.data.malicious_entity.entity.email_address",
                    );
                    event.remove("google_workspace.alert.data.name");
                    event.remove(
                        "google_workspace.alert.data.rule.violation_info.rule_info.display.name",
                    );
                    event.remove(
                        "google_workspace.alert.data.rule.violation_info.trigger.user.email",
                    );
                    event.remove("google_workspace.alert.data.source.ip");
                    event.remove("google_workspace.alert.end_time");
                    event.remove("google_workspace.alert.id");
                    event.remove("google_workspace.alert.metadata.assignee");
                    event.remove("google_workspace.alert.source");
                    event.remove("google_workspace.alert.start_time");
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("google_workspace.alert.data.messages") {
                    foreach_array(event, "google_workspace.alert.data.messages", |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.attachments_sha256_hash");
                                event.remove("_ingest._value.subject_text");
                                event.remove("_ingest._value.date");
                                event.remove("_ingest._value.id");
                                event.remove("_ingest._value.recipient_email");
                                Ok(())
                            })();
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
