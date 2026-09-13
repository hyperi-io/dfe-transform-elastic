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
                // Source: def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  ctx.event = ctx.event ?: new HashMap();\n  ctx.event.original = stringified_orig;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  ctx.event = ctx.event ?: new HashMap();\n  ctx.event.original = stringified_orig;\n}\n"#
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

            let _cond = {
                event.has_value("device.id")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("users-entities"))
                        }
                        serde_json::Value::String(s) => s.contains("users-entities"),
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.has_value("user.id")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("devices-entities"))
                        }
                        serde_json::Value::String(s) => s.contains("devices-entities"),
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("event.kind", json!("asset"))?;

            let _cond = { event.has_value("device.id") };
            if _cond {
                // Begin nested pipeline: "device"
                event.append("event.category", json!("host"))?;
                event.append("event.type", json!("info"))?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("microsoft_entra_id_device"))?;
                if event.has_value("azure_ad") {
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
                        if let Some(val) =
                            event.get("entityanalytics_entra_id.device.account_enabled")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "entityanalytics_entra_id.device.account_enabled".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "entityanalytics_entra_id.device.account_enabled",
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
                let _cond = {
                    event.get_bool("entityanalytics_entra_id.device.account_enabled") == Some(true)
                };
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
                    event.has_value(
                        "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                    ) && event.get_str(
                        "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                    ) != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "entityanalytics_entra_id.device.approximate_last_sign_in_date_time",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("entityanalytics_entra_id.device.approximate_last_sign_in_date_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "entityanalytics_entra_id.device.approximate_last_sign_in_date_time".into(),
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
                    event.has_value(
                        "entityanalytics_entra_id.device.compliance_expiration_date_time",
                    ) && event
                        .get_str("entityanalytics_entra_id.device.compliance_expiration_date_time")
                        != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "entityanalytics_entra_id.device.compliance_expiration_date_time",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("entityanalytics_entra_id.device.compliance_expiration_date_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "entityanalytics_entra_id.device.compliance_expiration_date_time".into(),
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
                            "date_compliance_expiration_date_time",
                        )?;
                        event.remove(
                            "entityanalytics_entra_id.device.compliance_expiration_date_time",
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
                if event.has_value("entityanalytics_entra_id.device.device_category") {
                    event.rename(
                        "entityanalytics_entra_id.device.device_category",
                        "entityanalytics_entra_id.device.category",
                    )?;
                }
                if event.has_value("entityanalytics_entra_id.device.device_id") {
                    event.rename(
                        "entityanalytics_entra_id.device.device_id",
                        "entityanalytics_entra_id.device.d_id",
                    )?;
                }
                if event.has_value("entityanalytics_entra_id.device.device_metadata") {
                    event.rename(
                        "entityanalytics_entra_id.device.device_metadata",
                        "entityanalytics_entra_id.device.metadata",
                    )?;
                }
                if event.has_value("entityanalytics_entra_id.device.device_ownership") {
                    event.rename(
                        "entityanalytics_entra_id.device.device_ownership",
                        "entityanalytics_entra_id.device.ownership",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("entityanalytics_entra_id.device.device_version") {
                        if let Some(val) =
                            event.get("entityanalytics_entra_id.device.device_version")
                        {
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
                        if let Some(val) = event.get("entityanalytics_entra_id.device.is_compliant")
                        {
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
                    event.has_value(
                        "entityanalytics_entra_id.device.on_premises_last_sync_date_time",
                    ) && event
                        .get_str("entityanalytics_entra_id.device.on_premises_last_sync_date_time")
                        != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "entityanalytics_entra_id.device.on_premises_last_sync_date_time",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("entityanalytics_entra_id.device.on_premises_last_sync_date_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "entityanalytics_entra_id.device.on_premises_last_sync_date_time".into(),
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
                            "date_on_premises_last_sync_date_time",
                        )?;
                        event.remove(
                            "entityanalytics_entra_id.device.on_premises_last_sync_date_time",
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
                            let converted =
                                convert_value(val, "boolean").map_err(|message| {
                                    TransformError::ParseError {
                path: "entityanalytics_entra_id.device.on_premises_sync_enabled".into(),
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
                    if let Err(err) =
                        (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "entityanalytics_entra_id.device.registration_date_time",
                            ) {
                                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                    Some(parsed) => event.set(
                                        "entityanalytics_entra_id.device.registration_date_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                path: "entityanalytics_entra_id.device.registration_date_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                                    }
                                }
                            }
                            Ok(())
                        })()
                    {
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
                            if event.has_value("_ingest._value.userPrincipalName") {
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
                            if event.has_value("_ingest._value.displayName") {
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
                            if event.has_value("_ingest._value.givenName") {
                                event.rename(
                                    "_ingest._value.givenName",
                                    "_ingest._value.given_name",
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
                            if event.has_value("_ingest._value.jobTitle") {
                                event.rename(
                                    "_ingest._value.jobTitle",
                                    "_ingest._value.job_title",
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
                            if event.has_value("_ingest._value.mobilePhone") {
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
                            if event.has_value("_ingest._value.businessPhones") {
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
                            if event.has_value("_ingest._value.userPrincipalName") {
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
                            if event.has_value("_ingest._value.displayName") {
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
                            if event.has_value("_ingest._value.givenName") {
                                event.rename(
                                    "_ingest._value.givenName",
                                    "_ingest._value.given_name",
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
                            if event.has_value("_ingest._value.jobTitle") {
                                event.rename(
                                    "_ingest._value.jobTitle",
                                    "_ingest._value.job_title",
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
                            if event.has_value("_ingest._value.mobilePhone") {
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
                            if event.has_value("_ingest._value.businessPhones") {
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
                // End nested pipeline: "device"
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                // Begin nested pipeline: "user"
                event.append("event.category", json!("iam"))?;
                event.append("event.type", json!("user"))?;
                event.append("event.type", json!("info"))?;
                event.set("asset.category", json!("entity"))?;
                event.set("asset.type", json!("microsoft_entra_id_user"))?;
                if event.has_value("azure_ad") {
                    event.rename("azure_ad", "entityanalytics_entra_id.user")?;
                }
                // Painless script
                // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.entityanalytics_entra_id?.user != null) {\n  ctx.entityanalytics_entra_id.user = convertToSnakeCase(ctx.entityanalytics_entra_id.user);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.entityanalytics_entra_id?.user != null) {\n  ctx.entityanalytics_entra_id.user = convertToSnakeCase(ctx.entityanalytics_entra_id.user);\n}\n"#
                    ),
                )?;
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("entityanalytics_entra_id.user.account_enabled") {
                        if let Some(val) =
                            event.get("entityanalytics_entra_id.user.account_enabled")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "entityanalytics_entra_id.user.account_enabled".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("entityanalytics_entra_id.user.account_enabled", converted)?;
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
                        .remove("entityanalytics_entra_id.user.account_enabled")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "entityanalytics_entra_id.user.account_enabled".into(),
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
                let _cond = {
                    event.get_bool("entityanalytics_entra_id.user.account_enabled") == Some(true)
                };
                if _cond {
                    event.set("user.enabled", json!(true))?;
                }
                let _cond = { event.has_value("entityanalytics_entra_id.user.mail") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.mail")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.mail")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond =
                    { event.has_value("entityanalytics_entra_id.user.user_principal_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.user_principal_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.user_principal_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                let _cond = { event.has_value("entityanalytics_entra_id.user.display_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.display_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.full_name", v)?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.given_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.first_name", v)?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.surname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.last_name", v)?;
                }
                let _cond = { event.has_value("entityanalytics_entra_id.user.mobile_phone") };
                if _cond {
                    event.append_unique(
                        "user.phone",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.mobile_phone")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("entityanalytics_entra_id.user.business_phones")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "user.phone",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.business_phones")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("entityanalytics_entra_id.user.business_phones")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "entityanalytics_entra_id.user.business_phones",
                        |event| {
                            event.append_unique(
                                "user.phone",
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
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.job_title")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.job_title", v)?;
                }
                if let Some(v) = event
                    .get("entityanalytics_entra_id.user.office_location")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.work.location_name", v)?;
                }
                if let Some(v) = event
                    .get("user.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("entityanalytics_entra_id.user.id", v)?;
                }
                if let Some(v) = event
                    .get("user.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("asset.id", v)?;
                }
                let _cond = { event.has_value("entityanalytics_entra_id.user.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("entityanalytics_entra_id.user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("user.group")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("entityanalytics_entra_id.user.group", v)?;
                }
                if let Some(v) = event
                    .get("user.group")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("asset.group", v)?;
                }
                let _cond = {
                    event.has_value(
                        "entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time",
                    ) && event.get_str(
                        "entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time",
                    ) != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time".into(),
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
                            "date_sign_in_activity_last_sign_in_date_time",
                        )?;
                        event.remove(
                            "entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time",
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
                    .get("entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.entity.lifecycle.last_activity", v)?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("entityanalytics_entra_id.user.mfa.is_mfa_registered") {
                        if let Some(val) =
                            event.get("entityanalytics_entra_id.user.mfa.is_mfa_registered")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "entityanalytics_entra_id.user.mfa.is_mfa_registered"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "entityanalytics_entra_id.user.mfa.is_mfa_registered",
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
                        "convert_isMfaRegistered_to_boolean",
                    )?;
                    event.remove("entityanalytics_entra_id.user.mfa.is_mfa_registered");
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
                    .get("entityanalytics_entra_id.user.mfa.is_mfa_registered")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.entity.attributes.mfa_enabled", v)?;
                }
                let _cond = {
                    event
                        .get("entityanalytics_entra_id.user.direct_reports")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // Painless script
                    // Source: def ids = new ArrayList();\ndef names = new ArrayList();\ndef emails = new ArrayList();\nfor (def report : ctx.entityanalytics_entra_id.user.direct_reports) {\n  if (report == null) { continue; }\n  if (report.id != null) { ids.add(report.id); }\n  if (report.user_principal_name != null) { names.add(report.user_principal_name); }\n  if (report.mail != null) { emails.add(report.mail); }\n}\ndef userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\nif (!names.isEmpty()) { userObj.put(\"name\", names); }\nif (!emails.isEmpty()) { userObj.put(\"email\", emails); }\nif (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def ids = new ArrayList();\ndef names = new ArrayList();\ndef emails = new ArrayList();\nfor (def report : ctx.entityanalytics_entra_id.user.direct_reports) {\n  if (report == null) { continue; }\n  if (report.id != null) { ids.add(report.id); }\n  if (report.user_principal_name != null) { names.add(report.user_principal_name); }\n  if (report.mail != null) { emails.add(report.mail); }\n}\ndef userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\nif (!names.isEmpty()) { userObj.put(\"name\", names); }\nif (!emails.isEmpty()) { userObj.put(\"email\", emails); }\nif (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n  ctx.user.entity = ctx.user.entity ?: new HashMap();\n  ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n  ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("entityanalytics_entra_id.user.app_role_assignments")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(
                            event,
                            "entityanalytics_entra_id.user.app_role_assignments",
                            |event| {
                                event.append_unique(
                                    "user.entity.attributes.permissions",
                                    json!(
                                        event
                                            .get("_ingest._value.app_role_display_name")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "user"
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
                event.remove("entityanalytics_entra_id.user.account_enabled");
                event.remove("entityanalytics_entra_id.user.mail");
                event.remove("entityanalytics_entra_id.user.user_principal_name");
                event.remove("entityanalytics_entra_id.user.display_name");
                event.remove("entityanalytics_entra_id.user.given_name");
                event.remove("entityanalytics_entra_id.user.surname");
                event.remove("entityanalytics_entra_id.user.mobile_phone");
                event.remove("entityanalytics_entra_id.user.business_phones");
                event.remove("entityanalytics_entra_id.user.job_title");
                event.remove("entityanalytics_entra_id.user.office_location");
                event.remove("entityanalytics_entra_id.user.id");
                event.remove("entityanalytics_entra_id.user.app_role_assignments");
                event.remove("entityanalytics_entra_id.user.mfa");
                event.remove("entityanalytics_entra_id.user.direct_reports");
                event.remove("entityanalytics_entra_id.user.sign_in_activity");
                event.remove("entityanalytics_entra_id.user.manager");
                event.remove("entityanalytics_entra_id.device.approximate_last_sign_in_date_time");
                event.remove("entityanalytics_entra_id.device.version");
                event.remove("entityanalytics_entra_id.device.display_name");
                event.remove("entityanalytics_entra_id.device.is_managed");
                event.remove("entityanalytics_entra_id.device.manufacturer");
                event.remove("entityanalytics_entra_id.device.model");
                event.remove("entityanalytics_entra_id.device.on_premises_last_sync_date_time");
                event.remove("entityanalytics_entra_id.device.operating_system");
                event.remove("entityanalytics_entra_id.device.operating_system_version");
                event.remove("entityanalytics_entra_id.device.registration_date_time");
                event.remove("entityanalytics_entra_id.device.system_labels");
                event.remove("entityanalytics_entra_id.device.id");
            }

            let _cond = {
                event
                    .get("entityanalytics_entra_id.device.registered_users")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "entityanalytics_entra_id.device.registered_users",
                        |event| {
                            let _cond = {
                                !event.has_value("tags")
                                    || !(event.get("tags").is_some_and(|v| match v {
                                        serde_json::Value::Array(a) => a.iter().any(|x| {
                                            x.as_str() == Some("preserve_duplicate_custom_fields")
                                        }),
                                        serde_json::Value::String(s) => {
                                            s.contains("preserve_duplicate_custom_fields")
                                        }
                                        _ => false,
                                    }))
                            };
                            if _cond {
                                event.remove("_ingest._value.id");
                                event.remove("_ingest._value.user_principal_name");
                                event.remove("_ingest._value.mail");
                                event.remove("_ingest._value.display_name");
                                event.remove("_ingest._value.given_name");
                                event.remove("_ingest._value.surname");
                                event.remove("_ingest._value.job_title");
                                event.remove("_ingest._value.mobile_phone");
                                event.remove("_ingest._value.business_phones");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("entityanalytics_entra_id.device.registered_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "entityanalytics_entra_id.device.registered_owners",
                        |event| {
                            let _cond = {
                                !event.has_value("tags")
                                    || !(event.get("tags").is_some_and(|v| match v {
                                        serde_json::Value::Array(a) => a.iter().any(|x| {
                                            x.as_str() == Some("preserve_duplicate_custom_fields")
                                        }),
                                        serde_json::Value::String(s) => {
                                            s.contains("preserve_duplicate_custom_fields")
                                        }
                                        _ => false,
                                    }))
                            };
                            if _cond {
                                event.remove("_ingest._value.id");
                                event.remove("_ingest._value.user_principal_name");
                                event.remove("_ingest._value.mail");
                                event.remove("_ingest._value.display_name");
                                event.remove("_ingest._value.given_name");
                                event.remove("_ingest._value.surname");
                                event.remove("_ingest._value.job_title");
                                event.remove("_ingest._value.mobile_phone");
                                event.remove("_ingest._value.business_phones");
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("entityanalytics_entra_id.device.group")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "entityanalytics_entra_id.device.group", |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.id");
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("entityanalytics_entra_id.user.group")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "entityanalytics_entra_id.user.group", |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.id");
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
