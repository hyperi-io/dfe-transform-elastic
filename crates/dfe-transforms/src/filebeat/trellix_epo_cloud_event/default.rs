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

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

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
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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
                            .get("_ingest.pipeline")
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
                event.has_value("json.data")
                    && event.get("json.data").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.attributes.detectedutc") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.attributes.receivedutc") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.attributes.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("json.attributes.timestamp")
                    && event.get_str("json.attributes.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("json.attributes.timestamp")
                    && event.get_str("json.attributes.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("trellix_epo_cloud.event.attributes.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.targethostname") {
                event.rename(
                    "json.attributes.targethostname",
                    "trellix_epo_cloud.event.attributes.target.hostname",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.target.hostname")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.target.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.domain") {
                    if let Some(domain_str) = event.get_string("destination.domain") {
                        let domain = domain_str.to_string();
                        event.set("destination.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("destination.registered_domain", json!(registered))?;
                            }
                            event
                                .set("destination.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("destination.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "registered_domain_for_destination_domain",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.targetipv4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.targetipv4") {
                        if let Some(val) = event.get("json.attributes.targetipv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.targetipv4".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.event.attributes.target.ipv4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_targetipv4_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.target.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.target.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trellix_epo_cloud.event.attributes.target.ipv4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.targetipv6") {
                    gsub_field(
                        event,
                        "json.attributes.targetipv6",
                        "json.attributes.targetipv6",
                        cached_regex!("/"),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_targetipv6_remove_forward_slash",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.targetipv6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.targetipv6") {
                        if let Some(val) = event.get("json.attributes.targetipv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.targetipv6".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.event.attributes.target.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_targetipv6_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.target.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.target.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.target.ipv6") };
            if _cond {
                event.append_unique(
                    "destination.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.target.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !(event
                    .get_str("json.attributes.targetmac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.targetmac") {
                        gsub_field(
                            event,
                            "json.attributes.targetmac",
                            "json.attributes.targetmac",
                            cached_regex!("(..)(?!$)"),
                            "$1-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_targetmac")?;
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
                                .get("_ingest.pipeline")
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
                !(event
                    .get_str("json.attributes.targetmac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if event.has_value("json.attributes.targetmac") {
                    map_strings(
                        event,
                        "json.attributes.targetmac",
                        "json.attributes.targetmac",
                        str::to_uppercase,
                    )?;
                }
            }

            if event.has_value("json.attributes.targetmac") {
                event.rename(
                    "json.attributes.targetmac",
                    "trellix_epo_cloud.event.attributes.target.mac",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.target.mac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.target.mac")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.mac", v)?;
                }
            }

            let _cond = { event.get_str("json.attributes.targetport") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.targetport") {
                        if let Some(val) = event.get("json.attributes.targetport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.targetport".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.event.attributes.target.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_targetport_to_long",
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
                                .get("_ingest.pipeline")
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
                .get("trellix_epo_cloud.event.attributes.target.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.attributes.targetusername") {
                event.rename(
                    "json.attributes.targetusername",
                    "trellix_epo_cloud.event.attributes.target.user_name",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.target.user_name")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.target.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.user.name", v)?;
                }
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "trellix_epo_cloud.event.id")?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.id")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
            }

            if event.has_value("json.links.self") {
                event.rename("json.links.self", "trellix_epo_cloud.event.links.self")?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.links.self")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.links.self")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reference", v)?;
                }
            }

            let _cond = { event.get_str("json.attributes.threatseverity") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.threatseverity") {
                        if let Some(val) = event.get("json.attributes.threatseverity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.threatseverity".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.event.attributes.threat.severity",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_threatseverity_to_long",
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
                                .get("_ingest.pipeline")
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
                .get("trellix_epo_cloud.event.attributes.threat.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if event.has_value("json.attributes.targetfilename") {
                event.rename(
                    "json.attributes.targetfilename",
                    "trellix_epo_cloud.event.attributes.target.file_name",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.target.file_name")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.target.file_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
            }

            if event.has_value("json.attributes.sourcefilepath") {
                event.rename(
                    "json.attributes.sourcefilepath",
                    "trellix_epo_cloud.event.attributes.source.file_path",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.file_path")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.file_path")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.path", v)?;
                }
            }

            if event.has_value("json.attributes.sourceprocesshash") {
                event.rename(
                    "json.attributes.sourceprocesshash",
                    "trellix_epo_cloud.event.attributes.source.process.hash",
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.source.process.hash")
                    && !(event
                        .get_str("trellix_epo_cloud.event.attributes.source.process.hash")
                        .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.process.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.source.process.hash")
                    && event
                        .get_as_string("trellix_epo_cloud.event.attributes.source.process.hash")
                        .is_some_and(|s| s.len() == 32)
            };
            if _cond {
                event.set(
                    "process.hash.md5",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.process.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.source.process.hash")
                    && event
                        .get_as_string("trellix_epo_cloud.event.attributes.source.process.hash")
                        .is_some_and(|s| s.len() == 40)
            };
            if _cond {
                event.set(
                    "process.hash.sha1",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.process.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.source.process.hash")
                    && event
                        .get_as_string("trellix_epo_cloud.event.attributes.source.process.hash")
                        .is_some_and(|s| s.len() == 64)
            };
            if _cond {
                event.set(
                    "process.hash.sha256",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.process.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.source.process.hash")
                    && event
                        .get_as_string("trellix_epo_cloud.event.attributes.source.process.hash")
                        .is_some_and(|s| s.len() == 128)
            };
            if _cond {
                event.set(
                    "process.hash.sha512",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.process.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attributes.sourceprocessname") {
                event.rename(
                    "json.attributes.sourceprocessname",
                    "trellix_epo_cloud.event.attributes.source.process.name",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.process.name")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.process.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
            }

            if event.has_value("json.attributes.sourceurl") {
                event.rename(
                    "json.attributes.sourceurl",
                    "trellix_epo_cloud.event.attributes.source.url",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.url")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.address", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(domain_str) = event.get_string("source.address") {
                        let domain = domain_str.to_string();
                        event.set("source.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("source.registered_domain", json!(registered))?;
                            }
                            event.set("source.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("source.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "registered_domain_for_source_address",
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
                            .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.sourcehostname") {
                event.rename(
                    "json.attributes.sourcehostname",
                    "trellix_epo_cloud.event.attributes.source.hostname",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.hostname")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.domain", v)?;
                }
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.domain") {
                    if let Some(domain_str) = event.get_string("source.domain") {
                        let domain = domain_str.to_string();
                        event.set("source.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("source.registered_domain", json!(registered))?;
                            }
                            event.set("source.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("source.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "registered_domain_for_source_domain",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.sourceipv4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.sourceipv4") {
                        if let Some(val) = event.get("json.attributes.sourceipv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.sourceipv4".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.event.attributes.source.ipv4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sourceipv4_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.source.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trellix_epo_cloud.event.attributes.source.ipv4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.sourceipv6") {
                    gsub_field(
                        event,
                        "json.attributes.sourceipv6",
                        "json.attributes.sourceipv6",
                        cached_regex!("/"),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_sourceipv6_remove_forward_slash",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.sourceipv6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.sourceipv6") {
                        if let Some(val) = event.get("json.attributes.sourceipv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.sourceipv6".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("trellix_epo_cloud.event.attributes.source.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sourceipv6_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.source.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.source.ipv6") };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.source.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !(event
                    .get_str("json.attributes.sourcemac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.sourcemac") {
                        gsub_field(
                            event,
                            "json.attributes.sourcemac",
                            "json.attributes.sourcemac",
                            cached_regex!("(..)(?!$)"),
                            "$1-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_sourcemac")?;
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
                                .get("_ingest.pipeline")
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
                !(event
                    .get_str("json.attributes.sourcemac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if event.has_value("json.attributes.sourcemac") {
                    map_strings(
                        event,
                        "json.attributes.sourcemac",
                        "json.attributes.sourcemac",
                        str::to_uppercase,
                    )?;
                }
            }

            if event.has_value("json.attributes.sourcemac") {
                event.rename(
                    "json.attributes.sourcemac",
                    "trellix_epo_cloud.event.attributes.source.mac",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.mac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.mac")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.mac", v)?;
                }
            }

            if event.has_value("json.attributes.sourceusername") {
                event.rename(
                    "json.attributes.sourceusername",
                    "trellix_epo_cloud.event.attributes.source.user_name",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.source.user_name")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.source.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.name", v)?;
                }
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.attributes.threatactiontaken") {
                event.rename(
                    "json.attributes.threatactiontaken",
                    "trellix_epo_cloud.event.attributes.threat.action_taken",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("trellix_epo_cloud.event.attributes.threat.action_taken")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if let Some(v) = event
                    .get("trellix_epo_cloud.event.attributes.threat.action_taken")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.description", v)?;
                }
            }

            if event.has_value("json.attributes.agentguid") {
                event.rename(
                    "json.attributes.agentguid",
                    "trellix_epo_cloud.event.attributes.agent.guid",
                )?;
            }

            if event.has_value("json.attributes.analyzer") {
                event.rename(
                    "json.attributes.analyzer",
                    "trellix_epo_cloud.event.attributes.analyzer.value",
                )?;
            }

            if event.has_value("json.attributes.analyzerdatversion") {
                event.rename(
                    "json.attributes.analyzerdatversion",
                    "trellix_epo_cloud.event.attributes.analyzer.dat_version",
                )?;
            }

            if event.has_value("json.attributes.analyzerdetectionmethod") {
                event.rename(
                    "json.attributes.analyzerdetectionmethod",
                    "trellix_epo_cloud.event.attributes.analyzer.detection_method",
                )?;
            }

            if event.has_value("json.attributes.analyzerengineversion") {
                event.rename(
                    "json.attributes.analyzerengineversion",
                    "trellix_epo_cloud.event.attributes.analyzer.engine_version",
                )?;
            }

            if event.has_value("json.attributes.analyzerhostname") {
                event.rename(
                    "json.attributes.analyzerhostname",
                    "trellix_epo_cloud.event.attributes.analyzer.hostname",
                )?;
            }

            let _cond = {
                event.has_value("trellix_epo_cloud.event.attributes.analyzer.hostname")
                    && !(event
                        .get_str("trellix_epo_cloud.event.attributes.analyzer.hostname")
                        .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.analyzer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trellix_epo_cloud.event.attributes.analyzer.hostname") {
                    if let Some(domain_str) =
                        event.get_string("trellix_epo_cloud.event.attributes.analyzer.hostname")
                    {
                        let domain = domain_str.to_string();
                        event.set(
                            "trellix_epo_cloud.event.attributes.analyzer.domain",
                            json!(domain.clone()),
                        )?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set(
                                    "trellix_epo_cloud.event.attributes.analyzer.registered_domain",
                                    json!(registered),
                                )?;
                            }
                            event.set(
                                "trellix_epo_cloud.event.attributes.analyzer.top_level_domain",
                                json!(rd.top_level_domain),
                            )?;
                            if let Some(sub) = rd.subdomain {
                                event.set(
                                    "trellix_epo_cloud.event.attributes.analyzer.subdomain",
                                    json!(sub),
                                )?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "registered_domain_for_analyzer_hostname",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.analyzeripv4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.analyzeripv4") {
                        if let Some(val) = event.get("json.attributes.analyzeripv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.analyzeripv4".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.event.attributes.analyzer.ipv4",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_analyzeripv4_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.analyzer.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.analyzer.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.analyzeripv6") {
                    gsub_field(
                        event,
                        "json.attributes.analyzeripv6",
                        "json.attributes.analyzeripv6",
                        cached_regex!("/"),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_analyzeripv6_remove_forward_slash",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.analyzeripv6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.analyzeripv6") {
                        if let Some(val) = event.get("json.attributes.analyzeripv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.analyzeripv6".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.event.attributes.analyzer.ipv6",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_analyzeripv6_to_ip",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("trellix_epo_cloud.event.attributes.analyzer.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trellix_epo_cloud.event.attributes.analyzer.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !(event
                    .get_str("json.attributes.analyzermac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.analyzermac") {
                        gsub_field(
                            event,
                            "json.attributes.analyzermac",
                            "json.attributes.analyzermac",
                            cached_regex!("(..)(?!$)"),
                            "$1-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_analyzermac")?;
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
                                .get("_ingest.pipeline")
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
                !(event
                    .get_str("json.attributes.analyzermac")
                    .is_some_and(|s| s.to_lowercase() == "none"))
            };
            if _cond {
                if event.has_value("json.attributes.analyzermac") {
                    map_strings(
                        event,
                        "json.attributes.analyzermac",
                        "json.attributes.analyzermac",
                        str::to_uppercase,
                    )?;
                }
            }

            if event.has_value("json.attributes.analyzermac") {
                event.rename(
                    "json.attributes.analyzermac",
                    "trellix_epo_cloud.event.attributes.analyzer.mac",
                )?;
            }

            if event.has_value("json.attributes.analyzername") {
                event.rename(
                    "json.attributes.analyzername",
                    "trellix_epo_cloud.event.attributes.analyzer.name",
                )?;
            }

            if event.has_value("json.attributes.analyzerversion") {
                event.rename(
                    "json.attributes.analyzerversion",
                    "trellix_epo_cloud.event.attributes.analyzer.version",
                )?;
            }

            if event.has_value("json.attributes.autoguid") {
                event.rename(
                    "json.attributes.autoguid",
                    "trellix_epo_cloud.event.attributes.auto_guid",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.detectedutc")
                    && event.get_str("json.attributes.detectedutc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.detectedutc") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("trellix_epo_cloud.event.attributes.detected_utc", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.detectedutc".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_detectedutc")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.nodepath") {
                event.rename(
                    "json.attributes.nodepath",
                    "trellix_epo_cloud.event.attributes.node.path",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.receivedutc")
                    && event.get_str("json.attributes.receivedutc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.receivedutc") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("trellix_epo_cloud.event.attributes.received_utc", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.receivedutc".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_receivedutc")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.sourceprocesssigned") {
                event.rename(
                    "json.attributes.sourceprocesssigned",
                    "trellix_epo_cloud.event.attributes.source.process.signed",
                )?;
            }

            if event.has_value("json.attributes.sourceprocesssigner") {
                event.rename(
                    "json.attributes.sourceprocesssigner",
                    "trellix_epo_cloud.event.attributes.source.process.signer",
                )?;
            }

            if event.has_value("json.attributes.targethash") {
                event.rename(
                    "json.attributes.targethash",
                    "trellix_epo_cloud.event.attributes.target.hash",
                )?;
            }

            if event.has_value("json.attributes.targetprocessname") {
                event.rename(
                    "json.attributes.targetprocessname",
                    "trellix_epo_cloud.event.attributes.target.process_name",
                )?;
            }

            if event.has_value("json.attributes.targetprotocol") {
                event.rename(
                    "json.attributes.targetprotocol",
                    "trellix_epo_cloud.event.attributes.target.protocol",
                )?;
            }

            if event.has_value("json.attributes.threatcategory") {
                event.rename(
                    "json.attributes.threatcategory",
                    "trellix_epo_cloud.event.attributes.threat.category",
                )?;
            }

            let _cond = { event.get_str("json.attributes.threateventid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.threateventid") {
                        if let Some(val) = event.get("json.attributes.threateventid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.threateventid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.event.attributes.threat.event.id",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_threateventid_to_string",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.attributes.threathandled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.threathandled") {
                        if let Some(val) = event.get("json.attributes.threathandled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.threathandled".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "trellix_epo_cloud.event.attributes.threat.handled",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_threathandled_to_boolean",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.attributes.threatname") {
                event.rename(
                    "json.attributes.threatname",
                    "trellix_epo_cloud.event.attributes.threat.name",
                )?;
            }

            if event.has_value("json.attributes.threattype") {
                event.rename(
                    "json.attributes.threattype",
                    "trellix_epo_cloud.event.attributes.threat.type",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "trellix_epo_cloud.type")?;
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
                event.remove("trellix_epo_cloud.event.attributes.timestamp");
                event.remove("trellix_epo_cloud.event.attributes.target.hostname");
                event.remove("trellix_epo_cloud.event.attributes.target.ipv4");
                event.remove("trellix_epo_cloud.event.attributes.target.ipv6");
                event.remove("trellix_epo_cloud.event.attributes.target.mac");
                event.remove("trellix_epo_cloud.event.attributes.target.port");
                event.remove("trellix_epo_cloud.event.attributes.target.user_name");
                event.remove("trellix_epo_cloud.event.id");
                event.remove("trellix_epo_cloud.event.links.self");
                event.remove("trellix_epo_cloud.event.attributes.threat.severity");
                event.remove("trellix_epo_cloud.event.attributes.target.file_name");
                event.remove("trellix_epo_cloud.event.attributes.source.file_path");
                event.remove("trellix_epo_cloud.event.attributes.source.process.name");
                event.remove("trellix_epo_cloud.event.attributes.source.url");
                event.remove("trellix_epo_cloud.event.attributes.source.hostname");
                event.remove("trellix_epo_cloud.event.attributes.source.ipv4");
                event.remove("trellix_epo_cloud.event.attributes.source.ipv6");
                event.remove("trellix_epo_cloud.event.attributes.source.mac");
                event.remove("trellix_epo_cloud.event.attributes.source.user_name");
                event.remove("trellix_epo_cloud.event.attributes.threat.action_taken");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '' || object == ' ') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '' || object == ' ') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
