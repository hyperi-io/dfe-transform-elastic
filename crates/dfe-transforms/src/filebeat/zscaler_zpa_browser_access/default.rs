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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.LogTimestamp")
                    && event.get_str("json.LogTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LogTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LogTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.LogTimestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.LogTimestamp".into(),
                        });
                    }
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

            event.append("event.category", json!("network"))?;

            event.append("event.category", json!("session"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("connection"))?;

            if event.has_value("json.ConnectionReason") {
                event.rename("json.ConnectionReason", "event.reason")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ClientPublicIp") {
                    if let Some(val) = event.get("json.ClientPublicIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ClientPublicIp".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ClientPublicIp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ClientPublicIp".into(),
                    });
                }
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

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ClientPublicPort") {
                    if let Some(val) = event.get("json.ClientPublicPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ClientPublicPort".into(),
                                message,
                            }
                        })?;
                        event.set("client.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ClientPublicPort").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ClientPublicPort".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.RequestSize") {
                    if let Some(val) = event.get("json.RequestSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.RequestSize".into(),
                                message,
                            }
                        })?;
                        event.set("http.request.body.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.RequestSize").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.RequestSize".into(),
                    });
                }
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

            if event.has_value("json.Method") {
                event.rename("json.Method", "http.request.method")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ResponseSize") {
                    if let Some(val) = event.get("json.ResponseSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ResponseSize".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.body.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ResponseSize").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ResponseSize".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.StatusCode") {
                    if let Some(val) = event.get("json.StatusCode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.StatusCode".into(),
                                message,
                            }
                        })?;
                        event.set("http.response.status_code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.StatusCode").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.StatusCode".into(),
                    });
                }
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

            if event.has_value("json.Customer") {
                event.rename("json.Customer", "organization.name")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ApplicationPort") {
                    if let Some(val) = event.get("json.ApplicationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ApplicationPort".into(),
                                message,
                            }
                        })?;
                        event.set("server.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ApplicationPort").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ApplicationPort".into(),
                    });
                }
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.Host").cloned() {
                    event.set("server.address", v)?;
                }
                Ok(())
            })();

            // Painless script
            // Source: ctx.url = new HashMap();\ndef protocol = ctx.json?.Protocol?.toLowerCase();\ndef domain = ctx.json?.Host?.toLowerCase();\ndef endpoint = ctx.json?.URL?.toLowerCase();\nif (protocol != null && domain != null && endpoint != null) {\n  ctx.url.full = protocol + '://' + domain + endpoint;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.url = new HashMap();\ndef protocol = ctx.json?.Protocol?.toLowerCase();\ndef domain = ctx.json?.Host?.toLowerCase();\ndef endpoint = ctx.json?.URL?.toLowerCase();\nif (protocol != null && domain != null && endpoint != null) {\n  ctx.url.full = protocol + '://' + domain + endpoint;\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("url.full") };
            if _cond {
                uri_parts(event, "url.full", "url", true, false)?;
            }

            if event.has_value("json.UserAgent") {
                if let Some(ua_str) = event.get_string("json.UserAgent") {
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

            if event.has_value("json.NameID") {
                event.rename("json.NameID", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.ConnectionStatus") {
                event.rename(
                    "json.ConnectionStatus",
                    "zscaler_zpa.browser_access.connection.status",
                )?;
            }

            if event.has_value("json.ConnectionID") {
                event.rename(
                    "json.ConnectionID",
                    "zscaler_zpa.browser_access.connection.id",
                )?;
            }

            if event.has_value("json.Exporter") {
                event.rename("json.Exporter", "zscaler_zpa.browser_access.exporter")?;
            }

            let _cond = {
                event.has_value("json.TimestampRequestReceiveStart")
                    && event.get_str("json.TimestampRequestReceiveStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampRequestReceiveStart")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.request.receive.start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampRequestReceiveStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampRequestReceiveStart").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampRequestReceiveStart".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampRequestReceiveHeaderFinish")
                    && event.get_str("json.TimestampRequestReceiveHeaderFinish") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampRequestReceiveHeaderFinish")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("zscaler_zpa.browser_access.timestamp.request.receive.header_finish", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.TimestampRequestReceiveHeaderFinish".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampRequestReceiveHeaderFinish")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampRequestReceiveHeaderFinish".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampRequestReceiveFinish")
                    && event.get_str("json.TimestampRequestReceiveFinish") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampRequestReceiveFinish")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.request.receive.finish",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampRequestReceiveFinish".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampRequestReceiveFinish").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampRequestReceiveFinish".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampRequestTransmitStart")
                    && event.get_str("json.TimestampRequestTransmitStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampRequestTransmitStart")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.request.transmit.start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampRequestTransmitStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampRequestTransmitStart").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampRequestTransmitStart".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampRequestTransmitFinish")
                    && event.get_str("json.TimestampRequestTransmitFinish") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampRequestTransmitFinish")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.request.transmit.finish",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampRequestTransmitFinish".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampRequestTransmitFinish")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampRequestTransmitFinish".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampResponseReceiveStart")
                    && event.get_str("json.TimestampResponseReceiveStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampResponseReceiveStart")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.response.receive.start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampResponseReceiveStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampResponseReceiveStart").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampResponseReceiveStart".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampResponseReceiveFinish")
                    && event.get_str("json.TimestampResponseReceiveFinish") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampResponseReceiveFinish")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.response.receive.finish",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampResponseReceiveFinish".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampResponseReceiveFinish")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampResponseReceiveFinish".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampResponseTransmitStart")
                    && event.get_str("json.TimestampResponseTransmitStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampResponseTransmitStart")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.response.transmit.start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampResponseTransmitStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampResponseTransmitStart")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampResponseTransmitStart".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("json.TimestampResponseTransmitFinish")
                    && event.get_str("json.TimestampResponseTransmitFinish") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampResponseTransmitFinish")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.browser_access.timestamp.response.transmit.finish",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampResponseTransmitFinish".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampResponseTransmitFinish")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampResponseTransmitFinish".into(),
                        });
                    }
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

            if event.has_value("json.TotalTimeRequestReceive") {
                event.rename(
                    "json.TotalTimeRequestReceive",
                    "zscaler_zpa.browser_access.total_time.request.receive",
                )?;
            }

            if event.has_value("json.TotalTimeRequestTransmit") {
                event.rename(
                    "json.TotalTimeRequestTransmit",
                    "zscaler_zpa.browser_access.total_time.request.transmit",
                )?;
            }

            if event.has_value("json.TotalTimeResponseReceive") {
                event.rename(
                    "json.TotalTimeResponseReceive",
                    "zscaler_zpa.browser_access.total_time.response.receive",
                )?;
            }

            if event.has_value("json.TotalTimeResponseTransmit") {
                event.rename(
                    "json.TotalTimeResponseTransmit",
                    "zscaler_zpa.browser_access.total_time.response.transmit",
                )?;
            }

            if event.has_value("json.TotalTimeConnectionSetup") {
                event.rename(
                    "json.TotalTimeConnectionSetup",
                    "zscaler_zpa.browser_access.total_time.connection.setup",
                )?;
            }

            if event.has_value("json.TotalTimeServerResponse") {
                event.rename(
                    "json.TotalTimeServerResponse",
                    "zscaler_zpa.browser_access.total_time.server.response",
                )?;
            }

            if event.has_value("json.XFF") {
                event.rename("json.XFF", "zscaler_zpa.browser_access.xff")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ClientPrivateIp") {
                    if let Some(val) = event.get("json.ClientPrivateIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ClientPrivateIp".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.browser_access.client_private_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ClientPrivateIp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ClientPrivateIp".into(),
                    });
                }
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

            let _cond = { event.has_value("zscaler_zpa.browser_access.client_private_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.browser_access.client_private_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.CorsToken") {
                event.rename("json.CorsToken", "zscaler_zpa.browser_access.cors_token")?;
            }

            if event.has_value("json.Origin") {
                event.rename("json.Origin", "zscaler_zpa.browser_access.origin")?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            event.remove("json.LogTimestamp");
            event.remove("json.ClientPublicIp");
            event.remove("json.ClientPublicPort");
            event.remove("json.RequestSize");
            event.remove("json.ResponseSize");
            event.remove("json.StatusCode");
            event.remove("json.ApplicationPort");
            event.remove("json.Host");
            event.remove("json.URL");
            event.remove("json.Protocol");
            event.remove("json.UserAgent");
            event.remove("json.TimestampRequestReceiveStart");
            event.remove("json.TimestampRequestReceiveHeaderFinish");
            event.remove("json.TimestampRequestReceiveFinish");
            event.remove("json.TimestampRequestTransmitStart");
            event.remove("json.TimestampRequestTransmitFinish");
            event.remove("json.TimestampResponseReceiveStart");
            event.remove("json.TimestampResponseReceiveFinish");
            event.remove("json.TimestampResponseTransmitStart");
            event.remove("json.TimestampResponseTransmitFinish");
            event.remove("json.ClientPrivateIp");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.browser_access[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.browser_access[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
            }

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
