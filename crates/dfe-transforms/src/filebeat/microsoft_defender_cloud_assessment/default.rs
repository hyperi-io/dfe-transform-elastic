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
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.name") {
                event.rename("json.name", "json.properties.name")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "json.properties.type")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "json.properties.id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "json.properties.additionalData.CVEs",
                    "json.properties.additionalData.CVEs",
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_properties_additionalData_CVEs",
                )?;
                event.remove("json.properties.additionalData.CVEs");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.properties") };
            if _cond {
                // Painless script
                // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def sb = new StringBuilder();\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        sb.append(\"_\");\n      }\n      sb.append(Character.toLowerCase(c));\n      lastCharWasUpperCase = true;\n    } else {\n      sb.append(c);\n      lastCharWasUpperCase = false;\n    }\n  }\n  return sb.toString();\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nctx.microsoft_defender_cloud = ctx.microsoft_defender_cloud ?: [:];\nctx.microsoft_defender_cloud.assessment = convertToSnakeCase(ctx.json.properties);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def sb = new StringBuilder();\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        sb.append(\"_\");\n      }\n      sb.append(Character.toLowerCase(c));\n      lastCharWasUpperCase = true;\n    } else {\n      sb.append(c);\n      lastCharWasUpperCase = false;\n    }\n  }\n  return sb.toString();\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nctx.microsoft_defender_cloud = ctx.microsoft_defender_cloud ?: [:];\nctx.microsoft_defender_cloud.assessment = convertToSnakeCase(ctx.json.properties);\n"#
                    ),
                )?;
            }

            if event.has_value(
                "microsoft_defender_cloud.assessment.additional_data.sub_assessment.name",
            ) {
                event.rename("microsoft_defender_cloud.assessment.additional_data.sub_assessment.name", "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.name")?;
            }

            if event.has_value(
                "microsoft_defender_cloud.assessment.additional_data.sub_assessment.type",
            ) {
                event.rename("microsoft_defender_cloud.assessment.additional_data.sub_assessment.type", "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.type")?;
            }

            if event.has_value(
                "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.id",
            ) {
                event.rename("microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.id", "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.event_id")?;
            }

            if event
                .has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.id")
            {
                event.rename("microsoft_defender_cloud.assessment.additional_data.sub_assessment.id", "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties.id")?;
            }

            if event.has_value(
                "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties",
            ) {
                event.rename_over(
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.properties",
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "microsoft_defender_cloud.assessment.additional_data.can onboard to _byol",
                ) {
                    if let Some(val) = event.get(
                        "microsoft_defender_cloud.assessment.additional_data.can onboard to _byol",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.can onboard to _byol".into(),
                            message,
                        })?;
                        event.set("microsoft_defender_cloud.assessment.additional_data.can_onboard_to_byol", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_additional_data_can_onboard_to_byol_to_boolean",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "microsoft_defender_cloud.assessment.additional_data.cves.base_score",
                ) {
                    if let Some(val) = event
                        .get("microsoft_defender_cloud.assessment.additional_data.cves.base_score")
                    {
                        let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.cves.base_score".into(),
                            message,
                        })?;
                        event.set(
                            "microsoft_defender_cloud.assessment.additional_data.cves.base_score",
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
                    "convert_additional_data_cves_base_score_to_float",
                )?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.cves.base_score");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.cves_count")
                {
                    if let Some(val) =
                        event.get("microsoft_defender_cloud.assessment.additional_data.cves_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "microsoft_defender_cloud.assessment.additional_data.cves_count"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_defender_cloud.assessment.additional_data.cves_count",
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
                    "convert_additional_data_cves_count_to_long",
                )?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.cves_count");
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
                if event.has_value(
                    "microsoft_defender_cloud.assessment.additional_data.max_cvss30_score",
                ) {
                    if let Some(val) = event
                        .get("microsoft_defender_cloud.assessment.additional_data.max_cvss30_score")
                    {
                        let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.max_cvss30_score".into(),
                            message,
                        })?;
                        event.set(
                            "microsoft_defender_cloud.assessment.additional_data.max_cvss30_score",
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
                    "convert_additional_data_max_cvss30_score_to_float",
                )?;
                event
                    .remove("microsoft_defender_cloud.assessment.additional_data.max_cvss30_score");
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("microsoft_defender_cloud.assessment.additional_data.nsg open ports")
                {
                    if let Some(val) = event
                        .get("microsoft_defender_cloud.assessment.additional_data.nsg open ports")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.nsg open ports".into(),
                            message,
                        })?;
                        event.set(
                            "microsoft_defender_cloud.assessment.additional_data.nsg_open_ports",
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
                    "convert_additional_data_nsg_open_ports_to_long",
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve").is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve") {
                    event.rename("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve", "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list")?;
                }
            }

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.cvss_score") {
                                    if let Some(val) = event.get("_ingest._value.cvss_score") {
                                        let converted =
                                            convert_value(val, "float").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.cvss_score".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.cvss_score", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_cvss_score_to_float")?;
                                if event.remove("_ingest._value.cvss_score").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.cvss_score".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_cvss_score_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_cvss_version_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version");
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list",
                    |event| {
                        if event.has_value("_ingest._value.cvss_version") {
                            if let Some(val) = event.get("_ingest._value.cvss_version") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.cvss_version".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.cvss_version", converted)?;
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version", converted)?;
                }
            }

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.has_public_exploit") {
                                    if let Some(val) =
                                        event.get("_ingest._value.has_public_exploit")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.has_public_exploit"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.has_public_exploit", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_has_public_exploit_to_boolean")?;
                                if event.remove("_ingest._value.has_public_exploit").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.has_public_exploit".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.has_public_exploit") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.has_public_exploit") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.has_public_exploit".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.has_public_exploit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_has_public_exploit_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.has_public_exploit");
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.is_exploit_in_kit") {
                                    if let Some(val) = event.get("_ingest._value.is_exploit_in_kit")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.is_exploit_in_kit".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.is_exploit_in_kit", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_exploit_in_kit_to_boolean")?;
                                if event.remove("_ingest._value.is_exploit_in_kit").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.is_exploit_in_kit".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_in_kit") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_in_kit") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_in_kit".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_in_kit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_exploit_in_kit_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_in_kit");
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.is_exploit_verified") {
                                    if let Some(val) =
                                        event.get("_ingest._value.is_exploit_verified")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.is_exploit_verified"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.is_exploit_verified", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_exploit_verified_to_boolean")?;
                                if event.remove("_ingest._value.is_exploit_verified").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.is_exploit_verified".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_verified") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_verified") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_verified".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_verified", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_exploit_verified_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_exploit_verified");
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.is_zero_day") {
                                    if let Some(val) = event.get("_ingest._value.is_zero_day") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.is_zero_day".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.is_zero_day", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_zero_day_to_boolean")?;
                                if event.remove("_ingest._value.is_zero_day").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.is_zero_day".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_zero_day") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_zero_day") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_zero_day".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_zero_day", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cve_is_zero_day_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.is_zero_day");
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

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.last_modified_date")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.last_modified_date", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_modified_date".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_cve_last_modified_date")?;
                                event.remove("_ingest._value.last_modified_date");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_cve_last_modified_date")?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.last_modified_date");
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
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.published_date")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.published_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.published_date".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_cve_published_date")?;
                                event.remove("_ingest._value.published_date");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date".into(),
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
                        "date_additional_data_sub_assessment_additional_data_cve_published_date",
                    )?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cvss_v30_score") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cvss_v30_score") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cvss_v30_score".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cvss_v30_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_cvss_v30_score_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cvss_v30_score");
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

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.resources")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.resources",
                        "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.resources_object",
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.resources_object")
            };
            if _cond {
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.resources");
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date") {
                    match parse_date_out(&date_str, &["M/d/yyyy h:mm:ss a"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_data_signature_update_date")?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.data.signature_update_date");
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
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_software_details_fix_reference_release_date")?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fix_reference.release_date");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.patchable") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.patchable") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.patchable".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.patchable", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_software_details_patchable_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.patchable");
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

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def cvss = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss; if (cvss.containsKey('2.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2 = cvss.get('2.0');\n} if (cvss.containsKey('3.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3 = cvss.get('3.0');\n} if (cvss.containsKey('4.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4 = cvss.get('4.0');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def cvss = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss; if (cvss.containsKey('2.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2 = cvss.get('2.0');\n} if (cvss.containsKey('3.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3 = cvss.get('3.0');\n} if (cvss.containsKey('4.0')) {\n  ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4 = cvss.get('4.0');\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_parse_vulnerability_details_cvss",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_cvss_v2_base_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_cvss_v3_base_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_cvss_v4_base_to_float")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_in_exploit_kit") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_in_exploit_kit") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_in_exploit_kit".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_in_exploit_kit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_exploitability_assessment_is_in_exploit_kit_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_in_exploit_kit");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_publicly_disclosed") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_publicly_disclosed") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_publicly_disclosed".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_publicly_disclosed", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_exploitability_assessment_is_publicly_disclosed_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_publicly_disclosed");
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
                if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_verified") {
                if let Some(val) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_verified") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_verified".into(),
                            message,
                        })?;
                    event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_verified", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_additional_data_sub_assessment_additional_data_vulnerability_details_exploitability_assessment_is_verified_to_boolean")?;
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.exploitability_assessment.is_verified");
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

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_vulnerability_details_last_modified_date")?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.last_modified_date");
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
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_additional_data_sub_assessment_additional_data_vulnerability_details_published_date")?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date");
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
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated".into(),
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
                        "date_additional_data_sub_assessment_time_generated",
                    )?;
                    event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.time_generated");
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
                event.has_value("microsoft_defender_cloud.assessment.status.first_evaluation_date")
                    && event
                        .get_str("microsoft_defender_cloud.assessment.status.first_evaluation_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "microsoft_defender_cloud.assessment.status.first_evaluation_date",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_defender_cloud.assessment.status.first_evaluation_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.status.first_evaluation_date".into(),
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
                        "date_status_first_evaluation_date",
                    )?;
                    event
                        .remove("microsoft_defender_cloud.assessment.status.first_evaluation_date");
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
                event.has_value("microsoft_defender_cloud.assessment.status.status_change_date")
                    && event
                        .get_str("microsoft_defender_cloud.assessment.status.status_change_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "microsoft_defender_cloud.assessment.status.status_change_date",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_defender_cloud.assessment.status.status_change_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "microsoft_defender_cloud.assessment.status.status_change_date".into(),
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
                        "date_status_status_change_date",
                    )?;
                    event.remove("microsoft_defender_cloud.assessment.status.status_change_date");
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
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.metadata.scanner") == Some("MicrosoftDefenderVulnerabilityManagement") || ["Agentless Microsoft Defender vulnerability management", "Microsoft Defender vulnerability management"].contains(&event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.source").unwrap_or("")) || event.has_value("microsoft_defender_cloud.assessment.additional_data.cves_count")
            };
            if _cond {
                event.set(
                    "microsoft_defender_cloud.assessment.class",
                    json!("vulnerability"),
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.metadata.scanner") != Some("MicrosoftDefenderVulnerabilityManagement") && !(["Agentless Microsoft Defender vulnerability management", "Microsoft Defender vulnerability management"].contains(&event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.source").unwrap_or(""))) && !event.has_value("microsoft_defender_cloud.assessment.additional_data.cves_count")
            };
            if _cond {
                event.set(
                    "microsoft_defender_cloud.assessment.class",
                    json!("misconfiguration"),
                )?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            let _cond = { !event.has_value("message") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                event.set("event.kind", json!("state"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                event.append("event.category", json!("vulnerability"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { !event.has_value("event.id") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def status_change_date = ctx.microsoft_defender_cloud?.assessment?.status?.status_change_date; def first_evaluation_date = ctx.microsoft_defender_cloud?.assessment?.status?.first_evaluation_date; if (status_change_date != null && first_evaluation_date != null) {\n  ZonedDateTime date1 = ZonedDateTime.parse(status_change_date);\n  ZonedDateTime date2 = ZonedDateTime.parse(first_evaluation_date);\n\n  if (date1.isAfter(date2)) {\n    ctx.event.created = status_change_date;\n  } else {\n    ctx.event.created = first_evaluation_date;\n  }\n} else if (first_evaluation_date != null) {\n  ctx.event.created = first_evaluation_date;\n} else if (status_change_date != null) {\n  ctx.event.created = status_change_date;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def status_change_date = ctx.microsoft_defender_cloud?.assessment?.status?.status_change_date; def first_evaluation_date = ctx.microsoft_defender_cloud?.assessment?.status?.first_evaluation_date; if (status_change_date != null && first_evaluation_date != null) {\n  ZonedDateTime date1 = ZonedDateTime.parse(status_change_date);\n  ZonedDateTime date2 = ZonedDateTime.parse(first_evaluation_date);\n\n  if (date1.isAfter(date2)) {\n    ctx.event.created = status_change_date;\n  } else {\n    ctx.event.created = first_evaluation_date;\n  }\n} else if (first_evaluation_date != null) {\n  ctx.event.created = first_evaluation_date;\n} else if (status_change_date != null) {\n  ctx.event.created = status_change_date;\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_parse_event_created",
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

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.code") == Some("Healthy")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.code") == Some("Unhealthy")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.code") == Some("NotApplicable")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.get_str("microsoft_defender_cloud.assessment.status.code")
                        == Some("Healthy")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.get_str("microsoft_defender_cloud.assessment.status.code")
                        == Some("Unhealthy")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.get_str("microsoft_defender_cloud.assessment.status.code")
                        == Some("NotApplicable")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            let _cond = { !event.has_value("event.reason") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.status.description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reason", v)?;
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.severity")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def severity = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.severity;\nif (severity == 'Low') {\n  ctx.event.severity = 21;\n} else if (severity == 'Medium') {\n  ctx.event.severity = 47;\n} else if (severity == 'High') {\n  ctx.event.severity = 73;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def severity = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.severity;\nif (severity == 'Low') {\n  ctx.event.severity = 21;\n} else if (severity == 'Medium') {\n  ctx.event.severity = 47;\n} else if (severity == 'High') {\n  ctx.event.severity = 73;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_severity",
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

            event.set("observer.vendor", json!("Microsoft Defender for Cloud"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "microsoft_defender_cloud.assessment.resource_details.native_resource_id",
                ) {
                    if let Some(input) = event.get_string(
                        "microsoft_defender_cloud.assessment.resource_details.native_resource_id",
                    ) {
                        // Grok pattern: ^/subscriptions/%{DATA:cloud.account.id}/%{GREEDYDATA}$
                        // Grok pattern: ^%{GREEDYDATA}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^/subscriptions/%{DATA:cloud.account.id}/%{GREEDYDATA}$"
                                ),
                                cached_grok!("^%{GREEDYDATA}$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "grok_to_extract_cloud_account_id",
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

            let _cond = { !event.has_value("cloud.account.id") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("microsoft_defender_cloud.assessment.resource_details.id") {
                        if let Some(input) = event
                            .get_string("microsoft_defender_cloud.assessment.resource_details.id")
                        {
                            // Grok pattern: ^/subscriptions/%{DATA:cloud.account.id}/%{GREEDYDATA}$
                            // Grok pattern: ^%{GREEDYDATA}$
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^/subscriptions/%{DATA:cloud.account.id}/%{GREEDYDATA}$"
                                    ),
                                    cached_grok!("^%{GREEDYDATA}$"),
                                ],
                                &input,
                                event,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_to_extract_cloud_account_id",
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

            if let Some(v) = event
                .get("microsoft_defender_cloud.assessment.resource_details.native_resource_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            let _cond = { !event.has_value("cloud.instance.id") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            if let Some(v) = event
                .get("microsoft_defender_cloud.assessment.resource_details.resource_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.name", v)?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_provider").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("cloud.service.name", v)?;
            }

            let _cond = { !event.has_value("cloud.service.name") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.resource_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.service.name", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.resource_details.source")
                    == Some("Azure")
            };
            if _cond {
                event.set("cloud.provider", json!("azure"))?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.container_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.name", v)?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.repository_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.image.name", v)?;
            }

            let _cond = { !event.has_value("container.image.name") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.repo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.image.name", v)?;
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest")
            };
            if _cond {
                event.append_unique("container.image.hash.all", json!(event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                !event.has_value("container.image.hash.all")
                    && event.has_value("microsoft_defender_cloud.assessment.additional_data.digest")
            };
            if _cond {
                event.append_unique(
                    "container.image.hash.all",
                    json!(
                        event
                            .get("microsoft_defender_cloud.assessment.additional_data.digest")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_type").is_some_and(|s| s.to_lowercase() == "virtualmachines")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }
            }

            let _cond = {
                !event.has_value("host.name")
                    && event
                        .get_str(
                            "microsoft_defender_cloud.assessment.resource_details.resource_type",
                        )
                        .is_some_and(|s| s.to_lowercase() == "virtualmachines")
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.resource_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.family", v)?;
            }

            let _cond = { !event.has_value("host.os.family") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.os_distribution")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.family", v)?;
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform").is_some_and(|s| s.to_lowercase().contains("linux"))
            };
            if _cond {
                event.set("host.os.platform", json!("linux"))?;
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform").is_some_and(|s| s.to_lowercase().contains("windows"))
            };
            if _cond {
                event.set("host.os.platform", json!("windows"))?;
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform").is_some_and(|s| s.to_lowercase().contains("mac"))
            };
            if _cond {
                event.set("host.os.platform", json!("darwin"))?;
            }

            let _cond = {
                !event.has_value("host.os.platform")
                    && event
                        .has_value("microsoft_defender_cloud.assessment.additional_data.os_type")
                    && event
                        .get_str("microsoft_defender_cloud.assessment.additional_data.os_type")
                        .is_some_and(|s| s.to_lowercase().contains("linux"))
            };
            if _cond {
                event.set("host.os.platform", json!("linux"))?;
            }

            let _cond = {
                !event.has_value("host.os.platform")
                    && event
                        .has_value("microsoft_defender_cloud.assessment.additional_data.os_type")
                    && event
                        .get_str("microsoft_defender_cloud.assessment.additional_data.os_type")
                        .is_some_and(|s| s.to_lowercase().contains("windows"))
            };
            if _cond {
                event.set("host.os.platform", json!("windows"))?;
            }

            let _cond = {
                !event.has_value("host.os.platform")
                    && event
                        .has_value("microsoft_defender_cloud.assessment.additional_data.os_type")
                    && event
                        .get_str("microsoft_defender_cloud.assessment.additional_data.os_type")
                        .is_some_and(|s| s.to_lowercase().contains("mac"))
            };
            if _cond {
                event.set("host.os.platform", json!("darwin"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: String os_platform;\nif (ctx.microsoft_defender_cloud?.assessment?.additional_data?.sub_assessment?.additional_data?.software_details?.os_details?.os_platform != null) {\n  os_platform = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform.toLowerCase();\n} else if (ctx.microsoft_defender_cloud?.assessment?.additional_data?.os_type != null) {\n  os_platform = ctx.microsoft_defender_cloud.assessment.additional_data.os_type.toLowerCase();\n} else {\n  return;\n}\nfor (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    if (os == 'mac') {\n      ctx.host.os.put('type', 'macos');\n    } else {\n      ctx.host.os.put('type', os);\n    }\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String os_platform;\nif (ctx.microsoft_defender_cloud?.assessment?.additional_data?.sub_assessment?.additional_data?.software_details?.os_details?.os_platform != null) {\n  os_platform = ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_platform.toLowerCase();\n} else if (ctx.microsoft_defender_cloud?.assessment?.additional_data?.os_type != null) {\n  os_platform = ctx.microsoft_defender_cloud.assessment.additional_data.os_type.toLowerCase();\n} else {\n  return;\n}\nfor (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    if (os == 'mac') {\n      ctx.host.os.put('type', 'macos');\n    } else {\n      ctx.host.os.put('type', os);\n    }\n    return;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"mac\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_map_host_os_type",
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

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.controller_kind").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.resource.parent.type", v)?;
            }

            let _cond = { !event.has_value("orchestrator.resource.parent.type") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.controller_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.parent.type", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.pod_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.resource.name", v)?;
            }

            let _cond = { !event.has_value("orchestrator.resource.name") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.pod_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.name", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.namespace").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.namespace", v)?;
            }

            let _cond = { !event.has_value("orchestrator.namespace") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.namespace")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.namespace", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.cluster_resource_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.cluster.id", v)?;
            }

            let _cond = { !event.has_value("orchestrator.cluster.id") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cluster_details.cluster_resource_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.cluster.id", v)?;
            }
            }

            let _cond = { !event.has_value("orchestrator.cluster.id") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.k8s_cluster_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.cluster.id", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.cluster_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.cluster.name", v)?;
            }

            let _cond = { !event.has_value("orchestrator.cluster.name") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cluster_details.cluster_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.cluster.name", v)?;
            }
            }

            let _cond = { !event.has_value("orchestrator.cluster.name") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.k8s_cluster_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.cluster.name", v)?;
                }
            }

            let _cond = {
                event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details") || event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_context") || event.has_value("microsoft_defender_cloud.assessment.additional_data.k8s_cluster_id")
            };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.package_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.name", v)?;
            }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.version", v)?;
            }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fixed_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.fixed_version", v)?;
            }
            }

            let _cond = {
                !event.has_value("package.name")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.name", v)?;
            }
            }

            let _cond = { !event.has_value("package.version") && event.has_value("package.name") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.version", v)?;
            }
            }

            let _cond =
                { !event.has_value("package.fixed_version") && event.has_value("package.name") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.recommended_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("package.fixed_version", v)?;
            }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.native_resource_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.id", v)?;
            }

            let _cond = { !event.has_value("resource.id") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.id", v)?;
            }
            }

            let _cond = { !event.has_value("resource.id") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.native_resource_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("resource.id", v)?;
                }
            }

            let _cond = { !event.has_value("resource.id") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("resource.id", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.name", v)?;
            }

            let _cond = { !event.has_value("resource.name") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.resource_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("resource.name", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_provider").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.type", v)?;
            }

            let _cond = { !event.has_value("resource.type") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.resource_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("resource.type", v)?;
                }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.sub_type", v)?;
            }

            let _cond = { !event.has_value("resource.sub_type") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.resource_details.resource_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("resource.sub_type", v)?;
                }
            }

            let _cond = {
                event.get_str("event.outcome") == Some("success")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("misconfiguration")
            };
            if _cond {
                event.set("result.evaluation", json!("passed"))?;
            }

            let _cond = {
                event.get_str("event.outcome") == Some("failure")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("misconfiguration")
            };
            if _cond {
                event.set("result.evaluation", json!("failed"))?;
            }

            let _cond = {
                event.get_str("event.outcome") == Some("unknown")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("misconfiguration")
            };
            if _cond {
                event.set("result.evaluation", json!("unknown"))?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.name", v)?;
            }
            }

            let _cond = {
                !event.has_value("rule.name")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("rule.name") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("rule.uuid", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.description", v)?;
            }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event
                    .get(
                        "microsoft_defender_cloud.assessment.additional_data.sub_assessment.impact",
                    )
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.impact", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.remediation").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.remediation", v)?;
            }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class")
                    == Some("misconfiguration")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.category").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.category", v)?;
            }
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.base", v)?;
            }

            let _cond = {
                !event.has_value("vulnerability.score.version")
                    && event.has_value("vulnerability.score.base")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("4.0"))?;
            }

            let _cond = { !event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v3.base").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.base", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.score.version")
                    && event.has_value("vulnerability.score.base")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("3.0"))?;
            }

            let _cond = { !event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v2.base").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.base", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.score.version")
                    && event.has_value("vulnerability.score.base")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("2.0"))?;
            }

            let _cond = { !event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_score").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.base", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.score.version")
                    && event.has_value("vulnerability.score.base")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.cvss_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.version", v)?;
            }
            }

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array()) && event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: double max_score = 0.0;\nString severity, version;\nfor (def cve: ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list) {\n  if (cve.cvss_score != null) {\n    if (max_score < cve.cvss_score) {\n      max_score = cve.cvss_score;\n      severity = cve.severity;\n      version = cve.cvss_version;\n    }\n  }\n}\nctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.score = ctx.vulnerability.score ?: [:];\nctx.vulnerability.score.base = max_score;\nctx.vulnerability.severity = severity;\nctx.vulnerability.score.version = version;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"double max_score = 0.0;\nString severity, version;\nfor (def cve: ctx.microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list) {\n  if (cve.cvss_score != null) {\n    if (max_score < cve.cvss_score) {\n      max_score = cve.cvss_score;\n      severity = cve.severity;\n      version = cve.cvss_version;\n    }\n  }\n}\nctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.score = ctx.vulnerability.score ?: [:];\nctx.vulnerability.score.base = max_score;\nctx.vulnerability.severity = severity;\nctx.vulnerability.score.version = version;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_map_vulnerability_score_base_vulnerability_score_version_vulnerability_severity")?;
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
                event.get("microsoft_defender_cloud.assessment.additional_data.cves").is_some_and(|v| v.is_array()) && event.get("microsoft_defender_cloud.assessment.additional_data.cves").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: double max_score = 0.0;\nString severity;\nfor (def cve: ctx.microsoft_defender_cloud.assessment.additional_data.cves) {\n  if (cve.base_score != null) {\n    if (max_score < cve.base_score) {\n      max_score = cve.base_score;\n      severity = cve.severity;\n    }\n  }\n}\nctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.score = ctx.vulnerability.score ?: [:];\nctx.vulnerability.score.base = max_score;\nctx.vulnerability.severity = severity;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"double max_score = 0.0;\nString severity;\nfor (def cve: ctx.microsoft_defender_cloud.assessment.additional_data.cves) {\n  if (cve.base_score != null) {\n    if (max_score < cve.base_score) {\n      max_score = cve.base_score;\n      severity = cve.severity;\n    }\n  }\n}\nctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.score = ctx.vulnerability.score ?: [:];\nctx.vulnerability.score.base = max_score;\nctx.vulnerability.severity = severity;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_vulnerability_score_base_vulnerability_severity",
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

            let _cond = { !event.has_value("vulnerability.severity") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.severity", v)?;
            }
            }

            let _cond = { !event.has_value("vulnerability.severity") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.severity", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.severity")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("vulnerability")
            };
            if _cond {
                event.set("vulnerability.severity", json!("Unknown"))?;
            }

            let _cond = { event.has_value("vulnerability.score.base") };
            if _cond {
                event.set("vulnerability.classification", json!("CVSS"))?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.description", v)?;
            }

            let _cond = {
                !event.has_value("vulnerability.description")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.description", v)?;
            }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.event_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.id", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.id")
                    && event
                        .get("microsoft_defender_cloud.assessment.additional_data.cves")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_defender_cloud.assessment.additional_data.cves",
                    |event| {
                        event.append_unique(
                            "vulnerability.id",
                            json!(
                                event
                                    .get("_ingest._value.cve")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.published_date", v)?;
            }

            let _cond = { !event.has_value("vulnerability.published_date") };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.published_date").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.published_date", v)?;
            }
            }

            let _cond = {
                event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.references").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.references",
                    |event| {
                        event.append_unique(
                            "vulnerability.reference",
                            json!(
                                event
                                    .get("_ingest._value.link")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.class") == Some("vulnerability")
            };
            if _cond {
                event.set(
                    "vulnerability.scanner.vendor",
                    json!("Microsoft Defender Vulnerability Management"),
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cve_id").is_some_and(|s| s.starts_with("CVE"))
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cve_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.cve", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.cve") && event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.title").is_some_and(|s| s.starts_with("CVE"))
            };
            if _cond {
                if let Some(v) = event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.title").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.cve", v)?;
            }
            }

            let _cond = {
                !event.has_value("vulnerability.cve") && event.get("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve_list",
                    |event| {
                        event.append_unique(
                            "vulnerability.cve",
                            json!(
                                event
                                    .get("_ingest._value.title")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                !event.has_value("vulnerability.cve")
                    && event
                        .get("microsoft_defender_cloud.assessment.additional_data.cves")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_defender_cloud.assessment.additional_data.cves",
                    |event| {
                        event.append_unique(
                            "vulnerability.cve",
                            json!(
                                event
                                    .get("_ingest._value.cve")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("vulnerability.cve") };
            if _cond {
                event.set("vulnerability.enumeration", json!("CVE"))?;
            }

            let _cond = {
                event.has_value("package.name")
                    && event.has_value("package.version")
                    && event
                        .get("vulnerability.cve")
                        .is_some_and(|v| v.is_string())
                    && !event.has_value("vulnerability.title")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {} {} - {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("vulnerability.cve")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("package.name")
                    && event
                        .get("vulnerability.cve")
                        .is_some_and(|v| v.is_string())
                    && !event.has_value("vulnerability.title")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {} - {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("vulnerability.cve")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("package.name")
                    && event.has_value("package.version")
                    && !event.has_value("vulnerability.title")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("vulnerability")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {} {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event
                    .get("vulnerability.cve")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("vulnerability.title")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found - {}",
                        event
                            .get("vulnerability.cve")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("package.name")
                    && !event.has_value("vulnerability.title")
                    && event.get_str("microsoft_defender_cloud.assessment.class")
                        == Some("vulnerability")
            };
            if _cond {
                event.set(
                    "vulnerability.title",
                    json!(format!(
                        "Vulnerability found in {}",
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.resource_type")
                    == Some("User")
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.resource_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.resource_type")
                    == Some("User")
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_defender_cloud.assessment.additional_data.resource_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.digest")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("microsoft_defender_cloud.assessment.additional_data.digest")
                    {
                        if let Some(input) = event.get_string(
                            "microsoft_defender_cloud.assessment.additional_data.digest",
                        ) {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("sha256:") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp.digest", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path:
                                        "microsoft_defender_cloud.assessment.additional_data.digest"
                                            .into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_to_extract_digest_hash",
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

            let _cond = { event.has_value("_temp.digest") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_temp.digest")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest") {
                if let Some(input) = event.get_string("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("sha256:") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("_temp.artifact_digest", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_to_extract_artifact_digest_hash",
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

            let _cond = { event.has_value("_temp.artifact_digest") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("_temp.artifact_digest")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");
            event.remove("_temp");
            event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss");
            event.remove("microsoft_defender_cloud.assessment.additional_data.nsg open ports");
            event
                .remove("microsoft_defender_cloud.assessment.additional_data.can onboard to _byol");

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
                event.remove(
                    "microsoft_defender_cloud.assessment.additional_data.sub_assessment.id",
                );
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.status.description");
                event.remove(
                    "microsoft_defender_cloud.assessment.resource_details.native_resource_id",
                );
                event.remove("microsoft_defender_cloud.assessment.resource_details.resource_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.native_resource_id");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_provider");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.resource_details.resource_type");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.container_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.controller_kind");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.pod_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.namespace");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.cluster_resource_id");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.kubernetes_details.cluster_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.repository_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.artifact_details.digest");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.os_details.os_version");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.package_name");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.version");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.software_details.fixed_version");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.cvss_v4.base");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.vulnerability_details.published_date");
                event.remove("microsoft_defender_cloud.assessment.additional_data.sub_assessment.additional_data.cve.description");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
