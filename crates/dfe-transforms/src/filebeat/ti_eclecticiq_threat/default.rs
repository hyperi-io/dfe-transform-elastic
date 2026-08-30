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
                !([
                    "asn",
                    "domain",
                    "email",
                    "file",
                    "file-size",
                    "hash-md5",
                    "hash-md5",
                    "hash-sha1",
                    "hash-sha256",
                    "hash-sha384",
                    "hash-sha512",
                    "hash-ssdeep",
                    "ipv4",
                    "ipv4-cidr",
                    "ipv6",
                    "ipv6-cidr",
                    "mac-48",
                    "mutex",
                    "port",
                    "process",
                    "process-name",
                    "uri",
                    "winregistry",
                    "certificate-serial-number",
                    "malware",
                    "rule",
                    "user-agent",
                    "organization",
                    "email-subject",
                    "host",
                    "cve",
                ]
                .contains(&event.get_str("json.type").unwrap_or("")))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // SKIPPED: condition not transpiled: ctx.json["source.names"] != null && ctx.json["source.names"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "source.names")?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tags"] != null && ctx.json["meta.tags"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "meta.tags")?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.estimated_observed_time"] != null && ctx.json["meta.estimated_observed_time"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "meta.estimated_observed_time")?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.estimated_threat_start_time"] != null && ctx.json["meta.estimated_threat_start_time"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "meta.estimated_threat_start_time")?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.estimated_threat_end_time"] != null && ctx.json["meta.estimated_threat_end_time"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "meta.estimated_threat_end_time")?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.ingest_time"] != null && ctx.json["meta.ingest_time"] != ""
            #[allow(unreachable_code, unused_variables)]
            if false {
                dot_expand(event, "json", "meta.ingest_time")?;
            }

            let _cond = {
                event.has_value("json.source.names")
                    && event.get_str("json.source.names") != Some("")
            };
            if _cond {
                event.set(
                    "event.provider",
                    json!(
                        event
                            .get("json.source.names")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set(
                "event.url",
                json!(
                    event
                        .get("json.value_url")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.meta.ingest_time")
                    && event.get_str("json.meta.ingest_time") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.meta.ingest_time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.meta.ingest_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.meta.estimated_threat_start_time")
                    && event.get_str("json.meta.estimated_threat_start_time") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.meta.estimated_threat_start_time")
                {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.meta.estimated_threat_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.meta.estimated_threat_end_time")
                    && event.get_str("json.meta.estimated_threat_end_time") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.meta.estimated_threat_end_time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.meta.estimated_threat_end_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                match parse_date_out(
                    &date_str,
                    &[
                        "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                        "yyyy-MM-dd HH:mm:ssz",
                        "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                        "yyyy-MM-dd'T'HH:mm:ssz",
                    ],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = {
                event.has_value("json.meta.tags") && event.get_str("json.meta.tags") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.meta.tags") {
                    let mut parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("tags", Value::Array(parts))?;
                }
            }

            event.set(
                "threat.indicator.name",
                json!(
                    event
                        .get("json.value")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.meta.estimated_observed_time")
                    && event.get_str("json.meta.estimated_observed_time") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.meta.estimated_observed_time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.meta.estimated_observed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.classification"] == "unknown"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.confidence", json!("Not Specified"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.classification"] == "good"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.confidence", json!("None"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.classification"] == "good"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.severity", json!(1))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "low"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.confidence", json!("Low"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "low"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.severity", json!(2))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "medium"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.confidence", json!("Medium"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "medium"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.severity", json!(3))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "high"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.confidence", json!("High"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.confidence"] == "high"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.severity", json!(4))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tlp"] == "WHITE"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.marking.tlp", json!("WHITE"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tlp"] == "NONE"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.marking.tlp", json!("CLEAR"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tlp"] == "GREEN"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.marking.tlp", json!("GREEN"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tlp"] == "AMBER"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.marking.tlp", json!("AMBER"))?;
            }

            // SKIPPED: condition not transpiled: ctx.json["meta.tlp"] == "RED"
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.marking.tlp", json!("RED"))?;
            }

            let _cond = { event.get_str("json.type") == Some("asn") };
            if _cond {
                event.set("threat.indicator.type", json!("autonomous-system"))?;
            }

            let _cond = { event.get_str("json.type") == Some("asn") };
            if _cond {
                event.set(
                    "threat.indicator.as.number",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("asn") };
            if _cond {
                if let Some(val) = event.get("threat.indicator.as.number") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "threat.indicator.as.number".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.as.number", converted)?;
                }
            }

            let _cond = { event.get_str("json.type") == Some("domain") };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = { event.get_str("json.type") == Some("domain") };
            if _cond {
                event.set(
                    "threat.indicator.url.domain",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("email") };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.get_str("json.type") == Some("email") };
            if _cond {
                event.set(
                    "threat.indicator.email.address",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.type") == Some("file")
                    || event.get_str("json.type") == Some("file-size")
                    || event.get_str("json.type") == Some("hash-md5")
                    || event.get_str("json.type") == Some("hash-sha1")
                    || event.get_str("json.type") == Some("hash-sha256")
                    || event.get_str("json.type") == Some("hash-sha384")
                    || event.get_str("json.type") == Some("hash-sha512")
                    || event.get_str("json.type") == Some("hash-ssdeep")
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("json.type") == Some("file") };
            if _cond {
                event.set(
                    "threat.indicator.file.path",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("file-size") };
            if _cond {
                event.set(
                    "threat.indicator.file.size",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("file-size") };
            if _cond {
                if let Some(val) = event.get("threat.indicator.file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "threat.indicator.file.size".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.file.size", converted)?;
                }
            }

            let _cond = { event.get_str("json.type") == Some("hash-md5") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.md5",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("hash-sha1") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.sha1",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("hash-sha256") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.sha256",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("hash-sha384") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.sha384",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("hash-sha512") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.sha512",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("hash-ssdeep") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.ssdeep",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.type") == Some("ipv4")
                    || event.get_str("json.type") == Some("ipv4-cidr")
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = {
                event.get_str("json.type") == Some("ipv6")
                    || event.get_str("json.type") == Some("ipv6-cidr")
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                event.get_str("json.type") == Some("ipv4")
                    || event.get_str("json.type") == Some("ipv6")
            };
            if _cond {
                event.set(
                    "threat.indicator.ip",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("mac-48") };
            if _cond {
                event.set("threat.indicator.type", json!("mac-addr"))?;
            }

            let _cond = { event.get_str("json.type") == Some("mutex") };
            if _cond {
                event.set("threat.indicator.type", json!("mutex"))?;
            }

            let _cond = { event.get_str("json.type") == Some("port") };
            if _cond {
                event.set("threat.indicator.type", json!("port"))?;
            }

            let _cond = { event.get_str("json.type") == Some("port") };
            if _cond {
                event.set(
                    "threat.indicator.url.port",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("port") };
            if _cond {
                if let Some(val) = event.get("threat.indicator.url.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "threat.indicator.url.port".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.url.port", converted)?;
                }
            }

            let _cond = { event.get_str("json.type") == Some("process") };
            if _cond {
                event.set("threat.indicator.type", json!("process"))?;
            }

            let _cond = { event.get_str("json.type") == Some("process-name") };
            if _cond {
                event.set("threat.indicator.type", json!("process"))?;
            }

            let _cond = { event.get_str("json.type") == Some("uri") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("json.type") == Some("uri") };
            if _cond {
                event.set(
                    "threat.indicator.url.full",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("winregistry") };
            if _cond {
                event.set("threat.indicator.type", json!("windows-registry-key"))?;
            }

            let _cond = { event.get_str("json.type") == Some("winregistry") };
            if _cond {
                event.set(
                    "threat.indicator.registry.value",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("certificate-serial-number") };
            if _cond {
                event.set("threat.indicator.type", json!("x509-certificate"))?;
            }

            let _cond = { event.get_str("json.type") == Some("certificate-serial-number") };
            if _cond {
                event.set(
                    "threat.indicator.x509.serial_number",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.type") == Some("file")
                    || event.get_str("json.type") == Some("file-size")
                    || event.get_str("json.type") == Some("hash-md5")
                    || event.get_str("json.type") == Some("hash-sha1")
                    || event.get_str("json.type") == Some("hash-sha256")
                    || event.get_str("json.type") == Some("hash-sha384")
                    || event.get_str("json.type") == Some("hash-sha512")
                    || event.get_str("json.type") == Some("hash-ssdeep")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.type") == Some("ipv4")
                    || event.get_str("json.type") == Some("ipv6")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.type") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.type".into(),
                    });
                }
                if let Some(v) = event.get("json.value") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.value".into(),
                    });
                }
                if !values.is_empty() {
                    event.set(
                        "eclecticiq.threat.observable_id",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            let _cond = { event.get_str("json.diff") == Some("del") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
                            "yyyy-MM-dd'T'HH:mm:ssz",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("eclecticiq.threat.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.type") == Some("malware") };
            if _cond {
                event.set(
                    "threat.software.name",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("malware") };
            if _cond {
                event.set("threat.software.type", json!("Malware"))?;
            }

            let _cond = { event.get_str("json.type") == Some("mac-48") };
            if _cond {
                event.set(
                    "server.mac",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("mac-48") };
            if _cond {
                gsub_field(event, "server.mac", "server.mac", cached_regex!(":"), "-")?;
            }

            let _cond = { event.get_str("json.type") == Some("mac-48") };
            if _cond {
                map_strings(event, "server.mac", "server.mac", str::to_uppercase)?;
            }

            let _cond = { event.get_str("json.type") == Some("rule") };
            if _cond {
                event.set(
                    "rule.name",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("user-agent") };
            if _cond {
                event.set(
                    "user_agent.original",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("organization") };
            if _cond {
                event.set(
                    "organization.name",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("process") };
            if _cond {
                event.set(
                    "process.command_line",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("process-name") };
            if _cond {
                event.set(
                    "process.name",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("email-subject") };
            if _cond {
                event.set(
                    "email.subject",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("host") };
            if _cond {
                event.set(
                    "host.hostname",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("cve") };
            if _cond {
                event.set(
                    "vulnerability.id",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.timestamp") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.timestamp".into(),
                    });
                }
                if let Some(v) = event.get("json.type") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.type".into(),
                    });
                }
                if let Some(v) = event.get("json.value") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.value".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("event.id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set(
                "_id",
                json!(
                    event
                        .get("event.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                if event.remove("threat.indicator").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator".into(),
                    });
                }
            }

            let _cond =
                { !event.has_value("threat.indicator") && !event.has_value("threat.software") };
            if _cond {
                if event.remove("threat").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat".into(),
                    });
                }
            }

            if event.remove("json").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json".into(),
                });
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
