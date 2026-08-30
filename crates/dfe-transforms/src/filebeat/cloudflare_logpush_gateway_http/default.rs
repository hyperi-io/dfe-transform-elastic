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

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.get_str("json.Action") != Some("")
                    && event.get_str("json.Action") == Some("allow")
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.get_str("json.Action") != Some("")
                    && event.get_str("json.Action") == Some("bypass")
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.get_str("json.Action") != Some("")
                    && event.get_str("json.Action") == Some("block")
            };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond =
                { event.has_value("json.Datetime") && event.get_str("json.Datetime") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Datetime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Datetime".into(),
                                message,
                            }
                        })?;
                        event.set("json.Datetime", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Datetime")
                    && event.get("json.Datetime").is_some_and(|v| v.is_number())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: long t = (long)(ctx.json.Datetime);\nif (t > (long)(1e18)) {\n  ctx.json.Datetime = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Datetime = t*(long)(1e3)\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long t = (long)(ctx.json.Datetime);\nif (t > (long)(1e18)) {\n  ctx.json.Datetime = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Datetime = t*(long)(1e3)\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_datetime_to_milli",
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

            let _cond =
                { event.has_value("json.Datetime") && event.get_str("json.Datetime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Datetime") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Datetime".into(),
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
                        "date_json_Datetime_138a0e6b",
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
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.gateway_http.timestamp", v)?;
            }

            if event.has_value("json.Action") {
                event.rename("json.Action", "cloudflare_logpush.gateway_http.action")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("json.DeviceID") {
                event.rename("json.DeviceID", "cloudflare_logpush.gateway_http.host.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.DeviceName") {
                event.rename(
                    "json.DeviceName",
                    "cloudflare_logpush.gateway_http.host.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = {
                event.has_value("json.DestinationIP")
                    && event.get_str("json.DestinationIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.DestinationIP") {
                        if let Some(val) = event.get("json.DestinationIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.DestinationIP".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.gateway_http.destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destinationip_to_ip",
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
                .get("cloudflare_logpush.gateway_http.destination.ip")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.DestinationPort") {
                    if let Some(val) = event.get("json.DestinationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.DestinationPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_http.destination.port",
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
                    "convert_destinationport_to_long",
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

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.HTTPMethod") {
                event.rename(
                    "json.HTTPMethod",
                    "cloudflare_logpush.gateway_http.request.method",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.HTTPStatusCode") {
                    if let Some(val) = event.get("json.HTTPStatusCode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.HTTPStatusCode".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_http.response.status_code",
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
                    "convert_httpstatuscode_to_long",
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

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.response.status_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if event.has_value("json.HTTPVersion") {
                event.rename(
                    "json.HTTPVersion",
                    "cloudflare_logpush.gateway_http.request.version",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.request.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.version", v)?;
            }

            if event.has_value("json.Referer") {
                event.rename(
                    "json.Referer",
                    "cloudflare_logpush.gateway_http.request.referrer",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.request.referrer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
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
                            event.set("cloudflare_logpush.gateway_http.source.ip", converted)?;
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
                .get("cloudflare_logpush.gateway_http.source.ip")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.SourcePort") {
                    if let Some(val) = event.get("json.SourcePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.SourcePort".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_http.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sourceport_to_long",
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

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.URL") {
                event.rename("json.URL", "cloudflare_logpush.gateway_http.url")?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.url") };
            if _cond {
                uri_parts(
                    event,
                    "cloudflare_logpush.gateway_http.url",
                    "url",
                    true,
                    false,
                )?;
            }

            if event.has_value("json.UserAgent") {
                event.rename(
                    "json.UserAgent",
                    "cloudflare_logpush.gateway_http.user_agent",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.user_agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

            if event.has_value("json.UserID") {
                event.rename("json.UserID", "cloudflare_logpush.gateway_http.user.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.Email") {
                event.rename("json.Email", "cloudflare_logpush.gateway_http.user.email")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if event.has_value("json.AccountID") {
                event.rename(
                    "json.AccountID",
                    "cloudflare_logpush.gateway_http.account_id",
                )?;
            }

            if event.has_value("json.ApplicationIDs") {
                if let Some(val) = event.get("json.ApplicationIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ApplicationIDs".into(),
                            message,
                        }
                    })?;
                    event.set("cloudflare_logpush.gateway_http.application.ids", converted)?;
                }
            }

            if event.has_value("json.ApplicationNames") {
                event.rename(
                    "json.ApplicationNames",
                    "cloudflare_logpush.gateway_http.application.names",
                )?;
            }

            if event.has_value("json.BlockedFileHash") {
                event.rename(
                    "json.BlockedFileHash",
                    "cloudflare_logpush.gateway_http.blocked_file.hash",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.blocked_file.hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if event.has_value("json.BlockedFileName") {
                event.rename(
                    "json.BlockedFileName",
                    "cloudflare_logpush.gateway_http.blocked_file.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.blocked_file.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if event.has_value("json.BlockedFileReason") {
                event.rename(
                    "json.BlockedFileReason",
                    "cloudflare_logpush.gateway_http.blocked_file.reason",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.BlockedFileSize") {
                    if let Some(val) = event.get("json.BlockedFileSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.BlockedFileSize".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_http.blocked_file.size",
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
                    "convert_blockedfilesize_to_long",
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

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.blocked_file.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if event.has_value("json.BlockedFileType") {
                event.rename(
                    "json.BlockedFileType",
                    "cloudflare_logpush.gateway_http.blocked_file.type",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_http.blocked_file.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.extension", v)?;
            }

            if event.has_value("json.CategoryIDs") {
                if let Some(val) = event.get("json.CategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.CategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set("cloudflare_logpush.gateway_http.category.ids", converted)?;
                }
            }

            if event.has_value("json.CategoryNames") {
                event.rename(
                    "json.CategoryNames",
                    "cloudflare_logpush.gateway_http.category.names",
                )?;
            }

            if event.has_value("json.DestinationIPContinentCode") {
                event.rename(
                    "json.DestinationIPContinentCode",
                    "cloudflare_logpush.gateway_http.destination_ip.continent_code",
                )?;
            }

            if event.has_value("json.DestinationIPCountryCode") {
                event.rename(
                    "json.DestinationIPCountryCode",
                    "cloudflare_logpush.gateway_http.destination_ip.country_code",
                )?;
            }

            if event.has_value("json.DownloadedFileNames") {
                event.rename(
                    "json.DownloadedFileNames",
                    "cloudflare_logpush.gateway_http.downloaded_files",
                )?;
            }

            if event.has_value("json.DownloadMatchedDlpProfileEntries") {
                event.rename(
                    "json.DownloadMatchedDlpProfileEntries",
                    "cloudflare_logpush.gateway_http.download_matched_dlp.profile_entries",
                )?;
            }

            if event.has_value("json.DownloadMatchedDlpProfiles") {
                event.rename(
                    "json.DownloadMatchedDlpProfiles",
                    "cloudflare_logpush.gateway_http.download_matched_dlp.profiles",
                )?;
            }

            if event.has_value("json.FileInfo") {
                event.rename("json.FileInfo", "cloudflare_logpush.gateway_http.file_info")?;
            }

            if event.has_value("json.ForensicCopyStatus") {
                event.rename(
                    "json.ForensicCopyStatus",
                    "cloudflare_logpush.gateway_http.forensic_copy_status",
                )?;
            }

            if event.has_value("json.HTTPHost") {
                event.rename(
                    "json.HTTPHost",
                    "cloudflare_logpush.gateway_http.request.host",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.IsIsolated") {
                    if let Some(val) = event.get("json.IsIsolated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.IsIsolated".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_http.isolated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isisolated_to_boolean",
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

            if event.has_value("json.PolicyID") {
                event.rename("json.PolicyID", "cloudflare_logpush.gateway_http.policy.id")?;
            }

            if event.has_value("json.PolicyName") {
                event.rename(
                    "json.PolicyName",
                    "cloudflare_logpush.gateway_http.policy.name",
                )?;
            }

            if event.has_value("json.PrivateAppAUD") {
                event.rename(
                    "json.PrivateAppAUD",
                    "cloudflare_logpush.gateway_http.private_app_aud",
                )?;
            }

            if event.has_value("json.ProxyEndpoint") {
                event.rename(
                    "json.ProxyEndpoint",
                    "cloudflare_logpush.gateway_http.proxy_endpoint",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Quarantined") {
                    if let Some(val) = event.get("json.Quarantined") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Quarantined".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_http.quarantined", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_Quarantined_to_cloudflare_logpush_gateway_http_quarantined_f31dfd9b")?;
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

            if event.has_value("json.RequestID") {
                event.rename(
                    "json.RequestID",
                    "cloudflare_logpush.gateway_http.request_id",
                )?;
            }

            if event.has_value("json.SessionID") {
                event.rename(
                    "json.SessionID",
                    "cloudflare_logpush.gateway_http.session_id",
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
                                "cloudflare_logpush.gateway_http.source.internal_ip",
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

            if event.has_value("json.SourceIPContinentCode") {
                event.rename(
                    "json.SourceIPContinentCode",
                    "cloudflare_logpush.gateway_http.source_ip.continent_code",
                )?;
            }

            if event.has_value("json.SourceIPCountryCode") {
                event.rename(
                    "json.SourceIPCountryCode",
                    "cloudflare_logpush.gateway_http.source_ip.country_code",
                )?;
            }

            if event.has_value("json.UntrustedCertificateAction") {
                event.rename(
                    "json.UntrustedCertificateAction",
                    "cloudflare_logpush.gateway_http.untrusted_certificate_action",
                )?;
            }

            if event.has_value("json.UploadMatchedDlpProfileEntries") {
                event.rename(
                    "json.UploadMatchedDlpProfileEntries",
                    "cloudflare_logpush.gateway_http.upload_matched_dlp.profile_entries",
                )?;
            }

            if event.has_value("json.UploadMatchedDlpProfiles") {
                event.rename(
                    "json.UploadMatchedDlpProfiles",
                    "cloudflare_logpush.gateway_http.upload_matched_dlp.profiles",
                )?;
            }

            if event.has_value("json.UploadedFileNames") {
                event.rename(
                    "json.UploadedFileNames",
                    "cloudflare_logpush.gateway_http.uploaded_files",
                )?;
            }

            if event.has_value("json.VirtualNetworkID") {
                event.rename(
                    "json.VirtualNetworkID",
                    "cloudflare_logpush.gateway_http.virtual_network.id",
                )?;
            }

            if event.has_value("json.VirtualNetworkName") {
                event.rename(
                    "json.VirtualNetworkName",
                    "cloudflare_logpush.gateway_http.virtual_network.name",
                )?;
            }

            if event.has_value("json.AppControlInfo") {
                event.rename(
                    "json.AppControlInfo",
                    "cloudflare_logpush.gateway_http.app_control_info",
                )?;
            }

            if event.has_value("json.ApplicationStatuses") {
                event.rename(
                    "json.ApplicationStatuses",
                    "cloudflare_logpush.gateway_http.application.statuses",
                )?;
            }

            if event.has_value("json.RedirectTargetURI") {
                event.rename(
                    "json.RedirectTargetURI",
                    "cloudflare_logpush.gateway_http.redirect_target_uri",
                )?;
            }

            if event.has_value("json.RegistrationID") {
                event.rename(
                    "json.RegistrationID",
                    "cloudflare_logpush.gateway_http.registration_id",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.gateway_http.source.internal_ip")
                    && event.get_str("cloudflare_logpush.gateway_http.source.internal_ip")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.source.internal_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.request.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.request.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.blocked_file.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.blocked_file.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_http.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_http.user.email")
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
                event.remove("cloudflare_logpush.gateway_http.timestamp");
                event.remove("cloudflare_logpush.gateway_http.action");
                event.remove("cloudflare_logpush.gateway_http.destination.ip");
                event.remove("cloudflare_logpush.gateway_http.destination.port");
                event.remove("cloudflare_logpush.gateway_http.host.id");
                event.remove("cloudflare_logpush.gateway_http.host.name");
                event.remove("cloudflare_logpush.gateway_http.request.method");
                event.remove("cloudflare_logpush.gateway_http.response.status_code");
                event.remove("cloudflare_logpush.gateway_http.request.referrer");
                event.remove("cloudflare_logpush.gateway_http.request.version");
                event.remove("cloudflare_logpush.gateway_http.source.ip");
                event.remove("cloudflare_logpush.gateway_http.source.port");
                event.remove("cloudflare_logpush.gateway_http.url");
                event.remove("cloudflare_logpush.gateway_http.user_agent");
                event.remove("cloudflare_logpush.gateway_http.user.id");
                event.remove("cloudflare_logpush.gateway_http.user.email");
                event.remove("cloudflare_logpush.gateway_http.blocked_file.hash");
                event.remove("cloudflare_logpush.gateway_http.blocked_file.name");
                event.remove("cloudflare_logpush.gateway_http.blocked_file.size");
                event.remove("cloudflare_logpush.gateway_http.blocked_file.type");
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
