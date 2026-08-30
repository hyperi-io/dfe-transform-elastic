// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_account_scoring` pipeline.
pub struct PipelineAccountScoring;

impl Transform for PipelineAccountScoring {
    fn name(&self) -> &str {
        "pipeline_account_scoring"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if event.has_value("json.tags") {
                    event.rename("json.tags", "vectra_detect.log.tags")?;
                }

            let _cond = { event.has_value("vectra_detect.log.tags") };
            if _cond {
                event.append_unique("tags", json!(event.get("vectra_detect.log.tags").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.href") {
                    event.rename("json.href", "vectra_detect.log.href")?;
                }

            if let Some(v) = event.get("vectra_detect.log.href").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("event.reference") };
            if _cond {
                uri_parts(event, "event.reference", "url", true, false)?;
            }

            let _cond = { event.get_str("json.account_id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.account_id") {
                if let Some(val) = event.get("json.account_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.account_id".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.account.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_account_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.account.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.id", v)?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.target.id").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.account_uid") {
                    event.rename("json.account_uid", "vectra_detect.log.account.uid")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("vectra_detect.log.account.uid") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else { break 'dissect false };
                        captured.push(("vectra_detect.log.account.user_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("vectra_detect.log.account.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "vectra_detect.log.account.uid".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_account_uid")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("vectra_detect.log.account.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if event.has_value("user.domain") {
                if let Some(domain_str) = event.get_string("user.domain") {
                    let domain = domain_str.to_string();
                    event.set("vectra_detect.log.user.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("vectra_detect.log.user.registered_domain", json!(registered))?;
                        }
                        event.set("vectra_detect.log.user.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("vectra_detect.log.user.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            if let Some(v) = event.get("vectra_detect.log.account.user_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.id").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("vectra_detect.log.account.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

            let _cond = { event.get_str("json.threat") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.threat") {
                if let Some(val) = event.get("json.threat") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.threat".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.threat.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_threat_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.quadrant") {
                    event.rename("json.quadrant", "vectra_detect.log.quadrant")?;
                }

            if let Some(v) = event.get("vectra_detect.log.quadrant").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.risk.static_level", v)?;
            }

            let _cond = { event.has_value("user.risk.static_level") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.risk.static_level").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.certainty") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.certainty") {
                if let Some(val) = event.get("json.certainty") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.certainty".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.certainty", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_certainty_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.score_decreases") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.score_decreases") {
                if let Some(val) = event.get("json.score_decreases") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.score_decreases".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.score_decreases", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_decreases_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.privilege") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.privilege") {
                if let Some(val) = event.get("json.privilege") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.privilege".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.privilege", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_privilege_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.host_access_history").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.host_access_history", "json.host_access_history")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_host_access_history")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.host_access_history") {
                    event.rename("json.host_access_history", "vectra_detect.log.host.access_history")?;
                }

            let _cond = { event.get("vectra_detect.log.host.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.host.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.id") {
                    if let Some(val) = event.get("_ingest._value.id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_access_history_id_to_string")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.host.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("vectra_detect.log.host.access_history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            if let Some(date_str) = event.get_as_string("_ingest._value.lastSeen") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.last_seen", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.lastSeen".into(),
                            message: format!("unable to parse date [{date_str}]"),
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
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("vectra_detect.log.host.access_history", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("vectra_detect.log.host.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.host.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.privilege") {
                    if let Some(val) = event.get("_ingest._value.privilege") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.privilege".into(),
                    message,
                    })?;
                    event.set("_ingest._value.privilege_value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_access_history_privilege_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.host.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.host.access_history", |event| {
                    if event.has_value("_ingest._value.privilegeCategory") {
                    event.rename("_ingest._value.privilegeCategory", "_ingest._value.privilege_category")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.host.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.host.access_history", |event| {
                    event.remove("_ingest._value.lastSeen");
                    event.remove("_ingest._value.privilege");
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.service_access_history").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.service_access_history", "json.service_access_history")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_service_access_history")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.service_access_history") {
                    event.rename("json.service_access_history", "vectra_detect.log.service.access_history")?;
                }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.id") {
                    if let Some(val) = event.get("_ingest._value.id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_access_history_id_to_string")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("vectra_detect.log.service.access_history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            if let Some(date_str) = event.get_as_string("_ingest._value.lastSeen") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.last_seen", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.lastSeen".into(),
                            message: format!("unable to parse date [{date_str}]"),
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
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("vectra_detect.log.service.access_history", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.privilege") {
                    if let Some(val) = event.get("_ingest._value.privilege") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.privilege".into(),
                    message,
                    })?;
                    event.set("_ingest._value.privilege_value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_access_history_privilege_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    if event.has_value("_ingest._value.privilegeCategory") {
                    event.rename("_ingest._value.privilegeCategory", "_ingest._value.privilege_category")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.access_history") {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    event.remove("_ingest._value.lastSeen");
                    event.remove("_ingest._value.privilege");
                    Ok(())
                })?;
            }

                if event.has_value("json.category") {
                    event.rename("json.category", "vectra_detect.log.category")?;
                }

                if event.has_value("json.last_detection_type") {
                    event.rename("json.last_detection_type", "vectra_detect.log.last_detection_type")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
