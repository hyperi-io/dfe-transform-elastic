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
                parse_json_field(event, "event.original", "json")?;
            }

            event.set(
                "event.category",
                Value::Array(vec![json!("network"), json!("session")]),
            )?;

            event.set("event.type", Value::Array(vec![json!("connection")]))?;

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond = {
                event.has_value("json.SessionStartTime")
                    && event.get_str("json.SessionStartTime") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.SessionStartTime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.SessionStartTime".into(),
                                message,
                            }
                        })?;
                        event.set("json.SessionStartTime", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.SessionEndTime")
                    && event.get_str("json.SessionEndTime") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.SessionEndTime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.SessionEndTime".into(),
                                message,
                            }
                        })?;
                        event.set("json.SessionEndTime", converted)?;
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.SessionStartTime != null && ctx.json.SessionStartTime instanceof Number) {\n  ctx.json.SessionStartTime = convertToMillis(ctx.json.SessionStartTime);\n}\nif (ctx.json?.SessionEndTime != null && ctx.json.SessionEndTime instanceof Number) {\n  ctx.json.SessionEndTime = convertToMillis(ctx.json.SessionEndTime);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.SessionStartTime != null && ctx.json.SessionStartTime instanceof Number) {\n  ctx.json.SessionStartTime = convertToMillis(ctx.json.SessionStartTime);\n}\nif (ctx.json?.SessionEndTime != null && ctx.json.SessionEndTime instanceof Number) {\n  ctx.json.SessionEndTime = convertToMillis(ctx.json.SessionEndTime);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "painless_session_start_time_to_milli",
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

            let _cond = {
                event.has_value("json.SessionStartTime")
                    && event.get_str("json.SessionStartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.SessionStartTime") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.network_session.session.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.SessionStartTime".into(),
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
                        "date_json_SessionStartTime_4818ec3f",
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
                .get("cloudflare_logpush.network_session.session.start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.session.start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.network_session.timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.SessionEndTime")
                    && event.get_str("json.SessionEndTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.SessionEndTime") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.network_session.session.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.SessionEndTime".into(),
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
                        "date_json_SessionEndTime_to_json_SessionEndTime_d82a46a9",
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
                .get("cloudflare_logpush.network_session.session.end")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has_value("json.BytesReceived") {
                event.rename(
                    "json.BytesReceived",
                    "cloudflare_logpush.network_session.destination.bytes",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.destination.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.bytes", v)?;
            }

            if event.has_value("json.BytesSent") {
                event.rename(
                    "json.BytesSent",
                    "cloudflare_logpush.network_session.source.bytes",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.source.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            if event.has_value("json.DetectedProtocol") {
                event.rename(
                    "json.DetectedProtocol",
                    "cloudflare_logpush.network_session.detected_protocol",
                )?;
            }

            if event.has_value("json.DeviceID") {
                event.rename(
                    "json.DeviceID",
                    "cloudflare_logpush.network_session.host.id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.DeviceName") {
                event.rename(
                    "json.DeviceName",
                    "cloudflare_logpush.network_session.host.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond =
                { event.has_value("json.OriginIP") && event.get_str("json.OriginIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginIP") {
                        if let Some(val) = event.get("json.OriginIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginIP".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_session.destination.ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_originip_to_ip")?;
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
                .get("cloudflare_logpush.network_session.destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
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

            if event.has_value("json.OriginPort") {
                event.rename(
                    "json.OriginPort",
                    "cloudflare_logpush.network_session.destination.port",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.OriginTLSCertificateIssuer") {
                event.rename(
                    "json.OriginTLSCertificateIssuer",
                    "cloudflare_logpush.network_session.tls.server.certificate.issuer",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.tls.server.certificate.issuer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.issuer", v)?;
            }

            if event.has_value("json.Protocol") {
                event.rename(
                    "json.Protocol",
                    "cloudflare_logpush.network_session.transport",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.transport")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("json.SessionID") {
                event.rename(
                    "json.SessionID",
                    "cloudflare_logpush.network_session.session.id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.session.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("event.start")
                    && event.has_value("event.end")
                    && !event.has_value("event.duration")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\nZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\nctx.event.duration = ChronoUnit.NANOS.between(start, end);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\nZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\nctx.event.duration = ChronoUnit.NANOS.between(start, end);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_efba3c4b")?;
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

            let _cond =
                { event.has_value("json.SourceIP") && event.get_str("json.SourceIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SourceIP") {
                        if let Some(val) = event.get("json.SourceIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SourceIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.network_session.source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_sourceip_to_ip")?;
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
                .get("cloudflare_logpush.network_session.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("json.SourcePort") {
                event.rename(
                    "json.SourcePort",
                    "cloudflare_logpush.network_session.source.port",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.UserID") {
                event.rename("json.UserID", "cloudflare_logpush.network_session.user.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.Email") {
                event.rename(
                    "json.Email",
                    "cloudflare_logpush.network_session.user.email",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if event.has_value("json.VirtualNetworkID") {
                event.rename(
                    "json.VirtualNetworkID",
                    "cloudflare_logpush.network_session.vlan.id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.vlan.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.vlan.id", v)?;
            }

            if event.has_value("json.AccountID") {
                event.rename(
                    "json.AccountID",
                    "cloudflare_logpush.network_session.account_id",
                )?;
            }

            if event.has_value("json.ClientTCPHandshakeDurationMs") {
                event.rename(
                    "json.ClientTCPHandshakeDurationMs",
                    "cloudflare_logpush.network_session.tcp.client.handshake_time_ms",
                )?;
            }

            if event.has_value("json.ConnectionCloseReason") {
                event.rename(
                    "json.ConnectionCloseReason",
                    "cloudflare_logpush.network_session.tcp.connection.close_reason",
                )?;
            }

            if event.has_value("json.ConnectionReuse") {
                event.rename(
                    "json.ConnectionReuse",
                    "cloudflare_logpush.network_session.tcp.connection.reuse",
                )?;
            }

            if event.has_value("json.DestinationTunnelID") {
                event.rename(
                    "json.DestinationTunnelID",
                    "cloudflare_logpush.network_session.destination.tunnel_id",
                )?;
            }

            if event.has_value("json.EgressColoName") {
                event.rename(
                    "json.EgressColoName",
                    "cloudflare_logpush.network_session.egress.colo_name",
                )?;
            }

            let _cond =
                { event.has_value("json.EgressIP") && event.get_str("json.EgressIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EgressIP") {
                        if let Some(val) = event.get("json.EgressIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EgressIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.network_session.egress.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_egressip_to_ip")?;
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

            if event.has_value("json.EgressPort") {
                event.rename(
                    "json.EgressPort",
                    "cloudflare_logpush.network_session.egress.port",
                )?;
            }

            if event.has_value("json.EgressRuleID") {
                event.rename(
                    "json.EgressRuleID",
                    "cloudflare_logpush.network_session.egress.rule.id",
                )?;
            }

            if event.has_value("json.EgressRuleName") {
                event.rename(
                    "json.EgressRuleName",
                    "cloudflare_logpush.network_session.egress.rule.name",
                )?;
            }

            if event.has_value("json.IngressColoName") {
                event.rename(
                    "json.IngressColoName",
                    "cloudflare_logpush.network_session.ingress.colo_name",
                )?;
            }

            if event.has_value("json.Offramp") {
                event.rename("json.Offramp", "cloudflare_logpush.network_session.offramp")?;
            }

            if event.has_value("json.RuleEvaluationDurationMs") {
                event.rename(
                    "json.RuleEvaluationDurationMs",
                    "cloudflare_logpush.network_session.rule_evaluation.time_ms",
                )?;
            }

            let _cond = {
                event.has_value("json.SourceInternalIP")
                    && event.get_str("json.SourceInternalIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SourceInternalIP") {
                        if let Some(val) = event.get("json.SourceInternalIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SourceInternalIP".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_session.source.internal_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sourceinternalip_to_ip",
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

            if event.has_value("json.ClientTLSCipher") {
                event.rename(
                    "json.ClientTLSCipher",
                    "cloudflare_logpush.network_session.tls.client.cipher",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.tls.client.cipher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.cipher", v)?;
            }

            if event.has_value("json.ClientTLSHandshakeDurationMs") {
                event.rename(
                    "json.ClientTLSHandshakeDurationMs",
                    "cloudflare_logpush.network_session.tls.client.handshake_time_ms",
                )?;
            }

            if event.has_value("json.ClientTLSVersion") {
                event.rename(
                    "json.ClientTLSVersion",
                    "cloudflare_logpush.network_session.tls.client.version",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_session.tls.client.version")
                    && event.get_str("cloudflare_logpush.network_session.tls.client.version")
                        != Some("none")
                    && event.get_str("cloudflare_logpush.network_session.tls.client.version")
                        != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_session.tls.client.version") {
                        if let Some(input) = event
                            .get_string("cloudflare_logpush.network_session.tls.client.version")
                        {
                            // Grok pattern: %{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}
                            // Grok pattern: %{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "%{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}"
                                    ),
                                    cached_grok!(
                                        "%{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_tls_client_version_to_tls_version_protocol_and_tls_version",
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

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.OriginTLSCertificateValidationResult") {
                event.rename(
                    "json.OriginTLSCertificateValidationResult",
                    "cloudflare_logpush.network_session.tls.server.certificate.validation_result",
                )?;
            }

            if event.has_value("json.OriginTLSCipher") {
                event.rename(
                    "json.OriginTLSCipher",
                    "cloudflare_logpush.network_session.tls.server.cipher",
                )?;
            }

            if event.has_value("json.OriginTLSHandshakeDurationMs") {
                event.rename(
                    "json.OriginTLSHandshakeDurationMs",
                    "cloudflare_logpush.network_session.tls.server.handshake_time_ms",
                )?;
            }

            if event.has_value("json.OriginTLSVersion") {
                event.rename(
                    "json.OriginTLSVersion",
                    "cloudflare_logpush.network_session.tls.server.version",
                )?;
            }

            let _cond = {
                event.has_value("json.InitialOriginIP")
                    && event.get_str("json.InitialOriginIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.InitialOriginIP") {
                        if let Some(val) = event.get("json.InitialOriginIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.InitialOriginIP".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_session.initial_origin_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_InitialOriginIP_to_cloudflare_logpush_network_session_initial_origin_ip_45718b53")?;
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

            if event.has_value("json.RegistrationID") {
                event.rename(
                    "json.RegistrationID",
                    "cloudflare_logpush.network_session.registration_id",
                )?;
            }

            if event.has_value("json.ResolvedFQDN") {
                event.rename(
                    "json.ResolvedFQDN",
                    "cloudflare_logpush.network_session.resolved_fqdn",
                )?;
            }

            if event.has_value("json.SNI") {
                event.rename("json.SNI", "cloudflare_logpush.network_session.sni")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_session.sni")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.server_name", v)?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.egress.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.egress.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.network_session.source.internal_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.source.internal_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.initial_origin_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.initial_origin_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.resolved_fqdn") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.resolved_fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_session.sni") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.network_session.sni")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");
            event.remove("_conf");

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
                event.remove("cloudflare_logpush.network_session.timestamp");
                event.remove("cloudflare_logpush.network_session.destination.bytes");
                event.remove("cloudflare_logpush.network_session.source.bytes");
                event.remove("cloudflare_logpush.network_session.destination.ip");
                event.remove("cloudflare_logpush.network_session.destination.port");
                event.remove("cloudflare_logpush.network_session.host.id");
                event.remove("cloudflare_logpush.network_session.host.name");
                event.remove("cloudflare_logpush.network_session.tls.server.certificate.issuer");
                event.remove("cloudflare_logpush.network_session.sni");
                event.remove("cloudflare_logpush.network_session.transport");
                event.remove("cloudflare_logpush.network_session.session.id");
                event.remove("cloudflare_logpush.network_session.session.start");
                event.remove("cloudflare_logpush.network_session.session.end");
                event.remove("cloudflare_logpush.network_session.source.ip");
                event.remove("cloudflare_logpush.network_session.source.port");
                event.remove("cloudflare_logpush.network_session.user.id");
                event.remove("cloudflare_logpush.network_session.user.email");
                event.remove("cloudflare_logpush.network_session.vlan.id");
                event.remove("cloudflare_logpush.network_session.tls.client.cipher");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
