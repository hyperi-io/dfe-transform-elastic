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

            event.set("event.kind", json!("event"))?;

            if event.has_value("cef.device.event_class_id") {
                event.rename(
                    "cef.device.event_class_id",
                    "trendmicro.deep_security.event_class_id",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.event_class_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.code", v)?;
            }

            let _cond = { event.get_str("event.code") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.code") {
                        if let Some(val) = event.get("event.code") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "event.code".into(),
                                    message,
                                }
                            })?;
                            event.set("trendmicro.deep_security.signature_id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_event_code")?;
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
                .get("cef.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { event.has_value("event.action") };
            if _cond {
                gsub_field(
                    event,
                    "event.action",
                    "event.action",
                    cached_regex!("[^a-zA-Z0-0]"),
                    " ",
                )?;
            }

            if event.has_value("cef.device.product") {
                event.rename(
                    "cef.device.product",
                    "trendmicro.deep_security.device.product",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.device.product")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.product", v)?;
            }

            if event.has_value("cef.device.vendor") {
                event.rename(
                    "cef.device.vendor",
                    "trendmicro.deep_security.device.vendor",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.device.vendor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            if event.has_value("cef.device.version") {
                event.rename(
                    "cef.device.version",
                    "trendmicro.deep_security.device.version",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.device.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.baseEventCount") {
                    if let Some(val) = event.get("cef.extensions.baseEventCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.baseEventCount".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.base_event_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cef_extensions_baseEventCount_to_long",
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
                if event.has_value("cef.extensions.bytesIn") {
                    if let Some(val) = event.get("cef.extensions.bytesIn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.bytesIn".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.bytes_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cef_extensions_bytesIn_to_long",
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
                .get("trendmicro.deep_security.bytes_in")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            let _cond = { event.get_str("cef.extensions.destinationAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.destinationAddress") {
                        if let Some(val) = event.get("cef.extensions.destinationAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.destinationAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("trendmicro.deep_security.destination.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cef_extensions_destinationAddress_to_ip",
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
                .get("trendmicro.deep_security.destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
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
                if event.has_value("cef.extensions.destinationMacAddress") {
                    gsub_field(
                        event,
                        "cef.extensions.destinationMacAddress",
                        "trendmicro.deep_security.destination.mac_address",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_destinationMacAddress",
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
                event.get_str("trendmicro.deep_security.extensions.destination.mac_address")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("trendmicro.deep_security.destination.mac_address") {
                        map_strings(
                            event,
                            "trendmicro.deep_security.destination.mac_address",
                            "trendmicro.deep_security.destination.mac_address",
                            str::to_uppercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "uppercase_destination_mac_address",
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
                .get("trendmicro.deep_security.destination.mac_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.mac", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.destinationPort") {
                    if let Some(val) = event.get("cef.extensions.destinationPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.destinationPort".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cef_extensions_destinationPort_to_long",
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
                .get("trendmicro.deep_security.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("cef.extensions.deviceAction") {
                event.rename(
                    "cef.extensions.deviceAction",
                    "trendmicro.deep_security.action",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.action")
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
                            let parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
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

            let _cond =
                { event.has_value("event.action") && event.get_str("event.action") != Some("") };
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

            if event.has_value("cef.extensions.deviceCustomNumber1Label") {
                event.rename(
                    "cef.extensions.deviceCustomNumber1Label",
                    "trendmicro.deep_security.device.custom_number1.label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomNumber1") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber1") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber1".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.device.custom_number1.value",
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
                    "convert_cef_extensions_deviceCustomNumber1_to_string",
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
                .get("trendmicro.deep_security.device.custom_number1.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomNumber2Label") {
                event.rename(
                    "cef.extensions.deviceCustomNumber2Label",
                    "trendmicro.deep_security.device.custom_number2.label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomNumber2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber2") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber2".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.device.custom_number2.value",
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
                    "convert_cef_extensions_deviceCustomNumber2_to_long",
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

            if event.has_value("cef.extensions.deviceCustomNumber3Label") {
                event.rename(
                    "cef.extensions.deviceCustomNumber3Label",
                    "trendmicro.deep_security.device.custom_number3.label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomNumber3") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomNumber3") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomNumber3".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.device.custom_number3.value",
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
                    "convert_cef_extensions_deviceCustomNumber3_to_long",
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

            if event.has_value("cef.extensions.deviceCustomString1Label") {
                event.rename(
                    "cef.extensions.deviceCustomString1Label",
                    "trendmicro.deep_security.device.custom_string1.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString1") {
                event.rename(
                    "cef.extensions.deviceCustomString1",
                    "trendmicro.deep_security.device.custom_string1.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString2Label") {
                event.rename(
                    "cef.extensions.deviceCustomString2Label",
                    "trendmicro.deep_security.device.custom_string2.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString2") {
                event.rename(
                    "cef.extensions.deviceCustomString2",
                    "trendmicro.deep_security.device.custom_string2.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString3Label") {
                event.rename(
                    "cef.extensions.deviceCustomString3Label",
                    "trendmicro.deep_security.device.custom_string3.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString3") {
                event.rename(
                    "cef.extensions.deviceCustomString3",
                    "trendmicro.deep_security.device.custom_string3.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4Label") {
                event.rename(
                    "cef.extensions.deviceCustomString4Label",
                    "trendmicro.deep_security.device.custom_string4.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4") {
                event.rename(
                    "cef.extensions.deviceCustomString4",
                    "trendmicro.deep_security.device.custom_string4.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString5Label") {
                event.rename(
                    "cef.extensions.deviceCustomString5Label",
                    "trendmicro.deep_security.device.custom_string5.label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomString5") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString5") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString5".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.device.custom_string5.value",
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
                    "convert_cef_extensions_deviceCustomString5_to_string",
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

            if event.has_value("cef.extensions.deviceCustomString6Label") {
                event.rename(
                    "cef.extensions.deviceCustomString6Label",
                    "trendmicro.deep_security.device.custom_string6.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString6") {
                event.rename(
                    "cef.extensions.deviceCustomString6",
                    "trendmicro.deep_security.device.custom_string6.value",
                )?;
            }

            if event.has_value("cef.extensions.cs7Label") {
                event.rename(
                    "cef.extensions.cs7Label",
                    "trendmicro.deep_security.device.custom_string7.label",
                )?;
            }

            if event.has_value("cef.extensions.cs7") {
                event.rename(
                    "cef.extensions.cs7",
                    "trendmicro.deep_security.device.custom_string7.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceHostName") {
                event.rename(
                    "cef.extensions.deviceHostName",
                    "trendmicro.deep_security.deviceHostName",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.deviceHostName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            if event.has_value("cef.extensions.filePath") {
                event.rename(
                    "cef.extensions.filePath",
                    "trendmicro.deep_security.file_path",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if event.has_value("cef.extensions.message") {
                event.rename("cef.extensions.message", "trendmicro.deep_security.message")?;
            }

            let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.sourceAddress") {
                        if let Some(val) = event.get("cef.extensions.sourceAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.sourceAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("trendmicro.deep_security.source.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cef_extensions_sourceAddress_to_ip",
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
                .get("trendmicro.deep_security.source.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.sourceMacAddress") {
                    gsub_field(
                        event,
                        "cef.extensions.sourceMacAddress",
                        "trendmicro.deep_security.source.mac_address",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_sourceMacAddress")?;
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
                event.get_str("trendmicro.deep_security.extensions.source.mac_address") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("trendmicro.deep_security.source.mac_address") {
                        map_strings(
                            event,
                            "trendmicro.deep_security.source.mac_address",
                            "trendmicro.deep_security.source.mac_address",
                            str::to_uppercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "uppercase_source_mac_address",
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
                .get("trendmicro.deep_security.source.mac_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.mac", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.sourcePort") {
                    if let Some(val) = event.get("cef.extensions.sourcePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.sourcePort".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cef_extensions_sourcePort_to_long",
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
                .get("trendmicro.deep_security.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("cef.extensions.sourceProcessName") {
                event.rename(
                    "cef.extensions.sourceProcessName",
                    "trendmicro.deep_security.source.process_name",
                )?;
            }

            if event.has_value("cef.extensions.sourceUserName") {
                event.rename(
                    "cef.extensions.sourceUserName",
                    "trendmicro.deep_security.source.user_name",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.source.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
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

            if event.has_value("cef.extensions.transportProtocol") {
                event.rename(
                    "cef.extensions.transportProtocol",
                    "trendmicro.deep_security.transport_protocol",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.transport_protocol")
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

            if event.has_value("cef.extensions.TrendMicroDsFrameType") {
                event.rename(
                    "cef.extensions.TrendMicroDsFrameType",
                    "trendmicro.deep_security.trendmicro.ds_frame_type",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsPacketData") {
                event.rename(
                    "cef.extensions.TrendMicroDsPacketData",
                    "trendmicro.deep_security.trendmicro.ds_packet_data",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsTenant") {
                event.rename(
                    "cef.extensions.TrendMicroDsTenant",
                    "trendmicro.deep_security.trendmicro.ds_tenant",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsTenantId") {
                event.rename(
                    "cef.extensions.TrendMicroDsTenantId",
                    "trendmicro.deep_security.trendmicro.ds_tenant_id",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsTags") {
                event.rename(
                    "cef.extensions.TrendMicroDsTags",
                    "trendmicro.deep_security.trendmicro.ds_tags",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsReasonId") {
                event.rename(
                    "cef.extensions.TrendMicroDsReasonId",
                    "trendmicro.deep_security.trendmicro.ds_reason_id",
                )?;
            }

            if event.has_value("cef.name") {
                event.rename("cef.name", "trendmicro.deep_security.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.severity") {
                    if let Some(val) = event.get("cef.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.severity".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cef_severity_to_long",
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
                .get("trendmicro.deep_security.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if event.has_value("cef.version") {
                event.rename("cef.version", "trendmicro.deep_security.version")?;
            }

            if event.has_value("cef.extensions.TrendMicroDsBehaviorRuleID") {
                event.rename(
                    "cef.extensions.TrendMicroDsBehaviorRuleID",
                    "trendmicro.deep_security.trendmicro.ds_behavior.rule_id",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsBehaviorType") {
                event.rename(
                    "cef.extensions.TrendMicroDsBehaviorType",
                    "trendmicro.deep_security.trendmicro.ds_behavior.type",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsCommandLine") {
                event.rename(
                    "cef.extensions.TrendMicroDsCommandLine",
                    "trendmicro.deep_security.trendmicro.ds_command_line",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.trendmicro.ds_command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if event.has_value("cef.extensions.TrendMicroDsCve") {
                event.rename(
                    "cef.extensions.TrendMicroDsCve",
                    "trendmicro.deep_security.trendmicro.ds_cve",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.trendmicro.ds_cve")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            if event.has_value("cef.extensions.filename") {
                event.rename(
                    "cef.extensions.filename",
                    "trendmicro.deep_security.filename",
                )?;
            }

            if event.has_value("cef.extensions.result") {
                event.rename("cef.extensions.result", "trendmicro.deep_security.result")?;
            }

            if event.has_value("cef.extensions.sourceHostName") {
                event.rename(
                    "cef.extensions.sourceHostName",
                    "trendmicro.deep_security.source.host_name",
                )?;
            }

            if event.has_value("cef.extensions.target") {
                event.rename(
                    "cef.extensions.target",
                    "trendmicro.deep_security.target.value",
                )?;
            }

            if event.has_value("cef.extensions.targetID") {
                event.rename(
                    "cef.extensions.targetID",
                    "trendmicro.deep_security.target.id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.TrendMicroDsDetectionConfidence") {
                    if let Some(val) = event.get("cef.extensions.TrendMicroDsDetectionConfidence") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.TrendMicroDsDetectionConfidence".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.trendmicro.ds_detection_confidence",
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
                    "convert_extensions_TrendMicroDsDetectionConfidence_to_long",
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

            if event.has_value("cef.extensions.TrendMicroDsFileMD5") {
                event.rename(
                    "cef.extensions.TrendMicroDsFileMD5",
                    "trendmicro.deep_security.trendmicro.ds_file.md5",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.trendmicro.ds_file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond =
                { event.has_value("trendmicro.deep_security.extensions.trendmicro.ds_file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trendmicro.deep_security.trendmicro.ds_file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsFileSHA1") {
                event.rename(
                    "cef.extensions.TrendMicroDsFileSHA1",
                    "trendmicro.deep_security.trendmicro.ds_file.sha1",
                )?;
            }

            let _cond =
                { event.has_value("trendmicro.deep_security.extensions.trendmicro.ds_file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trendmicro.deep_security.trendmicro.ds_file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.trendmicro.ds_file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if event.has_value("cef.extensions.TrendMicroDsFileSHA256") {
                event.rename(
                    "cef.extensions.TrendMicroDsFileSHA256",
                    "trendmicro.deep_security.trendmicro.ds_file.sha256",
                )?;
            }

            let _cond = {
                event.has_value("trendmicro.deep_security.extensions.trendmicro.ds_file.sha256")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trendmicro.deep_security.trendmicro.ds_file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.trendmicro.ds_file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.TrendMicroDsMalwareTargetCount") {
                    if let Some(val) = event.get("cef.extensions.TrendMicroDsMalwareTargetCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.TrendMicroDsMalwareTargetCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trendmicro.deep_security.trendmicro.ds_malware_target.count",
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
                    "convert_extensions_TrendMicroDsMalwareTargetCount_to_long",
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

            if event.has_value("cef.extensions.TrendMicroDsMalwareTargetType") {
                event.rename(
                    "cef.extensions.TrendMicroDsMalwareTargetType",
                    "trendmicro.deep_security.trendmicro.ds_malware_target.type",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsMalwareTarget") {
                event.rename(
                    "cef.extensions.TrendMicroDsMalwareTarget",
                    "trendmicro.deep_security.trendmicro.ds_malware_target.value",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsMitre") {
                event.rename(
                    "cef.extensions.TrendMicroDsMitre",
                    "trendmicro.deep_security.trendmicro.ds_mitre",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsProcess") {
                event.rename(
                    "cef.extensions.TrendMicroDsProcess",
                    "trendmicro.deep_security.trendmicro.ds_process",
                )?;
            }

            if event.has_value("cef.extensions.TrendMicroDsRelevantDetectionNames") {
                event.rename(
                    "cef.extensions.TrendMicroDsRelevantDetectionNames",
                    "trendmicro.deep_security.trendmicro.ds_relevant_detection_names",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.ip") {
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
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "community_id")?;
                event.set("_ingest.on_failure_processor_tag", "community_id")?;
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
                            .get("_ingest.pipeline")
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

            if event.has_value("cef.extensions.aggregationType") {
                event.rename(
                    "cef.extensions.aggregationType",
                    "trendmicro.deep_security.aggregation_type",
                )?;
            }

            if event.has_value("cef.extensions.computerName") {
                event.rename(
                    "cef.extensions.computerName",
                    "trendmicro.deep_security.computer_name",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.computer_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("cef.extensions.destinationUserName") {
                event.rename(
                    "cef.extensions.destinationUserName",
                    "trendmicro.deep_security.destination.user_name",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.destination.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
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

            if event.has_value("cef.extensions.deviceType") {
                event.rename("cef.extensions.deviceType", "trendmicro.deep_security.type")?;
            }

            if event.has_value("cef.extensions.domainName") {
                event.rename(
                    "cef.extensions.domainName",
                    "trendmicro.deep_security.domain_name",
                )?;
            }

            if event.has_value("cef.extensions.fileHash") {
                event.rename(
                    "cef.extensions.fileHash",
                    "trendmicro.deep_security.file.hash",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.file.hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.fileSize") {
                    if let Some(val) = event.get("cef.extensions.fileSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.fileSize".into(),
                                message,
                            }
                        })?;
                        event.set("trendmicro.deep_security.file.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extensions_fileSize_to_long",
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
                .get("trendmicro.deep_security.file.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if event.has_value("cef.extensions.model") {
                event.rename("cef.extensions.model", "trendmicro.deep_security.model")?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.identifier", v)?;
            }

            if event.has_value("cef.extensions.permission") {
                event.rename(
                    "cef.extensions.permission",
                    "trendmicro.deep_security.permission",
                )?;
            }

            if event.has_value("cef.extensions.processName") {
                event.rename(
                    "cef.extensions.processName",
                    "trendmicro.deep_security.process.name",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if event.has_value("cef.extensions.repeatCount") {
                event.rename(
                    "cef.extensions.repeatCount",
                    "trendmicro.deep_security.repeat_count",
                )?;
            }

            if event.has_value("cef.extensions.requestUrl") {
                event.rename(
                    "cef.extensions.requestUrl",
                    "trendmicro.deep_security.request_url",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.request_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            if event.has_value("cef.extensions.serial") {
                event.rename("cef.extensions.serial", "trendmicro.deep_security.serial")?;
            }

            if event.has_value("cef.extensions.sourceUserId") {
                event.rename(
                    "cef.extensions.sourceUserId",
                    "trendmicro.deep_security.source.user_id",
                )?;
            }

            if let Some(v) = event
                .get("trendmicro.deep_security.source.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
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

            let _cond = { event.get_str("cef.extensions.xff") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.xff") {
                        if let Some(val) = event.get("cef.extensions.xff") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.xff".into(),
                                    message,
                                }
                            })?;
                            event.set("trendmicro.deep_security.xff", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extensions_xff_to_ip",
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

            let _cond = { event.has_value("trendmicro.deep_security.extensions.xff") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("trendmicro.deep_security.xff")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get_i64("trendmicro.deep_security.signature_id")
                    .is_some_and(|n| n >= 6000000)
                    && event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n <= 6999999)
            };
            if _cond {
                // Begin nested pipeline: "application-control-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("application-control-event"),
                )?;
                event.append_unique("event.category", json!("intrusion_detection"))?;
                event.append_unique("event.type", json!("info"))?;
                let _cond = {
                    event.has_value("trendmicro.deep_security.signature_id")
                        && event.get_i64("trendmicro.deep_security.signature_id") != Some(6002100)
                        && event.get_i64("trendmicro.deep_security.signature_id") != Some(6002200)
                };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                let _cond = {
                    event.has_value(
                        "trendmicro.deep_security.extensions.device.custom_string2.value",
                    )
                };
                if _cond {
                    event.append_unique("related.hash", json!(event.get("trendmicro.deep_security.extensions.device.custom_string2.value").map_or_else(String::new, template_to_string)))?;
                }
                let _cond = {
                    event.has_value(
                        "trendmicro.deep_security.extensions.device.custom_string3.value",
                    )
                };
                if _cond {
                    event.append_unique("related.hash", json!(event.get("trendmicro.deep_security.extensions.device.custom_string3.value").map_or_else(String::new, template_to_string)))?;
                }
                // End nested pipeline: "application-control-event"
            }

            let _cond = {
                event.get_i64("trendmicro.deep_security.signature_id") == Some(20)
                    || event.get_i64("trendmicro.deep_security.signature_id") == Some(21)
            };
            if _cond {
                // Begin nested pipeline: "firewall-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("firewall-event"),
                )?;
                event.append_unique("event.category", json!("network"))?;
                event.append_unique("event.type", json!("info"))?;
                // End nested pipeline: "firewall-event"
            }

            let _cond = {
                event.get_i64("trendmicro.deep_security.signature_id") == Some(10)
                    || (event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n >= 1000000)
                        && event
                            .get_i64("trendmicro.deep_security.signature_id")
                            .is_some_and(|n| n <= 1999999))
            };
            if _cond {
                // Begin nested pipeline: "intrusion-prevention-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("intrusion-prevention-event"),
                )?;
                event.append_unique("event.category", json!("intrusion_detection"))?;
                event.append_unique("event.type", json!("info"))?;
                // End nested pipeline: "intrusion-prevention-event"
            }

            let _cond = {
                event.get_i64("trendmicro.deep_security.signature_id") == Some(30)
                    || (event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n >= 2000000)
                        && event
                            .get_i64("trendmicro.deep_security.signature_id")
                            .is_some_and(|n| n <= 2999999))
            };
            if _cond {
                // Begin nested pipeline: "integrity-monitoring-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("integrity-monitoring-log-event"),
                )?;
                event.append_unique("event.category", json!("configuration"))?;
                event.append_unique("event.type", json!("info"))?;
                // End nested pipeline: "integrity-monitoring-event"
            }

            let _cond = {
                event
                    .get_i64("trendmicro.deep_security.signature_id")
                    .is_some_and(|n| n >= 4000000)
                    && event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n <= 4999999)
            };
            if _cond {
                // Begin nested pipeline: "malware-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("anti-malware-event"),
                )?;
                if let Some(v) = event
                    .get("trendmicro.deep_security.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("trendmicro.deep_security.malware.name", v)?;
                }
                let _cond = { event.has_value("trendmicro.deep_security.signature_id") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def signatureIds = [4000000L, 4000001L, 4000002L, 4000003L, 4000010L, 4000011L, 4000012L, 4000013L, 4000020L, 4000030L]; if (signatureIds.contains(ctx.trendmicro.deep_security.signature_id)) {\n  ctx.event.category = [\"malware\"];\n  ctx.event.kind = \"alert\";\n  ctx.event.type = [\"info\"];\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def signatureIds = [4000000L, 4000001L, 4000002L, 4000003L, 4000010L, 4000011L, 4000012L, 4000013L, 4000020L, 4000030L]; if (signatureIds.contains(ctx.trendmicro.deep_security.signature_id)) {\n  ctx.event.category = [\"malware\"];\n  ctx.event.kind = \"alert\";\n  ctx.event.type = [\"info\"];\n}"#
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_to_set_ecs_categorization_in_malware_pipeline",
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
                                    .get("_ingest.pipeline")
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
                // End nested pipeline: "malware-event"
            }

            let _cond = {
                event
                    .get_i64("trendmicro.deep_security.signature_id")
                    .is_some_and(|n| n >= 100)
                    && event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n <= 7499)
            };
            if _cond {
                // Begin nested pipeline: "system-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("system-event"),
                )?;
                let _cond = {
                    event.get_i64("trendmicro.deep_security.signature_id") == Some(397)
                        || event.get_i64("trendmicro.deep_security.signature_id") == Some(160)
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.get_i64("trendmicro.deep_security.signature_id") == Some(397)
                        || event.get_i64("trendmicro.deep_security.signature_id") == Some(160)
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.get_i64("trendmicro.deep_security.signature_id") == Some(397)
                        || event.get_i64("trendmicro.deep_security.signature_id") == Some(160)
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // End nested pipeline: "system-event"
            }

            let _cond = {
                event.get_i64("trendmicro.deep_security.signature_id") == Some(40)
                    || (event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n >= 3000000)
                        && event
                            .get_i64("trendmicro.deep_security.signature_id")
                            .is_some_and(|n| n <= 3999999))
            };
            if _cond {
                // Begin nested pipeline: "log-inspection"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("log-inspection-event"),
                )?;
                // End nested pipeline: "log-inspection"
            }

            let _cond = {
                event
                    .get_i64("trendmicro.deep_security.signature_id")
                    .is_some_and(|n| n >= 5000000)
                    && event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n <= 5999999)
            };
            if _cond {
                // Begin nested pipeline: "web-reputation"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("web-reputation-event"),
                )?;
                event.append_unique("event.category", json!("network"))?;
                event.append_unique("event.type", json!("info"))?;
                let _cond = {
                    event.has_value("trendmicro.deep_security.signature_id")
                        && event.get_i64("trendmicro.deep_security.signature_id") == Some(5000001)
                };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                // End nested pipeline: "web-reputation"
            }

            let _cond = {
                event
                    .get_i64("trendmicro.deep_security.signature_id")
                    .is_some_and(|n| n >= 7000000)
                    && event
                        .get_i64("trendmicro.deep_security.signature_id")
                        .is_some_and(|n| n <= 7999999)
            };
            if _cond {
                // Begin nested pipeline: "device-control-event"
                event.set(
                    "trendmicro.deep_security.event_category",
                    json!("device-control-event"),
                )?;
                event.append_unique("event.category", json!("intrusion_detection"))?;
                event.append_unique("event.type", json!("info"))?;
                // End nested pipeline: "device-control-event"
            }

            let _cond = { !event.has_value("cef.extensions.deviceReceiptTime") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?:%{NONNEGINT}%{SPACE})?(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                    // Grok pattern: ^(?:%{NONNEGINT}%{SPACE})?(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                    // Grok pattern: ^(?:%{NONNEGINT}%{SPACE})?(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT} (?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^(?:%{NONNEGINT}%{SPACE})?(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
                            ),
                            cached_grok!(
                                "^(?:%{NONNEGINT}%{SPACE})?(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
                            ),
                            cached_grok!(
                                "^(?:%{NONNEGINT}%{SPACE})?(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT} (?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond =
                { event.has_value("_tmp.timestamp8601") && !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp8601".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp8601")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("_tmp.timestamp") && !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

            let _cond =
                { event.has_value("_tmp.timestamp8601") && event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp8601".into(),
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
                        "date_tmp_timestamp8601_tz",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("_tmp.timestamp") && event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp_tz")?;
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
                                .get("_ingest.pipeline")
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
                event.remove("trendmicro.deep_security.filename");
                event.remove("trendmicro.deep_security.source.host_name");
                event.remove("trendmicro.deep_security.event_class_id");
                event.remove("trendmicro.deep_security.device.product");
                event.remove("trendmicro.deep_security.device.vendor");
                event.remove("trendmicro.deep_security.device.version");
                event.remove("trendmicro.deep_security.bytes_in");
                event.remove("trendmicro.deep_security.destination.address");
                event.remove("trendmicro.deep_security.destination.mac_address");
                event.remove("trendmicro.deep_security.destination.port");
                event.remove("trendmicro.deep_security.device.custom_number1.value");
                event.remove("trendmicro.deep_security.deviceHostName");
                event.remove("trendmicro.deep_security.file_path");
                event.remove("trendmicro.deep_security.message");
                event.remove("trendmicro.deep_security.source.address");
                event.remove("trendmicro.deep_security.source.mac_address");
                event.remove("trendmicro.deep_security.source.port");
                event.remove("trendmicro.deep_security.source.user_name");
                event.remove("trendmicro.deep_security.transport_protocol");
                event.remove("trendmicro.deep_security.name");
                event.remove("trendmicro.deep_security.severity");
                event.remove("trendmicro.deep_security.destination.user_name");
                event.remove("trendmicro.deep_security.file.hash");
                event.remove("trendmicro.deep_security.file.size");
                event.remove("trendmicro.deep_security.process.name");
                event.remove("trendmicro.deep_security.source.user_id");
                event.remove("trendmicro.deep_security.request_url");
                event.remove("trendmicro.deep_security.trendmicro.ds_cve");
                event.remove("trendmicro.deep_security.trendmicro.ds_file.md5");
                event.remove("trendmicro.deep_security.trendmicro.ds_file.sha1");
                event.remove("trendmicro.deep_security.trendmicro.ds_file.sha256");
                event.remove("trendmicro.deep_security.model");
                event.remove("trendmicro.deep_security.computer_name");
                event.remove("trendmicro.deep_security.trendmicro.ds_command_line");
            }

            event.remove("_tmp");
            event.remove("cef");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
