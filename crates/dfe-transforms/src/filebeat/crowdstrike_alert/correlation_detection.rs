// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `correlation_detection` pipeline.
pub struct CorrelationDetection;

impl Transform for CorrelationDetection {
    fn name(&self) -> &str {
        "correlation_detection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

            if let Some(v) = event.get("crowdstrike.alert.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
            if let Some(v) = event.get("crowdstrike.alert.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }
            }

            if let Some(v) = event.get("crowdstrike.alert.falcon_host_link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.url", v)?;
            }

            if let Some(v) = event.get("crowdstrike.alert.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event.get("crowdstrike.alert.correlation_rule_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.id", v)?;
            }

            let _cond = { event.get("crowdstrike.alert.source_products").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.source_products").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nctx.event.provider = ctx.crowdstrike.alert.source_products[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event = ctx.event ?: [:];\nctx.event.provider = ctx.crowdstrike.alert.source_products[0];"#))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.has_truncated_entities") {
                if let Some(val) = event.get("crowdstrike.alert.has_truncated_entities") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.has_truncated_entities".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.has_truncated_entities", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_has_truncated_entities_to_boolean")?;
                        event.remove("crowdstrike.alert.has_truncated_entities");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.correlation_rule_create_case") {
                if let Some(val) = event.get("crowdstrike.alert.correlation_rule_create_case") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.correlation_rule_create_case".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.correlation_rule_create_case", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_correlation_rule_create_case_to_boolean")?;
                        event.remove("crowdstrike.alert.correlation_rule_create_case");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.original_correlation_rules_entities_count") {
                if let Some(val) = event.get("crowdstrike.alert.original_correlation_rules_entities_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.original_correlation_rules_entities_count".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.original_correlation_rules_entities_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_original_correlation_rules_entities_count_to_long")?;
                        event.remove("crowdstrike.alert.original_correlation_rules_entities_count");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("crowdstrike.alert.original_indicator_entities_count") {
                if let Some(val) = event.get("crowdstrike.alert.original_indicator_entities_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "crowdstrike.alert.original_indicator_entities_count".into(),
                            message,
                        })?;
                    event.set("crowdstrike.alert.original_indicator_entities_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_original_indicator_entities_count_to_long")?;
                        event.remove("crowdstrike.alert.original_indicator_entities_count");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get("crowdstrike.alert.users").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.users", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.full_name_is_enriched") {
                    if let Some(val) = event.get("_ingest._value.full_name_is_enriched") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.full_name_is_enriched".into(),
                    message,
                    })?;
                    event.set("_ingest._value.full_name_is_enriched", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_users_full_name_is_enriched_to_boolean")?;
                    event.remove("_ingest._value.full_name_is_enriched");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.comments").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("crowdstrike.alert.comments").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.timestamp", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.timestamp".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_comments_timestamp")?;
                        event.remove("_ingest._value.timestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("crowdstrike.alert.comments", Value::Array(out))?;
                }
            }

