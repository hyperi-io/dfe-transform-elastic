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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get_str("cef.name") != Some("Normal") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("cef.name") == Some("Normal") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.end") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("event.start") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("file.inode") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("cef.device.event_class_id") {
                event.rename(
                    "cef.device.event_class_id",
                    "imperva_cloud_waf.event.device.event_class_id",
                )?;
            }

            if event.has_value("cef.device.product") {
                event.rename(
                    "cef.device.product",
                    "imperva_cloud_waf.event.device.product",
                )?;
            }

            if event.has_value("cef.device.vendor") {
                event.rename("cef.device.vendor", "imperva_cloud_waf.event.device.vendor")?;
            }

            if event.has_value("cef.device.version") {
                event.rename(
                    "cef.device.version",
                    "imperva_cloud_waf.event.device.version",
                )?;
            }

            if event.has_value("cef.extensions.deviceAction") {
                event.rename(
                    "cef.extensions.deviceAction",
                    "imperva_cloud_waf.event.extensions.action",
                )?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.action") {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!("_"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
                event.remove("event.action");
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
                event.has_value("cef.extensions.additionalReqHeaders")
                    && event.get_str("cef.extensions.additionalReqHeaders") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "cef.extensions.additionalReqHeaders",
                        "imperva_cloud_waf.event.extensions.additional.req_headers",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_extensions_additionalReqHeaders",
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
                event.has_value("cef.extensions.additionalResHeaders")
                    && event.get_str("cef.extensions.additionalResHeaders") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "cef.extensions.additionalResHeaders",
                        "imperva_cloud_waf.event.extensions.additional.res_headers",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_extensions_additionalResHeaders",
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

            if event.has_value("cef.extensions.applicationProtocol") {
                event.rename(
                    "cef.extensions.applicationProtocol",
                    "imperva_cloud_waf.event.extensions.application_protocol",
                )?;
            }

            if event.has_value("network.application") {
                map_strings(
                    event,
                    "network.application",
                    "network.application",
                    str::to_lowercase,
                )?;
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
                        event.set("imperva_cloud_waf.event.extensions.bytes_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extensions_bytesIn_to_long",
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

            if event.has_value("cef.extensions.ccode") {
                event.rename(
                    "cef.extensions.ccode",
                    "imperva_cloud_waf.event.extensions.ccode",
                )?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.ccode")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            if event.has_value("cef.extensions.cicode") {
                event.rename(
                    "cef.extensions.cicode",
                    "imperva_cloud_waf.event.extensions.cicode",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.cs10")
                    && event.get_str("cef.extensions.cs10") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "cef.extensions.cs10",
                        "imperva_cloud_waf.event.extensions.cs10",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_extensions_cs10")?;
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

            if event.has_value("cef.extensions.cs10Label") {
                event.rename(
                    "cef.extensions.cs10Label",
                    "imperva_cloud_waf.event.extensions.cs10Label",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.cs11")
                    && event.get_str("cef.extensions.cs11") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "cef.extensions.cs11",
                        "imperva_cloud_waf.event.extensions.cs11",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_extensions_cs11")?;
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

            if event.has_value("cef.extensions.cs11Label") {
                event.rename(
                    "cef.extensions.cs11Label",
                    "imperva_cloud_waf.event.extensions.cs11Label",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.cs7")
                    && event.get_str("cef.extensions.cs7") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.cs7") {
                        if let Some(val) = event.get("cef.extensions.cs7") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.cs7".into(),
                                    message,
                                }
                            })?;
                            event.set("imperva_cloud_waf.event.extensions.cs7", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extensions_cs7_to_double",
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

            if event.has_value("cef.extensions.cs7Label") {
                event.rename(
                    "cef.extensions.cs7Label",
                    "imperva_cloud_waf.event.extensions.cs7Label",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.cs8")
                    && event.get_str("cef.extensions.cs8") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.cs8") {
                        if let Some(val) = event.get("cef.extensions.cs8") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.cs8".into(),
                                    message,
                                }
                            })?;
                            event.set("imperva_cloud_waf.event.extensions.cs8", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extensions_cs8_to_double",
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

            if event.has_value("cef.extensions.cs8Label") {
                event.rename(
                    "cef.extensions.cs8Label",
                    "imperva_cloud_waf.event.extensions.cs8Label",
                )?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.cs7")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.location.lat", v)?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.cs8")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.location.lon", v)?;
            }

            if event.has_value("cef.extensions.cs9") {
                event.rename(
                    "cef.extensions.cs9",
                    "imperva_cloud_waf.event.extensions.cs9",
                )?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.cs9")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("cef.extensions.cs9Label") {
                event.rename(
                    "cef.extensions.cs9Label",
                    "imperva_cloud_waf.event.extensions.cs9Label",
                )?;
            }

            if event.has_value("cef.extensions.Customer") {
                event.rename(
                    "cef.extensions.Customer",
                    "imperva_cloud_waf.event.extensions.customer",
                )?;
            }

            if event.has_value("cef.extensions.destinationProcessName") {
                event.rename(
                    "cef.extensions.destinationProcessName",
                    "imperva_cloud_waf.event.extensions.destination_process_name",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomNumber1") {
                event.rename(
                    "cef.extensions.deviceCustomNumber1",
                    "imperva_cloud_waf.event.extensions.device.custom_number1",
                )?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.device.custom_number1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if event.has_value("cef.extensions.deviceCustomString1") {
                event.rename(
                    "cef.extensions.deviceCustomString1",
                    "imperva_cloud_waf.event.extensions.device.custom_string1",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString1Label") {
                event.rename(
                    "cef.extensions.deviceCustomString1Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string1_label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString2") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString2".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "imperva_cloud_waf.event.extensions.device.custom_string2",
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
                    "convert_extensions_deviceCustomString2_to_boolean",
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

            if event.has_value("cef.extensions.deviceCustomString2Label") {
                event.rename(
                    "cef.extensions.deviceCustomString2Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string2_label",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.deviceCustomString3") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString3") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString3".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "imperva_cloud_waf.event.extensions.device.custom_string3",
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
                    "convert_extensions_deviceCustomString3_to_boolean",
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

            if event.has_value("cef.extensions.deviceCustomString3Label") {
                event.rename(
                    "cef.extensions.deviceCustomString3Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string3_label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4") {
                event.rename(
                    "cef.extensions.deviceCustomString4",
                    "imperva_cloud_waf.event.extensions.device.custom_string4",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4Label") {
                event.rename(
                    "cef.extensions.deviceCustomString4Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string4_label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString5") {
                event.rename(
                    "cef.extensions.deviceCustomString5",
                    "imperva_cloud_waf.event.extensions.device.custom_string5",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString5Label") {
                event.rename(
                    "cef.extensions.deviceCustomString5Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string5_label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString6") {
                event.rename(
                    "cef.extensions.deviceCustomString6",
                    "imperva_cloud_waf.event.extensions.device.custom_string6",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString6Label") {
                event.rename(
                    "cef.extensions.deviceCustomString6Label",
                    "imperva_cloud_waf.event.extensions.device.custom_string6_label",
                )?;
            }

            if event.has_value("cef.extensions.deviceExternalId") {
                event.rename(
                    "cef.extensions.deviceExternalId",
                    "imperva_cloud_waf.event.extensions.device.externalId",
                )?;
            }

            if event.has_value("cef.extensions.deviceFacility") {
                event.rename(
                    "cef.extensions.deviceFacility",
                    "imperva_cloud_waf.event.extensions.device.facility",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.endTime")
                    && event.get_str("cef.extensions.endTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.endTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("imperva_cloud_waf.event.extensions.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.endTime".into(),
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
                        "date_extensions_endTime",
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

            if event.has_value("cef.extensions.filePermission") {
                event.rename(
                    "cef.extensions.filePermission",
                    "imperva_cloud_waf.event.extensions.file.permission",
                )?;
            }

            if event.has_value("cef.extensions.fileType") {
                event.rename(
                    "cef.extensions.fileType",
                    "imperva_cloud_waf.event.extensions.file.type",
                )?;
            }

            if event.has_value("cef.extensions.fileId") {
                event.rename(
                    "cef.extensions.fileId",
                    "imperva_cloud_waf.event.extensions.file_id",
                )?;
            }

            if event.has_value("cef.extensions.postbody") {
                event.rename(
                    "cef.extensions.postbody",
                    "imperva_cloud_waf.event.extensions.postbody",
                )?;
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.postbody")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.body.content", v)?;
            }

            if event.has_value("cef.extensions.qstr") {
                event.rename(
                    "cef.extensions.qstr",
                    "imperva_cloud_waf.event.extensions.qstr",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("url.original") {
                    gsub_field(
                        event,
                        "url.original",
                        "url.original",
                        cached_regex!(" "),
                        "%20",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_url_original")?;
                event.remove("url.original");
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
                event.has_value("url.original")
                    && event.get_str("url.original") != Some("")
                    && event.has_value("network.application")
                    && event.get_str("network.application") != Some("")
            };
            if _cond {
                event.set(
                    "url.original",
                    json!(format!(
                        "{}://{}",
                        event
                            .get("network.application")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.original")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("url.original") {
                        if !uri_parts(event, "url.original", "url", true, false)?
                            && event
                                .get_str("url.original")
                                .is_some_and(|value| !value.is_empty())
                        {
                            return Err(TransformError::ParseError {
                                path: "url.original".into(),
                                message: "uri_parts: not a parseable URI".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
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
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.qstr")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.query", v)?;
            }

            if event.has_value("cef.extensions.ref") {
                event.rename(
                    "cef.extensions.ref",
                    "imperva_cloud_waf.event.extensions.ref",
                )?;
            }

            if event.has_value("cef.extensions.requestClientApplication") {
                event.rename(
                    "cef.extensions.requestClientApplication",
                    "imperva_cloud_waf.event.extensions.request.client_application",
                )?;
            }

            if event.has_value("cef.extensions.requestMethod") {
                event.rename(
                    "cef.extensions.requestMethod",
                    "imperva_cloud_waf.event.extensions.request.method",
                )?;
            }

            if event.has_value("cef.extensions.requestUrl") {
                event.rename(
                    "cef.extensions.requestUrl",
                    "imperva_cloud_waf.event.extensions.request.url",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.sip")
                    && event.get_str("cef.extensions.sip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.sip") {
                        if let Some(val) = event.get("cef.extensions.sip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cef.extensions.sip".into(),
                                    message,
                                }
                            })?;
                            event.set("imperva_cloud_waf.event.extensions.sip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_extensions_sip_to_ip",
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
                .get("imperva_cloud_waf.event.extensions.sip")
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

            if event.has_value("cef.extensions.siteid") {
                event.rename(
                    "cef.extensions.siteid",
                    "imperva_cloud_waf.event.extensions.site.id",
                )?;
            }

            if event.has_value("cef.extensions.siteTag") {
                event.rename(
                    "cef.extensions.siteTag",
                    "imperva_cloud_waf.event.extensions.site.tag",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.sourceAddress")
                    && event.get_str("cef.extensions.sourceAddress") != Some("")
            };
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
                            event.set(
                                "imperva_cloud_waf.event.extensions.source.address",
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
                        "convert_extensions_sourceAddress_to_ip",
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

            let _cond = { event.has_value("imperva_cloud_waf.event.extensions.source.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("imperva_cloud_waf.event.extensions.source.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                        event.set("imperva_cloud_waf.event.extensions.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extensions_sourcePort_to_long",
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

            let _cond = { event.has_value("imperva_cloud_waf.event.extensions.source.port") };
            if _cond {
                event.remove("source.port");
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cef.extensions.cpt") {
                    if let Some(val) = event.get("cef.extensions.cpt") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.cpt".into(),
                                message,
                            }
                        })?;
                        event.set("imperva_cloud_waf.event.extensions.cpt", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_extensions_cpt_to_long",
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
                .get("imperva_cloud_waf.event.extensions.cpt")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("cef.extensions.sourceServiceName") {
                event.rename(
                    "cef.extensions.sourceServiceName",
                    "imperva_cloud_waf.event.extensions.source.service_name",
                )?;
            }

            if event.has_value("cef.extensions.sourceUserId") {
                event.rename(
                    "cef.extensions.sourceUserId",
                    "imperva_cloud_waf.event.extensions.source.user_id",
                )?;
            }

            let _cond = { event.has_value("imperva_cloud_waf.event.extensions.source.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("imperva_cloud_waf.event.extensions.source.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.startTime")
                    && event.get_str("cef.extensions.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.startTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("imperva_cloud_waf.event.extensions.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.startTime".into(),
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
                        "date_extensions_startTime",
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

            if event.has_value("cef.extensions.tag") {
                event.rename(
                    "cef.extensions.tag",
                    "imperva_cloud_waf.event.extensions.tag",
                )?;
            }

            if event.has_value("cef.extensions.ver") {
                event.rename(
                    "cef.extensions.ver",
                    "imperva_cloud_waf.event.extensions.ver",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("imperva_cloud_waf.event.extensions.ver") {
                    if let Some(input) = event.get_string("imperva_cloud_waf.event.extensions.ver")
                    {
                        // Grok pattern: ^(TLSv%{GREEDYDATA:tls.version} %{GREEDYDATA:tls.cipher})$
                        if !cached_grok!(
                            "^(TLSv%{GREEDYDATA:tls.version} %{GREEDYDATA:tls.cipher})$"
                        )
                        .extract_into(&input, event)?
                        {
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
                    "grok_to_extract_tls_cipher_and_version",
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
                event
                    .get("cef.extensions.xff")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cef.extensions.xff") {
                        if let Some(s) = event.get_string("cef.extensions.xff") {
                            let mut parts: Vec<Value> = cached_regex!(",\\s?")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("cef.extensions.xff", Value::Array(parts))?;
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

            let _cond = {
                event
                    .get("cef.extensions.xff")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "cef.extensions.xff", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
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
                        event.remove("_ingest._value");
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

            let _cond = {
                event
                    .get("cef.extensions.xff")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("cef.extensions.xff") {
                    event.rename(
                        "cef.extensions.xff",
                        "imperva_cloud_waf.event.extensions.xff",
                    )?;
                }
            }

            if let Some(v) = event
                .get("imperva_cloud_waf.event.extensions.xff")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.forwarded_ip", v)?;
            }

            let _cond = {
                event
                    .get("network.forwarded_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "network.forwarded_ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("cef.name") {
                event.rename("cef.name", "imperva_cloud_waf.event.name")?;
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
                        event.set("imperva_cloud_waf.event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_to_long",
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

            if event.has_value("cef.version") {
                event.rename("cef.version", "imperva_cloud_waf.event.version")?;
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
                event.remove("imperva_cloud_waf.event.device.event_class_id");
                event.remove("imperva_cloud_waf.event.device.product");
                event.remove("imperva_cloud_waf.event.device.vendor");
                event.remove("imperva_cloud_waf.event.device.version");
                event.remove("imperva_cloud_waf.event.extensions.action");
                event.remove("imperva_cloud_waf.event.extensions.application_protocol");
                event.remove("imperva_cloud_waf.event.extensions.bytes_in");
                event.remove("imperva_cloud_waf.event.extensions.ccode");
                event.remove("imperva_cloud_waf.event.extensions.cpt");
                event.remove("imperva_cloud_waf.event.extensions.cs9");
                event.remove("imperva_cloud_waf.event.extensions.device.custom_number1");
                event.remove("imperva_cloud_waf.event.extensions.end_time");
                event.remove("imperva_cloud_waf.event.extensions.file_id");
                event.remove("imperva_cloud_waf.event.extensions.postbody");
                event.remove("imperva_cloud_waf.event.extensions.qstr");
                event.remove("imperva_cloud_waf.event.extensions.request.client_application");
                event.remove("imperva_cloud_waf.event.extensions.request.method");
                event.remove("imperva_cloud_waf.event.extensions.request.url");
                event.remove("imperva_cloud_waf.event.extensions.sip");
                event.remove("imperva_cloud_waf.event.extensions.source.address");
                event.remove("imperva_cloud_waf.event.extensions.source.port");
                event.remove("imperva_cloud_waf.event.extensions.source.user_id");
                event.remove("imperva_cloud_waf.event.extensions.start_time");
                event.remove("imperva_cloud_waf.event.extensions.xff");
                event.remove("imperva_cloud_waf.event.severity");
                event.remove("imperva_cloud_waf.event.extensions.cs7");
                event.remove("imperva_cloud_waf.event.extensions.cs8");
                event.remove("imperva_cloud_waf.event.extensions.file.permission");
                event.remove("imperva_cloud_waf.event.extensions.file.type");
            }

            event.remove("cef");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
