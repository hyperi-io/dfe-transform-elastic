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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
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

            event.remove("observer.hostname");
            event.remove("@timestamp");

            if event.has_value("cef.name") {
                event.rename("cef.name", "ping_federate.audit.event")?;
            }

            if let Some(v) = event
                .get("ping_federate.audit.event")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.event")
                    && (event.get_str("ping_federate.audit.event") == Some("SLO")
                        || event.get_str("ping_federate.audit.event") == Some("SSO")
                        || event.get_str("ping_federate.audit.event") == Some("OAuth")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_ATTEMPT")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_REQUEST"))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.event")
                    && (event.get_str("ping_federate.audit.event") == Some("AUTHN_SESSION_CREATED")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_SESSION_USED")
                        || event.get_str("ping_federate.audit.event")
                            == Some("AUTHN_SESSION_DELETED")
                        || event.get_str("ping_federate.audit.event") == Some("SRI_REVOKED"))
            };
            if _cond {
                event.append("event.category", json!("session"))?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.event")
                    && (event.get_str("ping_federate.audit.event") == Some("OAuth")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_SESSION_USED"))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.event")
                    && (event.get_str("ping_federate.audit.event") == Some("SSO")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_ATTEMPT")
                        || event.get_str("ping_federate.audit.event") == Some("AUTHN_REQUEST")
                        || event.get_str("ping_federate.audit.event")
                            == Some("AUTHN_SESSION_CREATED"))
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.event")
                    && (event.get_str("ping_federate.audit.event") == Some("SLO")
                        || event.get_str("ping_federate.audit.event") == Some("SRI_REVOKED")
                        || event.get_str("ping_federate.audit.event")
                            == Some("AUTHN_SESSION_DELETED"))
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = { event.get_str("ping_federate.audit.severity") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.severity") {
                        if let Some(val) = event.get("cef.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("ping_federate.audit.severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_severity_to_long",
                    )?;
                    if event.remove("ping_federate.audit.severity").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.audit.severity".into(),
                        });
                    }
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

            let _cond = { event.get_str("ping_federate.audit.host.ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.deviceHostName") {
                        if let Some(val) = event.get("cef.extensions.deviceHostName") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.deviceHostName".into(),
                                    message,
                                }
                            })?;
                            event.set("ping_federate.audit.host.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_deviceHostName_to_ip",
                    )?;
                    event.rename(
                        "cef.extensions.deviceHostName",
                        "ping_federate.audit.host.name",
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("ping_federate.audit.host.ip") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("ping_federate.audit.host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("ping_federate.audit.host.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("ping_federate.audit.host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("ping_federate.audit.host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("ping_federate.audit.host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("ping_federate.audit.host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
                    && !event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("ping_federate.audit.response_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
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
                        "date_ping_federate_audit_response_time",
                    )?;
                    if event.remove("ping_federate.audit.response_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.audit.response_time".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("ping_federate.audit.response_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
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
                        "date_ping_federate_audit_response_time_timezone",
                    )?;
                    if event.remove("ping_federate.audit.response_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.audit.response_time".into(),
                        });
                    }
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
                .get("ping_federate.audit.response_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.app", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("ping_federate.audit.app") {
                    if !uri_parts(event, "ping_federate.audit.app", "url", true, false)?
                        && event
                            .get_str("ping_federate.audit.app")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "ping_federate.audit.app".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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
                .get("ping_federate.audit.app")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString2")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.connection_id", v)?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.protocol", v)?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.local_user_id", v)?;
            }

            let _cond = { event.has_value("ping_federate.audit.local_user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ping_federate.audit.local_user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("cef.extensions.destinationUserId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.subject", v)?;
            }

            if let Some(v) = event
                .get("ping_federate.audit.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("ping_federate.audit.subject") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ping_federate.audit.subject")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString6")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.attributes", v)?;
            }

            if event.has_value("cef.extensions.externalId") {
                event.rename(
                    "cef.extensions.externalId",
                    "ping_federate.audit.tracking_id",
                )?;
            }

            if event.has_value("cef.extensions.message") {
                event.rename("cef.extensions.message", "ping_federate.audit.status")?;
            }

            let v = json!("unknown");
            if !painless_is_empty_value(&v) {
                event.set("event.outcome", v)?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.status")
                    && event
                        .get_str("ping_federate.audit.status")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.status")
                    && event
                        .get_str("ping_federate.audit.status")
                        .is_some_and(|s| s.to_lowercase().contains("fail"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if let Some(v) = event
                .get("cef.extensions.deviceCustomString4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("ping_federate.audit.role", v)?;
            }

            let _cond = {
                event.has_value("ping_federate.audit.role")
                    && event
                        .get("ping_federate.audit.role")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("ping_federate.audit.role") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("ping_federate.audit.role", Value::Array(parts))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event.remove("ping_federate.audit.role").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.audit.role".into(),
                        });
                    }
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

            let _cond = {
                event
                    .get("ping_federate.audit.role")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("ping_federate.audit.role").cloned();
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
                                let _cond = { event.has_value("ping_federate.audit.role") };
                                if _cond {
                                    event.append_unique(
                                        "user.roles",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
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
                                "ping_federate.audit.role",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("ping_federate.audit.role") && !event.has_value("user.roles") };
            if _cond {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("ping_federate.audit.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("ping_federate.audit.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.sourceAddress") {
                        if let Some(val) = event.get("cef.extensions.sourceAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.sourceAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("ping_federate.audit.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sourceAddress_to_ip",
                    )?;
                    if event.remove("ping_federate.audit.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.audit.ip".into(),
                        });
                    }
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
                .get("ping_federate.audit.ip")
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

            let _cond = { event.has_value("ping_federate.audit.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("ping_federate.audit.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("ping_federate.audit.event");
                event.remove("ping_federate.audit.subject");
                event.remove("ping_federate.audit.ip");
                event.remove("ping_federate.audit.app");
                event.remove("ping_federate.audit.host.ip");
                event.remove("ping_federate.audit.host.name");
                event.remove("ping_federate.audit.role");
                event.remove("ping_federate.audit.status");
                event.remove("ping_federate.audit.response_time");
                event.remove("ping_federate.audit.severity");
            }

            event.remove("cef");
            event.remove("destination.user.id");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
