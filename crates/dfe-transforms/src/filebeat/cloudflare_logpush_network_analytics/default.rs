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
                    event.set("_ingest.on_failure_processor_tag", "date_json_datetime")?;
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
                event.set("cloudflare_logpush.network_analytics.timestamp", v)?;
            }

            let _cond = { event.get_str("json.Outcome") == Some("pass") };
            if _cond {
                event.set(
                    "cloudflare_logpush.network_analytics.outcome",
                    json!("success"),
                )?;
            }

            let _cond = { event.get_str("json.Outcome") == Some("drop") };
            if _cond {
                event.set(
                    "cloudflare_logpush.network_analytics.outcome",
                    json!("failure"),
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.outcome")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.outcome", v)?;
            }

            let _cond = { event.get_str("json.DestinationASN") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.DestinationASN") {
                        if let Some(val) = event.get("json.DestinationASN") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.DestinationASN".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.destination.asn",
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
                        "convert_json_destination_asn",
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
                .get("cloudflare_logpush.network_analytics.destination.asn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.as.number", v)?;
            }

            let _cond = {
                event.has_value("json.IPDestinationAddress")
                    && event.get_str("json.IPDestinationAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPDestinationAddress") {
                        if let Some(val) = event.get("json.IPDestinationAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPDestinationAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.destination.ip",
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
                        "convert_json_ip_destinations_address",
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
                .get("cloudflare_logpush.network_analytics.destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.get_str("json.DestinationPort") != Some("") };
            if _cond {
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
                                "cloudflare_logpush.network_analytics.destination.port",
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
                        "convert_json_destination_port",
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
                .get("cloudflare_logpush.network_analytics.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.Direction") {
                event.rename(
                    "json.Direction",
                    "cloudflare_logpush.network_analytics.direction",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.direction")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.direction", v)?;
            }

            if event.has_value("json.IPProtocolName") {
                event.rename(
                    "json.IPProtocolName",
                    "cloudflare_logpush.network_analytics.ip.protocol.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.ip.protocol.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("json.IPSourceAddress")
                    && event.get_str("json.IPSourceAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPSourceAddress") {
                        if let Some(val) = event.get("json.IPSourceAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPSourceAddress".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ip_source_address",
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
                .get("cloudflare_logpush.network_analytics.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get_str("json.SourceASN") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SourceASN") {
                        if let Some(val) = event.get("json.SourceASN") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SourceASN".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.source.asn",
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
                        "convert_json_source_asn",
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
                .get("cloudflare_logpush.network_analytics.source.asn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.number", v)?;
            }

            let _cond = { event.get_str("json.SourcePort") != Some("") };
            if _cond {
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
                            event.set(
                                "cloudflare_logpush.network_analytics.source.port",
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
                        "convert_json_source_port",
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
                .get("cloudflare_logpush.network_analytics.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.RuleID") {
                event.rename(
                    "json.RuleID",
                    "cloudflare_logpush.network_analytics.rule.id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.rule.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.AttackCampaignID") {
                event.rename(
                    "json.AttackCampaignID",
                    "cloudflare_logpush.network_analytics.attack.campaign.id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.attack.campaign.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.group.id", v)?;
            }

            if event.has_value("json.AttackID") {
                event.rename(
                    "json.AttackID",
                    "cloudflare_logpush.network_analytics.attack.id",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.attack.id") };
            if _cond {
                event.append_unique(
                    "threat.indicator.id",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.attack.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.AttackVector") {
                event.rename(
                    "json.AttackVector",
                    "cloudflare_logpush.network_analytics.attack.vector",
                )?;
            }

            if event.has_value("json.ColoCity") {
                event.rename(
                    "json.ColoCity",
                    "cloudflare_logpush.network_analytics.colo.city",
                )?;
            }

            if event.has_value("json.ColoCode") {
                event.rename(
                    "json.ColoCode",
                    "cloudflare_logpush.network_analytics.colo.code",
                )?;
            }

            if event.has_value("json.ColoCountry") {
                event.rename(
                    "json.ColoCountry",
                    "cloudflare_logpush.network_analytics.colo.country",
                )?;
            }

            if event.has_value("json.ColoGeoHash") {
                event.rename(
                    "json.ColoGeoHash",
                    "cloudflare_logpush.network_analytics.colo.geo_hash",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.colo.geo_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.network_analytics.colo.geo_location", v)?;
            }

            let _cond = { event.get_str("json.ColoID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ColoID") {
                        if let Some(val) = event.get("json.ColoID") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ColoID".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.network_analytics.colo.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_colo_id")?;
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

            if event.has_value("json.ColoName") {
                event.rename(
                    "json.ColoName",
                    "cloudflare_logpush.network_analytics.colo.name",
                )?;
            }

            if event.has_value("json.DestinationASNDescription") {
                event.rename(
                    "json.DestinationASNDescription",
                    "cloudflare_logpush.network_analytics.destination.as.number.description",
                )?;
            }

            if event.has_value("json.DestinationASNName") {
                event.rename(
                    "json.DestinationASNName",
                    "cloudflare_logpush.network_analytics.destination.as.number.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.destination.as.number.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.as.organization.name", v)?;
            }

            if event.has_value("json.DestinationCountry") {
                event.rename(
                    "json.DestinationCountry",
                    "cloudflare_logpush.network_analytics.destination.country",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.destination.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.country_iso_code", v)?;
            }

            if event.has_value("json.DestinationGeoHash") {
                event.rename(
                    "json.DestinationGeoHash",
                    "cloudflare_logpush.network_analytics.destination.geo_hash",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.destination.geo_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set(
                    "cloudflare_logpush.network_analytics.destination.geo_location",
                    v,
                )?;
            }

            let _cond = { event.get_str("json.GREChecksum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREChecksum") {
                        if let Some(val) = event.get("json.GREChecksum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREChecksum".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.checksum",
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
                        "convert_json_gre_checksum",
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

            let _cond = { event.get_str("json.GREEthertype") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREEthertype") {
                        if let Some(val) = event.get("json.GREEthertype") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREEthertype".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.ether.type",
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
                        "convert_json_gre_ethertype",
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

            let _cond = { event.get_str("json.GREEtherType") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREEtherType") {
                        if let Some(val) = event.get("json.GREEtherType") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREEtherType".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.ether.type",
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
                        "convert_json_gre_etherType",
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

            let _cond = { event.get_str("json.GREHeaderLength") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREHeaderLength") {
                        if let Some(val) = event.get("json.GREHeaderLength") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREHeaderLength".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.header.length",
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
                        "convert_json_gre_header_length",
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

            let _cond = { event.get_str("json.GREKey") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREKey") {
                        if let Some(val) = event.get("json.GREKey") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREKey".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.network_analytics.gre.key", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_key")?;
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

            let _cond = { event.get_str("json.GRESequenceNumber") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GRESequenceNumber") {
                        if let Some(val) = event.get("json.GRESequenceNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GRESequenceNumber".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.sequence.number",
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
                        "convert_json_sequence_number",
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

            let _cond = { event.get_str("json.GREVersion") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.GREVersion") {
                        if let Some(val) = event.get("json.GREVersion") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.GREVersion".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.gre.version",
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
                        "convert_json_gre_version",
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

            let _cond = { event.get_str("json.ICMPChecksum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ICMPChecksum") {
                        if let Some(val) = event.get("json.ICMPChecksum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ICMPChecksum".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.icmp.checksum",
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
                        "convert_json_icmp_checksum",
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

            let _cond = { event.get_str("json.ICMPCode") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ICMPCode") {
                        if let Some(val) = event.get("json.ICMPCode") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ICMPCode".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.icmp.code", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_icmp_code")?;
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

            let _cond = { event.get_str("json.ICMPType") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ICMPType") {
                        if let Some(val) = event.get("json.ICMPType") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ICMPType".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.icmp.type", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_icmp_type")?;
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

            if event.has_value("json.IPDestinationSubnet") {
                event.rename(
                    "json.IPDestinationSubnet",
                    "cloudflare_logpush.network_analytics.ip.destination.subnet",
                )?;
            }

            let _cond = { event.get_str("json.IPFragmentOffset") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPFragmentOffset") {
                        if let Some(val) = event.get("json.IPFragmentOffset") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPFragmentOffset".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.fragment.offset",
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
                        "convert_json_ip_fragment_offset",
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

            let _cond = { event.get_str("json.IPHeaderLength") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPHeaderLength") {
                        if let Some(val) = event.get("json.IPHeaderLength") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPHeaderLength".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.header.length",
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
                        "convert_json_ip_header_length",
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

            let _cond = { event.get_str("json.IPMoreFragments") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPMoreFragments") {
                        if let Some(val) = event.get("json.IPMoreFragments") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPMoreFragments".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.more.fragments",
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
                        "convert_json_ip_more_fragments",
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

            let _cond = { event.get_str("json.IPProtocol") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPProtocol") {
                        if let Some(val) = event.get("json.IPProtocol") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPProtocol".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.protocol.value",
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
                        "convert_json_ip_protocol",
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

            if event.has_value("json.IPSourceSubnet") {
                event.rename(
                    "json.IPSourceSubnet",
                    "cloudflare_logpush.network_analytics.ip.source.subnet",
                )?;
            }

            let _cond = { event.get_str("json.IPTotalLength") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPTotalLength") {
                        if let Some(val) = event.get("json.IPTotalLength") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPTotalLength".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.total.length.value",
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
                        "convert_json_ip_total_length",
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

            let _cond = { event.get_str("json.IPTotalLengthBuckets") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPTotalLengthBuckets") {
                        if let Some(val) = event.get("json.IPTotalLengthBuckets") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPTotalLengthBuckets".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.total.length.buckets",
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
                        "convert_json_ip_total_length_buckets",
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
                { event.has_value("json.IPTTL") && event.get_str("json.IPTTL") != Some("") };
            if _cond {
                if event.has_value("json.IPTTL") {
                    event.rename(
                        "json.IPTTL",
                        "cloudflare_logpush.network_analytics.ip.ttl.value",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ip.ttl.value")
                    && event.has_value("json.IPTtl")
                    && event.get_str("json.IPTtl") != Some("")
                    && !condition_eq(
                        event.get("json.IPTtl"),
                        event.get("cloudflare_logpush.network_analytics.ip.ttl.value"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ip.ttl.value",
                    json!(
                        event
                            .get("json.IPTtl")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ip.ttl.value")
                    && event.has_value("json.IPTtl")
                    && event.get_str("json.IPTtl") != Some("")
            };
            if _cond {
                if event.has_value("json.IPTtl") {
                    event.rename(
                        "json.IPTtl",
                        "cloudflare_logpush.network_analytics.ip.ttl.value",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ip.ttl.value")
                    && event.get_str("cloudflare_logpush.network_analytics.ip.ttl.value")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ip.ttl.value") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ip.ttl.value")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ip.ttl.value"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.ttl.value",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ip_ttl_value")?;
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
                event.has_value("json.IPTTLBuckets")
                    && event.get_str("json.IPTTLBuckets") != Some("")
            };
            if _cond {
                if event.has_value("json.IPTTLBuckets") {
                    event.rename(
                        "json.IPTTLBuckets",
                        "cloudflare_logpush.network_analytics.ip.ttl.buckets",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ip.ttl.buckets")
                    && event.has_value("json.IPTtlBuckets")
                    && event.get_str("json.IPTtlBuckets") != Some("")
                    && !condition_eq(
                        event.get("json.IPTtlBuckets"),
                        event.get("cloudflare_logpush.network_analytics.ip.ttl.buckets"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ip.ttl.buckets",
                    json!(
                        event
                            .get("json.IPTtlBuckets")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ip.ttl.buckets")
                    && event.has_value("json.IPTtlBuckets")
                    && event.get_str("json.IPTtlBuckets") != Some("")
            };
            if _cond {
                if event.has_value("json.IPTtlBuckets") {
                    event.rename(
                        "json.IPTtlBuckets",
                        "cloudflare_logpush.network_analytics.ip.ttl.buckets",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ip.ttl.buckets")
                    && event.get_str("cloudflare_logpush.network_analytics.ip.ttl.buckets")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ip.ttl.buckets") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ip.ttl.buckets")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ip.ttl.buckets"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ip.ttl.buckets",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ip_ttl_buckets")?;
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

            let _cond = { event.get_str("json.IPv4Checksum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv4Checksum") {
                        if let Some(val) = event.get("json.IPv4Checksum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv4Checksum".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv4.checksum",
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
                        "convert_json_ipv4_checksum",
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

            let _cond = { event.get_str("json.IPv4DontFragment") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv4DontFragment") {
                        if let Some(val) = event.get("json.IPv4DontFragment") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv4DontFragment".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv4.dont_fragment",
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
                        "convert_json_ipv4_dont_fragment",
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
                { event.has_value("json.IPv4DSCP") && event.get_str("json.IPv4DSCP") != Some("") };
            if _cond {
                if event.has_value("json.IPv4DSCP") {
                    event.rename(
                        "json.IPv4DSCP",
                        "cloudflare_logpush.network_analytics.ipv4.dscp",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv4.dscp")
                    && event.has_value("json.IPv4Dscp")
                    && event.get_str("json.IPv4Dscp") != Some("")
                    && !condition_eq(
                        event.get("json.IPv4Dscp"),
                        event.get("cloudflare_logpush.network_analytics.ipv4.dscp"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ipv4.dscp",
                    json!(
                        event
                            .get("json.IPv4Dscp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ipv4.dscp")
                    && event.has_value("json.IPv4Dscp")
                    && event.get_str("json.IPv4Dscp") != Some("")
            };
            if _cond {
                if event.has_value("json.IPv4Dscp") {
                    event.rename(
                        "json.IPv4Dscp",
                        "cloudflare_logpush.network_analytics.ipv4.dscp",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv4.dscp")
                    && event.get_str("cloudflare_logpush.network_analytics.ipv4.dscp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ipv4.dscp") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ipv4.dscp")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ipv4.dscp".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.ipv4.dscp", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv4_dscp")?;
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
                { event.has_value("json.IPv4ECN") && event.get_str("json.IPv4ECN") != Some("") };
            if _cond {
                if event.has_value("json.IPv4ECN") {
                    event.rename(
                        "json.IPv4ECN",
                        "cloudflare_logpush.network_analytics.ipv4.ecn",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv4.ecn")
                    && event.has_value("json.IPv4Ecn")
                    && event.get_str("json.IPv4Ecn") != Some("")
                    && !condition_eq(
                        event.get("json.IPv4Ecn"),
                        event.get("cloudflare_logpush.network_analytics.ipv4.ecn"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ipv4.ecn",
                    json!(
                        event
                            .get("json.IPv4Ecn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ipv4.ecn")
                    && event.has_value("json.IPv4Ecn")
                    && event.get_str("json.IPv4Ecn") != Some("")
            };
            if _cond {
                if event.has_value("json.IPv4Ecn") {
                    event.rename(
                        "json.IPv4Ecn",
                        "cloudflare_logpush.network_analytics.ipv4.ecn",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv4.ecn")
                    && event.get_str("cloudflare_logpush.network_analytics.ipv4.ecn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ipv4.ecn") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ipv4.ecn")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ipv4.ecn".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.ipv4.ecn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv4_ecn")?;
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

            let _cond = { event.get_str("json.IPv4Identification") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv4Identification") {
                        if let Some(val) = event.get("json.IPv4Identification") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv4Identification".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv4.identification",
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
                        "convert_json_ipv4_identification",
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

            let _cond = {
                event.get("json.IPv4Options").is_some_and(|v| v.is_string())
                    && event.get_str("json.IPv4Options") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.IPv4Options") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("json.IPv4Options", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("json.IPv4Options") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv4Options") {
                        if let Some(val) = event.get("json.IPv4Options") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv4Options".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv4.options",
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
                        "convert_json_ipv4_options",
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
                { event.has_value("json.IPv6DSCP") && event.get_str("json.IPv6DSCP") != Some("") };
            if _cond {
                if event.has_value("json.IPv6DSCP") {
                    event.rename(
                        "json.IPv6DSCP",
                        "cloudflare_logpush.network_analytics.ipv6.dscp",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv6.dscp")
                    && event.has_value("json.IPv6Dscp")
                    && event.get_str("json.IPv6Dscp") != Some("")
                    && !condition_eq(
                        event.get("json.IPv6Dscp"),
                        event.get("cloudflare_logpush.network_analytics.ipv6.dscp"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ipv6.dscp",
                    json!(
                        event
                            .get("json.IPv6Dscp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ipv6.dscp")
                    && event.has_value("json.IPv6Dscp")
                    && event.get_str("json.IPv6Dscp") != Some("")
            };
            if _cond {
                if event.has_value("json.IPv6Dscp") {
                    event.rename(
                        "json.IPv6Dscp",
                        "cloudflare_logpush.network_analytics.ipv6.dscp",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv6.dscp")
                    && event.get_str("cloudflare_logpush.network_analytics.ipv6.dscp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ipv6.dscp") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ipv6.dscp")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ipv6.dscp".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.ipv6.dscp", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv6_dscp")?;
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
                { event.has_value("json.IPv6ECN") && event.get_str("json.IPv6ECN") != Some("") };
            if _cond {
                if event.has_value("json.IPv6ECN") {
                    event.rename(
                        "json.IPv6ECN",
                        "cloudflare_logpush.network_analytics.ipv6.ecn",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv6.ecn")
                    && event.has_value("json.IPv6Ecn")
                    && event.get_str("json.IPv6Ecn") != Some("")
                    && !condition_eq(
                        event.get("json.IPv6Ecn"),
                        event.get("cloudflare_logpush.network_analytics.ipv6.ecn"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.ipv6.ecn",
                    json!(
                        event
                            .get("json.IPv6Ecn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.ipv6.ecn")
                    && event.has_value("json.IPv6Ecn")
                    && event.get_str("json.IPv6Ecn") != Some("")
            };
            if _cond {
                if event.has_value("json.IPv6Ecn") {
                    event.rename(
                        "json.IPv6Ecn",
                        "cloudflare_logpush.network_analytics.ipv6.ecn",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.ipv6.ecn")
                    && event.get_str("cloudflare_logpush.network_analytics.ipv6.ecn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.ipv6.ecn") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.ipv6.ecn")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.ipv6.ecn".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.network_analytics.ipv6.ecn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv6_ecn")?;
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
                    .get("json.IPv6ExtensionHeaders")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("json.IPv6ExtensionHeaders") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.IPv6ExtensionHeaders") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("json.IPv6ExtensionHeaders", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("json.IPv6ExtensionHeaders") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv6ExtensionHeaders") {
                        if let Some(val) = event.get("json.IPv6ExtensionHeaders") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv6ExtensionHeaders".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv6.extension_headers",
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
                        "convert_json_ipv6_extension_headers",
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

            let _cond = { event.get_str("json.IPv6FlowLabel") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv6FlowLabel") {
                        if let Some(val) = event.get("json.IPv6FlowLabel") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv6FlowLabel".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv6.flow_label",
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
                        "convert_json_ipv6_flow_label",
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

            let _cond = { event.get_str("json.IPv6Identification") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPv6Identification") {
                        if let Some(val) = event.get("json.IPv6Identification") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPv6Identification".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.ipv6.identification",
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
                        "convert_json_ipv6_identification",
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

            if event.has_value("json.MitigationReason") {
                event.rename(
                    "json.MitigationReason",
                    "cloudflare_logpush.network_analytics.mitigation.reason",
                )?;
            }

            if event.has_value("json.MitigationScope") {
                event.rename(
                    "json.MitigationScope",
                    "cloudflare_logpush.network_analytics.mitigation.scope",
                )?;
            }

            if event.has_value("json.MitigationSystem") {
                event.rename(
                    "json.MitigationSystem",
                    "cloudflare_logpush.network_analytics.mitigation.system",
                )?;
            }

            if event.has_value("json.ProtocolState") {
                event.rename(
                    "json.ProtocolState",
                    "cloudflare_logpush.network_analytics.protocol_state",
                )?;
            }

            if event.has_value("json.RuleName") {
                event.rename(
                    "json.RuleName",
                    "cloudflare_logpush.network_analytics.rule.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.RulesetID") {
                event.rename(
                    "json.RulesetID",
                    "cloudflare_logpush.network_analytics.rule.set.id",
                )?;
            }

            if event.has_value("json.RulesetOverrideID") {
                event.rename(
                    "json.RulesetOverrideID",
                    "cloudflare_logpush.network_analytics.rule.set.override.id",
                )?;
            }

            let _cond = { event.get_str("json.SampleInterval") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SampleInterval") {
                        if let Some(val) = event.get("json.SampleInterval") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SampleInterval".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.sample_interval",
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
                        "convert_json_sample_interval",
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

            if event.has_value("json.SourceASNName") {
                event.rename(
                    "json.SourceASNName",
                    "cloudflare_logpush.network_analytics.source.as.number.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.source.as.number.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.organization.name", v)?;
            }

            if event.has_value("json.SourceASNDescription") {
                event.rename(
                    "json.SourceASNDescription",
                    "cloudflare_logpush.network_analytics.source.as.number.description",
                )?;
            }

            if event.has_value("json.SourceCountry") {
                event.rename(
                    "json.SourceCountry",
                    "cloudflare_logpush.network_analytics.source.country",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.source.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            if event.has_value("json.SourceGeoHash") {
                event.rename(
                    "json.SourceGeoHash",
                    "cloudflare_logpush.network_analytics.source.geo_hash",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.source.geo_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set(
                    "cloudflare_logpush.network_analytics.source.geo_location",
                    v,
                )?;
            }

            let _cond = { event.get_str("json.TCPAcknowledgementNumber") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPAcknowledgementNumber") {
                        if let Some(val) = event.get("json.TCPAcknowledgementNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPAcknowledgementNumber".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.acknowledgement_number",
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
                        "convert_json_tcp_acknowledgement_number",
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

            let _cond = { event.get_str("json.TCPChecksum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPChecksum") {
                        if let Some(val) = event.get("json.TCPChecksum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPChecksum".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.checksum",
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
                        "convert_json_tcp_checksum",
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

            let _cond = { event.get_str("json.TCPDataOffset") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPDataOffset") {
                        if let Some(val) = event.get("json.TCPDataOffset") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPDataOffset".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.dataoffset",
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
                        "convert_json_tcp_data_offset",
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

            let _cond = { event.get_str("json.TCPFlags") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPFlags") {
                        if let Some(val) = event.get("json.TCPFlags") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPFlags".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.flags.value",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_tcp_flags")?;
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

            if event.has_value("json.TCPFlagsString") {
                event.rename(
                    "json.TCPFlagsString",
                    "cloudflare_logpush.network_analytics.tcp.flags.string",
                )?;
            }

            let _cond =
                { event.has_value("json.TCPMSS") && event.get_str("json.TCPMSS") != Some("") };
            if _cond {
                if event.has_value("json.TCPMSS") {
                    event.rename(
                        "json.TCPMSS",
                        "cloudflare_logpush.network_analytics.tcp.mss",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.mss")
                    && event.has_value("json.TCPMss")
                    && event.get_str("json.TCPMss") != Some("")
                    && !condition_eq(
                        event.get("json.TCPMss"),
                        event.get("cloudflare_logpush.network_analytics.tcp.mss"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.tcp.mss",
                    json!(
                        event
                            .get("json.TCPMss")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.tcp.mss")
                    && event.has_value("json.TCPMss")
                    && event.get_str("json.TCPMss") != Some("")
            };
            if _cond {
                if event.has_value("json.TCPMss") {
                    event.rename(
                        "json.TCPMss",
                        "cloudflare_logpush.network_analytics.tcp.mss",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.mss")
                    && event.get_str("cloudflare_logpush.network_analytics.tcp.mss") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.tcp.mss") {
                        if let Some(val) = event.get("cloudflare_logpush.network_analytics.tcp.mss")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.tcp.mss".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.network_analytics.tcp.mss", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_mss")?;
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
                event.get("json.TCPOptions").is_some_and(|v| v.is_string())
                    && event.get_str("json.TCPOptions") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.TCPOptions") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("json.TCPOptions", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("json.TCPOptions") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPOptions") {
                        if let Some(val) = event.get("json.TCPOptions") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPOptions".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.options",
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
                        "convert_json_tcp_options",
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

            let _cond = {
                event
                    .get("json.TCPSACKBlocks")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("json.TCPSACKBlocks") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.TCPSACKBlocks") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("json.TCPSACKBlocks", Value::Array(parts))?;
                }
            }

            let _cond = {
                event
                    .get("json.TCPSackBlocks")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("json.TCPSackBlocks") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("json.TCPSackBlocks") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("json.TCPSackBlocks", Value::Array(parts))?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.TCPSACKBlocks != null && ctx.json.TCPSACKBlocks != '' && !(ctx.json.TCPSACKBlocks instanceof List)) {\n  ctx.json.TCPSACKBlocks = [ctx.json.TCPSACKBlocks];\n}\nif (ctx.json?.TCPSackBlocks != null && ctx.json.TCPSackBlocks != '' && !(ctx.json.TCPSackBlocks instanceof List)) {\n  ctx.json.TCPSackBlocks = [ctx.json.TCPSackBlocks];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.TCPSACKBlocks != null && ctx.json.TCPSACKBlocks != '' && !(ctx.json.TCPSACKBlocks instanceof List)) {\n  ctx.json.TCPSACKBlocks = [ctx.json.TCPSACKBlocks];\n}\nif (ctx.json?.TCPSackBlocks != null && ctx.json.TCPSackBlocks != '' && !(ctx.json.TCPSackBlocks instanceof List)) {\n  ctx.json.TCPSackBlocks = [ctx.json.TCPSackBlocks];\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_raise_non_string_tcp_sack_blocks_to_array",
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

            if event.has_value("json.TCPSACKBlocks") {
                event.rename(
                    "json.TCPSACKBlocks",
                    "cloudflare_logpush.network_analytics.tcp.sack.blocks",
                )?;
            }

            let _cond = {
                event
                    .get("json.TCPSackBlocks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.TCPSackBlocks", |event| {
                    event.append_unique(
                        "cloudflare_logpush.network_analytics.tcp.sack.blocks",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.sack.blocks")
                    && event.get_str("cloudflare_logpush.network_analytics.tcp.sack.blocks")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.tcp.sack.blocks") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.tcp.sack.blocks")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.tcp.sack.blocks"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.sack.blocks",
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
                        "convert_tcp_sack_blocks",
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

            let _cond = {
                event.has_value("json.TCPSACKPermitted")
                    && event.get_str("json.TCPSACKPermitted") != Some("")
            };
            if _cond {
                if event.has_value("json.TCPSACKPermitted") {
                    event.rename(
                        "json.TCPSACKPermitted",
                        "cloudflare_logpush.network_analytics.tcp.sack.permitted",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.sack.permitted")
                    && event.has_value("json.TCPSacksPermitted")
                    && event.get_str("json.TCPSacksPermitted") != Some("")
                    && !condition_eq(
                        event.get("json.TCPSacksPermitted"),
                        event.get("cloudflare_logpush.network_analytics.tcp.sack.permitted"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.tcp.sack.permitted",
                    json!(
                        event
                            .get("json.TCPSacksPermitted")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.tcp.sack.permitted")
                    && event.has_value("json.TCPSacksPermitted")
                    && event.get_str("json.TCPSacksPermitted") != Some("")
            };
            if _cond {
                if event.has_value("json.TCPSacksPermitted") {
                    event.rename(
                        "json.TCPSacksPermitted",
                        "cloudflare_logpush.network_analytics.tcp.sack.permitted",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.sack.permitted")
                    && event.get_str("cloudflare_logpush.network_analytics.tcp.sack.permitted")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.tcp.sack.permitted") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.tcp.sack.permitted")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.tcp.sack.permitted"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.sack.permitted",
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
                        "convert_tcp_sack_permitted",
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

            let _cond = { event.get_str("json.TCPSequenceNumber") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPSequenceNumber") {
                        if let Some(val) = event.get("json.TCPSequenceNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPSequenceNumber".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.sequence_number",
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
                        "convert_json_tcp_sequence_number",
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

            let _cond = {
                event.has_value("json.TCPTimestampECR")
                    && event.get_str("json.TCPTimestampECR") != Some("")
            };
            if _cond {
                if event.has_value("json.TCPTimestampECR") {
                    event.rename(
                        "json.TCPTimestampECR",
                        "cloudflare_logpush.network_analytics.tcp.timestamp.ecr",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.timestamp.ecr")
                    && event.has_value("json.TCPTimestampEcr")
                    && event.get_str("json.TCPTimestampEcr") != Some("")
                    && !condition_eq(
                        event.get("json.TCPTimestampEcr"),
                        event.get("cloudflare_logpush.network_analytics.tcp.timestamp.ecr"),
                    )
            };
            if _cond {
                event.append_unique(
                    "cloudflare_logpush.network_analytics.tcp.timestamp.ecr",
                    json!(
                        event
                            .get("json.TCPTimestampEcr")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("cloudflare_logpush.network_analytics.tcp.timestamp.ecr")
                    && event.has_value("json.TCPTimestampEcr")
                    && event.get_str("json.TCPTimestampEcr") != Some("")
            };
            if _cond {
                if event.has_value("json.TCPTimestampEcr") {
                    event.rename(
                        "json.TCPTimestampEcr",
                        "cloudflare_logpush.network_analytics.tcp.timestamp.ecr",
                    )?;
                }
            }

            let _cond = {
                event.has_value("cloudflare_logpush.network_analytics.tcp.timestamp.ecr")
                    && event.get_str("cloudflare_logpush.network_analytics.tcp.timestamp.ecr")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.network_analytics.tcp.timestamp.ecr") {
                        if let Some(val) =
                            event.get("cloudflare_logpush.network_analytics.tcp.timestamp.ecr")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cloudflare_logpush.network_analytics.tcp.timestamp.ecr"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.timestamp.ecr",
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
                        "convert_tcp_timestamp_ecr",
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

            let _cond = { event.get_str("json.TCPTimestampValue") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPTimestampValue") {
                        if let Some(val) = event.get("json.TCPTimestampValue") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPTimestampValue".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.timestamp.value",
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
                        "convert_json_tcp_timestamp_value",
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

            let _cond = { event.get_str("json.TCPUrgentPointer") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPUrgentPointer") {
                        if let Some(val) = event.get("json.TCPUrgentPointer") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPUrgentPointer".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.urgent_pointer",
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
                        "convert_json_tcp_urgent_pointer",
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

            let _cond = { event.get_str("json.TCPWindowScale") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPWindowScale") {
                        if let Some(val) = event.get("json.TCPWindowScale") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPWindowScale".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.window.scale",
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
                        "convert_json_tcp_window_scale",
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

            let _cond = { event.get_str("json.TCPWindowSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.TCPWindowSize") {
                        if let Some(val) = event.get("json.TCPWindowSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.TCPWindowSize".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.tcp.window.size",
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
                        "convert_json_tcp_window_size",
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

            let _cond = { event.get_str("json.UDPChecksum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.UDPChecksum") {
                        if let Some(val) = event.get("json.UDPChecksum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.UDPChecksum".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.udp.checksum",
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
                        "convert_json_udp_checksum",
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

            let _cond = { event.get_str("json.UDPPayloadLength") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.UDPPayloadLength") {
                        if let Some(val) = event.get("json.UDPPayloadLength") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.UDPPayloadLength".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.network_analytics.udp.payload_length",
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
                        "convert_json_udp_payload_length",
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

            if event.has_value("json.Verdict") {
                event.rename(
                    "json.Verdict",
                    "cloudflare_logpush.network_analytics.verdict",
                )?;
            }

            if event.has_value("json.DNSQueryName") {
                event.rename(
                    "json.DNSQueryName",
                    "cloudflare_logpush.network_analytics.dns.query.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.dns.query.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            if event.has_value("json.DNSQueryType") {
                event.rename(
                    "json.DNSQueryType",
                    "cloudflare_logpush.network_analytics.dns.query.type",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.network_analytics.dns.query.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if event.has_value("json.PFPCustomTag") {
                if let Some(val) = event.get("json.PFPCustomTag") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.PFPCustomTag".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.network_analytics.pfp_custom_tag",
                        converted,
                    )?;
                }
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.dns.query.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.dns.query.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.source.geo_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.source.geo_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.network_analytics.destination.geo_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.destination.geo_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.colo.geo_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.colo.geo_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.network_analytics.destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.network_analytics.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
                Ok(())
            })();

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
                event.remove("cloudflare_logpush.network_analytics.timestamp");
                event.remove("cloudflare_logpush.network_analytics.outcome");
                event.remove("cloudflare_logpush.network_analytics.destination.asn");
                event.remove("cloudflare_logpush.network_analytics.destination.ip");
                event.remove("cloudflare_logpush.network_analytics.destination.port");
                event.remove("cloudflare_logpush.network_analytics.direction");
                event.remove("cloudflare_logpush.network_analytics.ip.protocol.name");
                event.remove("cloudflare_logpush.network_analytics.source.ip");
                event.remove("cloudflare_logpush.network_analytics.source.asn");
                event.remove("cloudflare_logpush.network_analytics.source.port");
                event.remove("cloudflare_logpush.network_analytics.rule.id");
                event.remove("cloudflare_logpush.network_analytics.dns.query.name");
                event.remove("cloudflare_logpush.network_analytics.dns.query.type");
                event.remove("cloudflare_logpush.network_analytics.attack.id");
                event.remove("cloudflare_logpush.network_analytics.attack.campaign.id");
                event.remove("cloudflare_logpush.network_analytics.destination.as.number.name");
                event.remove("cloudflare_logpush.network_analytics.destination.country");
                event.remove("cloudflare_logpush.network_analytics.rule.name");
                event.remove("cloudflare_logpush.network_analytics.source.as.number.name");
                event.remove("cloudflare_logpush.network_analytics.source.country");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into()],
                    ..DropPolicy::none()
                },
                None,
            );

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
