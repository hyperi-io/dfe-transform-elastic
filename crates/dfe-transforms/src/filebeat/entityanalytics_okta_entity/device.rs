// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `device` pipeline.
pub struct Device;

impl Transform for Device {
    fn name(&self) -> &str {
        "device"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.original = stringified_orig;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.original = stringified_orig;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("event.action") != Some("started")
                    && event.get_str("event.action") != Some("completed")
            };
            if _cond {
                event.remove("event.action");
            }

            event.set("event.kind", json!("asset"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("okta_device"))?;

            if event.has_value("okta.id") {
                event.rename("okta.id", "entityanalytics_okta.device.id")?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.id", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("okta.status") {
                event.rename("okta.status", "entityanalytics_okta.device.status")?;
            }

            if let Some(v) = event
                .get("entityanalytics_okta.device.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.status", v)?;
            }

            let _cond =
                { event.has_value("okta.created") && event.get_str("okta.created") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.device.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_device_created")?;
                    if event.remove("okta.created").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.created".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("entityanalytics_okta.device.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.create_date", v)?;
            }

            let _cond = {
                event.has_value("okta.activated") && event.get_str("okta.activated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.activated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.device.activated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.activated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_device_activated")?;
                    if event.remove("okta.activated").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.activated".into(),
                        });
                    }
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
                event.has_value("okta.statusChanged")
                    && event.get_str("okta.statusChanged") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.statusChanged") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.device.status_changed", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.statusChanged".into(),
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
                        "date_device_status_changed",
                    )?;
                    if event.remove("okta.statusChanged").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.statusChanged".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("entityanalytics_okta.device.status_changed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_status_change_date", v)?;
            }

            let _cond = {
                event.has_value("okta.lastUpdated") && event.get_str("okta.lastUpdated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("okta.lastUpdated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("entityanalytics_okta.device.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "okta.lastUpdated".into(),
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
                        "date_device_last_updated",
                    )?;
                    if event.remove("okta.lastUpdated").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "okta.lastUpdated".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("entityanalytics_okta.device.last_updated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_updated", v)?;
            }

            if event.has_value("okta.transitioningToStatus") {
                event.rename(
                    "okta.transitioningToStatus",
                    "entityanalytics_okta.device.transitioning_to_status",
                )?;
            }

            let _cond = { event.get("okta.users").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "okta.users", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("okta.users").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "okta.users", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.profile.login")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("okta.users").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "okta.users", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.profile.nickName")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("okta.users").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "okta.users", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.profile.displayName")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("okta.profile.platform") {
                map_strings(
                    event,
                    "okta.profile.platform",
                    "os.platform",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("okta.profile.displayName") {
                event.rename(
                    "okta.profile.displayName",
                    "entityanalytics_okta.device.profile.display_name",
                )?;
            }

            if event.has_value("okta.profile.osVersion") {
                event.rename("okta.profile.osVersion", "host.os.version")?;
            }

            if event.has_value("okta.profile.sid") {
                event.rename(
                    "okta.profile.sid",
                    "entityanalytics_okta.device.profile.sid",
                )?;
            }

            if event.has_value("okta.profile.serialNumber") {
                event.rename("okta.profile.serialNumber", "device.serial_number")?;
            }

            if event.has_value("okta.profile.diskEncryptionType") {
                event.rename(
                    "okta.profile.diskEncryptionType",
                    "entityanalytics_okta.device.profile.disk_encryption_type",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("okta.profile.registered") {
                    if let Some(val) = event.get("okta.profile.registered") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "okta.profile.registered".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_okta.device.profile.registered", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_device_profile_registered",
                )?;
                if event.remove("okta.profile.registered").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.profile.registered".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("okta.profile.secureHardwarePresent") {
                    if let Some(val) = event.get("okta.profile.secureHardwarePresent") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "okta.profile.secureHardwarePresent".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "entityanalytics_okta.device.profile.secure_hardware_present",
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
                    "convert_device_profile_secure_hardware_present",
                )?;
                if event
                    .remove("okta.profile.secure_hardware_present")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.profile.secure_hardware_present".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("okta.profile.managed") {
                    if let Some(val) = event.get("okta.profile.managed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "okta.profile.managed".into(),
                                message,
                            }
                        })?;
                        event.set("host.entity.attributes.managed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_device_profile_managed",
                )?;
                if event.remove("okta.profile.managed").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.profile.managed".into(),
                    });
                }
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

            if let Some(v) = event
                .get("entityanalytics_okta.device.profile.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.name", v)?;
            }

            if event.has_value("okta._links") {
                event.rename("okta._links", "entityanalytics_okta.device._links")?;
            }

            if event.has_value("okta._embedded") {
                event.rename("okta._embedded", "entityanalytics_okta.device._embedded")?;
            }

            event.remove("okta");

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
                event.remove("entityanalytics_okta.device.status");
                event.remove("entityanalytics_okta.device.activated");
                event.remove("entityanalytics_okta.device.status_changed");
                event.remove("entityanalytics_okta.device.created");
                event.remove("entityanalytics_okta.device.id");
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
