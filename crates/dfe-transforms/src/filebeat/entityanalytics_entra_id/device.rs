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
            event.append("event.category", json!("host"))?;

            event.append("event.type", json!("info"))?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("microsoft_entra_id_device"))?;

            if event.has("azure_ad") {
                event.rename("azure_ad", "entityanalytics_entra_id.device")?;
            }

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.entityanalytics_entra_id?.device != null) {\n  ctx.entityanalytics_entra_id.device = convertToSnakeCase(ctx.entityanalytics_entra_id.device);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.entityanalytics_entra_id?.device != null) {\n  ctx.entityanalytics_entra_id.device = convertToSnakeCase(ctx.entityanalytics_entra_id.device);\n}\n"#
                ),
            )?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("entityanalytics_entra_id.device.account_enabled") {
                    if let Some(val) = event.get("entityanalytics_entra_id.device.account_enabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.device.account_enabled".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_entra_id.device.account_enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_accountEnabled_to_boolean",
                )?;
                if event
                    .remove("entityanalytics_entra_id.device.account_enabled")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "entityanalytics_entra_id.device.account_enabled".into(),
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

            let _cond =
                { event.get_bool("entityanalytics_entra_id.device.account_enabled") == Some(true) };
            if _cond {
                event.set("asset.status", json!("enabled"))?;
            }

            let _cond = {
                event.get_bool("entityanalytics_entra_id.device.account_enabled") == Some(false)
            };
            if _cond {
                event.set("asset.status", json!("disabled"))?;
            }

            let _cond = {
                event
                    .get("entityanalytics_entra_id.device.alternative_security_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "entityanalytics_entra_id.device.alternative_security_ids",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.type") {
                                    if let Some(val) = event.get("_ingest._value.type") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.type".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.type", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_alternativeSecurityIds_type_to_long",
                                )?;
                                event.remove("_ingest._value.type");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .has_value("entityanalytics_entra_id.device.approximate_last_sign_in_date_time")
                    && event.get_str(
                        "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                    ) {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("entityanalytics_entra_id.device.approximate_last_sign_in_date_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_approximate_last_sign_in_date_time",
                    )?;
                    event.remove(
                        "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                    );
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
                .get("entityanalytics_entra_id.device.approximate_last_sign_in_date_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.lifecycle.last_activity", v)?;
            }

            let _cond = {
                event.has_value("entityanalytics_entra_id.device.compliance_expiration_date_time")
                    && event
                        .get_str("entityanalytics_entra_id.device.compliance_expiration_date_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "entityanalytics_entra_id.device.compliance_expiration_date_time",
                    ) {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set(
                                "entityanalytics_entra_id.device.compliance_expiration_date_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_compliance_expiration_date_time",
                    )?;
                    event.remove("entityanalytics_entra_id.device.compliance_expiration_date_time");
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

            if event.has("entityanalytics_entra_id.device.device_category") {
                event.rename(
                    "entityanalytics_entra_id.device.device_category",
                    "entityanalytics_entra_id.device.category",
                )?;
            }

            if event.has("entityanalytics_entra_id.device.device_id") {
                event.rename(
                    "entityanalytics_entra_id.device.device_id",
                    "entityanalytics_entra_id.device.d_id",
                )?;
            }

            if event.has("entityanalytics_entra_id.device.device_metadata") {
                event.rename(
                    "entityanalytics_entra_id.device.device_metadata",
                    "entityanalytics_entra_id.device.metadata",
                )?;
            }

            if event.has("entityanalytics_entra_id.device.device_ownership") {
                event.rename(
                    "entityanalytics_entra_id.device.device_ownership",
                    "entityanalytics_entra_id.device.ownership",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("entityanalytics_entra_id.device.device_version") {
                    if let Some(val) = event.get("entityanalytics_entra_id.device.device_version") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.device.device_version".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_entra_id.device.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_deviceVersion_to_string",
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
                .get("entityanalytics_entra_id.device.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.version", v)?;
            }

            let _cond = { event.has_value("entityanalytics_entra_id.device.display_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("entityanalytics_entra_id.device.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.name", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("entityanalytics_entra_id.device.is_compliant") {
                    if let Some(val) = event.get("entityanalytics_entra_id.device.is_compliant") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.device.is_compliant".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_entra_id.device.is_compliant", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isCompliant_to_boolean",
                )?;
                if event
                    .remove("entityanalytics_entra_id.device.is_compliant")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "entityanalytics_entra_id.device.is_compliant".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("entityanalytics_entra_id.device.is_managed") {
                    if let Some(val) = event.get("entityanalytics_entra_id.device.is_managed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.device.is_managed".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_entra_id.device.is_managed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isManaged_to_boolean",
                )?;
                if event
                    .remove("entityanalytics_entra_id.device.is_managed")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "entityanalytics_entra_id.device.is_managed".into(),
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
                .get("entityanalytics_entra_id.device.is_managed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.is_managed", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.is_managed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.attributes.managed", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.manufacturer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.vendor", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.model", v)?;
            }

            let _cond = {
                event.has_value("entityanalytics_entra_id.device.on_premises_last_sync_date_time")
                    && event
                        .get_str("entityanalytics_entra_id.device.on_premises_last_sync_date_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "entityanalytics_entra_id.device.on_premises_last_sync_date_time",
                    ) {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set(
                                "entityanalytics_entra_id.device.on_premises_last_sync_date_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_on_premises_last_sync_date_time",
                    )?;
                    event.remove("entityanalytics_entra_id.device.on_premises_last_sync_date_time");
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
                .get("entityanalytics_entra_id.device.on_premises_last_sync_date_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_updated", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.on_premises_last_sync_date_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.last_seen", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("entityanalytics_entra_id.device.on_premises_sync_enabled") {
                    if let Some(val) =
                        event.get("entityanalytics_entra_id.device.on_premises_sync_enabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.device.on_premises_sync_enabled"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "entityanalytics_entra_id.device.on_premises_sync_enabled",
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
                    "convert_onPremisesSyncEnabled_to_boolean",
                )?;
                if event
                    .remove("entityanalytics_entra_id.device.on_premises_sync_enabled")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "entityanalytics_entra_id.device.on_premises_sync_enabled".into(),
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
                .get("entityanalytics_entra_id.device.operating_system")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.type", v)?;
            }

            if event.has_value("host.os.type") {
                map_strings(event, "host.os.type", "host.os.type", str::to_lowercase)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.operating_system_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            let _cond = {
                event.has_value("entityanalytics_entra_id.device.registration_date_time")
                    && event.get_str("entityanalytics_entra_id.device.registration_date_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("entityanalytics_entra_id.device.registration_date_time")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set(
                                "entityanalytics_entra_id.device.registration_date_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_registration_date_time",
                    )?;
                    event.remove("entityanalytics_entra_id.device.registration_date_time");
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
                .get("entityanalytics_entra_id.device.registration_date_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.first_seen", v)?;
            }

            if let Some(v) = event
                .get("entityanalytics_entra_id.device.system_labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.tags", v)?;
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.userPrincipalName") {
                            event.rename(
                                "_ingest._value.userPrincipalName",
                                "_ingest._value.user_principal_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user_principal_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.mail")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.displayName") {
                            event.rename(
                                "_ingest._value.displayName",
                                "_ingest._value.display_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.givenName") {
                            event
                                .rename("_ingest._value.givenName", "_ingest._value.given_name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.jobTitle") {
                            event.rename("_ingest._value.jobTitle", "_ingest._value.job_title")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.mobilePhone") {
                            event.rename(
                                "_ingest._value.mobilePhone",
                                "_ingest._value.mobile_phone",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_owners", |event| {
                        if event.has("_ingest._value.businessPhones") {
                            event.rename(
                                "_ingest._value.businessPhones",
                                "_ingest._value.business_phones",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.userPrincipalName") {
                            event.rename(
                                "_ingest._value.userPrincipalName",
                                "_ingest._value.user_principal_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user_principal_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.mail")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.displayName") {
                            event.rename(
                                "_ingest._value.displayName",
                                "_ingest._value.display_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.givenName") {
                            event
                                .rename("_ingest._value.givenName", "_ingest._value.given_name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.jobTitle") {
                            event.rename("_ingest._value.jobTitle", "_ingest._value.job_title")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.mobilePhone") {
                            event.rename(
                                "_ingest._value.mobilePhone",
                                "_ingest._value.mobile_phone",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "device.registered_users", |event| {
                        if event.has("_ingest._value.businessPhones") {
                            event.rename(
                                "_ingest._value.businessPhones",
                                "_ingest._value.business_phones",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("entityanalytics_entra_id.device.id", v)?;
            }

            let _cond = { event.has_value("entityanalytics_entra_id.device.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("entityanalytics_entra_id.device.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.id", v)?;
            }

            if let Some(v) = event
                .get("device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("device.group")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("entityanalytics_entra_id.device.group", v)?;
            }

            if let Some(v) = event
                .get("device.group")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("asset.group", v)?;
            }

            if let Some(v) = event
                .get("device.registered_users")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("entityanalytics_entra_id.device.registered_users", v)?;
            }

            if let Some(v) = event
                .get("device.registered_owners")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("entityanalytics_entra_id.device.registered_owners", v)?;
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

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
