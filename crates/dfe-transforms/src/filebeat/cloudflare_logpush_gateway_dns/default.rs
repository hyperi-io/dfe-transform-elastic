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

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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
                event.set("cloudflare_logpush.gateway_dns.timestamp", v)?;
            }

            if event.has_value("json.DeviceID") {
                event.rename("json.DeviceID", "cloudflare_logpush.gateway_dns.host.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.DeviceName") {
                event.rename(
                    "json.DeviceName",
                    "cloudflare_logpush.gateway_dns.host.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("json.Email") {
                event.rename("json.Email", "cloudflare_logpush.gateway_dns.user.email")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond =
                { event.has_value("json.DstIP") && event.get_str("json.DstIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.DstIP") {
                        if let Some(val) = event.get("json.DstIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.DstIP".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.gateway_dns.destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dstip_to_ip")?;
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
                .get("cloudflare_logpush.gateway_dns.destination.ip")
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
                if event.has_value("json.DstPort") {
                    if let Some(val) = event.get("json.DstPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.DstPort".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_dns.destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_dstport_to_long",
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
                .get("cloudflare_logpush.gateway_dns.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.Protocol") {
                event.rename("json.Protocol", "cloudflare_logpush.gateway_dns.protocol")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("json.QueryName") {
                event.rename(
                    "json.QueryName",
                    "cloudflare_logpush.gateway_dns.question.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.question.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            if event.has_value("json.QueryTypeName") {
                event.rename(
                    "json.QueryTypeName",
                    "cloudflare_logpush.gateway_dns.question.type",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.question.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            let _cond = { event.has_value("json.RCode") && event.get_i64("json.RCode") == Some(0) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("json.RCode") && event.get_i64("json.RCode").is_some_and(|n| n > 0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("json.RCode") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.RCode") {
                        if let Some(val) = event.get("json.RCode") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.RCode".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.gateway_dns.response_code", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_RCode_to_cloudflare_logpush_gateway_dns_response_code_a0eb5c53")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.dns = ctx.dns ?: [:];\ndef code = ctx.cloudflare_logpush?.gateway_dns?.response_code;\nif (code != null && params.containsKey(code)) {\n  ctx.dns.response_code = params[code];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.dns = ctx.dns ?: [:];\ndef code = ctx.cloudflare_logpush?.gateway_dns?.response_code;\nif (code != null && params.containsKey(code)) {\n  ctx.dns.response_code = params[code];\n}\n"#
                    ),
                    cached_params!(
                        "{\"0\":\"NoError\",\"1\":\"FormErr\",\"2\":\"ServFail\",\"3\":\"NXDomain\",\"4\":\"NotImp\",\"5\":\"Refused\",\"6\":\"YXDomain\",\"7\":\"YXRRSet\",\"8\":\"NXRRSet\",\"9\":\"NotAuth\",\"10\":\"NotZone\",\"11\":\"DSOTYPENI\",\"16\":\"BADVERS\",\"17\":\"BADKEY\",\"18\":\"BADTIME\",\"19\":\"BADMODE\",\"20\":\"BADNAME\",\"21\":\"BADALG\",\"22\":\"BADTRUNC\",\"23\":\"BADCOOKIE\"}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_map_dns_response_code",
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

            if event.has_value("json.RData") {
                event.rename("json.RData", "cloudflare_logpush.gateway_dns.answers")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.answers")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.answers", v)?;
            }

            let _cond =
                { event.has_value("json.SrcIP") && event.get_str("json.SrcIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SrcIP") {
                        if let Some(val) = event.get("json.SrcIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SrcIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.gateway_dns.source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_srcip_to_ip")?;
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
                .get("cloudflare_logpush.gateway_dns.source.ip")
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
                if event.has_value("json.SrcPort") {
                    if let Some(val) = event.get("json.SrcPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.SrcPort".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_dns.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_srcport_to_long",
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
                .get("cloudflare_logpush.gateway_dns.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.TimeZone") {
                event.rename("json.TimeZone", "cloudflare_logpush.gateway_dns.timezone")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            if event.has_value("json.UserID") {
                event.rename("json.UserID", "cloudflare_logpush.gateway_dns.user.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.gateway_dns.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.ColoCode") {
                event.rename("json.ColoCode", "cloudflare_logpush.gateway_dns.colo.code")?;
            }

            if event.has_value("json.ColoID") {
                event.rename("json.ColoID", "cloudflare_logpush.gateway_dns.colo.id")?;
            }

            if event.has_value("json.AccountID") {
                event.rename(
                    "json.AccountID",
                    "cloudflare_logpush.gateway_dns.account_id",
                )?;
            }

            if event.has_value("json.ApplicationID") {
                if let Some(val) = event.get("json.ApplicationID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ApplicationID".into(),
                            message,
                        }
                    })?;
                    event.set("cloudflare_logpush.gateway_dns.application_id", converted)?;
                }
            }

            if event.has_value("json.ApplicationName") {
                event.rename(
                    "json.ApplicationName",
                    "cloudflare_logpush.gateway_dns.application_name",
                )?;
            }

            let _cond = {
                event.has_value("json.AuthoritativeNameServerIPs")
                    && event.get_str("json.AuthoritativeNameServerIPs") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.AuthoritativeNameServerIPs") {
                        if let Some(val) = event.get("json.AuthoritativeNameServerIPs") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.AuthoritativeNameServerIPs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.gateway_dns.authoritative_name_server_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_AuthoritativeNameServerIPs_to_cloudflare_logpush_gateway_dns_authoritative_name_server_ip_11d7c24b")?;
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

            if event.has_value("json.CNAMECategoryIDs") {
                if let Some(val) = event.get("json.CNAMECategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.CNAMECategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.cname_category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.CNAMECategoryNames") {
                event.rename(
                    "json.CNAMECategoryNames",
                    "cloudflare_logpush.gateway_dns.cname_category.names",
                )?;
            }

            if event.has_value("json.CNAMEs") {
                event.rename("json.CNAMEs", "cloudflare_logpush.gateway_dns.cname")?;
            }

            if event.has_value("json.CNAMEsReversed") {
                event.rename(
                    "json.CNAMEsReversed",
                    "cloudflare_logpush.gateway_dns.cname_reversed",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.CustomResolveDurationMs") {
                    if let Some(val) = event.get("json.CustomResolveDurationMs") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.CustomResolveDurationMs".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_dns.custom_resolver.duration_milli",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_CustomResolveDurationMs_to_cloudflare_logpush_gateway_dns_custom_resolver_duration_milli_7f0fc4e1")?;
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

            if event.has_value("json.CustomResolverAddress") {
                event.rename(
                    "json.CustomResolverAddress",
                    "cloudflare_logpush.gateway_dns.custom_resolver.address",
                )?;
            }

            if event.has_value("json.CustomResolverPolicyID") {
                event.rename(
                    "json.CustomResolverPolicyID",
                    "cloudflare_logpush.gateway_dns.custom_resolver.policy.ids",
                )?;
            }

            if event.has_value("json.CustomResolverPolicyName") {
                event.rename(
                    "json.CustomResolverPolicyName",
                    "cloudflare_logpush.gateway_dns.custom_resolver.policy.names",
                )?;
            }

            if event.has_value("json.CustomResolverResponse") {
                event.rename(
                    "json.CustomResolverResponse",
                    "cloudflare_logpush.gateway_dns.custom_resolver.response",
                )?;
            }

            if event.has_value("json.DoHSubdomain") {
                event.rename(
                    "json.DoHSubdomain",
                    "cloudflare_logpush.gateway_dns.doh_subdomain",
                )?;
            }

            if event.has_value("json.DoTSubdomain") {
                event.rename(
                    "json.DoTSubdomain",
                    "cloudflare_logpush.gateway_dns.dot_subdomain",
                )?;
            }

            if event.has_value("json.EDEErrors") {
                if let Some(val) = event.get("json.EDEErrors") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.EDEErrors".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.extended_dns_error_codes",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.InitialCategoryIDs") {
                if let Some(val) = event.get("json.InitialCategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.InitialCategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.initial_category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.InitialCategoryNames") {
                event.rename(
                    "json.InitialCategoryNames",
                    "cloudflare_logpush.gateway_dns.initial_category.names",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.IsResponseCached") {
                    if let Some(val) = event.get("json.IsResponseCached") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.IsResponseCached".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_dns.is_response_cached",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_IsResponseCached_to_cloudflare_logpush_gateway_dns_is_response_cached_8430241a")?;
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

            if event.has_value("json.Location") {
                event.rename(
                    "json.Location",
                    "cloudflare_logpush.gateway_dns.location.name",
                )?;
            }

            if event.has_value("json.LocationID") {
                event.rename(
                    "json.LocationID",
                    "cloudflare_logpush.gateway_dns.location.id",
                )?;
            }

            if event.has_value("json.MatchedCategoryIDs") {
                if let Some(val) = event.get("json.MatchedCategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.MatchedCategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.matched.category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.MatchedCategoryNames") {
                event.rename(
                    "json.MatchedCategoryNames",
                    "cloudflare_logpush.gateway_dns.matched.category.names",
                )?;
            }

            if event.has_value("json.MatchedIndicatorFeedIDs") {
                if let Some(val) = event.get("json.MatchedIndicatorFeedIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.MatchedIndicatorFeedIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.matched.indicator_feed.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.MatchedIndicatorFeedNames") {
                event.rename(
                    "json.MatchedIndicatorFeedNames",
                    "cloudflare_logpush.gateway_dns.matched.indicator_feed.names",
                )?;
            }

            if event.has_value("json.PolicyID") {
                event.rename("json.PolicyID", "cloudflare_logpush.gateway_dns.policy.id")?;
            }

            if event.has_value("json.PolicyName") {
                event.rename(
                    "json.PolicyName",
                    "cloudflare_logpush.gateway_dns.policy.name",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.gateway_dns.policy.name")
                    && event.has_value("json.Policy")
                    && event.get_str("json.Policy") != Some("")
                    && !condition_eq(
                        event.get("json.Policy"),
                        event.get("cloudflare_logpush.gateway_dns.policy.name"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.gateway_dns.policy.name",
                    json!(
                        event
                            .get("json.Policy")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("cloudflare_logpush.gateway_dns.policy.name") };
            if _cond {
                if event.has_value("json.Policy") {
                    event.rename("json.Policy", "cloudflare_logpush.gateway_dns.policy.name")?;
                }
            }

            if event.has_value("json.QueryCategoryIDs") {
                if let Some(val) = event.get("json.QueryCategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.QueryCategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.question.category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.QueryID") {
                event.rename("json.QueryID", "cloudflare_logpush.gateway_dns.question.id")?;
            }

            if event.has_value("json.QueryIndicatorFeedIDs") {
                if let Some(val) = event.get("json.QueryIndicatorFeedIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.QueryIndicatorFeedIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.question.indicator_feed.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.QueryIndicatorFeedNames") {
                event.rename(
                    "json.QueryIndicatorFeedNames",
                    "cloudflare_logpush.gateway_dns.question.indicator_feed.names",
                )?;
            }

            if event.has_value("json.QueryCategoryNames") {
                event.rename(
                    "json.QueryCategoryNames",
                    "cloudflare_logpush.gateway_dns.question.category.names",
                )?;
            }

            if event.has_value("json.QueryNameReversed") {
                event.rename(
                    "json.QueryNameReversed",
                    "cloudflare_logpush.gateway_dns.question.reversed",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.QuerySize") {
                    if let Some(val) = event.get("json.QuerySize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.QuerySize".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.gateway_dns.question.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_querysize_to_long",
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

            if event.has_value("json.QueryType") {
                event.rename(
                    "json.QueryType",
                    "cloudflare_logpush.gateway_dns.question.type_id",
                )?;
            }

            if event.has_value("json.ResolvedIPCategoryIDs") {
                if let Some(val) = event.get("json.ResolvedIPCategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ResolvedIPCategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.resolved_ip_details.category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.ResolvedIPCategoryNames") {
                event.rename(
                    "json.ResolvedIPCategoryNames",
                    "cloudflare_logpush.gateway_dns.resolved_ip_details.category.names",
                )?;
            }

            if event.has_value("json.ResolvedIPContinentCodes") {
                event.rename(
                    "json.ResolvedIPContinentCodes",
                    "cloudflare_logpush.gateway_dns.resolved_ip_details.continent_codes",
                )?;
            }

            if event.has_value("json.ResolvedIPCountryCodes") {
                event.rename(
                    "json.ResolvedIPCountryCodes",
                    "cloudflare_logpush.gateway_dns.resolved_ip_details.country_codes",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ResolvedIPs") {
                    if let Some(val) = event.get("json.ResolvedIPs") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ResolvedIPs".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_dns.resolved_ip_details.ips",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_ResolvedIPs_to_cloudflare_logpush_gateway_dns_resolved_ip_details_ips_d1c23f97")?;
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
                .get("cloudflare_logpush.gateway_dns.resolved_ip_details.ips")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.resolved_ip", v)?;
            }

            if event.has_value("json.ResolverPolicyID") {
                event.rename(
                    "json.ResolverPolicyID",
                    "cloudflare_logpush.gateway_dns.resolver.policy.ids",
                )?;
            }

            if event.has_value("json.ResolverPolicyName") {
                event.rename(
                    "json.ResolverPolicyName",
                    "cloudflare_logpush.gateway_dns.resolver.policy.names",
                )?;
            }

            if event.has_value("json.ResolverDecision") {
                event.rename(
                    "json.ResolverDecision",
                    "cloudflare_logpush.gateway_dns.resolver_decision",
                )?;
            }

            if event.has_value("json.ResourceRecords") {
                event.rename(
                    "json.ResourceRecords",
                    "cloudflare_logpush.gateway_dns.resource_records.object",
                )?;
            }

            if event.has_value("json.ResourceRecordsJSON") {
                event.rename(
                    "json.ResourceRecordsJSON",
                    "cloudflare_logpush.gateway_dns.resource_records.json",
                )?;
            }

            if event.has_value("json.SrcIPContinentCode") {
                event.rename(
                    "json.SrcIPContinentCode",
                    "cloudflare_logpush.gateway_dns.source_id.continent_code",
                )?;
            }

            if event.has_value("json.SrcIPCountryCode") {
                event.rename(
                    "json.SrcIPCountryCode",
                    "cloudflare_logpush.gateway_dns.source_id.country_code",
                )?;
            }

            if event.has_value("json.TimeZoneInferredMethod") {
                event.rename(
                    "json.TimeZoneInferredMethod",
                    "cloudflare_logpush.gateway_dns.timezone_inferred_method",
                )?;
            }

            let _cond = {
                event.has_value("json.InitialResolvedIPs")
                    && event.get_str("json.InitialResolvedIPs") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.InitialResolvedIPs") {
                        if let Some(val) = event.get("json.InitialResolvedIPs") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.InitialResolvedIPs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.gateway_dns.initial_resolved_ips",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_InitialResolvedIPs_to_cloudflare_logpush_gateway_dns_initial_resolved_ips_b54d142d")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.InternalDNSDurationMs") {
                    if let Some(val) = event.get("json.InternalDNSDurationMs") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.InternalDNSDurationMs".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_dns.internal_dns.duration_ms",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_InternalDNSDurationMs_to_cloudflare_logpush_gateway_dns_internal_dns_duration_ms_5cf064a7")?;
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

            if event.has_value("json.InternalDNSFallbackStrategy") {
                event.rename(
                    "json.InternalDNSFallbackStrategy",
                    "cloudflare_logpush.gateway_dns.internal_dns.fallback_strategy",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.InternalDNSRCode") {
                    if let Some(val) = event.get("json.InternalDNSRCode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.InternalDNSRCode".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.gateway_dns.internal_dns.rcode",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_InternalDNSRCode_to_cloudflare_logpush_gateway_dns_internal_dns_rcode_e0a7366e")?;
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

            if event.has_value("json.InternalDNSViewID") {
                event.rename(
                    "json.InternalDNSViewID",
                    "cloudflare_logpush.gateway_dns.internal_dns.view_id",
                )?;
            }

            if event.has_value("json.InternalDNSZoneID") {
                event.rename(
                    "json.InternalDNSZoneID",
                    "cloudflare_logpush.gateway_dns.internal_dns.zone_id",
                )?;
            }

            if event.has_value("json.QueryApplicationIDs") {
                if let Some(val) = event.get("json.QueryApplicationIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.QueryApplicationIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.question.application.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.QueryApplicationNames") {
                event.rename(
                    "json.QueryApplicationNames",
                    "cloudflare_logpush.gateway_dns.question.application.names",
                )?;
            }

            if event.has_value("json.RedirectTargetURI") {
                event.rename(
                    "json.RedirectTargetURI",
                    "cloudflare_logpush.gateway_dns.redirect_target_uri",
                )?;
            }

            if event.has_value("json.RegistrationID") {
                event.rename(
                    "json.RegistrationID",
                    "cloudflare_logpush.gateway_dns.registration_id",
                )?;
            }

            if event.has_value("json.RequestContextCategoryIDs") {
                if let Some(val) = event.get("json.RequestContextCategoryIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.RequestContextCategoryIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.gateway_dns.request_context_category.ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.RequestContextCategoryNames") {
                event.rename(
                    "json.RequestContextCategoryNames",
                    "cloudflare_logpush.gateway_dns.request_context_category.names",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.question.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.cname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.cname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.gateway_dns.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.gateway_dns.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.gateway_dns.initial_resolved_ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.gateway_dns.initial_resolved_ips",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.gateway_dns.resolved_ip_details.ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.gateway_dns.resolved_ip_details.ips",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.gateway_dns.authoritative_name_server_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.gateway_dns.authoritative_name_server_ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
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
                event.remove("cloudflare_logpush.gateway_dns.timestamp");
                event.remove("cloudflare_logpush.gateway_dns.host.id");
                event.remove("cloudflare_logpush.gateway_dns.host.name");
                event.remove("cloudflare_logpush.gateway_dns.destination.ip");
                event.remove("cloudflare_logpush.gateway_dns.destination.port");
                event.remove("cloudflare_logpush.gateway_dns.protocol");
                event.remove("cloudflare_logpush.gateway_dns.question.name");
                event.remove("cloudflare_logpush.gateway_dns.question.type");
                event.remove("cloudflare_logpush.gateway_dns.response_code");
                event.remove("cloudflare_logpush.gateway_dns.answers");
                event.remove("cloudflare_logpush.gateway_dns.resolved_ip_details.ips");
                event.remove("cloudflare_logpush.gateway_dns.source.ip");
                event.remove("cloudflare_logpush.gateway_dns.source.port");
                event.remove("cloudflare_logpush.gateway_dns.timezone");
                event.remove("cloudflare_logpush.gateway_dns.user.id");
                event.remove("cloudflare_logpush.gateway_dns.user.email");
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
