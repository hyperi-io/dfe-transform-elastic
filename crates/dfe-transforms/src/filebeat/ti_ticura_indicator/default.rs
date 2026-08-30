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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_original_event")),
                    serde_json::Value::String(s) => s.contains("preserve_original_event"),
                    _ => false,
                })
            };
            if _cond {
                if let Some(v) = event
                    .get("message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.original", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field_to_root(event, "message", true)?;
                Ok(())
            })();

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            event.remove("message");

            let _cond = { !event.has_value("ticura.indicator.uuid") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            let _cond = { event.has_value("threat.indicator.id") };
            if _cond {
                // Painless script
                // Source: if (!(ctx.threat.indicator.id instanceof List)) {\n  ctx.threat.indicator.id = [ctx.threat.indicator.id];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (!(ctx.threat.indicator.id instanceof List)) {\n  ctx.threat.indicator.id = [ctx.threat.indicator.id];\n}\n"#
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("ticura.indicator.fingerprint") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ticura.indicator.uuid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set(
                "ticura.indicator.feed_ingest_timestamp",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if !event.has("event.action") {
                event.set("event.action", json!("indicator-update"))?;
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threat.indicator.last_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.last_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("threat.indicator.last_seen")
                    && event.has_value("threat.indicator.first_seen")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("threat.indicator.first_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "threat.indicator.first_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if !event.has("event.provider") {
                event.set("event.provider", json!("Ticura"))?;
            }

            if !event.has("observer.vendor") {
                event.set("observer.vendor", json!("Ticura"))?;
            }

            if !event.has("observer.product") {
                event.set("observer.product", json!("Ticura"))?;
            }

            if let Some(v) = event
                .get("ticura.indicator.risk")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.severity") {
                    event.set("event.severity", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.id") {
                    event.set("threat.technique.id", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.name") {
                    event.set("threat.technique.name", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.reference") {
                    event.set("threat.technique.reference", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.subtechnique.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.subtechnique.id") {
                    event.set("threat.technique.subtechnique.id", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.subtechnique.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.subtechnique.name") {
                    event.set("threat.technique.subtechnique.name", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.technique.subtechnique.reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.technique.subtechnique.reference") {
                    event.set("threat.technique.subtechnique.reference", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.id") {
                    event.set("threat.software.id", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.name") {
                    event.set("threat.software.name", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.type") {
                    event.set("threat.software.type", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.platforms")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.platforms") {
                    event.set("threat.software.platforms", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.alias")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.alias") {
                    event.set("threat.software.alias", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.software.reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.software.reference") {
                    event.set("threat.software.reference", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.group.id") {
                    event.set("threat.group.id", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.group.name") {
                    event.set("threat.group.name", v)?;
                }
            }

            if let Some(v) = event
                .get("ticura.indicator.group.reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("threat.group.reference") {
                    event.set("threat.group.reference", v)?;
                }
            }

            let _cond = {
                event.get_str("ticura.indicator.sub_type") == Some("HASHSHA256")
                    && event.has_value("threat.indicator.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("threat.indicator.file.hash.sha256") {
                        event.set("threat.indicator.file.hash.sha256", v)?;
                    }
                }
            }

            let _cond = {
                event.get_str("ticura.indicator.sub_type") == Some("HASHMD5")
                    && event.has_value("threat.indicator.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("threat.indicator.file.hash.md5") {
                        event.set("threat.indicator.file.hash.md5", v)?;
                    }
                }
            }

            let _cond = {
                event.get_str("ticura.indicator.sub_type") == Some("HASHSHA1")
                    && event.has_value("threat.indicator.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("threat.indicator.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("threat.indicator.file.hash.sha1") {
                        event.set("threat.indicator.file.hash.sha1", v)?;
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("threat.indicator.url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("threat.indicator.ip") && !event.has_value("threat.indicator.geo")
            };
            if _cond {
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
            }

            let _cond = {
                event.has_value("threat.indicator.ip") && !event.has_value("threat.indicator.as")
            };
            if _cond {
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.ticura.indicator.merged == null || !(ctx.ticura.indicator.merged instanceof Map)) {\n    ctx.ticura.indicator.merged = [:];\n}\n\nList mergedCountries = [];\nif (ctx.ticura?.indicator?.countries != null) {\n    if (ctx.ticura.indicator.countries instanceof List) {\n        mergedCountries.addAll(ctx.ticura.indicator.countries);\n    } else {\n        mergedCountries.add(ctx.ticura.indicator.countries);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.countries != null) {\n    if (ctx.ticura.indicator.additional_info.countries instanceof List) {\n        mergedCountries.addAll(ctx.ticura.indicator.additional_info.countries);\n    } else {\n        mergedCountries.add(ctx.ticura.indicator.additional_info.countries);\n    }\n}\nctx.ticura.indicator.merged.countries = new ArrayList(new HashSet(mergedCountries));\nctx.ticura.indicator.merged.countries.sort(null);\n\nList mergedIndustries = [];\nif (ctx.ticura?.indicator?.industries != null) {\n    if (ctx.ticura.indicator.industries instanceof List) {\n        mergedIndustries.addAll(ctx.ticura.indicator.industries);\n    } else {\n        mergedIndustries.add(ctx.ticura.indicator.industries);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.industries != null) {\n    if (ctx.ticura.indicator.additional_info.industries instanceof List) {\n        mergedIndustries.addAll(ctx.ticura.indicator.additional_info.industries);\n    } else {\n        mergedIndustries.add(ctx.ticura.indicator.additional_info.industries);\n    }\n}\nctx.ticura.indicator.merged.industries = new ArrayList(new HashSet(mergedIndustries));\nctx.ticura.indicator.merged.industries.sort(null);\n\nList mergedActors = [];\nif (ctx.ticura?.indicator?.actors != null) {\n    if (ctx.ticura.indicator.actors instanceof List) {\n        mergedActors.addAll(ctx.ticura.indicator.actors);\n    } else {\n        mergedActors.add(ctx.ticura.indicator.actors);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.actors != null) {\n    if (ctx.ticura.indicator.additional_info.actors instanceof List) {\n        mergedActors.addAll(ctx.ticura.indicator.additional_info.actors);\n    } else {\n        mergedActors.add(ctx.ticura.indicator.additional_info.actors);\n    }\n}\nctx.ticura.indicator.merged.actors = new ArrayList(new HashSet(mergedActors));\nctx.ticura.indicator.merged.actors.sort(null);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.ticura.indicator.merged == null || !(ctx.ticura.indicator.merged instanceof Map)) {\n    ctx.ticura.indicator.merged = [:];\n}\n\nList mergedCountries = [];\nif (ctx.ticura?.indicator?.countries != null) {\n    if (ctx.ticura.indicator.countries instanceof List) {\n        mergedCountries.addAll(ctx.ticura.indicator.countries);\n    } else {\n        mergedCountries.add(ctx.ticura.indicator.countries);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.countries != null) {\n    if (ctx.ticura.indicator.additional_info.countries instanceof List) {\n        mergedCountries.addAll(ctx.ticura.indicator.additional_info.countries);\n    } else {\n        mergedCountries.add(ctx.ticura.indicator.additional_info.countries);\n    }\n}\nctx.ticura.indicator.merged.countries = new ArrayList(new HashSet(mergedCountries));\nctx.ticura.indicator.merged.countries.sort(null);\n\nList mergedIndustries = [];\nif (ctx.ticura?.indicator?.industries != null) {\n    if (ctx.ticura.indicator.industries instanceof List) {\n        mergedIndustries.addAll(ctx.ticura.indicator.industries);\n    } else {\n        mergedIndustries.add(ctx.ticura.indicator.industries);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.industries != null) {\n    if (ctx.ticura.indicator.additional_info.industries instanceof List) {\n        mergedIndustries.addAll(ctx.ticura.indicator.additional_info.industries);\n    } else {\n        mergedIndustries.add(ctx.ticura.indicator.additional_info.industries);\n    }\n}\nctx.ticura.indicator.merged.industries = new ArrayList(new HashSet(mergedIndustries));\nctx.ticura.indicator.merged.industries.sort(null);\n\nList mergedActors = [];\nif (ctx.ticura?.indicator?.actors != null) {\n    if (ctx.ticura.indicator.actors instanceof List) {\n        mergedActors.addAll(ctx.ticura.indicator.actors);\n    } else {\n        mergedActors.add(ctx.ticura.indicator.actors);\n    }\n}\nif (ctx.ticura?.indicator?.additional_info?.actors != null) {\n    if (ctx.ticura.indicator.additional_info.actors instanceof List) {\n        mergedActors.addAll(ctx.ticura.indicator.additional_info.actors);\n    } else {\n        mergedActors.add(ctx.ticura.indicator.additional_info.actors);\n    }\n}\nctx.ticura.indicator.merged.actors = new ArrayList(new HashSet(mergedActors));\nctx.ticura.indicator.merged.actors.sort(null);\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) { handleMap((Map) v); return ((Map) v).isEmpty(); }\n    else if (v instanceof List) { handleList((List) v); return ((List) v).isEmpty(); }\n    else if (v instanceof String) { return ((String) v).isEmpty(); }\n    return v == null;\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) { handleMap((Map) v); return ((Map) v).isEmpty(); }\n    else if (v instanceof List) { handleList((List) v); return ((List) v).isEmpty(); }\n    else if (v instanceof String) { return ((String) v).isEmpty(); }\n    return v == null;\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) { handleMap((Map) v); return ((Map) v).isEmpty(); }\n    else if (v instanceof List) { handleList((List) v); return ((List) v).isEmpty(); }\n    else if (v instanceof String) { return ((String) v).isEmpty(); }\n    return v == null;\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) { handleMap((Map) v); return ((Map) v).isEmpty(); }\n    else if (v instanceof List) { handleList((List) v); return ((List) v).isEmpty(); }\n    else if (v instanceof String) { return ((String) v).isEmpty(); }\n    return v == null;\n  });\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

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
