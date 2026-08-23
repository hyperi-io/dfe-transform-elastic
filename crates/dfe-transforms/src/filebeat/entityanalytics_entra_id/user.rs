// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `user` pipeline.
pub struct User;

impl Transform for User {
    fn name(&self) -> &str {
        "user"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("user"))?;
            event.append("event.type", json!("info"))?;

            event.set("asset.category", json!("entity"))?;

            event.set("asset.type", json!("microsoft_entra_id_user"))?;

            if event.has("azure_ad") {
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
                    if let Some(val) = event.get("entityanalytics_entra_id.user.account_enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "entityanalytics_entra_id.user.account_enabled".into(),
                                message,
                            }
                        })?;
                        event.set("entityanalytics_entra_id.user.account_enabled", converted)?;
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                { event.get_bool("entityanalytics_entra_id.user.account_enabled") == Some(true) };
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
                            .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.has_value("entityanalytics_entra_id.user.user_principal_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("entityanalytics_entra_id.user.user_principal_name")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                                    .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
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
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("entityanalytics_entra_id.user.sign_in_activity.last_sign_in_date_time", parsed)?;
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
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                                path: "entityanalytics_entra_id.user.mfa.is_mfa_registered".into(),
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
