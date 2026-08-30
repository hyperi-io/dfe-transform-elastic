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

            event.set("event.kind", json!("enrichment"))?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.response")
                    && event.get("json.response").is_some_and(|v| match v {
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
                if let Some(v) = event.get("json.Event.Attribute.first_seen") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Event.Attribute.uuid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Event.Object.Attribute.uuid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Event.timestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.Event") {
                event.rename("json.Event", "misp.event")?;
            }

            let _cond = { event.get_str("misp.event.Orgc.local") != Some("false") };
            if _cond {
                event.set("threat.indicator.provider", json!("misp"))?;
            }

            let _cond = { event.get_str("misp.event.Orgc.local") == Some("false") };
            if _cond {
                let v = json!(
                    event
                        .get("misp.event.Orgc.name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.provider", v)?;
                }
            }

            event.remove("misp.event.ShadowAttribute");
            event.remove("misp.event.RelatedEvent");
            event.remove("misp.event.Galaxy");
            event.remove("misp.event.Attribute.Galaxy");
            event.remove("misp.event.Attribute.ShadowAttribute");
            event.remove("misp.event.EventReport");
            event.remove("misp.event.Object.Attribute.Galaxy");
            event.remove("misp.event.Object.Attribute.ShadowAttribute");
            event.remove("misp.event.Object.ObjectReference");

            let _cond = {
                event.has_value("misp.event.Attribute") && event.get("misp.event.Attribute").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                event.remove("misp.event.Attribute");
            }

            let _cond = {
                event.has_value("misp.event.Object") && event.get("misp.event.Object").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                event.remove("misp.event.Object");
            }

            let _cond = { event.has_value("misp.event.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.event.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.event.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_timestamp")?;
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

            let _cond = { event.has_value("misp.event.publish_timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.event.publish_timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.event.publish_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.event.publish_timestamp".into(),
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

            if event.has_value("misp.event.Attribute") {
                event.rename("misp.event.Attribute", "misp.attribute")?;
            }

            if event.has_value("misp.event.Object") {
                event.rename("misp.event.Object", "misp.object")?;
            }

            if event.has_value("misp.object.Attribute") {
                event.rename("misp.object.Attribute", "misp.object.attribute")?;
            }

            if event.has_value("misp.object.meta-category") {
                event.rename("misp.object.meta-category", "misp.object.meta_category")?;
            }

            if event.has_value("misp.event.Orgc") {
                event.rename("misp.event.Orgc", "misp.orgc")?;
            }

            if event.has_value("misp.event.Org") {
                event.rename("misp.event.Org", "misp.org")?;
            }

            if event.has_value("misp.event.Tag") {
                event.rename("misp.event.Tag", "misp.tag")?;
            }

            let _cond = { event.has_value("misp.object") };
            if _cond {
                if event.has_value("misp.attribute") {
                    event.rename("misp.attribute", "misp.context.attribute")?;
                }
            }

            let _cond = { event.has_value("misp.object") };
            if _cond {
                if event.has_value("misp.object.attribute") {
                    event.rename("misp.object.attribute", "misp.attribute")?;
                }
            }

            let _cond = { event.has_value("misp.attribute.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.attribute.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.attribute.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.attribute.timestamp".into(),
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
                        "date_attribute_timestamp",
                    )?;
                    event.remove("misp.attribute.timestamp");
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

            let _cond = { event.has_value("misp.context.attribute.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.context.attribute.timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("misp.context.attribute.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.context.attribute.timestamp".into(),
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
                        "date_context_attribute_timestamp",
                    )?;
                    event.remove("misp.context.attribute.timestamp");
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

            let _cond = { event.has_value("misp.object.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("misp.object.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("misp.object.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "misp.object.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_object_timestamp")?;
                    event.remove("misp.object.timestamp");
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

            event.set("threat.feed.name", json!("MISP"))?;

            if event.has_value("misp.attribute.first_seen") {
                event.rename("misp.attribute.first_seen", "threat.indicator.first_seen")?;
            }

            if event.has_value("misp.attribute.last_seen") {
                event.rename("misp.attribute.last_seen", "threat.indicator.last_seen")?;
            }

            if event.has_value("misp.event.analysis") {
                if let Some(val) = event.get("misp.event.analysis") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.analysis".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.scanner_stats", converted)?;
                }
            }

            if event.has_value("misp.event.threat_level_id") {
                if let Some(val) = event.get("misp.event.threat_level_id") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.threat_level_id".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.threat_level_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ([
                        "md5",
                        "impfuzzy",
                        "imphash",
                        "pehash",
                        "sha1",
                        "sha224",
                        "sha256",
                        "sha3-224",
                        "sha3-256",
                        "sha3-384",
                        "sha3-512",
                        "sha384",
                        "sha512",
                        "sha512/224",
                        "sha512/256",
                        "ssdeep",
                        "tlsh",
                        "vhash",
                    ]
                    .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
                        || event
                            .get_str("misp.attribute.type")
                            .is_some_and(|s| s.starts_with("filename")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("file")
                    && event.has_value("misp.attribute.type")
                    && !(event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename")))
            };
            if _cond {
                if let Some(from) = resolve_path(event, "misp.attribute.value")
                    && let Some(to) = resolve_path(
                        event,
                        "threat.indicator.file.hash.{{{misp.attribute.type}}}",
                    )
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("file")
                    && event.get_str("misp.attribute.type") == Some("filename")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.file.name")?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
            };
            if _cond {
                if event.has_value("misp.attribute.type") {
                    if let Some(input) = event.get_string("misp.attribute.type") {
                        // Grok pattern: %{WORD}\\|%{WORD:_tmp.hashtype}
                        let _ = cached_grok!("%{WORD}\\|%{WORD:_tmp.hashtype}")
                            .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.file.name}\\|%{GREEDYDATA:_tmp.hashvalue}
                        let _ = cached_grok!(
                            "%{DATA:threat.indicator.file.name}\\|%{GREEDYDATA:_tmp.hashvalue}"
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("filename|"))
                    && event.has_value("_tmp.hashvalue")
                    && event.has_value("_tmp.hashtype")
            };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.{{{_tmp.hashtype}}}",
                    json!(
                        event
                            .get("_tmp.hashvalue")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["url", "link", "uri"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("url")
                    && event.get_str("misp.attribute.type") != Some("uri")
            };
            if _cond {
                uri_parts(
                    event,
                    "misp.attribute.value",
                    "threat.indicator.url",
                    true,
                    true,
                )?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("url")
                    && event.get_str("misp.attribute.type") != Some("uri")
            };
            if _cond {
                let v = json!(
                    event
                        .get("threat.indicator.url.original")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("regkey"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("windows-registry-key"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("windows-registry-key")
                    && event.get_str("misp.attribute.type") == Some("regkey")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.registry.key")?;
                }
            }

            let _cond = { event.get_str("misp.attribute.type") == Some("regkey|value") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.registry.key}\\|%{DATA:threat.indicator.registry.value}
                        let _ = cached_grok!("%{DATA:threat.indicator.registry.key}\\|%{DATA:threat.indicator.registry.value}").extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("misp.attribute.type") == Some("AS")
            };
            if _cond {
                event.set("threat.indicator.type", json!("autonomous-system"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("autonomous-system") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(val) = event.get("misp.attribute.value") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "misp.attribute.value".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.as.number", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && (event.get_str("misp.attribute.type") == Some("hostname")
                        || event
                            .get_str("misp.attribute.type")
                            .is_some_and(|s| s.starts_with("domain")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["ip-src", "ip-src|port", "ip-dst", "ip-dst|port"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("threat.indicator.type") == Some("domain-name")
                    && event.get_str("misp.attribute.type") != Some("domain|ip")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.url.domain")?;
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event.get_str("threat.indicator.type") == Some("ipv4-addr")
                    && !(["domain|ip", "ip-src|port", "ip-dst|port"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or("")))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.ip")?;
                }
            }

            let _cond = {
                event.get_str("misp.attribute.type") == Some("domain|ip")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{DATA:threat.indicator.url.domain}\\|%{IP:threat.indicator.ip}
                        let _ = cached_grok!(
                            "%{DATA:threat.indicator.url.domain}\\|%{IP:threat.indicator.ip}"
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                ["ip-src|port", "ip-dst|port"]
                    .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    if let Some(input) = event.get_string("misp.attribute.value") {
                        // Grok pattern: %{IP:threat.indicator.ip}\\|%{NUMBER:threat.indicator.port}
                        let _ = cached_grok!(
                            "%{IP:threat.indicator.ip}\\|%{NUMBER:threat.indicator.port}"
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["email-dst", "email-src"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && event
                        .get_str("misp.attribute.type")
                        .is_some_and(|s| s.starts_with("email"))
                    && !(["email-dst", "email-src"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or("")))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-message"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.email.address")?;
                }
            }

            if event.has_value("misp.event.event_creator_email") {
                event.rename("misp.event.event_creator_email", "user.email")?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append("user.roles", json!("reporting_user"))?;
            }

            let _cond = {
                event.has_value("misp.attribute.type")
                    && ["mac-address", "mac-eui-64"]
                        .contains(&event.get_str("misp.attribute.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("mac-addr"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("mac-addr") };
            if _cond {
                if event.has_value("misp.attribute.value") {
                    event.rename("misp.attribute.value", "threat.indicator.mac")?;
                }
            }

            let _cond = { event.has_value("misp.tag") };
            if _cond {
                // Painless script
                // Source: def tags = ctx.misp.tag.stream()\n   .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   .filter(t -> t.startsWith('tlp:'))\n   .map(t -> t.replace('tlp:', '').toUpperCase())\n   .collect(Collectors.toList());\n\nif (ctx.tags == null) {\n  ctx.tags = new ArrayList();\n}\nctx.tags.addAll(tags);\nctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def tags = ctx.misp.tag.stream()\n   .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   .filter(t -> t.startsWith('tlp:'))\n   .map(t -> t.replace('tlp:', '').toUpperCase())\n   .collect(Collectors.toList());\n\nif (ctx.tags == null) {\n  ctx.tags = new ArrayList();\n}\nctx.tags.addAll(tags);\nctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n"#
                    ),
                )?;
            }

            if event.has_value("misp.event.distribution") {
                if let Some(val) = event.get("misp.event.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.distribution", converted)?;
                }
            }

            if event.has_value("misp.object.distribution") {
                if let Some(val) = event.get("misp.object.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.object.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.object.distribution", converted)?;
                }
            }

            if event.has_value("misp.context.event.distribution") {
                if let Some(val) = event.get("misp.context.event.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.context.event.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.context.event.distribution", converted)?;
                }
            }

            if event.has_value("misp.attribute.distribution") {
                if let Some(val) = event.get("misp.attribute.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.attribute.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.attribute.distribution", converted)?;
                }
            }

            if event.has_value("misp.context.attribute.distribution") {
                if let Some(val) = event.get("misp.context.attribute.distribution") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.context.attribute.distribution".into(),
                            message,
                        }
                    })?;
                    event.set("misp.context.attribute.distribution", converted)?;
                }
            }

            if event.has_value("threat.indicator.port") {
                if let Some(val) = event.get("threat.indicator.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "threat.indicator.port".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.port", converted)?;
                }
            }

            if event.has_value("misp.event.attribute_count") {
                if let Some(val) = event.get("misp.event.attribute_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "misp.event.attribute_count".into(),
                            message,
                        }
                    })?;
                    event.set("misp.event.attribute_count", converted)?;
                }
            }

            let _cond = { event.has_value("misp") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                event.remove("misp.attribute.value");
            }

            event.remove("misp.event.Attribute.timestamp");
            event.remove("misp.event.timestamp");
            event.remove("misp.tag");
            event.remove("misp.org");
            event.remove("misp.event.analysis");
            event.remove("_tmp");
            event.remove("json");

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