            let _cond = { event.get("crowdstrike.alert.users").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.users", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.idp_id_is_enriched") {
                    if let Some(val) = event.get("_ingest._value.idp_id_is_enriched") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.idp_id_is_enriched".into(),
                    message,
                    })?;
                    event.set("_ingest._value.idp_id_is_enriched", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_users_idp_id_is_enriched_to_boolean")?;
                    event.remove("_ingest._value.idp_id_is_enriched");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.host_names").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.host_names").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.host = ctx.host ?: [:];\nif (ctx.host.name == null || ctx.host.name == '') {\n  ctx.host.name = ctx.crowdstrike.alert.host_names[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.host = ctx.host ?: [:];\nif (ctx.host.name == null || ctx.host.name == '') {\n  ctx.host.name = ctx.crowdstrike.alert.host_names[0];\n}"#))?;
            }

            let _cond = { event.get("crowdstrike.alert.host_names").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.host_names", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.source_hosts").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.source_hosts", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.destination_hosts").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.destination_hosts", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.source_hosts").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.source_hosts").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.source = ctx.source ?: [:];\nif (ctx.source.domain == null || ctx.source.domain == '') {\n  ctx.source.domain = ctx.crowdstrike.alert.source_hosts[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.source = ctx.source ?: [:];\nif (ctx.source.domain == null || ctx.source.domain == '') {\n  ctx.source.domain = ctx.crowdstrike.alert.source_hosts[0];\n}"#))?;
            }

            let _cond = { event.get("crowdstrike.alert.destination_hosts").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.destination_hosts").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.domain == null || ctx.destination.domain == '') {\n  ctx.destination.domain = ctx.crowdstrike.alert.destination_hosts[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.domain == null || ctx.destination.domain == '') {\n  ctx.destination.domain = ctx.crowdstrike.alert.destination_hosts[0];\n}"#))?;
            }

            let _cond = { event.get("crowdstrike.alert.source_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.source_ips", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_source_ips_to_ip")?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.source_ips").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.source_ips").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.source = ctx.source ?: [:];\nif (ctx.source.ip == null || ctx.source.ip == '') {\n  ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.source = ctx.source ?: [:];\nif (ctx.source.ip == null || ctx.source.ip == '') {\n  ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];\n}"#))?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }

                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }

            let _cond = { event.get("crowdstrike.alert.source_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.source_ips", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.destination_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.destination_ips", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_destination_ips_to_ip")?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.destination_ips").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.destination_ips").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.ip == null || ctx.destination.ip == '') {\n  ctx.destination.ip = ctx.crowdstrike.alert.destination_ips[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.destination = ctx.destination ?: [:];\nif (ctx.destination.ip == null || ctx.destination.ip == '') {\n  ctx.destination.ip = ctx.crowdstrike.alert.destination_ips[0];\n}"#))?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }

                if event.has_value("destination.as.organization_name") {
                    event.rename("destination.as.organization_name", "destination.as.organization.name")?;
                }

            let _cond = { event.get("crowdstrike.alert.destination_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.destination_ips", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("crowdstrike.alert.correlation_rule_user_id") && event.get_str("crowdstrike.alert.correlation_rule_user_id") != Some("") };
            if _cond {
                event.append_unique("related.user", json!(event.get("crowdstrike.alert.correlation_rule_user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("crowdstrike.alert.user_names").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.user_names").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.user = ctx.user ?: [:];\nif (ctx.user.name == null || ctx.user.name == '') {\n  ctx.user.name = ctx.crowdstrike.alert.user_names[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.user = ctx.user ?: [:];\nif (ctx.user.name == null || ctx.user.name == '') {\n  ctx.user.name = ctx.crowdstrike.alert.user_names[0];\n}"#))?;
            }

            let _cond = { event.get("crowdstrike.alert.user_names").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "crowdstrike.alert.user_names", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("crowdstrike.alert.users").is_some_and(|v| v.is_array()) && event.get("crowdstrike.alert.users").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: for (def userEntry : ctx.crowdstrike.alert.users) {\n  if (userEntry.user_name != null && userEntry.user_name != '') {\n    ctx.user = ctx.user ?: [:];\n    if (ctx.user.name == null || ctx.user.name == '') {\n      ctx.user.name = userEntry.user_name;\n    }\n\n    ctx.related = ctx.related ?: [:];\n    ctx.related.user = ctx.related.user ?: [];\n    if (!ctx.related.user.contains(userEntry.user_name)) {\n      ctx.related.user.add(userEntry.user_name);\n    }\n  }\n\n  ctx.user = ctx.user ?: [:];\n  if (userEntry.sid != null && userEntry.sid != '' && (ctx.user.id == null || ctx.user.id == '')) {\n    ctx.user.id = userEntry.sid;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"for (def userEntry : ctx.crowdstrike.alert.users) {\n  if (userEntry.user_name != null && userEntry.user_name != '') {\n    ctx.user = ctx.user ?: [:];\n    if (ctx.user.name == null || ctx.user.name == '') {\n      ctx.user.name = userEntry.user_name;\n    }\n\n    ctx.related = ctx.related ?: [:];\n    ctx.related.user = ctx.related.user ?: [];\n    if (!ctx.related.user.contains(userEntry.user_name)) {\n      ctx.related.user.add(userEntry.user_name);\n    }\n  }\n\n  ctx.user = ctx.user ?: [:];\n  if (userEntry.sid != null && userEntry.sid != '' && (ctx.user.id == null || ctx.user.id == '')) {\n    ctx.user.id = userEntry.sid;\n  }\n}"#))?;
            }

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
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
