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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            event.set("event.kind", json!("event"))?;

            if event.has_value("json.body") {
                event.rename("json.body", "beyondtrust_pra.access_session.body")?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.body")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.set("observer.product", json!("Privileged Remote Access"))?;

            event.set("observer.type", json!("Proxy"))?;

            event.set("observer.vendor", json!("BeyondTrust"))?;

            if event.has_value("json.destination.display_name") {
                event.rename(
                    "json.destination.display_name",
                    "beyondtrust_pra.access_session.destination.display_name",
                )?;
            }

            if event.has_value("json.destination.gsnumber") {
                event.rename(
                    "json.destination.gsnumber",
                    "beyondtrust_pra.access_session.destination.gsnumber",
                )?;
            }

            if event.has_value("json.destination.hostname") {
                event.rename(
                    "json.destination.hostname",
                    "beyondtrust_pra.access_session.destination.hostname",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.destination.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if event.has_value("json.destination.id") {
                event.rename(
                    "json.destination.id",
                    "beyondtrust_pra.access_session.destination.id",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.destination.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.id", v)?;
            }

            let _cond = { event.get_i64("json.destination.invited") == Some(1) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.destination.invited",
                    json!(true),
                )?;
            }

            let _cond = { event.get_i64("json.destination.invited") == Some(0) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.destination.invited",
                    json!(false),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_pra.access_session.destination.invited") {
                    if let Some(val) =
                        event.get("beyondtrust_pra.access_session.destination.invited")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_pra.access_session.destination.invited".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.destination.invited",
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
                    "convert_destination_invited_to_boolean",
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

            if event.has_value("json.destination.os") {
                event.rename(
                    "json.destination.os",
                    "beyondtrust_pra.access_session.destination.os",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.destination.private_ip") {
                    // Grok pattern: ^%{IP:json.destination.private_ip}:%{POSINT:json.destination.private_port}$
                    if !cached_grok!("^%{IP:json.destination.private_ip}:%{POSINT:json.destination.private_port}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.destination.private_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.destination.private_ip") {
                        if let Some(val) = event.get("json.destination.private_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.destination.private_ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "beyondtrust_pra.access_session.destination.private_ip",
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
                        "convert_destination_private_ip_to_ip",
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
                .get("beyondtrust_pra.access_session.destination.private_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.destination.private_port") {
                    if let Some(val) = event.get("json.destination.private_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.destination.private_port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.destination.private_port",
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
                    "convert_destination_private_port_to_long",
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
                .get("beyondtrust_pra.access_session.destination.private_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.port", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.destination.public_ip") {
                    // Grok pattern: ^%{IP:json.destination.public_ip}:%{POSINT:json.destination.public_port}$
                    if !cached_grok!(
                        "^%{IP:json.destination.public_ip}:%{POSINT:json.destination.public_port}$"
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.destination.public_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.destination.public_ip") {
                        if let Some(val) = event.get("json.destination.public_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.destination.public_ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "beyondtrust_pra.access_session.destination.public_ip",
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
                        "convert_destination_public_ip_to_ip",
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
                .get("beyondtrust_pra.access_session.destination.public_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.destination.public_port") {
                    if let Some(val) = event.get("json.destination.public_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.destination.public_port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.destination.public_port",
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
                    "convert_destination_public_port_to_long",
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
                .get("beyondtrust_pra.access_session.destination.public_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
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
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.destination.seconds_involved") {
                    if let Some(val) = event.get("json.destination.seconds_involved") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.destination.seconds_involved".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.destination.seconds_involved",
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
                    "convert_destination_seconds_involved_to_long",
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

            let _cond = { event.get_i64("json.destination.session_owner") == Some(1) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.destination.session_owner",
                    json!(true),
                )?;
            }

            let _cond = { event.get_i64("json.destination.session_owner") == Some(0) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.destination.session_owner",
                    json!(false),
                )?;
            }

            if event.has_value("json.destination.type") {
                event.rename(
                    "json.destination.type",
                    "beyondtrust_pra.access_session.destination.type",
                )?;
            }

            if event.has_value("json.destination.username") {
                event.rename(
                    "json.destination.username",
                    "beyondtrust_pra.access_session.destination.username",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.destination.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            if event.has_value("json.encoded_body") {
                event.rename(
                    "json.encoded_body",
                    "beyondtrust_pra.access_session.encoded_body",
                )?;
            }

            if event.has_value("json.event_type") {
                event.rename(
                    "json.event_type",
                    "beyondtrust_pra.access_session.event_type",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.event_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.get("event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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

            let _cond = { event.has_value("event.action") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (params.get(ctx.event.action) == null) {\n  ctx.event.category = [\"session\"]; // As each event belongs to a session\n  ctx.event.type = [\"info\"]; // Remaining are of type info\n  return;\n}\n // As each event belongs to a session\ndef event_category = new ArrayList([\"session\"]); params.get(ctx.event.action).forEach((k, v) -> {\n    if (k.equals(\"category\")) {\n      event_category.addAll(v);\n    }else{\n        ctx.event[k] = v;\n    }  \n}); ctx.event.category = event_category
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (params.get(ctx.event.action) == null) {\n  ctx.event.category = [\"session\"]; // As each event belongs to a session\n  ctx.event.type = [\"info\"]; // Remaining are of type info\n  return;\n}\n // As each event belongs to a session\ndef event_category = new ArrayList([\"session\"]); params.get(ctx.event.action).forEach((k, v) -> {\n    if (k.equals(\"category\")) {\n      event_category.addAll(v);\n    }else{\n        ctx.event[k] = v;\n    }  \n}); ctx.event.category = event_category"#
                        ),
                        cached_params!(
                            "{\"command-shell-session-started\":{\"category\":[\"intrusion_detection\"],\"type\":[\"info\"]},\"conference-member-added\":{\"category\":[\"iam\"],\"type\":[\"user\"]},\"conference-member-departed\":{\"category\":[\"iam\"],\"type\":[\"user\"]},\"conference-member-state-changed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"conference-owner-changed\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"credential-injection-attempt\":{\"category\":[\"authentication\",\"intrusion_detection\"],\"type\":[\"info\"]},\"credential-injection-attempt-failed\":{\"category\":[\"authentication\",\"intrusion_detection\"],\"type\":[\"info\",\"denied\"]},\"directory-created\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"file-deleted\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"file-download\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"file-download-failed\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"file-moved\":{\"category\":[\"file\"],\"type\":[\"change\"]},\"file-upload\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"file-upload-failed\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"registry-exported\":{\"category\":[\"registry\"],\"type\":[\"access\"]},\"registry-imported\":{\"category\":[\"registry\"],\"type\":[\"access\"]},\"registry-key-added\":{\"category\":[\"registry\"],\"type\":[\"creation\"]},\"registry-key-deleted\":{\"category\":[\"registry\"],\"type\":[\"deletion\"]},\"registry-key-renamed\":{\"category\":[\"registry\"],\"type\":[\"change\"]},\"registry-value-added\":{\"category\":[\"registry\"],\"type\":[\"creation\"]},\"registry-value-deleted\":{\"category\":[\"registry\"],\"type\":[\"deletion\"]},\"registry-value-modified\":{\"category\":[\"registry\"],\"type\":[\"change\"]},\"registry-value-renamed\":{\"category\":[\"registry\"],\"type\":[\"change\"]},\"service-access-allowed\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"access\"]},\"session-end\":{\"type\":[\"end\"]},\"session-start\":{\"type\":[\"start\"]},\"system-information-retrieved\":{\"category\":[\"intrusion_detection\",\"configuration\"],\"type\":[\"info\",\"access\"]}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "map_event_category_and_event_type_from_event_action",
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

            if event.has_value("json.filename") {
                event.rename("json.filename", "beyondtrust_pra.access_session.filename")?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filesize") {
                    if let Some(val) = event.get("json.filesize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.filesize".into(),
                                message,
                            }
                        })?;
                        event.set("beyondtrust_pra.access_session.filesize", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_filesize_to_long",
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
                .get("beyondtrust_pra.access_session.filesize")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            let _cond = { event.get("json.files.file").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.files.file", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.filesize") {
                            if let Some(val) = event.get("_ingest._value.filesize") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.filesize".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.filesize", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_files_file_filesize_to_long",
                        )?;
                        event.remove("_ingest._value.filesize");
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
                    Ok(())
                })?;
            }

            if event.has_value("json.files.file") {
                event.rename(
                    "json.files.file",
                    "beyondtrust_pra.access_session.files.file",
                )?;
            }

            if event.has_value("json.performed_by.display_name") {
                event.rename(
                    "json.performed_by.display_name",
                    "beyondtrust_pra.access_session.performed_by.display_name",
                )?;
            }

            if event.has_value("json.performed_by.gsnumber") {
                event.rename(
                    "json.performed_by.gsnumber",
                    "beyondtrust_pra.access_session.performed_by.gsnumber",
                )?;
            }

            if event.has_value("json.performed_by.hostname") {
                event.rename(
                    "json.performed_by.hostname",
                    "beyondtrust_pra.access_session.performed_by.hostname",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.performed_by.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if event.has_value("json.performed_by.id") {
                event.rename(
                    "json.performed_by.id",
                    "beyondtrust_pra.access_session.performed_by.id",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.performed_by.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            let _cond = { event.get_i64("json.performed_by.invited") == Some(1) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.performed_by.invited",
                    json!(true),
                )?;
            }

            let _cond = { event.get_i64("json.performed_by.invited") == Some(0) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.performed_by.invited",
                    json!(false),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_pra.access_session.performed_by.invited") {
                    if let Some(val) =
                        event.get("beyondtrust_pra.access_session.performed_by.invited")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_pra.access_session.performed_by.invited".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.performed_by.invited",
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
                    "convert_performed_by_invited_to_boolean",
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

            if event.has_value("json.performed_by.os") {
                event.rename(
                    "json.performed_by.os",
                    "beyondtrust_pra.access_session.performed_by.os",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.performed_by.private_ip") {
                    // Grok pattern: ^%{IP:json.performed_by.private_ip}:%{POSINT:json.performed_by.private_port}$
                    if !cached_grok!("^%{IP:json.performed_by.private_ip}:%{POSINT:json.performed_by.private_port}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.performed_by.private_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.performed_by.private_ip") {
                        if let Some(val) = event.get("json.performed_by.private_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.performed_by.private_ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "beyondtrust_pra.access_session.performed_by.private_ip",
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
                        "convert_performed_by_private_ip_to_ip",
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
                .get("beyondtrust_pra.access_session.performed_by.private_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.performed_by.private_port") {
                    if let Some(val) = event.get("json.performed_by.private_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.performed_by.private_port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.performed_by.private_port",
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
                    "convert_performed_by_private_port_to_long",
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
                .get("beyondtrust_pra.access_session.performed_by.private_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.port", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.performed_by.public_ip") {
                    // Grok pattern: ^%{IP:json.performed_by.public_ip}:%{POSINT:json.performed_by.public_port}$
                    if !cached_grok!("^%{IP:json.performed_by.public_ip}:%{POSINT:json.performed_by.public_port}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.performed_by.public_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.performed_by.public_ip") {
                        if let Some(val) = event.get("json.performed_by.public_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.performed_by.public_ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "beyondtrust_pra.access_session.performed_by.public_ip",
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
                        "convert_performed_by_public_ip_to_ip",
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
                .get("beyondtrust_pra.access_session.performed_by.public_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.performed_by.public_port") {
                    if let Some(val) = event.get("json.performed_by.public_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.performed_by.public_port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.performed_by.public_port",
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
                    "convert_performed_by_public_port_to_long",
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
                .get("beyondtrust_pra.access_session.performed_by.public_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
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

            if let Some(v) = event
                .get("source.geo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "host.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.performed_by.seconds_involved") {
                    if let Some(val) = event.get("json.performed_by.seconds_involved") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.performed_by.seconds_involved".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.performed_by.seconds_involved",
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
                    "convert_performed_by_seconds_involved_to_long",
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

            let _cond = { event.get_i64("json.performed_by.session_owner") == Some(1) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.performed_by.session_owner",
                    json!(true),
                )?;
            }

            let _cond = { event.get_i64("json.performed_by.session_owner") == Some(0) };
            if _cond {
                event.set(
                    "beyondtrust_pra.access_session.performed_by.session_owner",
                    json!(false),
                )?;
            }

            if event.has_value("json.performed_by.type") {
                event.rename(
                    "json.performed_by.type",
                    "beyondtrust_pra.access_session.performed_by.type",
                )?;
            }

            if event.has_value("json.performed_by.username") {
                event.rename(
                    "json.performed_by.username",
                    "beyondtrust_pra.access_session.performed_by.username",
                )?;
            }

            if let Some(v) = event
                .get("beyondtrust_pra.access_session.performed_by.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            if event.has_value("json.session.command_shell_recordings.command_shell_recording") {
                event.rename("json.session.command_shell_recordings.command_shell_recording", "beyondtrust_pra.access_session.session.command_shell_recordings.command_shell_recording")?;
            }

            let _cond = {
                event
                    .get("json.session.custom_attributes")
                    .is_some_and(|v| v.is_object())
                    && event
                        .get("json.session.custom_attributes.custom_attribute")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.session.custom_attributes.custom_attribute",
                        |event| {
                            if event.has_value("_ingest._value.#text") {
                                event.rename("_ingest._value.#text", "_ingest._value.text")?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.session.custom_attributes")
                    && event
                        .get("json.session.custom_attributes")
                        .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has_value("json.session.custom_attributes") {
                    event.rename(
                        "json.session.custom_attributes",
                        "beyondtrust_pra.access_session.session.custom_attributes",
                    )?;
                }
            }

            if event.has_value("json.session.duration") {
                event.rename(
                    "json.session.duration",
                    "beyondtrust_pra.access_session.session.duration",
                )?;
            }

            if event.has_value("json.session.end_time.#text") {
                event.rename(
                    "json.session.end_time.#text",
                    "beyondtrust_pra.access_session.session.end_time.text",
                )?;
            }

            let _cond = {
                event.has_value("beyondtrust_pra.access_session.session.end_time.text")
                    && event.get_str("beyondtrust_pra.access_session.session.end_time.text")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("beyondtrust_pra.access_session.session.end_time.text")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_pra.access_session.session.end_time.text"
                                        .into(),
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
                        "date_session_end_time_#text",
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
                event.has_value("json.session.end_time.timestamp")
                    && event.get_str("json.session.end_time.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.session.end_time.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "beyondtrust_pra.access_session.session.end_time.timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.session.end_time.timestamp".into(),
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
                        "date_session_end_time_timestamp",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.session.file_move_count") {
                    if let Some(val) = event.get("json.session.file_move_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.session.file_move_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.session.file_move_count",
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
                    "convert_session_file_move_count_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.session.file_transfer_count") {
                    if let Some(val) = event.get("json.session.file_transfer_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.session.file_transfer_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.session.file_transfer_count",
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
                    "convert_session_file_transfer_count_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.session.file_delete_count") {
                    if let Some(val) = event.get("json.session.file_delete_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.session.file_delete_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_pra.access_session.session.file_delete_count",
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
                    "convert_session_file_delete_count_to_long",
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

            if event.has_value("json.session.jump_group.#text") {
                event.rename(
                    "json.session.jump_group.#text",
                    "beyondtrust_pra.access_session.session.jump_group.text",
                )?;
            }

            if event.has_value("json.session.jump_group.id") {
                event.rename(
                    "json.session.jump_group.id",
                    "beyondtrust_pra.access_session.session.jump_group.id",
                )?;
            }

            if event.has_value("json.session.jump_group.type") {
                event.rename(
                    "json.session.jump_group.type",
                    "beyondtrust_pra.access_session.session.jump_group.type",
                )?;
            }

            if event.has_value("json.session.jumpoint.#text") {
                event.rename(
                    "json.session.jumpoint.#text",
                    "beyondtrust_pra.access_session.session.jumpoint.text",
                )?;
            }

            if event.has_value("json.session.jumpoint.id") {
                event.rename(
                    "json.session.jumpoint.id",
                    "beyondtrust_pra.access_session.session.jumpoint.id",
                )?;
            }

            if event.has_value("json.session.lseq") {
                event.rename(
                    "json.session.lseq",
                    "beyondtrust_pra.access_session.session.lseq",
                )?;
            }

            if event.has_value("json.session.lsid") {
                event.rename(
                    "json.session.lsid",
                    "beyondtrust_pra.access_session.session.lsid",
                )?;
            }

            if event.has_value("json.session.primary_customer.#text") {
                event.rename(
                    "json.session.primary_customer.#text",
                    "beyondtrust_pra.access_session.session.primary_customer.text",
                )?;
            }

            if event.has_value("json.session.primary_customer.gsnumber") {
                event.rename(
                    "json.session.primary_customer.gsnumber",
                    "beyondtrust_pra.access_session.session.primary_customer.gsnumber",
                )?;
            }

            if event.has_value("json.session.primary_rep.#text") {
                event.rename(
                    "json.session.primary_rep.#text",
                    "beyondtrust_pra.access_session.session.primary_rep.text",
                )?;
            }

            if event.has_value("json.session.primary_rep.gsnumber") {
                event.rename(
                    "json.session.primary_rep.gsnumber",
                    "beyondtrust_pra.access_session.session.primary_rep.gsnumber",
                )?;
            }

            if event.has_value("json.session.primary_rep.id") {
                event.rename(
                    "json.session.primary_rep.id",
                    "beyondtrust_pra.access_session.session.primary_rep.id",
                )?;
            }

            if event.has_value("json.session.session_chat_download_url") {
                event.rename(
                    "json.session.session_chat_download_url",
                    "beyondtrust_pra.access_session.session.session_chat_download_url",
                )?;
            }

            if event.has_value("json.session.session_chat_view_url") {
                event.rename(
                    "json.session.session_chat_view_url",
                    "beyondtrust_pra.access_session.session.session_chat_view_url",
                )?;
            }

            if event.has_value("json.session.session_recording_download_url") {
                event.rename(
                    "json.session.session_recording_download_url",
                    "beyondtrust_pra.access_session.session.session_recording_download_url",
                )?;
            }

            if event.has_value("json.session.session_recording_view_url") {
                event.rename(
                    "json.session.session_recording_view_url",
                    "beyondtrust_pra.access_session.session.session_recording_view_url",
                )?;
            }

            if event.has_value("json.session.session_type") {
                event.rename(
                    "json.session.session_type",
                    "beyondtrust_pra.access_session.session.session_type",
                )?;
            }

            if event.has_value("json.session.start_time.#text") {
                event.rename(
                    "json.session.start_time.#text",
                    "beyondtrust_pra.access_session.session.start_time.text",
                )?;
            }

            let _cond = {
                event.has_value("beyondtrust_pra.access_session.session.start_time.text")
                    && event.get_str("beyondtrust_pra.access_session.session.start_time.text")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("beyondtrust_pra.access_session.session.start_time.text")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_pra.access_session.session.start_time.text"
                                        .into(),
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
                        "date_session_start_time_#text",
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
                event.has_value("json.session.start_time.timestamp")
                    && event.get_str("json.session.start_time.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.session.start_time.timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "beyondtrust_pra.access_session.session.start_time.timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.session.start_time.timestamp".into(),
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
                        "date_session_start_time_timestamp",
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
                    .get("json.system_information.category")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.system_information.category", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.data.row", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    foreach_array(event, "_ingest._value.field", |event| {
                                        if event.has_value("_ingest._value.#text") {
                                            event.rename(
                                                "_ingest._value.#text",
                                                "_ingest._value.text",
                                            )?;
                                        }
                                        Ok(())
                                    })?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.system_information.category")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.system_information.category", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.description.field", |event| {
                                if event.has_value("_ingest._value.#text") {
                                    event.rename("_ingest._value.#text", "_ingest._value.text")?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.system_information.category") {
                event.rename(
                    "json.system_information.category",
                    "beyondtrust_pra.access_session.system_information.category",
                )?;
            }

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("beyondtrust_pra.access_session.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
                .get("beyondtrust_pra.access_session.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.data.value") {
                event.rename(
                    "json.data.value",
                    "beyondtrust_pra.access_session.data.value",
                )?;
            }

            if event.has_value("json.data.name") {
                event.rename("json.data.name", "beyondtrust_pra.access_session.data.name")?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondtrust_pra.access_session.performed_by.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondtrust_pra.access_session.performed_by.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
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
                event.remove("beyondtrust_pra.access_session.body");
                event.remove("beyondtrust_pra.access_session.destination.hostname");
                event.remove("beyondtrust_pra.access_session.destination.id");
                event.remove("beyondtrust_pra.access_session.destination.private_ip");
                event.remove("beyondtrust_pra.access_session.destination.private_port");
                event.remove("beyondtrust_pra.access_session.destination.public_ip");
                event.remove("beyondtrust_pra.access_session.destination.public_port");
                event.remove("beyondtrust_pra.access_session.destination.username");
                event.remove("beyondtrust_pra.access_session.filename");
                event.remove("beyondtrust_pra.access_session.filesize");
                event.remove("beyondtrust_pra.access_session.performed_by.hostname");
                event.remove("beyondtrust_pra.access_session.performed_by.id");
                event.remove("beyondtrust_pra.access_session.performed_by.private_ip");
                event.remove("beyondtrust_pra.access_session.performed_by.private_port");
                event.remove("beyondtrust_pra.access_session.performed_by.public_ip");
                event.remove("beyondtrust_pra.access_session.performed_by.public_port");
                event.remove("beyondtrust_pra.access_session.performed_by.username");
                event.remove("beyondtrust_pra.access_session.timestamp");
            }

            event.remove("json");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
