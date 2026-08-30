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

            event.set(
                "event.module",
                json!("ti_mandiant_advantage_threat_intelligence"),
            )?;

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

            event.set("threat.feed.name", json!("Mandiant Threat Intelligence"))?;

            if event.has_value("json.last_updated") {
                event.rename("json.last_updated", "json.last_update_date")?;
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

            let _cond = { event.get_str("json.type") == Some("url") };
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

            let _cond = { event.get_str("json.type") == Some("url") };
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
                            "{\"fqdn\":{\"type\":\"domain-name\",\"target\":\"threat.indicator.url.domain\"},\"ipv4\":{\"type\":\"ipv4-addr\",\"target\":\"threat.indicator.ip\"},\"md5\":{\"type\":\"file\",\"target\":\"threat.indicator.file.hash.md5\"}}"
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

            let _cond = {
                event.get_str("json.type") == Some("md5") && event.get("json.associated_hashes").is_some_and(|v| v.is_array()) && event.get("json.associated_hashes").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def hashes = new ArrayList();\nhashes.add(ctx.threat.indicator.file.hash.md5);\nfor (hash in ctx.json.associated_hashes) {\n  if (hash == null) {\n    return;\n  }\n    if (hash.containsKey(\"type\") && hash[\"type\"] == \"sha1\" && hash.containsKey(\"value\")) {\n    ctx.threat.indicator.file.hash.sha1 = hash[\"value\"];\n    hashes.add(hash[\"value\"]);\n  }\n    else if (hash.containsKey(\"type\") && hash[\"type\"] == \"sha256\" && hash.containsKey(\"value\")) {\n    ctx.threat.indicator.file.hash.sha256 = hash[\"value\"];\n    hashes.add(hash[\"value\"]);\n  }\n}\nif (ctx.json.hashes == null) {\n  ctx.json.hashes = new HashMap();\n}\nctx.json.hashes = hashes;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hashes = new ArrayList();\nhashes.add(ctx.threat.indicator.file.hash.md5);\nfor (hash in ctx.json.associated_hashes) {\n  if (hash == null) {\n    return;\n  }\n    if (hash.containsKey(\"type\") && hash[\"type\"] == \"sha1\" && hash.containsKey(\"value\")) {\n    ctx.threat.indicator.file.hash.sha1 = hash[\"value\"];\n    hashes.add(hash[\"value\"]);\n  }\n    else if (hash.containsKey(\"type\") && hash[\"type\"] == \"sha256\" && hash.containsKey(\"value\")) {\n    ctx.threat.indicator.file.hash.sha256 = hash[\"value\"];\n    hashes.add(hash[\"value\"]);\n  }\n}\nif (ctx.json.hashes == null) {\n  ctx.json.hashes = new HashMap();\n}\nctx.json.hashes = hashes;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Unable to extract sha1 and sha256 information for \"{}\": {}",
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

            if event.has_value("json.hashes") {
                event.rename("json.hashes", "related.hash")?;
            }

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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mscore") {
                    if let Some(val) = event.get("json.mscore") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.mscore".into(),
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
                if event.remove("json.mscore").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.mscore".into(),
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
                    .get("json.mscore")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.confidence", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.sources") && event.get("json.sources").is_some_and(|v| v.is_array()) && event.get("json.sources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: def providers = new ArrayList();\ndef categories = new ArrayList();\ndef is_osint = true;\nfor (source in ctx.json.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"source_name\") && source[\"source_name\"] != null && !providers.contains(source[\"source_name\"])) {\n    providers.add(source[\"source_name\"]);\n  }\n  if (source.containsKey(\"category\") && source[\"category\"] != null) {\n    categories.addAll(source[\"category\"]);\n  }\n  if (source.containsKey(\"osint\") && !source[\"osint\"]) {\n    is_osint = false;\n  }\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}\nif (categories.size() > 0) {\n  if (ctx.indicator_categories == null) {\n    ctx.indicator_categories = new HashMap();\n  }\n  ctx.indicator_categories = categories;\n}\nif (is_osint) {\n  ctx.tlp_color = \"GREEN\";\n}\nelse {\n  ctx.tlp_color = \"RED\";\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def providers = new ArrayList();\ndef categories = new ArrayList();\ndef is_osint = true;\nfor (source in ctx.json.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"source_name\") && source[\"source_name\"] != null && !providers.contains(source[\"source_name\"])) {\n    providers.add(source[\"source_name\"]);\n  }\n  if (source.containsKey(\"category\") && source[\"category\"] != null) {\n    categories.addAll(source[\"category\"]);\n  }\n  if (source.containsKey(\"osint\") && !source[\"osint\"]) {\n    is_osint = false;\n  }\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}\nif (categories.size() > 0) {\n  if (ctx.indicator_categories == null) {\n    ctx.indicator_categories = new HashMap();\n  }\n  ctx.indicator_categories = categories;\n}\nif (is_osint) {\n  ctx.tlp_color = \"GREEN\";\n}\nelse {\n  ctx.tlp_color = \"RED\";\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.attributed_associations") && event.get("json.attributed_associations").is_some_and(|v| v.is_array()) && event.get("json.attributed_associations").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: def groups = new ArrayList();\ndef software = new ArrayList();\nfor (association in ctx.json.attributed_associations) {\n  if (association == null) {\n    return;\n  }\n  if (association.containsKey(\"type\") && association[\"type\"] == \"threat-actor\" && !groups.contains(association[\"name\"])) {\n    groups.add(association[\"name\"]);\n  }\n  else if (association.containsKey(\"type\") && association[\"type\"] == \"malware\" && !software.contains(association[\"name\"])) {\n    software.add(association[\"name\"]);\n  }\n}\nif (groups.size() > 0) {\n  if (ctx.threat.group == null) {\n    ctx.threat.group = new HashMap();\n  }\n  if (ctx.threat.group.name == null) {\n    ctx.threat.group.name = new HashMap();\n  }\n  if (ctx.threat.group.id == null) {\n    ctx.threat.group.id = new HashMap();\n  }\n  ctx.threat.group.id = groups;\n  ctx.threat.group.name = groups;\n}\nif (software.size() > 0) {\n  if (ctx.threat.software == null) {\n    ctx.threat.software = new HashMap();\n  }\n  if (ctx.threat.software.type == null) {\n    ctx.threat.software.type = \"Malware\";\n  }\n  if (ctx.threat.software.name == null) {\n    ctx.threat.software.name = new HashMap();\n  }\n  ctx.threat.software.name = software;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def groups = new ArrayList();\ndef software = new ArrayList();\nfor (association in ctx.json.attributed_associations) {\n  if (association == null) {\n    return;\n  }\n  if (association.containsKey(\"type\") && association[\"type\"] == \"threat-actor\" && !groups.contains(association[\"name\"])) {\n    groups.add(association[\"name\"]);\n  }\n  else if (association.containsKey(\"type\") && association[\"type\"] == \"malware\" && !software.contains(association[\"name\"])) {\n    software.add(association[\"name\"]);\n  }\n}\nif (groups.size() > 0) {\n  if (ctx.threat.group == null) {\n    ctx.threat.group = new HashMap();\n  }\n  if (ctx.threat.group.name == null) {\n    ctx.threat.group.name = new HashMap();\n  }\n  if (ctx.threat.group.id == null) {\n    ctx.threat.group.id = new HashMap();\n  }\n  ctx.threat.group.id = groups;\n  ctx.threat.group.name = groups;\n}\nif (software.size() > 0) {\n  if (ctx.threat.software == null) {\n    ctx.threat.software = new HashMap();\n  }\n  if (ctx.threat.software.type == null) {\n    ctx.threat.software.type = \"Malware\";\n  }\n  if (ctx.threat.software.name == null) {\n    ctx.threat.software.name = new HashMap();\n  }\n  ctx.threat.software.name = software;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.misp") };
            if _cond {
                // Painless script
                // Source: def positive = new ArrayList();\ndef negative = new ArrayList();\ndef keys = ctx.json.misp.keySet().asList();\nkeys.sort((a, b) -> a.compareTo(b));\nfor (key in keys) {\n  if (ctx.json.misp[key]) {\n    positive.add(key);\n  } else {\n    negative.add(key);\n  }\n}\nctx.json[\"misp_warning_list_hits\"] = positive;\nctx.json[\"misp_warning_list_misses\"] = negative;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def positive = new ArrayList();\ndef negative = new ArrayList();\ndef keys = ctx.json.misp.keySet().asList();\nkeys.sort((a, b) -> a.compareTo(b));\nfor (key in keys) {\n  if (ctx.json.misp[key]) {\n    positive.add(key);\n  } else {\n    negative.add(key);\n  }\n}\nctx.json[\"misp_warning_list_hits\"] = positive;\nctx.json[\"misp_warning_list_misses\"] = negative;"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json.misp");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json") {
                    event.rename("json", "mandiant.threat_intelligence.ioc")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("indicator_categories") {
                    event.rename(
                        "indicator_categories",
                        "mandiant.threat_intelligence.ioc.categories",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("tlp_color") {
                    event.rename("tlp_color", "threat.indicator.marking.tlp")?;
                }
                Ok(())
            })();

            event.set("threat.indicator.marking.tlp_version", json!("2.0"))?;

            let _cond = { event.has_value("threat.indicator.confidence") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def value = ctx.threat.indicator.confidence; if (value <= 0 || value > 100) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} else if (value >= 1 && value < 40) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} else if (value >= 40 && value < 80) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} else if (value >= 80 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def value = ctx.threat.indicator.confidence; if (value <= 0 || value > 100) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} else if (value >= 1 && value < 40) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} else if (value >= 40 && value < 80) {\n  ctx.threat.indicator.confidence = \"Medium\";\n  return;\n} else if (value >= 80 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        let mut values = Vec::new();
                        if let Some(v) = event.get("event.original") {
                            values.push(v.clone());
                        }
                        if !values.is_empty() {
                            event.set("_id", json!(fingerprint_default(&values)))?;
                        }
                    }
                    Ok(())
                })();
            }

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
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
