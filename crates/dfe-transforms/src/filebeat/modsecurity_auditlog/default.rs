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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            if let Some(v) = event
                .get("json.audit_data.server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("modsec.audit.server", v)?;
            }

            if let Some(v) = event
                .get("json.transaction.response.headers.Server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("modsec.audit.server", v)?;
            }

            if let Some(v) = event
                .get("json.transaction.producer.connector")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("modsec.audit.connector", v)?;
            }

            let _cond = {
                (event.has_value("modsec.audit.server")
                    && event
                        .get_str("modsec.audit.server")
                        .is_some_and(|s| s.to_lowercase().contains("nginx")))
                    || (event.has_value("modsec.audit.connector")
                        && event
                            .get_str("modsec.audit.connector")
                            .is_some_and(|s| s.to_lowercase().contains("nginx")))
            };
            if _cond {
                // Begin nested pipeline: "nginx-modsec"
                if event.has_value("json.transaction.time_stamp") {
                    event.rename("json.transaction.time_stamp", "_temps.date")?;
                }
                let _cond = {
                    event.has_value("_conf.tz_offset")
                        && event.get_str("_conf.tz_offset") != Some("local")
                };
                if _cond {
                    if let Some(v) = event.get("_conf.tz_offset").cloned() {
                        event.set("_temps.tz", v)?;
                    }
                }
                let _cond = { event.has_value("event.timezone") };
                if _cond {
                    if let Some(v) = event.get("event.timezone").cloned() {
                        if !event.has("_temps.tz") {
                            event.set("_temps.tz", v)?;
                        }
                    }
                }
                if !event.has("_temps.tz") {
                    event.set("_temps.tz", json!("UTC"))?;
                }
                let _cond = { event.has_value("_temps.tz") };
                if _cond {
                    gsub_field(
                        event,
                        "_temps.tz",
                        "_temps.tz",
                        cached_regex!("^([-+]\\d{2})(\\d{2})$"),
                        "$1:$2",
                    )?;
                }
                if let Some(v) = event.get("_temps.tz").cloned() {
                    event.set("event.timezone", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temps.date") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "d/MMM/yyyy:HH:mm:ss.SSS",
                                "d/MMM/yyyy:HH:mm:ss.SSS Z",
                                "d/MMM/yyyy:HH:mm:ss.SSSSSS",
                                "d/MMM/yyyy:HH:mm:ss.SSSSSS Z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temps.date".into(),
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
                        "date__temps_date_3a8c247e",
                    )?;
                    event.remove("event.timezone");
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_temps.date") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "E MMM dd HH:mm:ss yyyy",
                                    "E MMM  d HH:mm:ss yyyy",
                                    "d/MMM/yyyy:HH:mm:ss.SSS",
                                    "d/MMM/yyyy:HH:mm:ss.SSS Z",
                                    "d/MMM/yyyy:HH:mm:ss.SSSSSS",
                                    "d/MMM/yyyy:HH:mm:ss.SSSSSS Z",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_temps.date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_date")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.transaction.client_ip") {
                    event.rename("json.transaction.client_ip", "source.ip")?;
                }
                if event.has_value("json.transaction.client_port") {
                    event.rename("json.transaction.client_port", "source.port")?;
                }
                if event.has_value("json.transaction.request.method") {
                    event.rename("json.transaction.request.method", "http.request.method")?;
                }
                if event.has_value("json.transaction.request.http_version") {
                    if let Some(val) = event.get("json.transaction.request.http_version") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.transaction.request.http_version".into(),
                                message,
                            }
                        })?;
                        event.set("http.version", converted)?;
                    }
                }
                if event.has_value("json.transaction.request.headers.host") {
                    event.rename(
                        "json.transaction.request.headers.host",
                        "json.transaction.request.headers.Host",
                    )?;
                }
                let _cond = { event.get_i64("json.transaction.host_port") == Some(443) };
                if _cond {
                    event.set(
                        "_temps.url",
                        json!(format!(
                            "https://{}:{}{}",
                            event
                                .get("json.transaction.request.headers.Host")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.host_port")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.request.uri")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.get_i64("json.transaction.host_port") == Some(80) };
                if _cond {
                    event.set(
                        "_temps.url",
                        json!(format!(
                            "http://{}:{}{}",
                            event
                                .get("json.transaction.request.headers.Host")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.host_port")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.request.uri")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "_temps.url", "url", true, true)?;
                    Ok(())
                })();
                if event.has_value("json.transaction.response.http_code") {
                    event.rename(
                        "json.transaction.response.http_code",
                        "http.response.status_code",
                    )?;
                }
                if event.has_value("json.transaction.response.headers.Content-Type") {
                    event.rename(
                        "json.transaction.response.headers.Content-Type",
                        "http.response.mime_type",
                    )?;
                }
                if event.has_value("json.transaction.response.Content-Length") {
                    event.rename(
                        "json.transaction.response.Content-Length",
                        "http.response.bytes",
                    )?;
                }
                if event.has_value("json.transaction.messages") {
                    foreach_array(event, "json.transaction.messages", |event| {
                        event.append(
                            "modsec.audit.messages",
                            json!(
                                event
                                    .get("_ingest._value.message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                if event.has_value("json.transaction.messages") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.transaction.messages").cloned();
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
                                if event.remove("_ingest._value.message").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.message".into(),
                                    });
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
                                "json.transaction.messages",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.transaction.messages") && event.get("json.transaction.messages").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } != 0)
                };
                if _cond {
                    event.rename("json.transaction.messages", "modsec.audit.details")?;
                }
                if event.has_value("json.transaction.request.headers.User-Agent") {
                    if let Some(ua_str) =
                        event.get_string("json.transaction.request.headers.User-Agent")
                    {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
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
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("web"))?;
                event.append("event.type", json!("access"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    event.remove("_conf");
                    event.remove("_temps");
                    Ok(())
                })();
                // End nested pipeline: "nginx-modsec"
            }

            let _cond = {
                (event.has_value("modsec.audit.server")
                    && event
                        .get_str("modsec.audit.server")
                        .is_some_and(|s| s.to_lowercase().contains("apache")))
                    || (event.has_value("modsec.audit.connector")
                        && event
                            .get_str("modsec.audit.connector")
                            .is_some_and(|s| s.to_lowercase().contains("apache")))
            };
            if _cond {
                // Begin nested pipeline: "apache-modsec"
                if event.has_value("json.transaction.time") {
                    event.rename("json.transaction.time", "_temps.date")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temps.date") {
                        // Grok pattern: (?:%{MONTHDAY}/%{MONTH}/(?:\\d{4}):%{TIME} (?:(?P<_temps_tz>(?:(?:[APMCE][SD]T|UTC|[-+]\\d{2}:?\\d{2})))|Z))
                        if !cached_grok_mapped!("(?:%{MONTHDAY}/%{MONTH}/(?:\\d{4}):%{TIME} (?:(?P<_temps_tz>(?:(?:[APMCE][SD]T|UTC|[-+]\\d{2}:?\\d{2})))|Z))", [("_temps_tz", "_temps.tz")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_temps.tz") };
                if _cond {
                    gsub_field(
                        event,
                        "_temps.tz",
                        "_temps.tz",
                        cached_regex!("^([-+]\\d{2})(\\d{2})$"),
                        "$1:$2",
                    )?;
                }
                let _cond = {
                    event.has_value("_conf.tz_offset")
                        && event.get_str("_conf.tz_offset") != Some("local")
                };
                if _cond {
                    if let Some(v) = event.get("_conf.tz_offset").cloned() {
                        if !event.has("_temps.tz") {
                            event.set("_temps.tz", v)?;
                        }
                    }
                }
                let _cond = { event.has_value("event.timezone") };
                if _cond {
                    if let Some(v) = event.get("event.timezone").cloned() {
                        if !event.has("_temps.tz") {
                            event.set("_temps.tz", v)?;
                        }
                    }
                }
                if !event.has("_temps.tz") {
                    event.set("_temps.tz", json!("UTC"))?;
                }
                if let Some(v) = event.get("_temps.tz").cloned() {
                    event.set("event.timezone", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temps.date") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "d/MMM/yyyy:HH:mm:ss",
                                "d/MMM/yyyy:HH:mm:ss Z",
                                "d/MMM/yyyy:HH:mm:ss.SSS",
                                "d/MMM/yyyy:HH:mm:ss.SSS Z",
                                "d/MMM/yyyy:HH:mm:ss.SSSSSS",
                                "d/MMM/yyyy:HH:mm:ss.SSSSSS Z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temps.date".into(),
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
                        "date__temps_date_7135cc5a",
                    )?;
                    event.remove("event.timezone");
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_temps.date") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "d/MMM/yyyy:HH:mm:ss",
                                    "d/MMM/yyyy:HH:mm:ss Z",
                                    "d/MMM/yyyy:HH:mm:ss.SSS",
                                    "d/MMM/yyyy:HH:mm:ss.SSS Z",
                                    "d/MMM/yyyy:HH:mm:ss.SSSSSS",
                                    "d/MMM/yyyy:HH:mm:ss.SSSSSS Z",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_temps.date".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_date")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.transaction.remote_address") {
                    event.rename("json.transaction.remote_address", "source.ip")?;
                }
                if event.has_value("json.transaction.local_address") {
                    event.rename("json.transaction.local_address", "destination.ip")?;
                }
                if event.has_value("json.transaction.remote_port") {
                    event.rename("json.transaction.remote_port", "source.port")?;
                }
                if let Some(input) = event.get_string("json.request.request_line") {
                    // Grok pattern: %{NOTSPACE:http.request.method} %{URIPATHPARAM:url.original}(?: HTTP/%{NUMBER:http.version})
                    if !cached_grok!("%{NOTSPACE:http.request.method} %{URIPATHPARAM:url.original}(?: HTTP/%{NUMBER:http.version})").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                if event.has_value("json.transaction.request.headers.host") {
                    event.rename(
                        "json.transaction.request.headers.host",
                        "json.transaction.request.headers.Host",
                    )?;
                }
                let _cond = { event.get_i64("json.transaction.local_port") == Some(443) };
                if _cond {
                    event.set(
                        "_temps.url",
                        json!(format!(
                            "https://{}:{}{}",
                            event
                                .get("json.request.headers.Host")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.#local_port")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("url.original")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.get_i64("json.transaction.local_port") == Some(80) };
                if _cond {
                    event.set(
                        "_temps.url",
                        json!(format!(
                            "http://{}:{}{}",
                            event
                                .get("json.request.headers.Host")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.transaction.#local_port")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("url.original")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "_temps.url", "url", true, true)?;
                    Ok(())
                })();
                if event.has_value("json.response.status") {
                    event.rename("json.response.status", "http.response.status_code")?;
                }
                if event.has_value("json.transaction.transaction_id") {
                    event.rename("json.transaction.transaction_id", "transaction.id")?;
                }
                if event.has_value("json.response.headers.Content-Type") {
                    event.rename(
                        "json.response.headers.Content-Type",
                        "http.response.mime_type",
                    )?;
                }
                if event.has_value("json.request.headers.Content-Type") {
                    event.rename(
                        "json.request.headers.Content-Type",
                        "http.request.mime_type",
                    )?;
                }
                if event.has_value("json.response.headers.Content-Length") {
                    event.rename(
                        "json.response.headers.Content-Length",
                        "http.response.bytes",
                    )?;
                }
                if event.has_value("http.response.bytes") {
                    if let Some(val) = event.get("http.response.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "http.response.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.bytes", converted)?;
                    }
                }
                if event.has_value("json.request.headers.Content-Length") {
                    event.rename("json.request.headers.Content-Length", "http.request.bytes")?;
                }
                if event.has_value("http.request.bytes") {
                    if let Some(val) = event.get("http.request.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "http.request.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("http.request.bytes", converted)?;
                    }
                }
                if event.has_value("json.request.body") {
                    event.rename("json.request.body", "http.request.body.content")?;
                }
                if event.has_value("json.response.body") {
                    event.rename("json.response.body", "http.response.body.content")?;
                }
                if event.has_value("json.request.headers.REMOTE_USER") {
                    event.rename("json.request.headers.REMOTE_USER", "user.name")?;
                }
                if event.has_value("json.request.headers.Referer") {
                    event.rename("json.request.headers.Referer", "http.request.referrer")?;
                }
                if event.has_value("json.audit_data.messages") {
                    event.rename("json.audit_data.messages", "modsec.audit.details")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.modsec?.audit?.details == null || ctx.modsec.audit.details.length == 0) {\n  return;\n}\ndef details = ctx.modsec.audit.details;\ndef messages = new ArrayList();\ndef detObj = new ArrayList();\nfor (def i = 0; i < details.length; i++) {\n  def idx = details[i].indexOf(' ['); // Find first key value mark.\n  if (idx < 0) {\n    idx = details[i].length;\n  }\n  messages.add(details[i].substring(0, idx));\n\n  // Make modsec.audit.details an object since it is a flattened and so\n  // must be an object, while apache leaves this as an array of strings.\n  def obj = new HashMap();\n  obj['value'] = details[i];\n  detObj.add(obj);\n}\nctx.modsec.audit.messages = messages;\nctx.modsec.audit.details = detObj;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.modsec?.audit?.details == null || ctx.modsec.audit.details.length == 0) {\n  return;\n}\ndef details = ctx.modsec.audit.details;\ndef messages = new ArrayList();\ndef detObj = new ArrayList();\nfor (def i = 0; i < details.length; i++) {\n  def idx = details[i].indexOf(' ['); // Find first key value mark.\n  if (idx < 0) {\n    idx = details[i].length;\n  }\n  messages.add(details[i].substring(0, idx));\n\n  // Make modsec.audit.details an object since it is a flattened and so\n  // must be an object, while apache leaves this as an array of strings.\n  def obj = new HashMap();\n  obj['value'] = details[i];\n  detObj.add(obj);\n}\nctx.modsec.audit.messages = messages;\nctx.modsec.audit.details = detObj;\n"#
                        ),
                    )?;
                    Ok(())
                })();
                if event.has_value("json.request.headers.User-Agent") {
                    if let Some(ua_str) = event.get_string("json.request.headers.User-Agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("web"))?;
                event.append("event.type", json!("access"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json");
                    event.remove("_conf");
                    event.remove("_temps");
                    Ok(())
                })();
                // End nested pipeline: "apache-modsec"
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
