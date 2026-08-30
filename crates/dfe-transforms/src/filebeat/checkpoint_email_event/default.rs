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
            event.set("ecs.version", json!("8.17.0"))?;

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
                if let Some(v) = event.get("json.customerId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.description") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.entityId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.eventCreated") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.eventId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("email"))?;

            let _cond = {
                event.has_value("json.type")
                    && (event
                        .get_str("json.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("malware"))
                        || event
                            .get_str("json.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("suspicious malware")))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("json.type")
                    && (event
                        .get_str("json.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("anomaly"))
                        || event
                            .get_str("json.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("shadow_it")))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.has_value("json.type")
                    && (event
                        .get_str("json.type")
                        .is_some_and(|s| s.eq_ignore_ascii_case("anomaly"))
                        || event
                            .get_str("json.type")
                            .is_some_and(|s| s.eq_ignore_ascii_case("shadow_it")))
            };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            event.set("observer.vendor", json!("Check Point"))?;

            event.set("observer.product", json!("Harmony Email & Collaboration"))?;

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    if event.has_value("_ingest._value.actionType") {
                        event.rename("_ingest._value.actionType", "_ingest._value.action_type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.actions").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.createTime")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.create_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.createTime".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_actions_createTime",
                                )?;
                                event.remove("_ingest._value.createTime");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
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
                            "json.actions",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    if event.has_value("_ingest._value.relatedEntityId") {
                        event.rename(
                            "_ingest._value.relatedEntityId",
                            "_ingest._value.related_entity_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    event.remove("_ingest._value.createTime");
                    Ok(())
                })?;
            }

            if event.has_value("json.actions") {
                event.rename("json.actions", "checkpoint_email.event.actions")?;
            }

            if event.has_value("json.additionalData") {
                event.rename(
                    "json.additionalData",
                    "checkpoint_email.event.additional_data",
                )?;
            }

            let _cond = {
                event
                    .get("json.availableEventActions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.availableEventActions", |event| {
                    if event.has_value("_ingest._value.actionName") {
                        event.rename("_ingest._value.actionName", "_ingest._value.action_name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.availableEventActions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.availableEventActions", |event| {
                    if event.has_value("_ingest._value.actionParameter") {
                        event.rename(
                            "_ingest._value.actionParameter",
                            "_ingest._value.action_parameter",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.availableEventActions") {
                event.rename(
                    "json.availableEventActions",
                    "checkpoint_email.event.available_event_actions",
                )?;
            }

            if event.has_value("json.confidenceIndicator") {
                event.rename(
                    "json.confidenceIndicator",
                    "checkpoint_email.event.confidence_indicator",
                )?;
            }

            if event.has_value("json.customerId") {
                event.rename("json.customerId", "checkpoint_email.event.customer_id")?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.customer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has_value("json.data") {
                event.rename("json.data", "checkpoint_email.event.data")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("checkpoint_email.event.data") {
                    if let Some(input) = event.get_string("checkpoint_email.event.data") {
                        // Grok pattern: ^#%{DATA:_temp.scan_information} attempt detected in an email from #%{DATA:_temp.source_user} - '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$
                        // Grok pattern: ^%{DATA}(A user|has detected) #%{DATA:_temp.scan_information} in an email from #%{DATA:_temp.source_user} - '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$
                        // Grok pattern: ^%{DATA} has detected malicious attachment\\(s\\) in '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$
                        // Grok pattern: ^User #%{DATA:_temp.destination_user} reported a phishing email #%{DATA:_temp.email_subject}$
                        // Grok pattern: ^Unusual geo activity for #%{DATA:_temp.user_address}: #%{DATA:_temp.user_action} for the first time from %{DATA:checkpoint_email.event.user.country}$
                        // Grok pattern: ^%{GREEDYDATA}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^#%{DATA:_temp.scan_information} attempt detected in an email from #%{DATA:_temp.source_user} - '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$"
                                ),
                                cached_grok!(
                                    "^%{DATA}(A user|has detected) #%{DATA:_temp.scan_information} in an email from #%{DATA:_temp.source_user} - '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$"
                                ),
                                cached_grok!(
                                    "^%{DATA} has detected malicious attachment\\(s\\) in '#%{DATA:_temp.email_subject}' \\(#%{DATA:_temp.destination_user}'s mailbox\\)$"
                                ),
                                cached_grok!(
                                    "^User #%{DATA:_temp.destination_user} reported a phishing email #%{DATA:_temp.email_subject}$"
                                ),
                                cached_grok!(
                                    "^Unusual geo activity for #%{DATA:_temp.user_address}: #%{DATA:_temp.user_action} for the first time from %{DATA:checkpoint_email.event.user.country}$"
                                ),
                                cached_grok!("^%{GREEDYDATA}$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_parse_data")?;
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

            let _cond = { event.has_value("_temp.scan_information") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "_temp.scan_information", "_temp.scan_information")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_temp_scan_information",
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
                .get("_temp.scan_information.entity_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("checkpoint_email.event.scan_type", v)?;
            }

            let _cond = { event.has_value("_temp.email_subject") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "_temp.email_subject", "_temp.email_subject")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_temp_email_subject",
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
                .get("_temp.email_subject.label")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("checkpoint_email.event.email_subject", v)?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.email_subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            let _cond = { event.has_value("_temp.destination_user") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "_temp.destination_user", "_temp.destination_user")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_temp_destination_user",
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

            let _cond = { event.get_str("_temp.destination_user.label") != Some("<Unknown>") };
            if _cond {
                if let Some(v) = event
                    .get("_temp.destination_user.label")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("checkpoint_email.event.destination_address", v)?;
                }
            }

            let _cond = { event.has_value("checkpoint_email.event.destination_address") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("checkpoint_email.event.destination_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.destination_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.email", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("destination.user.email") {
                    if let Some(input) = event.get_string("destination.user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("destination.user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("checkpoint_email.event.destination_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("checkpoint_email.event.destination_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.user_address") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "_temp.user_address", "_temp.user_address")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_temp_user_address")?;
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

            let _cond = { event.get_str("_temp.user_address.label") != Some("<Unknown>") };
            if _cond {
                if let Some(v) = event
                    .get("_temp.user_address.label")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("checkpoint_email.event.user.address", v)?;
                }
            }

            if let Some(v) = event
                .get("checkpoint_email.event.user.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.email") {
                    if let Some(input) = event.get_string("user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("checkpoint_email.event.user.address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("checkpoint_email.event.user.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.user_action") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "_temp.user_action", "_temp.user_action")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_temp_user_action")?;
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
                .get("_temp.user_action.label")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("checkpoint_email.event.user.action", v)?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "checkpoint_email.event.description")?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.entityId") {
                event.rename("json.entityId", "checkpoint_email.event.entity_id")?;
            }

            if event.has_value("json.entityLink") {
                event.rename("json.entityLink", "checkpoint_email.event.entity_link")?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.entity_link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            let _cond = {
                event.has_value("json.eventCreated")
                    && event.get_str("json.eventCreated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.eventCreated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("checkpoint_email.event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.eventCreated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_eventCreated")?;
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
                .get("checkpoint_email.event.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.eventId") {
                event.rename("json.eventId", "checkpoint_email.event.id")?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.saas") {
                event.rename("json.saas", "checkpoint_email.event.saas")?;
            }

            if event.has_value("json.senderAddress") {
                event.rename(
                    "json.senderAddress",
                    "checkpoint_email.event.sender_address",
                )?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.sender_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.user.email") {
                    if let Some(input) = event.get_string("source.user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("source.user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("source.user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("checkpoint_email.event.sender_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.sender.address", v)?;
            }

            let _cond = { event.has_value("checkpoint_email.event.sender_address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("checkpoint_email.event.sender_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.severity") && event.get_str("json.severity") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.severity") {
                        if let Some(val) = event.get("json.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("checkpoint_email.event.severity", converted)?;
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

            let _cond = { event.has_value("checkpoint_email.event.severity") };
            if _cond {
                // Painless script
                // Source: def severityValue = ctx.checkpoint_email.event.severity;\nif (severityValue > 0 && severityValue <= params.severity.length) {\n  ctx.checkpoint_email.event.put('severity_enum', params['severity'][(int)severityValue-1]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def severityValue = ctx.checkpoint_email.event.severity;\nif (severityValue > 0 && severityValue <= params.severity.length) {\n  ctx.checkpoint_email.event.put('severity_enum', params['severity'][(int)severityValue-1]);\n}"#
                    ),
                    cached_params!(
                        "{\"severity\":[\"Lowest\",\"Low\",\"Medium\",\"High\",\"Critical\"]}"
                    ),
                )?;
            }

            if let Some(v) = event
                .get("checkpoint_email.event.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if event.has_value("json.state") {
                event.rename("json.state", "checkpoint_email.event.state")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "checkpoint_email.event.type")?;
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
                event.remove("checkpoint_email.event.customer_id");
                event.remove("checkpoint_email.event.description");
                event.remove("checkpoint_email.event.entity_link");
                event.remove("checkpoint_email.event.id");
                event.remove("checkpoint_email.event.created");
                event.remove("checkpoint_email.event.severity");
                event.remove("checkpoint_email.event.sender_address");
                event.remove("checkpoint_email.event.destination_address");
                event.remove("checkpoint_email.event.email_subject");
            }

            event.remove("json");
            event.remove("_temp");

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
