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
                event
                    .get("cef.extensions.deviceEventCategory")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("cef.extensions.deviceEventCategory")
                        .is_some_and(|s| s.to_lowercase().contains("alert"))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event
                    .get("cef.extensions.deviceEventCategory")
                    .is_some_and(|v| v.is_string())
                    && event
                        .get_str("cef.extensions.deviceEventCategory")
                        .is_some_and(|s| s.to_lowercase().contains("event"))
            };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
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

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
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
                        "date_set_timestamp_timezone",
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
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("imperva.securesphere.device.receipt_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_deviceReceiptTime")?;
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
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("imperva.securesphere.device.receipt_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
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
                        "date_timezone_deviceReceiptTime",
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
                            event.set("imperva.securesphere.destination.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destinationAddress_to_ip",
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("imperva.securesphere.destination.address")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("cef.extensions.destinationPort") != Some("") };
            if _cond {
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
                            event.set("imperva.securesphere.destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destinationPort_to_long",
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

            if event.has_value("cef.extensions.destinationUserName") {
                event.rename(
                    "cef.extensions.destinationUserName",
                    "imperva.securesphere.destination.user_name",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("imperva.securesphere.destination.user_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("cef.extensions.deviceAction") {
                event.rename(
                    "cef.extensions.deviceAction",
                    "imperva.securesphere.device.action",
                )?;
            }

            if event.has_value("cef.device.event_class_id") {
                event.rename(
                    "cef.device.event_class_id",
                    "imperva.securesphere.device.event.class_id",
                )?;
            }

            if event.has_value("cef.name") {
                event.rename("cef.name", "imperva.securesphere.name")?;
            }

            if event.has_value("cef.extensions.transportProtocol") {
                event.rename(
                    "cef.extensions.transportProtocol",
                    "imperva.securesphere.transport_protocol",
                )?;
            }

            if event.has_value("cef.device.product") {
                event.rename("cef.device.product", "imperva.securesphere.device.product")?;
            }

            if event.has_value("cef.device.vendor") {
                event.rename("cef.device.vendor", "imperva.securesphere.device.vendor")?;
            }

            if event.has_value("cef.device.version") {
                event.rename("cef.device.version", "imperva.securesphere.device.version")?;
            }

            if event.has_value("cef.extensions.deviceCustomString3") {
                event.rename(
                    "cef.extensions.deviceCustomString3",
                    "imperva.securesphere.device.custom_string3.value",
                )?;
            }

            if let Some(v) = event
                .get("imperva.securesphere.device.custom_string3.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.name", v)?;
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
                            event.set("imperva.securesphere.source.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_ipAddress_to_string",
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("imperva.securesphere.source.address")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("cef.extensions.sourcePort") != Some("") };
            if _cond {
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
                            event.set("imperva.securesphere.source.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_port_to_long",
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

            if event.has_value("cef.extensions.sourceUserName") {
                event.rename(
                    "cef.extensions.sourceUserName",
                    "imperva.securesphere.source.user_name",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("imperva.securesphere.source.user_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("cef.extensions.deviceCustomString1Label") {
                event.rename(
                    "cef.extensions.deviceCustomString1Label",
                    "imperva.securesphere.device.custom_string1.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString1") {
                event.rename(
                    "cef.extensions.deviceCustomString1",
                    "imperva.securesphere.device.custom_string1.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString2Label") {
                event.rename(
                    "cef.extensions.deviceCustomString2Label",
                    "imperva.securesphere.device.custom_string2.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString2") {
                event.rename(
                    "cef.extensions.deviceCustomString2",
                    "imperva.securesphere.device.custom_string2.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString3Label") {
                event.rename(
                    "cef.extensions.deviceCustomString3Label",
                    "imperva.securesphere.device.custom_string3.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4Label") {
                event.rename(
                    "cef.extensions.deviceCustomString4Label",
                    "imperva.securesphere.device.custom_string4.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString4") {
                event.rename(
                    "cef.extensions.deviceCustomString4",
                    "imperva.securesphere.device.custom_string4.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString5Label") {
                event.rename(
                    "cef.extensions.deviceCustomString5Label",
                    "imperva.securesphere.device.custom_string5.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString5") {
                event.rename(
                    "cef.extensions.deviceCustomString5",
                    "imperva.securesphere.device.custom_string5.value",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString6Label") {
                event.rename(
                    "cef.extensions.deviceCustomString6Label",
                    "imperva.securesphere.device.custom_string6.label",
                )?;
            }

            if event.has_value("cef.extensions.deviceCustomString6") {
                event.rename(
                    "cef.extensions.deviceCustomString6",
                    "imperva.securesphere.device.custom_string6.value",
                )?;
            }

            // Painless script
            // Source: if (ctx.cef?.extensions instanceof Map) {\n  boolean hasCustomString = true;\n  for (int i = 7; i <= 21 && hasCustomString; i++) {\n    String key = \"cs\" + i;\n    String labelKey = key + \"Label\";\n    if (ctx.cef.extensions.containsKey(key) || ctx.cef.extensions.containsKey(labelKey)) {\n      String newKey = \"custom_string\" + i;\n      ctx.imperva.securesphere.device[newKey] = [:];\n\n      if (ctx.cef.extensions.containsKey(key)) {\n        ctx.imperva.securesphere.device[newKey][\"value\"] = ctx.cef.extensions[key];\n      }\n      if (ctx.cef.extensions.containsKey(labelKey)) {\n        ctx.imperva.securesphere.device[newKey][\"label\"] = ctx.cef.extensions[labelKey];\n      }\n\n    } else {\n      // Let it break the loop if the key is not found.\n      hasCustomString = false;\n    }\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.cef?.extensions instanceof Map) {\n  boolean hasCustomString = true;\n  for (int i = 7; i <= 21 && hasCustomString; i++) {\n    String key = \"cs\" + i;\n    String labelKey = key + \"Label\";\n    if (ctx.cef.extensions.containsKey(key) || ctx.cef.extensions.containsKey(labelKey)) {\n      String newKey = \"custom_string\" + i;\n      ctx.imperva.securesphere.device[newKey] = [:];\n\n      if (ctx.cef.extensions.containsKey(key)) {\n        ctx.imperva.securesphere.device[newKey][\"value\"] = ctx.cef.extensions[key];\n      }\n      if (ctx.cef.extensions.containsKey(labelKey)) {\n        ctx.imperva.securesphere.device[newKey][\"label\"] = ctx.cef.extensions[labelKey];\n      }\n\n    } else {\n      // Let it break the loop if the key is not found.\n      hasCustomString = false;\n    }\n  }\n}"#
                ),
            )?;

            if event.has_value("cef.extensions.deviceEventCategory") {
                event.rename(
                    "cef.extensions.deviceEventCategory",
                    "imperva.securesphere.device.event.category",
                )?;
            }

            if event.has_value("cef.severity") {
                event.rename("cef.severity", "imperva.securesphere.severity")?;
            }

            if event.has_value("cef.version") {
                event.rename("cef.version", "imperva.securesphere.version")?;
            }

            event.remove("cef");

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
                event.remove("imperva.securesphere.destination.address");
                event.remove("imperva.securesphere.device.receipt_time");
                event.remove("imperva.securesphere.destination.port");
                event.remove("imperva.securesphere.destination.user_name");
                event.remove("imperva.securesphere.device.action");
                event.remove("imperva.securesphere.device.event.class_id");
                event.remove("imperva.securesphere.name");
                event.remove("imperva.securesphere.transport_protocol");
                event.remove("imperva.securesphere.device.product");
                event.remove("imperva.securesphere.device.vendor");
                event.remove("imperva.securesphere.device.version");
                event.remove("imperva.securesphere.device.custom_string3.value");
                event.remove("imperva.securesphere.source.address");
                event.remove("imperva.securesphere.source.port");
                event.remove("imperva.securesphere.source.user_name");
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
