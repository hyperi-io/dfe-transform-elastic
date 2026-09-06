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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "maltiverse")?;
                Ok(())
            })();

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            event.set("threat.indicator.marking.tlp", json!("WHITE"))?;

            if let Some(date_str) = event.get_as_string("maltiverse.blacklist.last_seen") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "maltiverse.blacklist.last_seen".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("ip") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("hostname") };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("url") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("sample") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.has_value("maltiverse.tag") };
            if _cond {
                if event.has_value("maltiverse.tag") {
                    foreach_array(event, "maltiverse.tag", |event| {
                        event.append(
                            "tags",
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

            if event.has_value("_conf.feed") {
                event.rename("_conf.feed", "maltiverse.feed")?;
            }

            let _cond = { event.has_value("maltiverse.feed") };
            if _cond {
                event.set(
                    "threat.feed.reference",
                    json!(format!(
                        "https://maltiverse.com/feed/{}",
                        event
                            .get("maltiverse.feed")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("maltiverse.blacklist.first_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.blacklist.first_seen") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.blacklist.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.blacklist.first_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("maltiverse.blacklist.first_seen").cloned() {
                event.set("threat.indicator.first_seen", v)?;
            }

            let _cond = { event.has_value("maltiverse.blacklist.last_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.blacklist.last_seen") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.blacklist.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.blacklist.last_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("maltiverse.blacklist.last_seen").cloned() {
                event.set("threat.indicator.last_seen", v)?;
            }

            let _cond = { event.has_value("maltiverse.blacklist.count") };
            if _cond {
                if event.has_value("maltiverse.blacklist.count") {
                    event.rename("maltiverse.blacklist.count", "threat.indicator.sightings")?;
                }
            }

            let _cond = { event.has_value("maltiverse.blacklist.description") };
            if _cond {
                if let Some(v) = event.get("maltiverse.blacklist.description").cloned() {
                    event.set("threat.indicator.description", v)?;
                }
            }

            let _cond = { event.has_value("maltiverse.blacklist.source") };
            if _cond {
                if event.has_value("maltiverse.blacklist.source") {
                    event.rename("maltiverse.blacklist.source", "threat.indicator.provider")?;
                }
            }

            let _cond = { event.has_value("maltiverse.asn_date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.asn_date") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.asn_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.asn_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("maltiverse.creation_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.creation_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.creation_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.creation_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("maltiverse.modification_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.modification_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.modification_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.modification_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("maltiverse.last_online_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("maltiverse.last_online_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("maltiverse.last_online_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "maltiverse.last_online_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.get_str("maltiverse.classification") == Some("suspicious") };
            if _cond {
                event.set("event.severity", json!(6))?;
            }

            let _cond = { event.get_str("maltiverse.classification") == Some("suspicious") };
            if _cond {
                event.set("threat.indicator.confidence", json!("Medium"))?;
            }

            let _cond = { event.get_str("maltiverse.classification") == Some("malicious") };
            if _cond {
                event.set("event.severity", json!(9))?;
            }

            let _cond = { event.get_str("maltiverse.classification") == Some("malicious") };
            if _cond {
                event.set("threat.indicator.confidence", json!("High"))?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("ip") };
            if _cond {
                if event.has_value("maltiverse.ip_addr") {
                    event.rename("maltiverse.ip_addr", "threat.indicator.ip")?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("ip") };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://maltiverse.com/ip/{}",
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("ip") && event.has_value("maltiverse.city")
            };
            if _cond {
                if event.has_value("maltiverse.city") {
                    event.rename("maltiverse.city", "threat.indicator.geo.city_name")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("ip")
                    && event.has_value("maltiverse.country_code")
            };
            if _cond {
                if event.has_value("maltiverse.country_code") {
                    event.rename(
                        "maltiverse.country_code",
                        "threat.indicator.geo.country_iso_code",
                    )?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("ip")
                    && event.has_value("maltiverse.country_code")
            };
            if _cond {
                if event.has_value("maltiverse.country_code") {
                    event.rename(
                        "maltiverse.country_code",
                        "threat.indicator.geo.country_iso_code",
                    )?;
                }
            }

            let _cond = {
                event.has_value("maltiverse.location.lat")
                    && event.has_value("maltiverse.location.lon")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("maltiverse.location.lat") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "maltiverse.location.lat".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lat", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Cannot convert lat field \"{}\" to double",
                            event
                                .get("maltiverse.lat")
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
                event.has_value("maltiverse.location.lat")
                    && event.has_value("maltiverse.location.lon")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("maltiverse.location.lon") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "maltiverse.location.lon".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lon", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Cannot convert lon field \"{}\" to double",
                            event
                                .get("maltiverse.lon")
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

            let _cond = { event.get_str("maltiverse.type") == Some("hostname") };
            if _cond {
                if event.has_value("maltiverse.hostname") {
                    event.rename("maltiverse.hostname", "threat.indicator.url.domain")?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("hostname") };
            if _cond {
                if event.has_value("maltiverse.domain") {
                    event.rename(
                        "maltiverse.domain",
                        "threat.indicator.url.registered_domain",
                    )?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("hostname") };
            if _cond {
                if event.has_value("maltiverse.tld") {
                    event.rename("maltiverse.tld", "threat.indicator.url.top_level_domain")?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("hostname") };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://maltiverse.com/hostname/{}",
                        event
                            .get("threat.indicator.url.domain")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("url") };
            if _cond {
                if event.has_value("maltiverse.url") {
                    event.rename("maltiverse.url", "threat.indicator.url.full")?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("url") };
            if _cond {
                if event.has_value("maltiverse.url") {
                    event.rename("maltiverse.url", "threat.indicator.url.original")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("url") && event.has_value("maltiverse.tld")
            };
            if _cond {
                if event.has_value("maltiverse.tld") {
                    event.rename("maltiverse.tld", "threat.indicator.url.top_level_domain")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("url")
                    && event.has_value("maltiverse.domain")
            };
            if _cond {
                if event.has_value("maltiverse.domain") {
                    event.rename(
                        "maltiverse.domain",
                        "threat.indicator.url.registered_domain",
                    )?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("url")
                    && event.has_value("maltiverse.urlchecksum")
            };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://maltiverse.com/url/{}",
                        event
                            .get("maltiverse.urlchecksum")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.get_str("maltiverse.type") == Some("sample") };
            if _cond {
                if event.has_value("maltiverse.sha256") {
                    event.rename("maltiverse.sha256", "threat.indicator.file.hash.sha256")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("sample")
                    && event.has_value("maltiverse.md5")
            };
            if _cond {
                if event.has_value("maltiverse.md5") {
                    event.rename("maltiverse.md5", "threat.indicator.file.hash.md5")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("sample")
                    && event.has_value("maltiverse.sha512")
            };
            if _cond {
                if event.has_value("maltiverse.sha512") {
                    event.rename("maltiverse.sha512", "threat.indicator.file.hash.sha512")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("sample")
                    && event.has_value("maltiverse.filetype")
            };
            if _cond {
                if event.has_value("maltiverse.filetype") {
                    event.rename("maltiverse.filetype", "threat.indicator.file.type")?;
                }
            }

            let _cond = {
                event.get_str("maltiverse.type") == Some("sample")
                    && event.has_value("maltiverse.size")
            };
            if _cond {
                if event.has_value("maltiverse.size") {
                    event.rename("maltiverse.size", "threat.indicator.file.size")?;
                }
            }

            let _cond = { event.get_str("maltiverse.type") == Some("sample") };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://maltiverse.com/sample/{}",
                        event
                            .get("threat.indicator.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("threat.indicator.first_seen") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.first_seen".into(),
                    });
                }
                if let Some(v) = event.get("threat.indicator.last_seen") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.last_seen".into(),
                    });
                }
                if let Some(v) = event.get("threat.indicator.provider") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.provider".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("event.id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("event.id") };
            if _cond {
                event.set(
                    "_id",
                    json!(
                        event
                            .get("event.id")
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
                event.remove("maltiverse.tag");
                event.remove("maltiverse.blacklist.description");
                event.remove("maltiverse.blacklist.first_seen");
                event.remove("maltiverse.blacklist.last_seen");
                event.remove("maltiverse.location");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
