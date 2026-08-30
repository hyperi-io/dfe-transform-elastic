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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.module", json!("ti_rapid7_threat_command"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.content")
                    && event.get("json.content").is_some_and(|v| match v {
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

            if event.has_value("json.lastUpdateDate") {
                event.rename("json.lastUpdateDate", "json.last_update_date")?;
            }

            let _cond = { event.has_value("json.last_update_date") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_update_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.modified_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_update_date".into(),
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
                    if event.remove("json.last_update_date").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.last_update_date".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(v) = event.get("threat.indicator.modified_at").cloned() {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "set")?;
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("json.type") == Some("Urls") };
            if _cond {
                uri_parts(event, "json.value", "threat.indicator.url", true, false)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("threat.indicator.url.original")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.type") == Some("Urls") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.has_value("json.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: void _set(Map base, def path, def value) {\n    if (path.length == 0) return;\n    for (int i = 0; i < path.length - 1; i++) {\n        String c = path[i];\n        if (base[c] == null) base[c] = new HashMap();\n        base = base[c];\n    }\n    base[path[path.length - 1]] = value;\n} void set(Map base, String path, def value) {\n    _set(base, path.splitOnToken(\".\"), value);\n} def mapping = params[ctx.json.type.toLowerCase()]; if (mapping == null) return; set(ctx, \"threat.indicator.type\", mapping.type); def value = ctx.json.value; if (value == null) return; set(ctx, mapping.target, value);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"void _set(Map base, def path, def value) {\n    if (path.length == 0) return;\n    for (int i = 0; i < path.length - 1; i++) {\n        String c = path[i];\n        if (base[c] == null) base[c] = new HashMap();\n        base = base[c];\n    }\n    base[path[path.length - 1]] = value;\n} void set(Map base, String path, def value) {\n    _set(base, path.splitOnToken(\".\"), value);\n} def mapping = params[ctx.json.type.toLowerCase()]; if (mapping == null) return; set(ctx, \"threat.indicator.type\", mapping.type); def value = ctx.json.value; if (value == null) return; set(ctx, mapping.target, value);\n"#
                        ),
                        cached_params!(
                            "{\"domains\":{\"type\":\"domain-name\",\"target\":\"threat.indicator.url.domain\"},\"ipaddresses\":{\"type\":\"ipv4-addr\",\"target\":\"threat.indicator.ip\"},\"emails\":{\"type\":\"email-addr\",\"target\":\"threat.indicator.email.address\"},\"hashes\":{\"type\":\"file\",\"target\":\"json.file.hash\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Unable to determine indicator type from \"{}\": {}",
                            event
                                .get("json.type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threat.indicator.ip") {
                    if let Some(val) = event.get("threat.indicator.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "threat.indicator.ip".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.ip", converted)?;
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
                if event.remove("threat.indicator.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.ip".into(),
                    });
                }
                if event.remove("json.value").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.value".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("ipv4-addr")
                    && event.has_value("json.value")
                    && event.get("json.value").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.file.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("threat.indicator.ip") {
                if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("threat.indicator.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("threat.indicator.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("threat.indicator.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("threat.indicator.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("threat.indicator.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("threat.indicator.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("threat.indicator.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("threat.indicator.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("threat.indicator.ip") {
                if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("threat.indicator.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("threat.indicator.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("threat.indicator.as.asn") {
                event.rename("threat.indicator.as.asn", "threat.indicator.as.number")?;
            }

            if event.has_value("threat.indicator.as.organization_name") {
                event.rename(
                    "threat.indicator.as.organization_name",
                    "threat.indicator.as.organization.name",
                )?;
            }

            if event.has_value("json.firstSeen") {
                event.rename("json.firstSeen", "json.first_seen")?;
            }

            if event.has_value("json.lastSeen") {
                event.rename("json.lastSeen", "json.last_seen")?;
            }

            if event.has_value("json.relatedMalware") {
                event.rename("json.relatedMalware", "json.related.malware")?;
            }

            if event.has_value("json.relatedCampaigns") {
                event.rename("json.relatedCampaigns", "json.related.campaigns")?;
            }

            if event.has_value("json.relatedThreatActors") {
                event.rename("json.relatedThreatActors", "json.related.threat_actors")?;
            }

            if event.has_value("json.reportedFeeds") {
                event.rename("json.reportedFeeds", "json.reported_feeds")?;
            }

            let _cond = {
                event
                    .get_as_string("json.file.hash")
                    .is_some_and(|s| s.len() == 32)
            };
            if _cond {
                if event.has_value("json.file.hash") {
                    event.rename("json.file.hash", "threat.indicator.file.hash.md5")?;
                }
            }

            let _cond = {
                event
                    .get_as_string("json.file.hash")
                    .is_some_and(|s| s.len() == 40)
            };
            if _cond {
                if event.has_value("json.file.hash") {
                    event.rename("json.file.hash", "threat.indicator.file.hash.sha1")?;
                }
            }

            let _cond = {
                event
                    .get_as_string("json.file.hash")
                    .is_some_and(|s| s.len() == 64)
            };
            if _cond {
                if event.has_value("json.file.hash") {
                    event.rename("json.file.hash", "threat.indicator.file.hash.sha256")?;
                }
            }

            let _cond = {
                event
                    .get_as_string("json.file.hash")
                    .is_some_and(|s| s.len() == 96)
            };
            if _cond {
                if event.has_value("json.file.hash") {
                    event.rename("json.file.hash", "threat.indicator.file.hash.sha384")?;
                }
            }

            let _cond = {
                event
                    .get_as_string("json.file.hash")
                    .is_some_and(|s| s.len() == 128)
            };
            if _cond {
                if event.has_value("json.file.hash") {
                    event.rename("json.file.hash", "threat.indicator.file.hash.sha512")?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.score") {
                    if let Some(val) = event.get("json.score") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.score".into(),
                                message,
                            }
                        })?;
                        event.set("event.risk_score", converted)?;
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
                if event.remove("json.score").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.score".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.whitelisted") {
                    if let Some(val) = event.get("json.whitelisted") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.whitelisted".into(),
                                message,
                            }
                        })?;
                        event.set("json.whitelisted", converted)?;
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
                if event.remove("json.whitelisted").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.whitelisted".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.last_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_seen".into(),
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
                    if event.remove("json.last_seen").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.last_seen".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.first_seen") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.first_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.first_seen".into(),
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
                    if event.remove("json.first_seen").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.first_seen".into(),
                        });
                    }
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
                    .get("json.score")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.confidence", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("threat.indicator.geo.country_iso_code") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("json.geolocation")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.geo.country_iso_code", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("threat.indicator.url.original")
                    && event.get_str("threat.indicator.type") == Some("url")
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.url.original")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            let _cond = {
                event.has_value("threat.indicator.url.domain")
                    && event.get_str("threat.indicator.type") == Some("domain-name")
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.url.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && (event.get_str("threat.indicator.type") == Some("ipv4-addr")
                        || event.get_str("threat.indicator.type") == Some("ipv6-addr"))
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            let _cond = { event.get_str("threat.indicator.email.address") == Some("email-addr") };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.email.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.reported_feeds", |event| {
                    event.rename(
                        "_ingest._value.confidenceLevel",
                        "_ingest._value.confidence",
                    )?;
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.reported_feeds", |event| {
                    event.append(
                        "threat.indicator.provider",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
                Ok(())
            })();

            let _cond = { event.has_value("json.tags") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.tags", |event| {
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
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json") {
                    event.rename("json", "rapid7.tc.ioc")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("threat.indicator.confidence") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = ctx.threat.indicator.confidence; if (value <= 0 || value > 100) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} else if (value >= 1 && value < 30) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} else if (value >= 30 && value < 70) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} else if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def value = ctx.threat.indicator.confidence; if (value <= 0 || value > 100) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} else if (value >= 1 && value < 30) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} else if (value >= 30 && value < 70) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} else if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("rapid7.tc.ioc.deleted_at")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_last_seen; if (ctx.rapid7.tc.ioc.status instanceof String && ctx.rapid7.tc.ioc.status.toLowerCase().contains('retire')) {\n  _tmp_deleted_at = ZonedDateTime.parse(ctx['@timestamp']);\n} else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    if (ctx.threat?.indicator?.modified_at != null) {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.modified_at);\n    }\n    else if (ctx.threat?.indicator?.last_seen != null) {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n    }\n    else {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n    }\n    if (dur instanceof String){\n        String time_unit = dur.substring(dur.length() -  1);\n        String time_value = dur.substring(0, dur.length() - 1);\n        if (time_unit == 'd') {\n            _tmp_deleted_at = _tmp_last_seen.plusDays(Long.parseLong(time_value));\n        } else if (time_unit == 'h') {\n            _tmp_deleted_at = _tmp_last_seen.plusHours(Long.parseLong(time_value));\n        } else if (time_unit == 'm') {\n            _tmp_deleted_at = _tmp_last_seen.plusMinutes(Long.parseLong(time_value));\n        }\n    }\n    // Add default IOC expiration of `90 days` from last_seen if '_conf.ioc_expiration_duration' is an invalid value.\n    if (_tmp_deleted_at == null) {\n        _tmp_deleted_at = _tmp_last_seen.plusDays(90L);\n    }\n} ctx.rapid7.tc.ioc.deleted_at = _tmp_deleted_at;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_last_seen; if (ctx.rapid7.tc.ioc.status instanceof String && ctx.rapid7.tc.ioc.status.toLowerCase().contains('retire')) {\n  _tmp_deleted_at = ZonedDateTime.parse(ctx['@timestamp']);\n} else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    if (ctx.threat?.indicator?.modified_at != null) {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.modified_at);\n    }\n    else if (ctx.threat?.indicator?.last_seen != null) {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n    }\n    else {\n        _tmp_last_seen = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n    }\n    if (dur instanceof String){\n        String time_unit = dur.substring(dur.length() -  1);\n        String time_value = dur.substring(0, dur.length() - 1);\n        if (time_unit == 'd') {\n            _tmp_deleted_at = _tmp_last_seen.plusDays(Long.parseLong(time_value));\n        } else if (time_unit == 'h') {\n            _tmp_deleted_at = _tmp_last_seen.plusHours(Long.parseLong(time_value));\n        } else if (time_unit == 'm') {\n            _tmp_deleted_at = _tmp_last_seen.plusMinutes(Long.parseLong(time_value));\n        }\n    }\n    // Add default IOC expiration of `90 days` from last_seen if '_conf.ioc_expiration_duration' is an invalid value.\n    if (_tmp_deleted_at == null) {\n        _tmp_deleted_at = _tmp_last_seen.plusDays(90L);\n    }\n} ctx.rapid7.tc.ioc.deleted_at = _tmp_deleted_at;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-default-deleted_at",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("rapid7.tc.ioc.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("rapid7.tc.ioc.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "rapid7.tc.ioc.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_deleted_at")?;
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

            if event.has_value("_conf.ioc_expiration_duration") {
                event.rename(
                    "_conf.ioc_expiration_duration",
                    "rapid7.tc.ioc.expiration_duration",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n} drop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n} drop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("rapid7.tc.ioc.last_update_date") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("rapid7.tc.ioc.reported_feeds") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("rapid7.tc.ioc.value") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
