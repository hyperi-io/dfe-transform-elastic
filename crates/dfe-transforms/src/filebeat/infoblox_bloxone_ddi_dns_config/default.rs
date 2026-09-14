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

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("protocol")]))?;

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

            let _cond = {
                event.get("json.results").is_some_and(|v| v.is_array()) && event.get("json.results").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.updated_at") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get_str("json.add_edns_option_in_outgoing_query") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.add_edns_option_in_outgoing_query") {
                        if let Some(val) = event.get("json.add_edns_option_in_outgoing_query") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.add_edns_option_in_outgoing_query".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.add_edns.option_in.outgoing_query",
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
            }

            if event.has_value("json.comment") {
                event.rename("json.comment", "infoblox_bloxone_ddi.dns_config.comment")?;
            }

            let _cond = {
                event.has_value("json.created_at") && event.get_str("json.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("infoblox_bloxone_ddi.dns_config.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
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
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dns_config.created_at")
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("json.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.custom_root_ns") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.custom_root_ns").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.custom_root_ns",
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
            }

            let _cond = {
                event
                    .get("json.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.custom_root_ns") {
                        foreach_array(event, "json.custom_root_ns", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.custom_root_ns") {
                        foreach_array(event, "json.custom_root_ns", |event| {
                            if event.has_value("_ingest._value.protocol_fqdn") {
                                event.rename(
                                    "_ingest._value.protocol_fqdn",
                                    "_ingest._value.protocol.fqdn",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.custom_root_ns") {
                event.rename(
                    "json.custom_root_ns",
                    "infoblox_bloxone_ddi.dns_config.custom_root_ns",
                )?;
            }

            let _cond = { event.get_str("json.custom_root_ns_enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.custom_root_ns_enabled") {
                        if let Some(val) = event.get("json.custom_root_ns_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.custom_root_ns_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.custom_root_ns_enabled",
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
            }

            let _cond = { event.get_str("json.disabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.disabled") {
                        if let Some(val) = event.get("json.disabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.disabled".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.disabled", converted)?;
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
            }

            let _cond = { event.get_str("json.dnssec_enable_validation") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dnssec_enable_validation") {
                        if let Some(val) = event.get("json.dnssec_enable_validation") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dnssec_enable_validation".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.dnssec.enable_validation",
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
            }

            let _cond = { event.get_str("json.dnssec_enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dnssec_enabled") {
                        if let Some(val) = event.get("json.dnssec_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dnssec_enabled".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.dnssec.enabled", converted)?;
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
            }

            let _cond = {
                event
                    .get("json.dnssec_root_keys")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_root_keys") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.dnssec_root_keys").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.algorithm") {
                                            if let Some(val) = event.get("_ingest._value.algorithm")
                                            {
                                                let converted = convert_value(val, "long")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.algorithm".into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set("_ingest._value.algorithm", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        if event.remove("_ingest._value.algorithm").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.algorithm".into(),
                                            });
                                        }
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
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
                                    "json.dnssec_root_keys",
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
            }

            let _cond = {
                event
                    .get("json.dnssec_root_keys")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_root_keys") {
                        foreach_array(event, "json.dnssec_root_keys", |event| {
                            if event.has_value("_ingest._value.protocol_zone") {
                                event.rename(
                                    "_ingest._value.protocol_zone",
                                    "_ingest._value.protocol.zone",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.dnssec_root_keys")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_root_keys") {
                        foreach_array(event, "json.dnssec_root_keys", |event| {
                            if event.has_value("_ingest._value.public_key") {
                                event
                                    .rename("_ingest._value.public_key", "_ingest._value.public")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.dnssec_root_keys")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_root_keys") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.dnssec_root_keys").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.sep") {
                                            if let Some(val) = event.get("_ingest._value.sep") {
                                                let converted = convert_value(val, "boolean")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.sep".into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set("_ingest._value.sep", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        if event.remove("_ingest._value.sep").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.sep".into(),
                                            });
                                        }
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
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
                                    "json.dnssec_root_keys",
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
            }

            if event.has_value("json.dnssec_root_keys") {
                event.rename(
                    "json.dnssec_root_keys",
                    "infoblox_bloxone_ddi.dns_config.dnssec.root_keys",
                )?;
            }

            let _cond = {
                event
                    .get("json.dnssec_trust_anchors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_trust_anchors") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.dnssec_trust_anchors").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.algorithm") {
                                            if let Some(val) = event.get("_ingest._value.algorithm")
                                            {
                                                let converted = convert_value(val, "long")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.algorithm".into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set("_ingest._value.algorithm", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        if event.remove("_ingest._value.algorithm").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.algorithm".into(),
                                            });
                                        }
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
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
                                    "json.dnssec_trust_anchors",
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
            }

            let _cond = {
                event
                    .get("json.dnssec_trust_anchors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_trust_anchors") {
                        foreach_array(event, "json.dnssec_trust_anchors", |event| {
                            if event.has_value("_ingest._value.protocol_zone") {
                                event.rename(
                                    "_ingest._value.protocol_zone",
                                    "_ingest._value.protocol.zone",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.dnssec_trust_anchors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.dnssec_trust_anchors") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.dnssec_trust_anchors").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.sep") {
                                            if let Some(val) = event.get("_ingest._value.sep") {
                                                let converted = convert_value(val, "boolean")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.sep".into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set("_ingest._value.sep", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        if event.remove("_ingest._value.sep").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.sep".into(),
                                            });
                                        }
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
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
                                    "json.dnssec_trust_anchors",
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
            }

            if event.has_value("json.dnssec_trust_anchors") {
                event.rename(
                    "json.dnssec_trust_anchors",
                    "infoblox_bloxone_ddi.dns_config.dnssec.trust_anchors",
                )?;
            }

            let _cond = { event.get_str("json.dnssec_validate_expiry") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dnssec_validate_expiry") {
                        if let Some(val) = event.get("json.dnssec_validate_expiry") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dnssec_validate_expiry".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.dnssec.validate_expiry",
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
            }

            let _cond = { event.get_str("json.ecs_enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ecs_enabled") {
                        if let Some(val) = event.get("json.ecs_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ecs_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.ecs.enabled", converted)?;
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
            }

            let _cond = { event.get_str("json.ecs_forwarding") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ecs_forwarding") {
                        if let Some(val) = event.get("json.ecs_forwarding") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ecs_forwarding".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.ecs.forwarding", converted)?;
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
            }

            let _cond = { event.get_str("json.ecs_prefix_v4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ecs_prefix_v4") {
                        if let Some(val) = event.get("json.ecs_prefix_v4") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ecs_prefix_v4".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.ecs.prefix_v4", converted)?;
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
            }

            let _cond = { event.get_str("json.ecs_prefix_v6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ecs_prefix_v6") {
                        if let Some(val) = event.get("json.ecs_prefix_v6") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ecs_prefix_v6".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.ecs.prefix_v6", converted)?;
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
            }

            let _cond = { event.get("json.ecs_zones").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.ecs_zones") {
                        foreach_array(event, "json.ecs_zones", |event| {
                            if event.has_value("_ingest._value.protocol_fqdn") {
                                event.rename(
                                    "_ingest._value.protocol_fqdn",
                                    "_ingest._value.protocol.fqdn",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.ecs_zones") {
                event.rename(
                    "json.ecs_zones",
                    "infoblox_bloxone_ddi.dns_config.ecs.zones",
                )?;
            }

            let _cond = { event.get_str("json.edns_udp_size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.edns_udp_size") {
                        if let Some(val) = event.get("json.edns_udp_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.edns_udp_size".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.edns.udp.size", converted)?;
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
            }

            let _cond = { event.get("json.forwarders").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.forwarders") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.forwarders").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.forwarders",
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
            }

            let _cond = { event.get("json.forwarders").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.forwarders") {
                        foreach_array(event, "json.forwarders", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = { event.get("json.forwarders").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.forwarders") {
                        foreach_array(event, "json.forwarders", |event| {
                            if event.has_value("_ingest._value.protocol_fqdn") {
                                event.rename(
                                    "_ingest._value.protocol_fqdn",
                                    "_ingest._value.protocol.fqdn",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.forwarders") {
                event.rename(
                    "json.forwarders",
                    "infoblox_bloxone_ddi.dns_config.forwarders",
                )?;
            }

            let _cond = { event.get_str("json.forwarders_only") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.forwarders_only") {
                        if let Some(val) = event.get("json.forwarders_only") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.forwarders_only".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.forwarders_only",
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
            }

            let _cond = { event.get_str("json.gss_tsig_enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.gss_tsig_enabled") {
                        if let Some(val) = event.get("json.gss_tsig_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.gss_tsig_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.gss_tsig_enabled",
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
            }

            if event.has_value("json.id") {
                event.rename("json.id", "infoblox_bloxone_ddi.dns_config.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("infoblox_bloxone_ddi.dns_config.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.inheritance_sources.add_edns_option_in_outgoing_query.action")
            {
                event.rename("json.inheritance_sources.add_edns_option_in_outgoing_query.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.add_edns.option_in.outgoing_query.action")?;
            }

            if event.has_value(
                "json.inheritance_sources.add_edns_option_in_outgoing_query.display_name",
            ) {
                event.rename("json.inheritance_sources.add_edns_option_in_outgoing_query.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.add_edns.option_in.outgoing_query.display.name")?;
            }

            if event.has_value("json.inheritance_sources.add_edns_option_in_outgoing_query.source")
            {
                event.rename("json.inheritance_sources.add_edns_option_in_outgoing_query.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.add_edns.option_in.outgoing_query.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.add_edns_option_in_outgoing_query.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.add_edns_option_in_outgoing_query.value",
                    ) {
                        if let Some(val) = event
                            .get("json.inheritance_sources.add_edns_option_in_outgoing_query.value")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.add_edns_option_in_outgoing_query.value".into(),
                            message,
                        })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.add_edns.option_in.outgoing_query.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.custom_root_ns_block.action") {
                event.rename("json.inheritance_sources.custom_root_ns_block.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.custom_root_ns.block.action")?;
            }

            if event.has_value("json.inheritance_sources.custom_root_ns_block.display_name") {
                event.rename("json.inheritance_sources.custom_root_ns_block.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.custom_root_ns.block.display.name")?;
            }

            if event.has_value("json.inheritance_sources.custom_root_ns_block.source") {
                event.rename("json.inheritance_sources.custom_root_ns_block.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.custom_root_ns.block.source")?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns",
                    ) {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                event.set("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns",
                    ) {
                        foreach_array(
                            event,
                            "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns",
                    ) {
                        foreach_array(
                            event,
                            "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns",
                            |event| {
                                if event.has_value("_ingest._value.protocol_fqdn") {
                                    event.rename(
                                        "_ingest._value.protocol_fqdn",
                                        "_ingest._value.protocol.fqdn",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns")
            {
                event.rename("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns", "infoblox_bloxone_ddi.dns_config.inheritance.sources.custom_root_ns.block.value")?;
            }

            let _cond = {
                event.get_str(
                    "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns_enabled",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns_enabled") {
                if let Some(val) = event.get("json.inheritance_sources.custom_root_ns_block.value.custom_root_ns_enabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.custom_root_ns_block.value.custom_root_ns_enabled".into(),
                            message,
                        })?;
                    event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.custom_root_ns.block.value_enabled", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.dnssec_validation_block.action") {
                event.rename("json.inheritance_sources.dnssec_validation_block.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.action")?;
            }

            if event.has_value("json.inheritance_sources.dnssec_validation_block.display_name") {
                event.rename("json.inheritance_sources.dnssec_validation_block.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.display.name")?;
            }

            if event.has_value("json.inheritance_sources.dnssec_validation_block.source") {
                event.rename("json.inheritance_sources.dnssec_validation_block.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.dnssec_validation_block.value.dnssec_enable_validation") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.dnssec_validation_block.value.dnssec_enable_validation") {
                if let Some(val) = event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_enable_validation") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.dnssec_validation_block.value.dnssec_enable_validation".into(),
                            message,
                        })?;
                    event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.value.enable", converted)?;
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
            }

            let _cond = {
                event.get_str(
                    "json.inheritance_sources.dnssec_validation_block.value.dnssec_enabled",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.dnssec_validation_block.value.dnssec_enabled",
                    ) {
                        if let Some(val) = event.get(
                            "json.inheritance_sources.dnssec_validation_block.value.dnssec_enabled",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.dnssec_validation_block.value.dnssec_enabled".into(),
                            message,
                        })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.value.enabled", converted)?;
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
            }

            let _cond = {
                event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.algorithm") {
                            if let Some(val) = event.get("_ingest._value.algorithm") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.algorithm".into(),
                            message,
                            })?;
                            event.set("_ingest._value.algorithm", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            if event.remove("_ingest._value.algorithm").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.algorithm".into() });
                            }
                            event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                        event.set("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors") {
                foreach_array(event, "json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors", |event| {
                    if event.has_value("_ingest._value.protocol_zone") {
                    event.rename("_ingest._value.protocol_zone", "_ingest._value.protocol.zone")?;
                    }
                    Ok(())
                })?;
            }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.sep") {
                            if let Some(val) = event.get("_ingest._value.sep") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.sep".into(),
                            message,
                            })?;
                            event.set("_ingest._value.sep", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            if event.remove("_ingest._value.sep").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.sep".into() });
                            }
                            event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                        event.set("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }
                    Ok(())
                })();
            }

            if event.has_value(
                "json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors",
            ) {
                event.rename("json.inheritance_sources.dnssec_validation_block.value.dnssec_trust_anchors", "infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.value.trust_anchors")?;
            }

            let _cond = {
                event.get_str(
                    "json.inheritance_sources.dnssec_validation_block.value.dnssec_validate_expiry",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.dnssec_validation_block.value.dnssec_validate_expiry") {
                if let Some(val) = event.get("json.inheritance_sources.dnssec_validation_block.value.dnssec_validate_expiry") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.dnssec_validation_block.value.dnssec_validate_expiry".into(),
                            message,
                        })?;
                    event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.dnssec.validation.block.value.validate_expiry", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.ecs_block.action") {
                event.rename(
                    "json.inheritance_sources.ecs_block.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.ecs_block.display_name") {
                event.rename(
                    "json.inheritance_sources.ecs_block.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.ecs_block.source") {
                event.rename(
                    "json.inheritance_sources.ecs_block.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.source",
                )?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.ecs_block.value.ecs_enabled") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ecs_block.value.ecs_enabled") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.ecs_block.value.ecs_enabled")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.ecs_block.value.ecs_enabled"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.value.enabled", converted)?;
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
            }

            let _cond = {
                event.get_str("json.inheritance_sources.ecs_block.value.ecs_forwarding") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ecs_block.value.ecs_forwarding") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.ecs_block.value.ecs_forwarding")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.ecs_block.value.ecs_forwarding"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.value.forwarding", converted)?;
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
            }

            let _cond = {
                event.get_str("json.inheritance_sources.ecs_block.value.ecs_prefix_v4") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ecs_block.value.ecs_prefix_v4") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.ecs_block.value.ecs_prefix_v4")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.ecs_block.value.ecs_prefix_v4"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.value.prefix_v4", converted)?;
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
            }

            let _cond = {
                event.get_str("json.inheritance_sources.ecs_block.value.ecs_prefix_v6") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ecs_block.value.ecs_prefix_v6") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.ecs_block.value.ecs_prefix_v6")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.ecs_block.value.ecs_prefix_v6"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.value.prefix_v6", converted)?;
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.ecs_block.value.ecs_zones")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ecs_block.value.ecs_zones") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.ecs_block.value.ecs_zones",
                            |event| {
                                if event.has_value("_ingest._value.protocol_fqdn") {
                                    event.rename(
                                        "_ingest._value.protocol_fqdn",
                                        "_ingest._value.protocol.fqdn",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.ecs_block.value.ecs_zones") {
                event.rename(
                    "json.inheritance_sources.ecs_block.value.ecs_zones",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.ecs.block.value.zones",
                )?;
            }

            if event.has_value("json.inheritance_sources.edns_udp_size.action") {
                event.rename(
                    "json.inheritance_sources.edns_udp_size.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.edns.udp.size.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.edns_udp_size.display_name") {
                event.rename("json.inheritance_sources.edns_udp_size.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.edns.udp.size.display.name")?;
            }

            if event.has_value("json.inheritance_sources.edns_udp_size.source") {
                event.rename(
                    "json.inheritance_sources.edns_udp_size.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.edns.udp.size.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.edns_udp_size.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.edns_udp_size.value") {
                        if let Some(val) = event.get("json.inheritance_sources.edns_udp_size.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.edns_udp_size.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.edns.udp.size.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.forwarders_block.action") {
                event.rename(
                    "json.inheritance_sources.forwarders_block.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.forwarders.block.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.forwarders_block.display_name") {
                event.rename("json.inheritance_sources.forwarders_block.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.forwarders.block.display.name")?;
            }

            if event.has_value("json.inheritance_sources.forwarders_block.source") {
                event.rename(
                    "json.inheritance_sources.forwarders_block.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.forwarders.block.source",
                )?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.forwarders_block.value.forwarders")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.forwarders_block.value.forwarders")
                    {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event
                                .get("json.inheritance_sources.forwarders_block.value.forwarders")
                                .cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.inheritance_sources.forwarders_block.value.forwarders",
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.forwarders_block.value.forwarders")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.forwarders_block.value.forwarders")
                    {
                        foreach_array(
                            event,
                            "json.inheritance_sources.forwarders_block.value.forwarders",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.forwarders_block.value.forwarders")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.forwarders_block.value.forwarders")
                    {
                        foreach_array(
                            event,
                            "json.inheritance_sources.forwarders_block.value.forwarders",
                            |event| {
                                if event.has_value("_ingest._value.protocol_fqdn") {
                                    event.rename(
                                        "_ingest._value.protocol_fqdn",
                                        "_ingest._value.protocol.fqdn",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.forwarders_block.value.forwarders") {
                event.rename(
                    "json.inheritance_sources.forwarders_block.value.forwarders",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.forwarders.block.value",
                )?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.forwarders_block.value.forwarders_only")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.forwarders_block.value.forwarders_only",
                    ) {
                        if let Some(val) = event
                            .get("json.inheritance_sources.forwarders_block.value.forwarders_only")
                        {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.forwarders_block.value.forwarders_only".into(),
                            message,
                        })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.forwarders.block.value_only", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.gss_tsig_enabled.action") {
                event.rename(
                    "json.inheritance_sources.gss_tsig_enabled.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.gss_tsig_enabled.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.gss_tsig_enabled.display_name") {
                event.rename("json.inheritance_sources.gss_tsig_enabled.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.gss_tsig_enabled.display.name")?;
            }

            if event.has_value("json.inheritance_sources.gss_tsig_enabled.source") {
                event.rename(
                    "json.inheritance_sources.gss_tsig_enabled.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.gss_tsig_enabled.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.gss_tsig_enabled.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.gss_tsig_enabled.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.gss_tsig_enabled.value")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.gss_tsig_enabled.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.gss_tsig_enabled.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.lame_ttl.action") {
                event.rename(
                    "json.inheritance_sources.lame_ttl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.lame_ttl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.lame_ttl.display_name") {
                event.rename(
                    "json.inheritance_sources.lame_ttl.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.lame_ttl.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.lame_ttl.source") {
                event.rename(
                    "json.inheritance_sources.lame_ttl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.lame_ttl.source",
                )?;
            }

            let _cond = { event.get_str("json.inheritance_sources.lame_ttl.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.lame_ttl.value") {
                        if let Some(val) = event.get("json.inheritance_sources.lame_ttl.value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.lame_ttl.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.lame_ttl.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.match_recursive_only.action") {
                event.rename("json.inheritance_sources.match_recursive_only.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.match_recursive_only.action")?;
            }

            if event.has_value("json.inheritance_sources.match_recursive_only.display_name") {
                event.rename("json.inheritance_sources.match_recursive_only.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.match_recursive_only.display.name")?;
            }

            if event.has_value("json.inheritance_sources.match_recursive_only.source") {
                event.rename("json.inheritance_sources.match_recursive_only.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.match_recursive_only.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.match_recursive_only.value") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.match_recursive_only.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.match_recursive_only.value")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.match_recursive_only.value"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.match_recursive_only.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.max_cache_ttl.action") {
                event.rename(
                    "json.inheritance_sources.max_cache_ttl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_cache_ttl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.max_cache_ttl.display_name") {
                event.rename("json.inheritance_sources.max_cache_ttl.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_cache_ttl.display.name")?;
            }

            if event.has_value("json.inheritance_sources.max_cache_ttl.source") {
                event.rename(
                    "json.inheritance_sources.max_cache_ttl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_cache_ttl.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.max_cache_ttl.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.max_cache_ttl.value") {
                        if let Some(val) = event.get("json.inheritance_sources.max_cache_ttl.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.max_cache_ttl.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.max_cache_ttl.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.max_negative_ttl.action") {
                event.rename(
                    "json.inheritance_sources.max_negative_ttl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_negative_ttl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.max_negative_ttl.display_name") {
                event.rename("json.inheritance_sources.max_negative_ttl.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_negative_ttl.display.name")?;
            }

            if event.has_value("json.inheritance_sources.max_negative_ttl.source") {
                event.rename(
                    "json.inheritance_sources.max_negative_ttl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_negative_ttl.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.max_negative_ttl.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.max_negative_ttl.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.max_negative_ttl.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.max_negative_ttl.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.max_negative_ttl.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.max_udp_size.action") {
                event.rename(
                    "json.inheritance_sources.max_udp_size.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_udp_size.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.max_udp_size.display_name") {
                event.rename(
                    "json.inheritance_sources.max_udp_size.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_udp_size.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.max_udp_size.source") {
                event.rename(
                    "json.inheritance_sources.max_udp_size.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.max_udp_size.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.max_udp_size.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.max_udp_size.value") {
                        if let Some(val) = event.get("json.inheritance_sources.max_udp_size.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.max_udp_size.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.max_udp_size.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.minimal_responses.action") {
                event.rename(
                    "json.inheritance_sources.minimal_responses.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.minimal_responses.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.minimal_responses.display_name") {
                event.rename("json.inheritance_sources.minimal_responses.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.minimal_responses.display.name")?;
            }

            if event.has_value("json.inheritance_sources.minimal_responses.source") {
                event.rename(
                    "json.inheritance_sources.minimal_responses.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.minimal_responses.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.minimal_responses.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.minimal_responses.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.minimal_responses.value")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.minimal_responses.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.minimal_responses.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.notify.action") {
                event.rename(
                    "json.inheritance_sources.notify.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.notify.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.notify.display_name") {
                event.rename(
                    "json.inheritance_sources.notify.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.notify.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.notify.source") {
                event.rename(
                    "json.inheritance_sources.notify.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.notify.source",
                )?;
            }

            let _cond = { event.get_str("json.inheritance_sources.notify.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.notify.value") {
                        if let Some(val) = event.get("json.inheritance_sources.notify.value") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.notify.value".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.inheritance.sources.notify.value",
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
            }

            if event.has_value("json.inheritance_sources.query_acl.action") {
                event.rename(
                    "json.inheritance_sources.query_acl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.query_acl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.query_acl.display_name") {
                event.rename(
                    "json.inheritance_sources.query_acl.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.query_acl.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.query_acl.source") {
                event.rename(
                    "json.inheritance_sources.query_acl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.query_acl.source",
                )?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.query_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.query_acl.value") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event
                                .get("json.inheritance_sources.query_acl.value")
                                .cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.inheritance_sources.query_acl.value",
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.query_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.query_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.query_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.query_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.query_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.query_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.query_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.query_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.query_acl.value",
                            |event| {
                                if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                    event.rename(
                                        "_ingest._value.tsig_key.protocol_name",
                                        "_ingest._value.tsig_key.protocol.name",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.query_acl.value") {
                event.rename(
                    "json.inheritance_sources.query_acl.value",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.query_acl.value",
                )?;
            }

            if event.has_value("json.inheritance_sources.recursion_acl.action") {
                event.rename(
                    "json.inheritance_sources.recursion_acl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_acl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.recursion_acl.display_name") {
                event.rename("json.inheritance_sources.recursion_acl.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_acl.display.name")?;
            }

            if event.has_value("json.inheritance_sources.recursion_acl.source") {
                event.rename(
                    "json.inheritance_sources.recursion_acl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_acl.source",
                )?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.recursion_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.recursion_acl.value") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event
                                .get("json.inheritance_sources.recursion_acl.value")
                                .cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.inheritance_sources.recursion_acl.value",
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.recursion_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.recursion_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.recursion_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.recursion_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.recursion_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.recursion_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.recursion_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.recursion_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.recursion_acl.value",
                            |event| {
                                if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                    event.rename(
                                        "_ingest._value.tsig_key.protocol_name",
                                        "_ingest._value.tsig_key.protocol.name",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.recursion_acl.value") {
                event.rename(
                    "json.inheritance_sources.recursion_acl.value",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_acl.value",
                )?;
            }

            if event.has_value("json.inheritance_sources.recursion_enabled.action") {
                event.rename(
                    "json.inheritance_sources.recursion_enabled.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_enabled.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.recursion_enabled.display_name") {
                event.rename("json.inheritance_sources.recursion_enabled.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_enabled.display.name")?;
            }

            if event.has_value("json.inheritance_sources.recursion_enabled.source") {
                event.rename(
                    "json.inheritance_sources.recursion_enabled.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_enabled.source",
                )?;
            }

            let _cond =
                { event.get_str("json.inheritance_sources.recursion_enabled.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.recursion_enabled.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.recursion_enabled.value")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.recursion_enabled.value".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.recursion_enabled.value", converted)?;
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
            }

            if event
                .has_value("json.inheritance_sources.synthesize_address_records_from_https.action")
            {
                event.rename("json.inheritance_sources.synthesize_address_records_from_https.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.synthesize.address_records_from_https.action")?;
            }

            if event.has_value(
                "json.inheritance_sources.synthesize_address_records_from_https.display_name",
            ) {
                event.rename("json.inheritance_sources.synthesize_address_records_from_https.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.synthesize.address_records_from_https.display.name")?;
            }

            if event
                .has_value("json.inheritance_sources.synthesize_address_records_from_https.source")
            {
                event.rename("json.inheritance_sources.synthesize_address_records_from_https.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.synthesize.address_records_from_https.name")?;
            }

            let _cond = {
                event
                    .get_str("json.inheritance_sources.synthesize_address_records_from_https.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.inheritance_sources.synthesize_address_records_from_https.value",
                    ) {
                        if let Some(val) = event.get(
                            "json.inheritance_sources.synthesize_address_records_from_https.value",
                        ) {
                            let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.synthesize_address_records_from_https.value".into(),
                            message,
                        })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.synthesize.address_records_from_https.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.transfer_acl.action") {
                event.rename(
                    "json.inheritance_sources.transfer_acl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.transfer_acl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.transfer_acl.display_name") {
                event.rename(
                    "json.inheritance_sources.transfer_acl.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.transfer_acl.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.transfer_acl.source") {
                event.rename(
                    "json.inheritance_sources.transfer_acl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.transfer_acl.source",
                )?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.transfer_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.transfer_acl.value") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event
                                .get("json.inheritance_sources.transfer_acl.value")
                                .cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.inheritance_sources.transfer_acl.value",
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.transfer_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.transfer_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.transfer_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.transfer_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.transfer_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.transfer_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.transfer_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.transfer_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.transfer_acl.value",
                            |event| {
                                if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                    event.rename(
                                        "_ingest._value.tsig_key.protocol_name",
                                        "_ingest._value.tsig_key.protocol.name",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.transfer_acl.value") {
                event.rename(
                    "json.inheritance_sources.transfer_acl.value",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.transfer_acl.value",
                )?;
            }

            if event.has_value("json.inheritance_sources.update_acl.action") {
                event.rename(
                    "json.inheritance_sources.update_acl.action",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.update_acl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.update_acl.display_name") {
                event.rename(
                    "json.inheritance_sources.update_acl.display_name",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.update_acl.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.update_acl.source") {
                event.rename(
                    "json.inheritance_sources.update_acl.source",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.update_acl.source",
                )?;
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.update_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.update_acl.value") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event
                                .get("json.inheritance_sources.update_acl.value")
                                .cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.inheritance_sources.update_acl.value",
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.update_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.update_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.update_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.update_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.update_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.update_acl.value",
                            |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.inheritance_sources.update_acl.value")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.update_acl.value") {
                        foreach_array(
                            event,
                            "json.inheritance_sources.update_acl.value",
                            |event| {
                                if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                    event.rename(
                                        "_ingest._value.tsig_key.protocol_name",
                                        "_ingest._value.tsig_key.protocol.name",
                                    )?;
                                }
                                Ok(())
                            },
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.inheritance_sources.update_acl.value") {
                event.rename(
                    "json.inheritance_sources.update_acl.value",
                    "infoblox_bloxone_ddi.dns_config.inheritance.sources.update_acl.value",
                )?;
            }

            if event.has_value("json.inheritance_sources.use_forwarders_for_subzones.action") {
                event.rename("json.inheritance_sources.use_forwarders_for_subzones.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.use_forwarders_for_subzones.action")?;
            }

            if event.has_value("json.inheritance_sources.use_forwarders_for_subzones.display_name")
            {
                event.rename("json.inheritance_sources.use_forwarders_for_subzones.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.use_forwarders_for_subzones.display.name")?;
            }

            if event.has_value("json.inheritance_sources.use_forwarders_for_subzones.source") {
                event.rename("json.inheritance_sources.use_forwarders_for_subzones.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.use_forwarders_for_subzones.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.use_forwarders_for_subzones.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.use_forwarders_for_subzones.value")
                    {
                        if let Some(val) =
                            event.get("json.inheritance_sources.use_forwarders_for_subzones.value")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.inheritance_sources.use_forwarders_for_subzones.value"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.use_forwarders_for_subzones.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.default_ttl.action") {
                event.rename("json.inheritance_sources.zone_authority.default_ttl.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.default_ttl.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.default_ttl.display_name") {
                event.rename("json.inheritance_sources.zone_authority.default_ttl.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.default_ttl.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.default_ttl.source") {
                event.rename("json.inheritance_sources.zone_authority.default_ttl.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.default_ttl.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.zone_authority.default_ttl.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.default_ttl.value")
                    {
                        if let Some(val) =
                            event.get("json.inheritance_sources.zone_authority.default_ttl.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.inheritance_sources.zone_authority.default_ttl.value"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.default_ttl.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.expire.action") {
                event.rename("json.inheritance_sources.zone_authority.expire.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.expire.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.expire.display_name") {
                event.rename("json.inheritance_sources.zone_authority.expire.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.expire.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.expire.source") {
                event.rename("json.inheritance_sources.zone_authority.expire.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.expire.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.zone_authority.expire.value") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.expire.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.zone_authority.expire.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.zone_authority.expire.value"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.expire.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.mname_block.action") {
                event.rename("json.inheritance_sources.zone_authority.mname_block.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.mname_block.display_name") {
                event.rename("json.inheritance_sources.zone_authority.mname_block.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.mname_block.source") {
                event.rename("json.inheritance_sources.zone_authority.mname_block.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block.source")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.mname_block.value.mname") {
                event.rename("json.inheritance_sources.zone_authority.mname_block.value.mname", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block_value")?;
            }

            if event.has_value(
                "json.inheritance_sources.zone_authority.mname_block.value.protocol_mname",
            ) {
                event.rename("json.inheritance_sources.zone_authority.mname_block.value.protocol_mname", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block.value.protocol.mname")?;
            }

            let _cond = {
                event.get_str(
                    "json.inheritance_sources.zone_authority.mname_block.value.use_default_mname",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.mname_block.value.use_default_mname") {
                if let Some(val) = event.get("json.inheritance_sources.zone_authority.mname_block.value.use_default_mname") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.inheritance_sources.zone_authority.mname_block.value.use_default_mname".into(),
                            message,
                        })?;
                    event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.mname_block.value.isdefault", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.negative_ttl.action") {
                event.rename("json.inheritance_sources.zone_authority.negative_ttl.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.negative_ttl.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.negative_ttl.display_name")
            {
                event.rename("json.inheritance_sources.zone_authority.negative_ttl.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.negative_ttl.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.negative_ttl.source") {
                event.rename("json.inheritance_sources.zone_authority.negative_ttl.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.negative_ttl.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.zone_authority.negative_ttl.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.negative_ttl.value")
                    {
                        if let Some(val) =
                            event.get("json.inheritance_sources.zone_authority.negative_ttl.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.inheritance_sources.zone_authority.negative_ttl.value"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.negative_ttl.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.protocol_rname.action") {
                event.rename("json.inheritance_sources.zone_authority.protocol_rname.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.protocol_rname.action")?;
            }

            if event
                .has_value("json.inheritance_sources.zone_authority.protocol_rname.display_name")
            {
                event.rename("json.inheritance_sources.zone_authority.protocol_rname.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.protocol_rname.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.protocol_rname.source") {
                event.rename("json.inheritance_sources.zone_authority.protocol_rname.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.protocol_rname.source")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.protocol_rname.value") {
                event.rename("json.inheritance_sources.zone_authority.protocol_rname.value", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.protocol_rname.value")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.refresh.action") {
                event.rename("json.inheritance_sources.zone_authority.refresh.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.refresh.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.refresh.display_name") {
                event.rename("json.inheritance_sources.zone_authority.refresh.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.refresh.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.refresh.source") {
                event.rename("json.inheritance_sources.zone_authority.refresh.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.refresh.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.zone_authority.refresh.value") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.refresh.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.zone_authority.refresh.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.zone_authority.refresh.value"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.refresh.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.retry.action") {
                event.rename("json.inheritance_sources.zone_authority.retry.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.retry.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.retry.display_name") {
                event.rename("json.inheritance_sources.zone_authority.retry.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.retry.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.retry.source") {
                event.rename("json.inheritance_sources.zone_authority.retry.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.retry.source")?;
            }

            let _cond = {
                event.get_str("json.inheritance_sources.zone_authority.retry.value") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.zone_authority.retry.value") {
                        if let Some(val) =
                            event.get("json.inheritance_sources.zone_authority.retry.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.zone_authority.retry.value"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.retry.value", converted)?;
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
            }

            if event.has_value("json.inheritance_sources.zone_authority.rname.action") {
                event.rename("json.inheritance_sources.zone_authority.rname.action", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.rname.action")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.rname.display_name") {
                event.rename("json.inheritance_sources.zone_authority.rname.display_name", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.rname.display.name")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.rname.source") {
                event.rename("json.inheritance_sources.zone_authority.rname.source", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.rname.source")?;
            }

            if event.has_value("json.inheritance_sources.zone_authority.rname.value") {
                event.rename("json.inheritance_sources.zone_authority.rname.value", "infoblox_bloxone_ddi.dns_config.inheritance.sources.zone_authority.rname.value")?;
            }

            if event.has_value("json.ip_spaces") {
                event.rename(
                    "json.ip_spaces",
                    "infoblox_bloxone_ddi.dns_config.ip_spaces",
                )?;
            }

            let _cond = { event.get_str("json.lame_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.lame_ttl") {
                        if let Some(val) = event.get("json.lame_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.lame_ttl".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.lame_ttl", converted)?;
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
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dns_config.lame_ttl")
                    .cloned()
                {
                    event.set("dns.answers.ttl", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("json.match_clients_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_clients_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.match_clients_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.match_clients_acl",
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
            }

            let _cond = {
                event
                    .get("json.match_clients_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_clients_acl") {
                        foreach_array(event, "json.match_clients_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.match_clients_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_clients_acl") {
                        foreach_array(event, "json.match_clients_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.match_clients_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_clients_acl") {
                        foreach_array(event, "json.match_clients_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.match_clients_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_clients_acl") {
                        foreach_array(event, "json.match_clients_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.match_clients_acl") {
                event.rename(
                    "json.match_clients_acl",
                    "infoblox_bloxone_ddi.dns_config.match_clients_acl",
                )?;
            }

            let _cond = {
                event
                    .get("json.match_destinations_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_destinations_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.match_destinations_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.match_destinations_acl",
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
            }

            let _cond = {
                event
                    .get("json.match_destinations_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_destinations_acl") {
                        foreach_array(event, "json.match_destinations_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.match_destinations_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_destinations_acl") {
                        foreach_array(event, "json.match_destinations_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.match_destinations_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_destinations_acl") {
                        foreach_array(event, "json.match_destinations_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.match_destinations_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.match_destinations_acl") {
                        foreach_array(event, "json.match_destinations_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.match_destinations_acl") {
                event.rename(
                    "json.match_destinations_acl",
                    "infoblox_bloxone_ddi.dns_config.match_destinations_acl",
                )?;
            }

            let _cond = { event.get_str("json.match_recursive_only") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.match_recursive_only") {
                        if let Some(val) = event.get("json.match_recursive_only") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.match_recursive_only".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.match_recursive_only",
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
            }

            let _cond = { event.get_str("json.max_cache_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.max_cache_ttl") {
                        if let Some(val) = event.get("json.max_cache_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.max_cache_ttl".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_config.max_cache_ttl", converted)?;
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
            }

            let _cond = { event.get_str("json.max_negative_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.max_negative_ttl") {
                        if let Some(val) = event.get("json.max_negative_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.max_negative_ttl".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.max_negative_ttl",
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
            }

            let _cond = { event.get_str("json.max_udp_size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.max_udp_size") {
                        if let Some(val) = event.get("json.max_udp_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.max_udp_size".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.max_udp_size", converted)?;
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
            }

            let _cond = { event.get_str("json.minimal_responses") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.minimal_responses") {
                        if let Some(val) = event.get("json.minimal_responses") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.minimal_responses".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.minimal_responses",
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
            }

            if event.has_value("json.name") {
                event.rename("json.name", "infoblox_bloxone_ddi.dns_config.name")?;
            }

            let _cond = { event.get_str("json.notify") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.notify") {
                        if let Some(val) = event.get("json.notify") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.notify".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.notify", converted)?;
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
            }

            let _cond = { event.get("json.query_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.query_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.query_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.query_acl",
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
            }

            let _cond = { event.get("json.query_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.query_acl") {
                        foreach_array(event, "json.query_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = { event.get("json.query_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.query_acl") {
                        foreach_array(event, "json.query_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = { event.get("json.query_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.query_acl") {
                        foreach_array(event, "json.query_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("json.query_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.query_acl") {
                        foreach_array(event, "json.query_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.query_acl") {
                event.rename(
                    "json.query_acl",
                    "infoblox_bloxone_ddi.dns_config.query_acl",
                )?;
            }

            let _cond = {
                event
                    .get("json.recursion_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.recursion_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.recursion_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.recursion_acl",
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
            }

            let _cond = {
                event
                    .get("json.recursion_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.recursion_acl") {
                        foreach_array(event, "json.recursion_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = {
                event
                    .get("json.recursion_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.recursion_acl") {
                        foreach_array(event, "json.recursion_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = {
                event
                    .get("json.recursion_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.recursion_acl") {
                        foreach_array(event, "json.recursion_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.recursion_acl")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.recursion_acl") {
                        foreach_array(event, "json.recursion_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.recursion_acl") {
                event.rename(
                    "json.recursion_acl",
                    "infoblox_bloxone_ddi.dns_config.recursion_acl",
                )?;
            }

            let _cond = { event.get_str("json.recursion_enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.recursion_enabled") {
                        if let Some(val) = event.get("json.recursion_enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.recursion_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.recursion_enabled",
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
            }

            let _cond = { event.get_str("json.synthesize_address_records_from_https") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.synthesize_address_records_from_https") {
                        if let Some(val) = event.get("json.synthesize_address_records_from_https") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.synthesize_address_records_from_https".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_config.synthesize.address_records_from_https", converted)?;
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
            }

            if event.has_value("json.tags") {
                event.rename("json.tags", "infoblox_bloxone_ddi.dns_config.tags")?;
            }

            let _cond = { event.get("json.transfer_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.transfer_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.transfer_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.transfer_acl",
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
            }

            let _cond = { event.get("json.transfer_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.transfer_acl") {
                        foreach_array(event, "json.transfer_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = { event.get("json.transfer_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.transfer_acl") {
                        foreach_array(event, "json.transfer_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = { event.get("json.transfer_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.transfer_acl") {
                        foreach_array(event, "json.transfer_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("json.transfer_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.transfer_acl") {
                        foreach_array(event, "json.transfer_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.transfer_acl") {
                event.rename(
                    "json.transfer_acl",
                    "infoblox_bloxone_ddi.dns_config.transfer_acl",
                )?;
            }

            let _cond = { event.get("json.update_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.update_acl") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.update_acl").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    let _cond = { false };
                                    if _cond {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.address") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.address")
                                                {
                                                    let converted = convert_value(val, "ip")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.address"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event
                                                        .set("_ingest._value.address", converted)?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            if event.remove("_ingest._value.address").is_none() {
                                                return Err(TransformError::FieldNotFound {
                                                    path: "_ingest._value.address".into(),
                                                });
                                            }
                                            event.append(
                                                "error.message",
                                                json!(
                                                    event
                                                        .get("_ingest.on_failure_message")
                                                        .map_or_else(
                                                            String::new,
                                                            template_to_string
                                                        )
                                                ),
                                            )?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
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
                                    "json.update_acl",
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
            }

            let _cond = { event.get("json.update_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.update_acl") {
                        foreach_array(event, "json.update_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.address")
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
            }

            let _cond = { event.get("json.update_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.update_acl") {
                        foreach_array(event, "json.update_acl", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.tsig_key.algorithm")
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
            }

            let _cond = { event.get("json.update_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.update_acl") {
                        foreach_array(event, "json.update_acl", |event| {
                            if event.has_value("_ingest._value.acl") {
                                event.rename("_ingest._value.acl", "_ingest._value.value")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("json.update_acl").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.update_acl") {
                        foreach_array(event, "json.update_acl", |event| {
                            if event.has_value("_ingest._value.tsig_key.protocol_name") {
                                event.rename(
                                    "_ingest._value.tsig_key.protocol_name",
                                    "_ingest._value.tsig_key.protocol.name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.update_acl") {
                event.rename(
                    "json.update_acl",
                    "infoblox_bloxone_ddi.dns_config.update_acl",
                )?;
            }

            let _cond = {
                event.has_value("json.updated_at") && event.get_str("json.updated_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updated_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("infoblox_bloxone_ddi.dns_config.updated_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updated_at".into(),
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
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dns_config.updated_at")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.use_forwarders_for_subzones") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.use_forwarders_for_subzones") {
                        if let Some(val) = event.get("json.use_forwarders_for_subzones") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.use_forwarders_for_subzones".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.use_forwarders_for_subzones",
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
            }

            let _cond = { event.get_str("json.zone_authority.default_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.default_ttl") {
                        if let Some(val) = event.get("json.zone_authority.default_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.default_ttl".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.default_ttl",
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
            }

            let _cond = { event.get_str("json.zone_authority.expire") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.expire") {
                        if let Some(val) = event.get("json.zone_authority.expire") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.expire".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.expire",
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
            }

            if event.has_value("json.zone_authority.mname") {
                event.rename(
                    "json.zone_authority.mname",
                    "infoblox_bloxone_ddi.dns_config.zone_authority.mname",
                )?;
            }

            let _cond = { event.get_str("json.zone_authority.negative_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.negative_ttl") {
                        if let Some(val) = event.get("json.zone_authority.negative_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.negative_ttl".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.negative_ttl",
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
            }

            if event.has_value("json.zone_authority.protocol_mname") {
                event.rename(
                    "json.zone_authority.protocol_mname",
                    "infoblox_bloxone_ddi.dns_config.zone_authority.protocol.mname",
                )?;
            }

            if event.has_value("json.zone_authority.protocol_rname") {
                event.rename(
                    "json.zone_authority.protocol_rname",
                    "infoblox_bloxone_ddi.dns_config.zone_authority.protocol.rname",
                )?;
            }

            let _cond = { event.get_str("json.zone_authority.refresh") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.refresh") {
                        if let Some(val) = event.get("json.zone_authority.refresh") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.refresh".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.refresh",
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
            }

            let _cond = { event.get_str("json.zone_authority.retry") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.retry") {
                        if let Some(val) = event.get("json.zone_authority.retry") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.retry".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.retry",
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
            }

            if event.has_value("json.zone_authority.rname") {
                event.rename(
                    "json.zone_authority.rname",
                    "infoblox_bloxone_ddi.dns_config.zone_authority.rname",
                )?;
            }

            let _cond = { event.get_str("json.zone_authority.use_default_mname") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.zone_authority.use_default_mname") {
                        if let Some(val) = event.get("json.zone_authority.use_default_mname") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.zone_authority.use_default_mname".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_config.zone_authority.use_default_mname",
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
            }

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
                    event.remove("infoblox_bloxone_ddi.dns_config.updated_at");
                    event.remove("infoblox_bloxone_ddi.dns_config.lame_ttl");
                    event.remove("infoblox_bloxone_ddi.dns_config.created_at");
                    event.remove("infoblox_bloxone_ddi.dns_config.id");
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) { if (object == null || object == '') { return true; } else if (object instanceof Map) { ((Map) object).values().removeIf(value -> dropEmptyFields(value)); return (((Map) object).size() == 0); } else if (object instanceof List) { ((List) object).removeIf(value -> dropEmptyFields(value)); return (((List) object).length == 0); } return false; } dropEmptyFields(ctx);
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
