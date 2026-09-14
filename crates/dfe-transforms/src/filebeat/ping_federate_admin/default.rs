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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("observer.vendor", json!("Ping Identity"))?;

            event.set("observer.product", json!("PingFederate"))?;

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: %{DATA:ping_federate.admin.timestamp}\\s\\|\\s%{WORD:ping_federate.admin.user}\\s\\|\\s(%{DATA:ping_federate.admin.roles})?\\s\\|\\s(%{IP:ping_federate.admin.ip})?\\s\\|\\s(%{DATA:ping_federate.admin.event.detail_id})?\\s\\|\\s(%{WORD:ping_federate.admin.component})\\s\\|\\s(%{WORD:ping_federate.admin.event.type})?\\s\\|\\s(%{GREEDYDATA:ping_federate.admin.message})?
                        if !cached_grok!("%{DATA:ping_federate.admin.timestamp}\\s\\|\\s%{WORD:ping_federate.admin.user}\\s\\|\\s(%{DATA:ping_federate.admin.roles})?\\s\\|\\s(%{IP:ping_federate.admin.ip})?\\s\\|\\s(%{DATA:ping_federate.admin.event.detail_id})?\\s\\|\\s(%{WORD:ping_federate.admin.component})\\s\\|\\s(%{WORD:ping_federate.admin.event.type})?\\s\\|\\s(%{GREEDYDATA:ping_federate.admin.message})?").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
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
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && (event.get_str("ping_federate.admin.event.type") == Some("LOGIN_ATTEMPT")
                        || event.get_str("ping_federate.admin.event.type") == Some("LOGOUT")
                        || event.get_str("ping_federate.admin.event.type")
                            == Some("PASSWORD_CHANGE"))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event.has_value("ping_federate.admin.component")
                    && (event.get_str("ping_federate.admin.event.type") == Some("IMPORT")
                        || event.get_str("ping_federate.admin.event.type") == Some("ROTATE")
                        || (event.get_str("ping_federate.admin.event.type") == Some("CREATE")
                            && event.get_str("ping_federate.admin.component") != Some("USER"))
                        || event.get_str("ping_federate.admin.event.type") == Some("DELETE")
                        || event.get_str("ping_federate.admin.event.type") == Some("MODIFY"))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event.has_value("ping_federate.admin.component")
                    && ((event.get_str("ping_federate.admin.event.type") == Some("CREATE")
                        && event.get_str("ping_federate.admin.component") == Some("USER"))
                        || event.get_str("ping_federate.admin.event.type") == Some("ROLE_CHANGE")
                        || event.get_str("ping_federate.admin.event.type") == Some("ACTIVATE"))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event
                        .get("ping_federate.admin.event.type")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SESSION"))
                            }
                            serde_json::Value::String(s) => s.contains("SESSION"),
                            _ => false,
                        })
            };
            if _cond {
                event.append("event.category", json!("session"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && (event.get_str("ping_federate.admin.event.type") == Some("PASSWORD_CHANGE")
                        || event.get_str("ping_federate.admin.event.type") == Some("IMPORT"))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && (event.get_str("ping_federate.admin.event.type") == Some("SESSION_TIMEOUT")
                        || event.get_str("ping_federate.admin.event.type") == Some("LOGOUT"))
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && (event.get_str("ping_federate.admin.event.type") == Some("ROLE_CHANGE")
                        || event.get_str("ping_federate.admin.event.type") == Some("MODIFY")
                        || event.get_str("ping_federate.admin.event.type") == Some("ROTATE"))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && (event.get_str("ping_federate.admin.event.type") == Some("ROLE_CHANGE")
                        || event.get_str("ping_federate.admin.event.type") == Some("ACTIVATE"))
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event.get_str("ping_federate.admin.event.type") == Some("LOGIN_ATTEMPT")
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event.get_str("ping_federate.admin.event.type") == Some("DELETE")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.event.type")
                    && event.get_str("ping_federate.admin.event.type") == Some("CREATE")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.timestamp")
                    && event.get_str("ping_federate.admin.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ping_federate.admin.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss,SSS", "yyyy-MM-dd H:mm:ss,SSS"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("ping_federate.admin.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ping_federate.admin.timestamp".into(),
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
                        "date_ping_federate_admin_timestamp",
                    )?;
                    if event.remove("ping_federate.admin.timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.admin.timestamp".into(),
                        });
                    }
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
            }

            let _cond = {
                event.has_value("ping_federate.admin.timestamp")
                    && event.get_str("ping_federate.admin.timestamp") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("ping_federate.admin.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("ping_federate.admin.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "ping_federate.admin.timestamp".into(),
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
                        "date_ping_federate_admin_timestamp_timezone",
                    )?;
                    if event.remove("ping_federate.admin.timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.admin.timestamp".into(),
                        });
                    }
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
            }

            if let Some(v) = event
                .get("ping_federate.admin.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("ping_federate.admin.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("ping_federate.admin.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("ping_federate.admin.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("ping_federate.admin.roles")
                    && event
                        .get("ping_federate.admin.roles")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("ping_federate.admin.roles") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("ping_federate.admin.roles", Value::Array(parts))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event.remove("ping_federate.admin.roles").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.admin.roles".into(),
                        });
                    }
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
            }

            let _cond = {
                event
                    .get("ping_federate.admin.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("ping_federate.admin.roles").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                let _cond = { event.has_value("ping_federate.admin.roles") };
                                if _cond {
                                    event.append_unique(
                                        "user.roles",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
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
                                "ping_federate.admin.roles",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("ping_federate.admin.roles") && !event.has_value("user.roles") };
            if _cond {
                event.append_unique(
                    "user.roles",
                    json!(
                        event
                            .get("ping_federate.admin.roles")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("ping_federate.admin.event.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if let Some(v) = event
                .get("ping_federate.admin.event.detail_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("ping_federate.admin.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.get_str("ping_federate.admin.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("ping_federate.admin.ip") {
                        if let Some(val) = event.get("ping_federate.admin.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "ping_federate.admin.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("ping_federate.admin.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ping_federate_admin_ip_to_ip",
                    )?;
                    if event.remove("ping_federate.admin.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "ping_federate.admin.ip".into(),
                        });
                    }
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
            }

            if let Some(v) = event
                .get("ping_federate.admin.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            let _cond = { event.has_value("ping_federate.admin.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("ping_federate.admin.ip")
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
                event.remove("ping_federate.admin.timestamp");
                event.remove("ping_federate.admin.user");
                event.remove("ping_federate.admin.roles");
                event.remove("ping_federate.admin.ip");
                event.remove("ping_federate.admin.event.type");
                event.remove("ping_federate.admin.event.detail_id");
                event.remove("ping_federate.admin.message");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
