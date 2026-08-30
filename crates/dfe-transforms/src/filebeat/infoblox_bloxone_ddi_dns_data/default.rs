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

            if event.has_value("json.absolute_name_spec") {
                event.rename(
                    "json.absolute_name_spec",
                    "infoblox_bloxone_ddi.dns_data.absolute_name.spec",
                )?;
            }

            if event.has_value("json.absolute_zone_name") {
                event.rename(
                    "json.absolute_zone_name",
                    "infoblox_bloxone_ddi.dns_data.absolute_zone.name",
                )?;
            }

            if event.has_value("json.comment") {
                event.rename("json.comment", "infoblox_bloxone_ddi.dns_data.comment")?;
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
                                event.set("infoblox_bloxone_ddi.dns_data.created_at", parsed)?
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
                    .get("infoblox_bloxone_ddi.dns_data.created_at")
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.delegation") {
                event.rename(
                    "json.delegation",
                    "infoblox_bloxone_ddi.dns_data.delegation",
                )?;
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
                            event.set("infoblox_bloxone_ddi.dns_data.disabled", converted)?;
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

            if event.has_value("json.dns_absolute_name_spec") {
                event.rename(
                    "json.dns_absolute_name_spec",
                    "infoblox_bloxone_ddi.dns_data.absolute.name.spec",
                )?;
            }

            if event.has_value("json.dns_absolute_zone_name") {
                event.rename(
                    "json.dns_absolute_zone_name",
                    "infoblox_bloxone_ddi.dns_data.absolute.zone.name",
                )?;
            }

            if event.has_value("json.dns_name_in_zone") {
                event.rename(
                    "json.dns_name_in_zone",
                    "infoblox_bloxone_ddi.dns_data.name_in.zone",
                )?;
            }

            if event.has_value("json.dns_rdata") {
                event.rename(
                    "json.dns_rdata",
                    "infoblox_bloxone_ddi.dns_data.rdata_value",
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "infoblox_bloxone_ddi.dns_data.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("infoblox_bloxone_ddi.dns_data.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.inheritance_sources.ttl.action") {
                event.rename(
                    "json.inheritance_sources.ttl.action",
                    "infoblox_bloxone_ddi.dns_data.inheritance.sources.ttl.action",
                )?;
            }

            if event.has_value("json.inheritance_sources.ttl.display_name") {
                event.rename(
                    "json.inheritance_sources.ttl.display_name",
                    "infoblox_bloxone_ddi.dns_data.inheritance.sources.ttl.display.name",
                )?;
            }

            if event.has_value("json.inheritance_sources.ttl.source") {
                event.rename(
                    "json.inheritance_sources.ttl.source",
                    "infoblox_bloxone_ddi.dns_data.inheritance.sources.ttl.source",
                )?;
            }

            let _cond = { event.get_str("json.inheritance_sources.ttl.value") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.inheritance_sources.ttl.value") {
                        if let Some(val) = event.get("json.inheritance_sources.ttl.value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.inheritance_sources.ttl.value".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_data.inheritance.sources.ttl.value",
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

            if event.has_value("json.name_in_zone") {
                event.rename(
                    "json.name_in_zone",
                    "infoblox_bloxone_ddi.dns_data.name_in_zone",
                )?;
            }

            let _cond = { event.get("json.options").is_some_and(|v| v.is_string()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "json.options", "json.options")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
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

            let _cond = { event.get_str("json.options.create_ptr") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.options.create_ptr") {
                        if let Some(val) = event.get("json.options.create_ptr") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.options.create_ptr".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_data.options.create_ptr",
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

            let _cond = { event.get_str("json.options.check_rmz") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.options.check_rmz") {
                        if let Some(val) = event.get("json.options.check_rmz") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.options.check_rmz".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_data.options.check_rmz",
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

            let _cond = { event.get_str("json.options.address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.options.address") {
                        if let Some(val) = event.get("json.options.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.options.address".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_data.options.address", converted)?;
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
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("infoblox_bloxone_ddi.dns_data.options.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.provider_metadata") {
                event.rename(
                    "json.provider_metadata",
                    "infoblox_bloxone_ddi.dns_data.provider_metadata",
                )?;
            }

            let _cond = { event.get_str("json.rdata.address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.address") {
                        if let Some(val) = event.get("json.rdata.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.address".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.address", converted)?;
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
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("infoblox_bloxone_ddi.dns_data.rdata.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.rdata.flags") {
                event.rename(
                    "json.rdata.flags",
                    "infoblox_bloxone_ddi.dns_data.rdata.flags",
                )?;
            }

            if event.has_value("json.rdata.tag") {
                event.rename("json.rdata.tag", "infoblox_bloxone_ddi.dns_data.rdata.tag")?;
            }

            if event.has_value("json.rdata.value") {
                event.rename(
                    "json.rdata.value",
                    "infoblox_bloxone_ddi.dns_data.rdata.value",
                )?;
            }

            if event.has_value("json.rdata.cname") {
                event.rename(
                    "json.rdata.cname",
                    "infoblox_bloxone_ddi.dns_data.rdata.cname",
                )?;
            }

            if event.has_value("json.rdata.target") {
                event.rename(
                    "json.rdata.target",
                    "infoblox_bloxone_ddi.dns_data.rdata.target",
                )?;
            }

            if event.has_value("json.rdata.dhcid") {
                event.rename(
                    "json.rdata.dhcid",
                    "infoblox_bloxone_ddi.dns_data.rdata.dhcid",
                )?;
            }

            if event.has_value("json.rdata.exchange") {
                event.rename(
                    "json.rdata.exchange",
                    "infoblox_bloxone_ddi.dns_data.rdata.exchange",
                )?;
            }

            let _cond = { event.get_str("json.rdata.preference") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.preference") {
                        if let Some(val) = event.get("json.rdata.preference") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.preference".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("infoblox_bloxone_ddi.dns_data.rdata.preference", converted)?;
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

            let _cond = { event.get_str("json.rdata.order") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.order") {
                        if let Some(val) = event.get("json.rdata.order") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.order".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.order", converted)?;
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

            if event.has_value("json.rdata.regexp") {
                event.rename(
                    "json.rdata.regexp",
                    "infoblox_bloxone_ddi.dns_data.rdata.regexp",
                )?;
            }

            if event.has_value("json.rdata.replacement") {
                event.rename(
                    "json.rdata.replacement",
                    "infoblox_bloxone_ddi.dns_data.rdata.replacement",
                )?;
            }

            if event.has_value("json.rdata.services") {
                event.rename(
                    "json.rdata.services",
                    "infoblox_bloxone_ddi.dns_data.rdata.services",
                )?;
            }

            if event.has_value("json.rdata.dname") {
                event.rename(
                    "json.rdata.dname",
                    "infoblox_bloxone_ddi.dns_data.rdata.dname",
                )?;
            }

            let _cond = { event.get_str("json.rdata.expire") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.expire") {
                        if let Some(val) = event.get("json.rdata.expire") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.expire".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.expire", converted)?;
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

            if event.has_value("json.rdata.mname") {
                event.rename(
                    "json.rdata.mname",
                    "infoblox_bloxone_ddi.dns_data.rdata.mname",
                )?;
            }

            let _cond = { event.get_str("json.rdata.negative_ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.negative_ttl") {
                        if let Some(val) = event.get("json.rdata.negative_ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.negative_ttl".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_data.rdata.negative_ttl",
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

            let _cond = { event.get_str("json.rdata.refresh") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.refresh") {
                        if let Some(val) = event.get("json.rdata.refresh") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.refresh".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.refresh", converted)?;
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

            let _cond = { event.get_str("json.rdata.retry") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.retry") {
                        if let Some(val) = event.get("json.rdata.retry") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.retry".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.retry", converted)?;
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

            if event.has_value("json.rdata.rname") {
                event.rename(
                    "json.rdata.rname",
                    "infoblox_bloxone_ddi.dns_data.rdata.rname",
                )?;
            }

            let _cond = { event.get_str("json.rdata.serial") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.serial") {
                        if let Some(val) = event.get("json.rdata.serial") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.serial".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.serial", converted)?;
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

            let _cond = { event.get_str("json.rdata.port") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.port") {
                        if let Some(val) = event.get("json.rdata.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.port".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.port", converted)?;
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

            let _cond = { event.get_str("json.rdata.priority") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.priority") {
                        if let Some(val) = event.get("json.rdata.priority") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.priority".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.priority", converted)?;
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

            let _cond = { event.get_str("json.rdata.weight") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.weight") {
                        if let Some(val) = event.get("json.rdata.weight") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.weight".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.rdata.weight", converted)?;
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

            if event.has_value("json.rdata.text") {
                event.rename(
                    "json.rdata.text",
                    "infoblox_bloxone_ddi.dns_data.rdata.text",
                )?;
            }

            if event.has_value("json.rdata.type") {
                event.rename(
                    "json.rdata.type",
                    "infoblox_bloxone_ddi.dns_data.rdata.type",
                )?;
            }

            let _cond = { event.get_str("json.rdata.length_kind") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.rdata.length_kind") {
                        if let Some(val) = event.get("json.rdata.length_kind") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.rdata.length_kind".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "infoblox_bloxone_ddi.dns_data.rdata.length_kind",
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

            if event.has_value("json.tags") {
                event.rename("json.tags", "infoblox_bloxone_ddi.dns_data.tags")?;
            }

            if event.has_value("json.source") {
                event.rename("json.source", "infoblox_bloxone_ddi.dns_data.source")?;
            }

            let _cond = { event.get_str("json.ttl") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ttl") {
                        if let Some(val) = event.get("json.ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ttl".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dns_data.ttl", converted)?;
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

            if event.has_value("json.type") {
                event.rename("json.type", "infoblox_bloxone_ddi.dns_data.type")?;
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
                                event.set("infoblox_bloxone_ddi.dns_data.updated_at", parsed)?
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
                    .get("infoblox_bloxone_ddi.dns_data.updated_at")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.view") {
                event.rename("json.view", "infoblox_bloxone_ddi.dns_data.view")?;
            }

            if event.has_value("json.view_name") {
                event.rename("json.view_name", "infoblox_bloxone_ddi.dns_data.view_name")?;
            }

            if event.has_value("json.zone") {
                event.rename("json.zone", "infoblox_bloxone_ddi.dns_data.zone")?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.rdata_value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.answers.data", v)?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.ttl")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.answers.ttl", v)?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.answers.type", v)?;
            }

            let _cond = { event.has_value("dns.answers") };
            if _cond {
                // Painless script
                // Source: def a = new ArrayList();\na.add(ctx.dns.answers);\nctx.dns.answers = a;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def a = new ArrayList();\na.add(ctx.dns.answers);\nctx.dns.answers = a;\n"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.absolute.name.spec")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.absolute.zone.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.registered_domain", v)?;
            }

            if let Some(v) = event
                .get("infoblox_bloxone_ddi.dns_data.name_in.zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.subdomain", v)?;
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
                    event.remove("infoblox_bloxone_ddi.dns_data.updated_at");
                    event.remove("infoblox_bloxone_ddi.dns_data.lame_ttl");
                    event.remove("infoblox_bloxone_ddi.dns_data.created_at");
                    event.remove("infoblox_bloxone_ddi.dns_data.id");
                    event.remove("infoblox_bloxone_ddi.dns_data.type");
                    event.remove("infoblox_bloxone_ddi.dns_data.absolute.name.spec");
                    event.remove("infoblox_bloxone_ddi.dns_data.absolute.zone.name");
                    event.remove("infoblox_bloxone_ddi.dns_data.name_in.zone");
                    event.remove("infoblox_bloxone_ddi.dns_data.rdata_value");
                    Ok(())
                })();
            }

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
